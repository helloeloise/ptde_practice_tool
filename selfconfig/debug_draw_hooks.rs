use std::ptr;
use std::fs::OpenOptions;
use std::io::Write;

fn log_to_file(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug_draw_hooks.log")
    {
        let _ = writeln!(file, "{}", msg);
        let _ = file.flush(); // Ensure immediate write
    }
}

// Constants from the C++ code
const P_HG_MAN: u32 = 0x137C580;
const F_ADD_DRAW_PLAN_BEGIN_TARGET_SCENE: u32 = 0x4070F0;
const F_ADD_DRAW_PLAN_END_TARGET_SCENE: u32 = 0x407180;
const F_ADD_DRAW_PLAN_END_TARGET_CAMERA: u32 = 0x407080;

// Hook addresses
const HOOK_ADDRESS_1: u32 = 0xEB3798;
const HOOK_ADDRESS_2: u32 = 0xEB437D;
const HOOK_ADDRESS_3: u32 = 0xEB4483;

// Static variables for use in naked_asm (sym requires static symbols)
#[unsafe(no_mangle)]
static P_HG_MAN_PTR: u32 = P_HG_MAN;
#[unsafe(no_mangle)]
static F_BEGIN_TARGET_SCENE_PTR: u32 = F_ADD_DRAW_PLAN_BEGIN_TARGET_SCENE;
#[unsafe(no_mangle)]
static F_END_TARGET_SCENE_PTR: u32 = F_ADD_DRAW_PLAN_END_TARGET_SCENE;
#[unsafe(no_mangle)]
static F_END_TARGET_CAMERA_PTR: u32 = F_ADD_DRAW_PLAN_END_TARGET_CAMERA;

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

/// Callback for ilhook with Retn type
#[cfg(target_arch = "x86")]
unsafe extern "cdecl" fn hook_begin_target_scene_callback(regs: *mut ilhook::x86::Registers, _: usize, ret_addr: usize) -> usize {
    log_to_file("=== HOOK CALLBACK TRIGGERED ===");
    
    let regs_ref = &*regs;
    
    log_to_file(&format!(
        "Hook 1 - Registers: eax=0x{:X}, ecx=0x{:X}, edx=0x{:X}, ebx=0x{:X}, esp=0x{:X}, ebp=0x{:X}, esi=0x{:X}, edi=0x{:X}",
        regs_ref.eax, regs_ref.ecx, regs_ref.edx, regs_ref.ebx, regs_ref.esp, regs_ref.ebp, regs_ref.esi, regs_ref.edi
    ));
    
    let (new_ecx, new_eax) = hook_begin_target_scene_ez_draw_depth_impl(
        regs_ref.eax, regs_ref.ecx, regs_ref.edx, regs_ref.ebx, regs_ref.esp, regs_ref.ebp, regs_ref.esi, regs_ref.edi
    );
    
    // Update the registers that ilhook will restore with the values from the original instructions
    log_to_file(&format!("About to update registers: ecx=0x{:X}, eax=0x{:X}", new_ecx, new_eax));
    (*regs).ecx = new_ecx;
    (*regs).eax = new_eax;
    log_to_file(&format!("Registers updated in struct: ecx=0x{:X}, eax=0x{:X}", (*regs).ecx, (*regs).eax));
    
    log_to_file("=== HOOK CALLBACK COMPLETE ===");
    
    // Return the address to jump to (original return address)
    ret_addr
}

/// The actual implementation that can use normal Rust code and logging
/// Parameters: all registers for debugging
/// Returns: (new_ecx, new_eax) - the register values after executing original instructions
#[cfg(target_arch = "x86")]
unsafe extern "C" fn hook_begin_target_scene_ez_draw_depth_impl(
    eax: u32, ecx: u32, edx: u32, ebx: u32, esp: u32, ebp: u32, esi: u32, edi: u32
) -> (u32, u32) {
    use std::arch::asm;
    
    log_to_file(&format!(
        "Hook 1 - ALL REGS: eax=0x{:X}, ecx=0x{:X}, edx=0x{:X}, ebx=0x{:X}, esp=0x{:X}, ebp=0x{:X}, esi=0x{:X}, edi=0x{:X}",
        eax, ecx, edx, ebx, esp, ebp, esi, edi
    ));
    
    let phg_man = *(P_HG_MAN as *const u32);
    log_to_file(&format!("P_HG_MAN loaded: 0x{:X}", phg_man));
    
    if phg_man == 0 {
        log_to_file("P_HG_MAN is null, returning esi/ebp");
        return (esi, ebp);
    }
    
    let hg_man = *(phg_man as *const u32);
    log_to_file(&format!("hg_man loaded: 0x{:X}", hg_man));
    
    if hg_man == 0 {
        log_to_file("hg_man is null, returning esi/ebp");
        return (esi, ebp);
    }
    
    let hg_man_plus_50 = *((hg_man + 0x50) as *const u32);
    log_to_file(&format!("hg_man+0x50 loaded: 0x{:X}", hg_man_plus_50));
    
    if hg_man_plus_50 == 0 {
        log_to_file("hg_man+0x50 is null, returning esi/ebp");
        return (esi, ebp);
    }
    
    let param = *(hg_man_plus_50 as *const u32);
    log_to_file(&format!("param loaded: 0x{:X}", param));
    
    // C++ does: mov ecx, esi / mov eax, ebp
    log_to_file(&format!("Hook 1 executing: esi=0x{:X}, ebp=0x{:X}, param=0x{:X}", esi, ebp, param));
    
    // Call both BeginTargetScene and EndTargetScene, then jump directly to return address
    // This matches the C++ naked function which ends with JMP, not RET
    log_to_file("Calling BeginTargetScene and EndTargetScene, then jumping directly...");
    
    // Save stack pointer to verify alignment
    let mut stack_before: u32;
    asm!("mov {}, esp", out(reg) stack_before);
    log_to_file(&format!("Stack before calls: 0x{:X} (alignment: {})", stack_before, stack_before % 16));
    
    asm!(
        // BeginTargetScene
        "push {param}",
        "mov ecx, {esi}",
        "mov eax, {ebp}",
        "mov edx, 0x4070F0",
        "call edx",
        "add esp, 4",
        // Reload ECX and EAX like C++ does
        "mov ecx, {esi}",
        "mov eax, {ebp}",
        // EndTargetScene
        "mov edx, 0x407180",
        "call edx",
        // Set final register values and jump directly like C++ naked function
        "mov ecx, {esi}",
        "mov eax, {ebp}",
        "mov edx, 0xEB379D",
        "jmp edx",
        param = in(reg) param,
        esi = in(reg) esi,
        ebp = in(reg) ebp,
        options(noreturn),
    );
    
    (esi, ebp)
}

#[unsafe(no_mangle)]
static mut HOOK_1_RETURN_ADDR: usize = 0xEB379D;  // HOOK_ADDRESS_1 + 5

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
        let err = "Could not find placeholder JMP in hook function".to_string();
        log_to_file(&format!("ERROR: {}", err));
        return Err(err);
    }
    
    // Restore hook function protection
    let mut temp: u32 = 0;
    VirtualProtect(hook_func_ptr as *mut _, hook_size, old_protect, &mut temp);

    // Install the hook at the target address
    let to_hook_ptr_mut = to_hook as *mut u8;

    log_to_file("  Changing target memory protection...");
    if VirtualProtect(
        to_hook_ptr_mut as *mut _,
        len,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    ) == 0
    {
        let err = "Failed to change target memory protection".to_string();
        log_to_file(&format!("ERROR: {}", err));
        return Err(err);
    }

    log_to_file("  Writing NOPs...");
    ptr::write_bytes(to_hook_ptr_mut, 0x90, len);

    // Write JMP to our hook function
    let relative_address = (our_func as u32).wrapping_sub(to_hook).wrapping_sub(5);
    log_to_file(&format!("  Writing JMP with relative address 0x{:X}", relative_address));

    // Write JMP instruction (E9 XX XX XX XX)
    *to_hook_ptr_mut = 0xE9; // JMP opcode
    *(to_hook_ptr_mut.add(1) as *mut u32) = relative_address;

    log_to_file("  Restoring target memory protection...");
    // Restore original protection
    VirtualProtect(to_hook_ptr_mut as *mut _, len, old_protect, &mut temp);

    log_to_file("  Hook installed successfully!");
    Ok(())
}
*/

// Static to keep the hook alive
#[cfg(target_os = "windows")]
static mut HOOK_POINT_1: Option<ilhook::x86::HookPoint> = None;

/// Apply all memory patches for debug draw functionality
#[cfg(target_arch = "x86")]
pub unsafe fn apply_debug_draw_patches() -> Result<(), String> {
    use windows_sys::Win32::System::Memory::{VirtualProtect, VirtualQuery, PAGE_EXECUTE_READWRITE, MEMORY_BASIC_INFORMATION};

    log_to_file("=== Starting debug draw patches installation ===");
    
    // Verify we can read from game memory first
    log_to_file("Verifying game memory is accessible...");
    unsafe {
        let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
        let test_addr = HOOK_ADDRESS_1 as *const std::ffi::c_void;
        if VirtualQuery(test_addr, &mut mbi, std::mem::size_of::<MEMORY_BASIC_INFORMATION>()) == 0 {
            let err = "Failed to query memory at hook address - game memory may not be loaded yet".to_string();
            log_to_file(&err);
            return Err(err);
        }
        log_to_file(&format!("Memory at 0x{:X} is accessible (State: {}, Protect: {})", HOOK_ADDRESS_1, mbi.State, mbi.Protect));
    }
    
    // Verify critical pointers are accessible
    log_to_file("Verifying critical game pointers...");
    unsafe {
        // Check P_HG_MAN pointer
        let phgman_ptr = P_HG_MAN as *const u32;
        let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
        if VirtualQuery(phgman_ptr as *const _, &mut mbi, std::mem::size_of::<MEMORY_BASIC_INFORMATION>()) == 0 {
            let err = format!("P_HG_MAN at 0x{:X} is not accessible", P_HG_MAN);
            log_to_file(&err);
            return Err(err);
        }
        log_to_file(&format!("P_HG_MAN at 0x{:X} is accessible", P_HG_MAN));
        
        // Check function pointers
        let func_ptrs = [
            (F_ADD_DRAW_PLAN_BEGIN_TARGET_SCENE, "F_ADD_DRAW_PLAN_BEGIN_TARGET_SCENE"),
            (F_ADD_DRAW_PLAN_END_TARGET_SCENE, "F_ADD_DRAW_PLAN_END_TARGET_SCENE"),
            (F_ADD_DRAW_PLAN_END_TARGET_CAMERA, "F_ADD_DRAW_PLAN_END_TARGET_CAMERA"),
        ];
        
        for (addr, name) in &func_ptrs {
            let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
            if VirtualQuery(*addr as *const _, &mut mbi, std::mem::size_of::<MEMORY_BASIC_INFORMATION>()) == 0 {
                let err = format!("{} at 0x{:X} is not accessible", name, addr);
                log_to_file(&err);
                return Err(err);
            }
            log_to_file(&format!("{} at 0x{:X} is accessible", name, addr));
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Use manual naked function hook instead of ilhook to match C++ behavior
        log_to_file("Installing manual naked function hooks...");
        
        use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};
        
        // Hook 1 - BeginTargetScene at 0xEB3798 (5 bytes)
        log_to_file(&format!("Installing hook 1 at 0x{:X}", HOOK_ADDRESS_1));
        
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
        
        log_to_file("Hook 1 installed with manual naked function successfully");
        
        // Hook 2 - BeginTargetScene at 0xEB437D (9 bytes)
        log_to_file(&format!("Installing hook 2 at 0x{:X}", HOOK_ADDRESS_2));
        
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
        
        log_to_file("Hook 2 installed with manual naked function successfully");
        
        // Hook 3 - BeginTargetScene at 0xEB4483 (9 bytes)
        log_to_file(&format!("Installing hook 3 at 0x{:X}", HOOK_ADDRESS_3));
        
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
        
        log_to_file("Hook 3 installed with manual naked function successfully");
    }

    #[cfg(not(target_os = "windows"))]
    {
        log_to_file("Hooks disabled - not building for Windows target");
    }

    log_to_file("Installing memory patches...");
    
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
