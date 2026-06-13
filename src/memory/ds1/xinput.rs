use super::*;

impl Ds1 {
    // ============================================================================
    // XINPUT HOOK PUBLIC API
    // ============================================================================

    /// Get current XInput buttons state (for UI display)
    pub fn get_xinput_buttons(&self) -> u16 {
        unsafe { XINPUT_CURRENT_BUTTONS }
    }

    /// Get XInput hook call count (for diagnostics)
    pub fn get_xinput_call_count(&self) -> u32 {
        unsafe { XINPUT_CALL_COUNT }
    }

    /// Get last controller index that was polled
    pub fn get_xinput_last_user_index(&self) -> u32 {
        unsafe { XINPUT_LAST_USER_INDEX }
    }

    /// Enable XInput hook for button injection at Windows API level
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn enable_xinput_hook(&mut self) -> bool {
        unsafe {
            use windows_sys::Win32::System::LibraryLoader::{
                GetModuleHandleA, GetProcAddress, LoadLibraryA,
            };
            use windows_sys::Win32::System::Memory::{VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

            // Win11-era XInput binaries and thunks can make this inline hook unstable.
            // Keep it opt-in so controller support does not crash the game by default.
            let allow_inline_hook = std::env::var("PTDE_UNSAFE_XINPUT_INLINE_HOOK")
                .ok()
                .map(|v| {
                    let s = v.trim().to_ascii_lowercase();
                    s == "1" || s == "true" || s == "yes" || s == "on"
                })
                .unwrap_or(false);
            if !allow_inline_hook {
                eprintln!(
                    "[XINPUT HOOK] Inline hook is disabled by default (stability safeguard). Set PTDE_UNSAFE_XINPUT_INLINE_HOOK=1 to force-enable."
                );
                return false;
            }

            if XINPUT_HOOK_ENABLED {
                eprintln!("[XINPUT HOOK] Already enabled");
                return true;
            }

            eprintln!("[XINPUT HOOK] Installing XInputGetState hook...");

            // Get XInputGetState address from known XInput variants.
            let func_name = b"XInputGetState\0";

            let module_candidates: [(&str, &[u8]); 3] = [
                ("xinput1_4.dll", b"xinput1_4.dll\0"),
                ("xinput1_3.dll", b"xinput1_3.dll\0"),
                ("xinput9_1_0.dll", b"xinput9_1_0.dll\0"),
            ];

            let mut module = 0;
            let mut module_name = "unknown";
            for (name, bytes) in module_candidates {
                let mut h = GetModuleHandleA(bytes.as_ptr());
                if h == 0 {
                    h = LoadLibraryA(bytes.as_ptr());
                }
                if h != 0 {
                    module = h;
                    module_name = name;
                    break;
                }
            }

            if module == 0 {
                eprintln!("[XINPUT HOOK] Failed to load any XInput module (xinput1_4/xinput1_3/xinput9_1_0)");
                return false;
            }

            let xinput_addr = GetProcAddress(module, func_name.as_ptr());
            if xinput_addr.is_none() {
                eprintln!("[XINPUT HOOK] Failed to get XInputGetState address");
                return false;
            }

            let xinput_fn_ptr = xinput_addr.unwrap() as usize;
            eprintln!(
                "[XINPUT HOOK] XInputGetState at 0x{:08X} from {}",
                xinput_fn_ptr,
                module_name
            );

            // Read and log first bytes for debugging
            eprint!("[XINPUT HOOK] Original bytes: ");
            for i in 0..12 {
                eprint!("{:02X} ", std::ptr::read_volatile((xinput_fn_ptr + i) as *const u8));
            }
            eprintln!();

            // Check if already hooked (starts with 0xE9 = JMP)
            let first_byte = std::ptr::read_volatile(xinput_fn_ptr as *const u8);
            if first_byte == 0xE9 {
                eprintln!("[XINPUT HOOK] Warning: Function appears already hooked");
            }

            // Save original bytes (12 bytes to be safe for instruction boundaries)
            for i in 0..7 {
                XINPUT_ORIG_BYTES[i] = std::ptr::read_volatile((xinput_fn_ptr + i) as *const u8);
            }

            // Allocate trampoline (40 bytes: plenty for original bytes + JMP back)
            let trampoline = VirtualAlloc(
                std::ptr::null(),
                40,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_EXECUTE_READWRITE,
            ) as usize;

            if trampoline == 0 {
                eprintln!("[XINPUT HOOK] Failed to allocate trampoline");
                return false;
            }

            eprintln!("[XINPUT HOOK] Trampoline at 0x{:08X}", trampoline);

            // Build trampoline: copy 12 bytes + absolute JMP to original+12
            let mut tramp_offset = 0;

            // Copy first 12 bytes (should be enough for most instruction patterns)
            for i in 0..12 {
                let byte = std::ptr::read_volatile((xinput_fn_ptr + i) as *const u8);
                std::ptr::write_volatile((trampoline + tramp_offset) as *mut u8, byte);
                tramp_offset += 1;
            }

            // Add absolute JMP to original+12 (avoids our 5-byte hook)
            std::ptr::write_volatile((trampoline + tramp_offset) as *mut u8, 0xE9); // JMP rel32
            tramp_offset += 1;
            let jmp_offset = (xinput_fn_ptr + 12) as i32 - (trampoline + tramp_offset + 4) as i32;
            std::ptr::write_volatile((trampoline + tramp_offset) as *mut i32, jmp_offset);

            eprint!("[XINPUT HOOK] Trampoline bytes: ");
            for i in 0..20 {
                eprint!("{:02X} ", std::ptr::read_volatile((trampoline + i) as *const u8));
            }
            eprintln!();

            XINPUT_TRAMPOLINE = trampoline;
            XINPUT_HOOK_ADDR = xinput_fn_ptr;

            // Install hook: JMP to xinput_hook (5 bytes) + NOPs (2 bytes for alignment)
            let hook_addr = xinput_hook as usize;
            let jmp_rel = hook_addr as i32 - (xinput_fn_ptr + 5) as i32;

            eprintln!("[XINPUT HOOK] Hook function at 0x{:08X}, JMP offset: 0x{:08X}", hook_addr, jmp_rel);

            let mut patch = [0u8; 7];
            patch[0] = 0xE9; // JMP
            patch[1..5].copy_from_slice(&jmp_rel.to_le_bytes());
            patch[5] = 0x90; // NOP
            patch[6] = 0x90; // NOP

            if !write_exec_patch(xinput_fn_ptr, &patch) {
                eprintln!("[XINPUT HOOK] Failed to write hook");
                VirtualFree(trampoline as *mut c_void, 0, 0x8000);
                return false;
            }

            // Verify patch was written
            eprint!("[XINPUT HOOK] Patched bytes: ");
            for i in 0..7 {
                eprint!("{:02X} ", std::ptr::read_volatile((xinput_fn_ptr + i) as *const u8));
            }
            eprintln!();

            XINPUT_HOOK_ENABLED = true;
            eprintln!("[XINPUT HOOK] Installed successfully");
            true
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn enable_xinput_hook(&mut self) -> bool {
        false
    }

    /// Disable XInput hook
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn disable_xinput_hook(&mut self) -> bool {
        unsafe {
            use windows_sys::Win32::System::Memory::VirtualFree;
            use windows_sys::Win32::System::Threading::Sleep;

            if !XINPUT_HOOK_ENABLED {
                return true;
            }

            eprintln!("[XINPUT HOOK] Removing hook...");

            // Disable injection first to prevent any modifications during unhook
            XINPUT_INJECT_ENABLED = 0;
            XINPUT_INJECT_BUTTONS = 0;
            XINPUT_INJECT_LEFT_TRIGGER = 0;
            XINPUT_INJECT_RIGHT_TRIGGER = 0;
            XINPUT_INJECT_THUMB_LX = 0;
            XINPUT_INJECT_THUMB_LY = 0;
            XINPUT_INJECT_THUMB_RX = 0;
            XINPUT_INJECT_THUMB_RY = 0;

            // Mark as disabled BEFORE restoring bytes to prevent hook execution
            XINPUT_HOOK_ENABLED = false;

            // Give any in-flight calls time to complete
            Sleep(10);

            // Restore original bytes
            if !write_exec_patch(XINPUT_HOOK_ADDR, &XINPUT_ORIG_BYTES) {
                eprintln!("[XINPUT HOOK] Failed to restore original bytes");
            }

            Sleep(10);

            // Free trampoline
            if XINPUT_TRAMPOLINE != 0 {
                let result = VirtualFree(XINPUT_TRAMPOLINE as *mut c_void, 0, 0x8000); // MEM_RELEASE
                if result == 0 {
                    eprintln!("[XINPUT HOOK] Warning: VirtualFree failed");
                }
                XINPUT_TRAMPOLINE = 0;
            }

            XINPUT_HOOK_ADDR = 0;

            eprintln!("[XINPUT HOOK] Removed successfully");
            true
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn disable_xinput_hook(&mut self) -> bool {
        false
    }

    /// Set XInput injection
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn set_xinput_injection(&mut self, enabled: bool) {
        unsafe {
            XINPUT_INJECT_ENABLED = if enabled { 1 } else { 0 };
            if !enabled {
                XINPUT_INJECT_BUTTONS = 0;
                XINPUT_INJECT_LEFT_TRIGGER = 0;
                XINPUT_INJECT_RIGHT_TRIGGER = 0;
                XINPUT_INJECT_THUMB_LX = 0;
                XINPUT_INJECT_THUMB_LY = 0;
                XINPUT_INJECT_THUMB_RX = 0;
                XINPUT_INJECT_THUMB_RY = 0;
            }
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn set_xinput_injection(&mut self, _enabled: bool) {}

    /// Inject specific XInput buttons (A = 0x1000, B = 0x2000, etc.)
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn inject_xinput_buttons(&mut self, buttons: u16) {
        unsafe {
            XINPUT_INJECT_BUTTONS = buttons;
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn inject_xinput_buttons(&mut self, _buttons: u16) {}

    /// Get current analog stick and trigger values
    pub fn get_xinput_triggers(&self) -> (u8, u8) {
        unsafe { (XINPUT_CURRENT_LEFT_TRIGGER, XINPUT_CURRENT_RIGHT_TRIGGER) }
    }

    pub fn get_xinput_left_stick(&self) -> (i16, i16) {
        unsafe { (XINPUT_CURRENT_THUMB_LX, XINPUT_CURRENT_THUMB_LY) }
    }

    pub fn get_xinput_right_stick(&self) -> (i16, i16) {
        unsafe { (XINPUT_CURRENT_THUMB_RX, XINPUT_CURRENT_THUMB_RY) }
    }

    /// Inject analog stick and trigger values
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn inject_xinput_triggers(&mut self, left: u8, right: u8) {
        unsafe {
            XINPUT_INJECT_LEFT_TRIGGER = left;
            XINPUT_INJECT_RIGHT_TRIGGER = right;
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn inject_xinput_triggers(&mut self, _left: u8, _right: u8) {}

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn inject_xinput_left_stick(&mut self, x: i16, y: i16) {
        unsafe {
            XINPUT_INJECT_THUMB_LX = x;
            XINPUT_INJECT_THUMB_LY = y;
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn inject_xinput_left_stick(&mut self, _x: i16, _y: i16) {}

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn inject_xinput_right_stick(&mut self, x: i16, y: i16) {
        unsafe {
            XINPUT_INJECT_THUMB_RX = x;
            XINPUT_INJECT_THUMB_RY = y;
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn inject_xinput_right_stick(&mut self, _x: i16, _y: i16) {}
}
