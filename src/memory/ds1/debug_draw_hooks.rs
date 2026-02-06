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

unsafe fn patch_memory(address: u32, data: &[u8]) -> Result<(), String> {
    use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};
    
    let mut old_protect: u32 = 0;
    if unsafe { VirtualProtect(
        address as *const _,
        data.len(),
        PAGE_EXECUTE_READWRITE,
        &mut old_protect
    ) } == 0 {
        return Err(format!("Failed to change protection at 0x{:X}", address));
    }
    
    unsafe { ptr::copy_nonoverlapping(data.as_ptr(), address as *mut u8, data.len()); }
    
    let mut temp = 0;
    unsafe { VirtualProtect(
        address as *const _,
        data.len(),
        old_protect,
        &mut temp
    ); }
    
    Ok(())
}

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
        jmp_bytes[0] = 0xE9;
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
        
        ptr::copy_nonoverlapping(jmp_bytes.as_ptr(), HOOK_ADDRESS_1 as *mut u8, 5);
        
        let mut temp = 0;
        VirtualProtect(HOOK_ADDRESS_1 as *const _, 5, old_protect, &mut temp);
        
        // Hook 2 - BeginTargetScene at 0xEB437D (9 bytes)
        let naked_fn_addr = hook_begin_target_scene_ez_draw_normal1_naked as usize;
        let offset = (naked_fn_addr as i32) - (HOOK_ADDRESS_2 as i32) - 5;
        
        let mut jmp_bytes = [0u8; 9];
        jmp_bytes[0] = 0xE9;
        jmp_bytes[1..5].copy_from_slice(&offset.to_le_bytes());
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
        
        ptr::copy_nonoverlapping(jmp_bytes.as_ptr(), HOOK_ADDRESS_2 as *mut u8, 9);
        
        let mut temp = 0;
        VirtualProtect(HOOK_ADDRESS_2 as *const _, 9, old_protect, &mut temp);
        
        // Hook 3 - BeginTargetScene at 0xEB4483 (9 bytes)
        let naked_fn_addr = hook_begin_target_scene_ez_draw_normal2_naked as usize;
        let offset = (naked_fn_addr as i32) - (HOOK_ADDRESS_3 as i32) - 5;
        
        let mut jmp_bytes = [0u8; 9];
        jmp_bytes[0] = 0xE9;
        jmp_bytes[1..5].copy_from_slice(&offset.to_le_bytes());
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
        
        ptr::copy_nonoverlapping(jmp_bytes.as_ptr(), HOOK_ADDRESS_3 as *mut u8, 9);
        
        let mut temp = 0;
        VirtualProtect(HOOK_ADDRESS_3 as *const _, 9, old_protect, &mut temp);
    }

    // Memory patches
    patch_memory(0xFD6529, &vec![0x90; 19])?;
    patch_memory(0xFD6546, &vec![0x90; 3])?;
    patch_memory(0xC03F1B, &[0x50])?;
    patch_memory(0xC03180, &[0x50])?;
    patch_memory(0xC064E4, &0x140u32.to_le_bytes())?;
    patch_memory(0xC064EB, &0x140u32.to_le_bytes())?;

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
// Thanks to Horkrux for the initial c++ dinput8.dll that helped me immensly to understand this.