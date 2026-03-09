use std::ptr;

// Constants from the C++ code
const P_HG_MAN: u32 = 0x137C580;
const F_ADD_DRAW_PLAN_BEGIN_TARGET_SCENE: u32 = 0x4070F0;
const F_ADD_DRAW_PLAN_END_TARGET_SCENE: u32 = 0x407180;
const F_ADD_DRAW_PLAN_END_TARGET_CAMERA: u32 = 0x407080;

// Hook addresses
const HOOK_ADDRESS_1: u32 = 0xEB3798;
const HOOK_ADDRESS_2: u32 = 0xEB437D;
const HOOK_ADDRESS_3: u32 = 0xEB4483;

/// Hook function 1: BeginTargetSceneEzDrawDepth
/// Manual naked function hook that writes directly to game memory
#[cfg(target_arch = "x86")]
#[unsafe(naked)]
unsafe extern "C" fn hook_begin_target_scene_ez_draw_depth_naked() {
    use std::arch::naked_asm;
    
    naked_asm!(
        // Get parameter - load address into ECX, then dereference
        "mov ecx, 0x137C580",
        "mov ecx, [ecx]",
        "mov ecx, [ecx+0x50]",
        "mov eax, [ecx+4]",
        
        // Call BeginTargetScene
        "push eax",
        "mov ecx, esi",
        "mov eax, ebp",
        "mov edx, 0x4070F0",
        "call edx",
        "add esp, 4",
        
        // Call EndTargetScene
        "mov ecx, esi",
        "mov eax, ebp",
        "mov edx, 0x407180",
        "call edx",
        
        // Jump back
        "mov ecx, esi",
        "mov eax, ebp",
        "mov edx, 0xEB379D",
        "jmp edx",
    );
}

/// Hook function 2: BeginTargetSceneEzDrawNormal1
#[cfg(target_arch = "x86")]
#[unsafe(naked)]
unsafe extern "C" fn hook_begin_target_scene_ez_draw_normal1_naked() {
    use std::arch::naked_asm;
    
    naked_asm!(
        // Get parameter
        "mov ecx, 0x137C580",
        "mov ecx, [ecx]",
        "mov ecx, [ecx+0x50]",
        "mov eax, [ecx+4]",
        
        // Call BeginTargetScene
        "push eax",
        "mov ecx, esi",
        "mov eax, ebx",
        "mov edx, 0x4070F0",
        "call edx",
        "add esp, 4",
        
        // Call EndTargetScene
        "mov ecx, esi",
        "mov eax, ebx",
        "mov edx, 0x407180",
        "call edx",
        
        // Call EndTargetCamera
        "mov ecx, esi",
        "mov eax, ebx",
        "mov edx, 0x407080",
        "call edx",
        
        // Jump back (0xEB437D + 9 = 0xEB4386)
        "mov edx, 0xEB4386",
        "jmp edx",
    );
}

/// Hook function 3: BeginTargetSceneEzDrawNormal2
#[cfg(target_arch = "x86")]
#[unsafe(naked)]
unsafe extern "C" fn hook_begin_target_scene_ez_draw_normal2_naked() {
    use std::arch::naked_asm;
    
    naked_asm!(
        // Get parameter
        "mov ecx, 0x137C580",
        "mov ecx, [ecx]",
        "mov ecx, [ecx+0x50]",
        "mov eax, [ecx+4]",
        
        // Call BeginTargetScene
        "push eax",
        "mov ecx, esi",
        "mov eax, ebx",
        "mov edx, 0x4070F0",
        "call edx",
        "add esp, 4",
        
        // Call EndTargetScene
        "mov ecx, esi",
        "mov eax, ebx",
        "mov edx, 0x407180",
        "call edx",
        
        // Call EndTargetCamera
        "mov ecx, esi",
        "mov eax, ebx",
        "mov edx, 0x407080",
        "call edx",
        
        // Jump back (0xEB4483 + 9 = 0xEB448C)
        "mov edx, 0xEB448C",
        "jmp edx",
    );
}

/// Write a JMP instruction to the target address
unsafe fn write_jmp(target_addr: usize, dest_addr: usize) -> Result<(), String> {
    use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};
    
    // Calculate relative offset for JMP instruction
    // JMP instruction is: E9 [4-byte offset]
    // Offset = destination - (source + 5)
    let offset = (dest_addr as i32).wrapping_sub((target_addr + 5) as i32);
    
    // Prepare the 5-byte JMP instruction
    let mut jmp_bytes = [0u8; 5];
    jmp_bytes[0] = 0xE9;  // JMP opcode
    jmp_bytes[1..5].copy_from_slice(&offset.to_le_bytes());
    
    // Make memory writable
    let mut old_protect: u32 = 0;
    if VirtualProtect(
        target_addr as *const _,
        5,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect
    ) == 0 {
        return Err("VirtualProtect failed to make memory writable".to_string());
    }
    
    // Write the JMP instruction
    std::ptr::copy_nonoverlapping(jmp_bytes.as_ptr(), target_addr as *mut u8, 5);
    
    // Restore original protection
    let mut temp = 0;
    VirtualProtect(
        target_addr as *const _,
        5,
        old_protect,
        &mut temp
    );
    
    Ok(())
}

// Old ilhook-based hook functions removed - now using manual naked functions

/*
/// Install a hook by writing a jump instruction to the target address
/// and patching the return jump in the hook function
/// 
/// # Arguments
/// * `to_hook` - Address to hook
/// * `our_func` - Address of our hook function
/// * `len` - Length of bytes to overwrite (must be >= 5)
/// * `hook_size` - Approximate size of the hook function to search for the return JMP
/// 
/// # Returns
/// `Ok(())` on success, `Err` on failure
/// 
/// NOTE: This function is no longer used - we now use ilhook library instead
unsafe fn install_hook(to_hook: u32, our_func: *const (), len: usize, hook_size: usize) -> Result<(), String> {
    if len < 5 {
        let err = "Hook length must be at least 5 bytes".to_string();
        log_to_file(&format!("ERROR: {}", err));
        return Err(err);
    }

    use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};
    use std::ptr;

    log_to_file(&format!("  Hook function at: 0x{:X}", our_func as u32));
    
    // Patch the return JMP in our hook function to jump directly past the hooked code
    let hook_func_ptr = our_func as *mut u8;
    let mut old_protect: u32 = 0;
    
    // Make hook function writable
    log_to_file("  Making hook function writable...");
    if VirtualProtect(
        hook_func_ptr as *mut _,
        hook_size,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    ) == 0
    {
        let err = "Failed to change hook function memory protection".to_string();
        log_to_file(&format!("ERROR: {}", err));
        return Err(err);
    }
    
    // Find the placeholder JMP (E9 00 00 00 00) at the end of the hook
    let mut found_jmp = false;
    log_to_file("  Searching for placeholder JMP...");
    for i in 0..hook_size {
        let ptr = hook_func_ptr.add(i);
        if *ptr == 0xE9 && 
           *ptr.add(1) == 0x00 && 
           *ptr.add(2) == 0x00 && 
           *ptr.add(3) == 0x00 && 
           *ptr.add(4) == 0x00 {
            // Jump directly to the instruction after our hook point (skipping original code)
            let jmp_back_address = to_hook + len as u32;
            let jmp_instruction_addr = our_func as u32 + i as u32;
            let relative_offset = jmp_back_address.wrapping_sub(jmp_instruction_addr).wrapping_sub(5);
            
            log_to_file(&format!("  Found JMP at offset {}, jumping directly to 0x{:X} (offset 0x{:X})", 
                                i, jmp_back_address, relative_offset));
            
            *(ptr.add(1) as *mut u32) = relative_offset;
            
            found_jmp = true;
            break;
        }
    }
    
    if !found_jmp {
/// Apply all memory patches for debug draw functionality
#[cfg(target_arch = "x86")]
pub unsafe fn apply_debug_draw_patches() -> Result<(), String> {
    use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

    #[cfg(target_os = "windows")]
    {
        // Hook 1 - BeginTargetScene at 0xEB3798 (5 bytes)
        
        let naked_fn_addr = hook_begin_target_scene_ez_draw_depth_naked as usize;
        let offset = (naked_fn_addr as i32) - (HOOK_ADDRESS_1 as i32) - 5;
        
        let mut jmp_bytes = [0u8; 5];
        jmp_bytes[0] = 0xE9;  // JMP opcode
        jmp_bytes[1..5].copy_from_slice(&offset.to_le_bytes());
        
        let mut old_protect: u32 = 0;
        if VirtualProtect(
            HOOK_ADDRESS_1 as *const _,
            5,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect
        ) == 0 {
            return Err("VirtualProtect failed for hook 1".to_string());
        }
        
        std::ptr::copy_nonoverlapping(jmp_bytes.as_ptr(), HOOK_ADDRESS_1 as *mut u8, 5);
        
        let mut temp = 0;
        VirtualProtect(
            HOOK_ADDRESS_1 as *const _,
            5,
            old_protect,
            &mut temp
        );
        
        // Hook 2 - BeginTargetScene at 0xEB437D (9 bytes)
        
        let naked_fn_addr = hook_begin_target_scene_ez_draw_normal1_naked as usize;
        let offset = (naked_fn_addr as i32) - (HOOK_ADDRESS_2 as i32) - 5;
        
        let mut jmp_bytes = [0u8; 9];
        jmp_bytes[0] = 0xE9;  // JMP opcode
        jmp_bytes[1..5].copy_from_slice(&offset.to_le_bytes());
        // Fill remaining 4 bytes with NOPs
        jmp_bytes[5..9].fill(0x90);
        
        let mut old_protect: u32 = 0;
        if VirtualProtect(
            HOOK_ADDRESS_2 as *const _,
            9,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect
        ) == 0 {
            return Err("VirtualProtect failed for hook 2".to_string());
        }
        
        std::ptr::copy_nonoverlapping(jmp_bytes.as_ptr(), HOOK_ADDRESS_2 as *mut u8, 9);
        
        let mut temp = 0;
        VirtualProtect(
            HOOK_ADDRESS_2 as *const _,
            9,
            old_protect,
            &mut temp
        );
        
        // Hook 3 - BeginTargetScene at 0xEB4483 (9 bytes)
        
        let naked_fn_addr = hook_begin_target_scene_ez_draw_normal2_naked as usize;
        let offset = (naked_fn_addr as i32) - (HOOK_ADDRESS_3 as i32) - 5;
        
        let mut jmp_bytes = [0u8; 9];
        jmp_bytes[0] = 0xE9;  // JMP opcode
        jmp_bytes[1..5].copy_from_slice(&offset.to_le_bytes());
        // Fill remaining 4 bytes with NOPs
        jmp_bytes[5..9].fill(0x90);
        
        let mut old_protect: u32 = 0;
        if VirtualProtect(
            HOOK_ADDRESS_3 as *const _,
            9,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect
        ) == 0 {
            return Err("VirtualProtect failed for hook 3".to_string());
        }
        
        std::ptr::copy_nonoverlapping(jmp_bytes.as_ptr(), HOOK_ADDRESS_3 as *mut u8, 9);
        
        let mut temp = 0;
        VirtualProtect(
            HOOK_ADDRESS_3 as *const _,
            9,
            old_protect,
            &mut temp
        );
    }
    
    // NOP out memory at 0xFD6529 (19 bytes)
    log_to_file("Patching 0xFD6529...");
    unsafe {
        let addr_1 = 0xFD6529 as *mut u8;
        let mut old_protect: u32 = 0;
        if VirtualProtect(addr_1 as *mut _, 19, PAGE_EXECUTE_READWRITE, &mut old_protect) == 0 {
            let err = format!("Failed to change protection at 0xFD6529");
            log_to_file(&err);
            return Err(err);
        }
        ptr::write_bytes(addr_1, 0x90, 19);
        let mut temp: u32 = 0;
        VirtualProtect(addr_1 as *mut _, 19, old_protect, &mut temp);
        log_to_file("0xFD6529 patched successfully");
    }

    // NOP out memory at 0xFD6546 (3 bytes)
    log_to_file("Patching 0xFD6546...");
    unsafe {
        let addr_2 = 0xFD6546 as *mut u8;
        let mut old_protect: u32 = 0;
        if VirtualProtect(addr_2 as *mut _, 3, PAGE_EXECUTE_READWRITE, &mut old_protect) == 0 {
            let err = format!("Failed to change protection at 0xFD6546");
            log_to_file(&err);
            return Err(err);
        }
        ptr::write_bytes(addr_2, 0x90, 3);
        VirtualProtect(addr_2 as *mut _, 3, old_protect, &mut old_protect);
        log_to_file("0xFD6546 patched successfully");
    }

    // Patch byte at 0xC03F1B to 0x50
    log_to_file("Patching 0xC03F1B...");
    unsafe {
        let addr_3 = 0xC03F1B as *mut u8;
        let mut old_protect: u32 = 0;
        if VirtualProtect(addr_3 as *mut _, 1, PAGE_EXECUTE_READWRITE, &mut old_protect) == 0 {
            let err = format!("Failed to change protection at 0xC03F1B");
            log_to_file(&err);
            return Err(err);
        }
        *addr_3 = 0x50;
        VirtualProtect(addr_3 as *mut _, 1, old_protect, &mut old_protect);
        log_to_file("0xC03F1B patched successfully");
    }

    // Patch byte at 0xC03180 to 0x50
    log_to_file("Patching 0xC03180...");
    unsafe {
        let addr_4 = 0xC03180 as *mut u8;
        let mut old_protect: u32 = 0;
        if VirtualProtect(addr_4 as *mut _, 1, PAGE_EXECUTE_READWRITE, &mut old_protect) == 0 {
            let err = format!("Failed to change protection at 0xC03180");
            log_to_file(&err);
            return Err(err);
        }
        *addr_4 = 0x50;
        VirtualProtect(addr_4 as *mut _, 1, old_protect, &mut old_protect);
        log_to_file("0xC03180 patched successfully");
    }

    // Patch int at 0xC064E4 to 0x140
    log_to_file("Patching 0xC064E4...");
    unsafe {
        let addr_5 = 0xC064E4 as *mut i32;
        let mut old_protect: u32 = 0;
        if VirtualProtect(addr_5 as *mut _, 4, PAGE_EXECUTE_READWRITE, &mut old_protect) == 0 {
            let err = format!("Failed to change protection at 0xC064E4");
            log_to_file(&err);
            return Err(err);
        }
        *addr_5 = 0x140;
        VirtualProtect(addr_5 as *mut _, 4, old_protect, &mut old_protect);
        log_to_file("0xC064E4 patched successfully");
    }

    // Patch int at 0xC064EB to 0x140
    log_to_file("Patching 0xC064EB...");
    unsafe {
        let addr_6 = 0xC064EB as *mut i32;
        let mut old_protect: u32 = 0;
        if VirtualProtect(addr_6 as *mut _, 4, PAGE_EXECUTE_READWRITE, &mut old_protect) == 0 {
            let err = format!("Failed to change protection at 0xC064EB");
            log_to_file(&err);
            return Err(err);
        }
        *addr_6 = 0x140;
        VirtualProtect(addr_6 as *mut _, 4, old_protect, &mut old_protect);
        log_to_file("0xC064EB patched successfully");
    }

    log_to_file("=== All debug draw patches applied successfully! ===");
    Ok(())
}

/// Stub for non-x86 architectures
#[cfg(not(target_arch = "x86"))]
pub unsafe fn apply_debug_draw_patches() -> Result<(), String> {
    Err("Debug draw patches are only supported on x86 architecture".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(P_HG_MAN, 0x137C580);
        assert_eq!(F_ADD_DRAW_PLAN_BEGIN_TARGET_SCENE, 0x4070F0);
        assert_eq!(F_ADD_DRAW_PLAN_END_TARGET_SCENE, 0x407180);
        assert_eq!(F_ADD_DRAW_PLAN_END_TARGET_CAMERA, 0x407080);
    }
}
