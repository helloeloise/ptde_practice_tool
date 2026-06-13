use super::*;

impl Ds1 {
    /// Disable camera follow logic by patching je to jmp
    /// Call this AFTER freecam injection is complete and stable
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn disable_camera_follow(&mut self) -> bool {
        use crate::memory::ds1::offsets;

        unsafe {
            eprintln!("[CAMERA FOLLOW DISABLE] NOPing camera write instructions...");

            let base = 0x00400000usize;

            // Location 1 - X/Y write (7 bytes)
            let addr1_xy = base + offsets::CAMERA_WRITE_1_XY_RVA;
            let patch1_xy = vec![0x90u8; offsets::CAMERA_WRITE_1_XY_SIZE];
            let ok1_xy = write_exec_patch(addr1_xy, &patch1_xy);
            eprintln!(
                "[WRITE 1 X/Y] 0x{:08X} ({} bytes) -> {}",
                addr1_xy,
                offsets::CAMERA_WRITE_1_XY_SIZE,
                if ok1_xy { "OK" } else { "FAIL" }
            );

            // Location 1 - Z write (8 bytes)
            let addr1_z = base + offsets::CAMERA_WRITE_1_Z_RVA;
            let patch1_z = vec![0x90u8; offsets::CAMERA_WRITE_1_Z_SIZE];
            let ok1_z = write_exec_patch(addr1_z, &patch1_z);
            eprintln!(
                "[WRITE 1 Z] 0x{:08X} ({} bytes) -> {}",
                addr1_z,
                offsets::CAMERA_WRITE_1_Z_SIZE,
                if ok1_z { "OK" } else { "FAIL" }
            );

            // Location 2 - X/Y write (8 bytes)
            let addr2_xy = base + offsets::CAMERA_WRITE_2_XY_RVA;
            let patch2_xy = vec![0x90u8; offsets::CAMERA_WRITE_2_XY_SIZE];
            let ok2_xy = write_exec_patch(addr2_xy, &patch2_xy);
            eprintln!(
                "[WRITE 2 X/Y] 0x{:08X} ({} bytes) -> {}",
                addr2_xy,
                offsets::CAMERA_WRITE_2_XY_SIZE,
                if ok2_xy { "OK" } else { "FAIL" }
            );

            // Location 2 - Z write (8 bytes)
            let addr2_z = base + offsets::CAMERA_WRITE_2_Z_RVA;
            let patch2_z = vec![0x90u8; offsets::CAMERA_WRITE_2_Z_SIZE];
            let ok2_z = write_exec_patch(addr2_z, &patch2_z);
            eprintln!(
                "[WRITE 2 Z] 0x{:08X} ({} bytes) -> {}",
                addr2_z,
                offsets::CAMERA_WRITE_2_Z_SIZE,
                if ok2_z { "OK" } else { "FAIL" }
            );

            let success_count = [ok1_xy, ok1_z, ok2_xy, ok2_z]
                .iter()
                .filter(|&&x| x)
                .count();
            eprintln!(
                "[CAMERA FOLLOW DISABLE] {}/4 writes NOPed successfully",
                success_count
            );

            success_count == 4
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn disable_camera_follow(&mut self) -> bool {
        false
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub(super) fn enable_free_cam_injection(&mut self) -> bool {
        if self.free_cam_enabled {
            return true;
        }

        unsafe {
            // Check if addresses were scanned
            if FREECAM_INJECT_ADDR_1 == 0 || FREECAM_INJECT_ADDR_2 == 0 {
                eprintln!("Freecam: Injection addresses not found. Make sure the game is loaded.");
                return false;
            }

            if FREECAM_CAM_MGR_PTR == 0 {
                eprintln!("Freecam: Camera manager pointer not found.");
                return false;
            }

            if FREECAM_MOTION_CALL_SAFE == 1 && FREECAM_MOTION_FUNC == 0 {
                eprintln!("Freecam: Motion update function not resolved for safe call path.");
                return false;
            }

            // Reset hook state
            FREECAM_MODE = 0;
            FREECAM_L3R3_HELD = 0;
            FREECAM_XINPUT_HELD = 0;
            FREECAM_MOVE_DX = 0.0;
            FREECAM_MOVE_DY = 0.0;
            FREECAM_MOVE_DZ = 0.0;
            FREECAM_IN_HOOK = 0;
            FREECAM_INJECTION_SUCCESS = false;
            FREECAM_INJECTION_PENDING = false;

            // Reset debug counters
            FREECAM_DEBUG_FUNC_ADDR = 0;
            FREECAM_DEBUG_CAM_OBJ = 0;
            FREECAM_DEBUG_CALLED_FUNC = 0;
            FREECAM_DEBUG_HOOK_ENTRY = 0;
            FREECAM_DEBUG_INPUT_RAW_8 = 0;
            FREECAM_DEBUG_INPUT_RAW_C = 0;
            FREECAM_DEBUG_HOOK2_ENTRY_COUNT = 0;
            FREECAM_DEBUG_HOOK2_SKIP_COUNT = 0;
            FREECAM_DEBUG_HOOK2_ORIG_COUNT = 0;
            FREECAM_DEBUG_BUF_18 = 0;
            FREECAM_DEBUG_BUF_30 = 0;
            FREECAM_DEBUG_BUF_254 = 0;
            FREECAM_DEBUG_BUF30_TA_X = 0.0;
            FREECAM_DEBUG_BUF30_TA_Y = 0.0;
            FREECAM_DEBUG_BUF30_TA_Z = 0.0;
            FREECAM_DEBUG_BUF30_TB_X = 0.0;
            FREECAM_DEBUG_BUF30_TB_Y = 0.0;
            FREECAM_DEBUG_BUF30_TB_Z = 0.0;
            FREECAM_DEBUG_HOOK_APPLY_COUNT = 0;
            FREECAM_DEBUG_HOOK_LAST_DST = 0;
            FREECAM_DEBUG_HOOK_LAST_SRC = 0;
            FREECAM_DEBUG_HOOK_MOVE_DX = 0.0;
            FREECAM_DEBUG_HOOK_MOVE_DY = 0.0;
            FREECAM_DEBUG_HOOK_MOVE_DZ = 0.0;
            FREECAM_DEBUG_HOOK_POST_40 = 0.0;
            FREECAM_DEBUG_HOOK_POST_44 = 0.0;
            FREECAM_DEBUG_HOOK_POST_48 = 0.0;
            FREECAM_FALLBACK_BUF_BASE = 0;

            // Verify addresses contain the expected original opcodes
            let addr1_bytes = std::slice::from_raw_parts(FREECAM_INJECT_ADDR_1 as *const u8, 7);
            let addr2_bytes = std::slice::from_raw_parts(FREECAM_INJECT_ADDR_2 as *const u8, 6);

            if addr1_bytes != FREECAM_ORIG_PATCH_1 {
                eprintln!("Freecam: Addr1 bytes don't match expected pattern");
                eprintln!("  Expected: {:02X?}", FREECAM_ORIG_PATCH_1);
                eprintln!("  Found:    {:02X?}", addr1_bytes);
                return false;
            }

            if addr2_bytes != FREECAM_ORIG_PATCH_2 {
                eprintln!("Freecam: Addr2 bytes don't match expected pattern");
                eprintln!("  Expected: {:02X?}", FREECAM_ORIG_PATCH_2);
                eprintln!("  Found:    {:02X?}", addr2_bytes);
                return false;
            }

            // Create jump patches
            let patch_1 = match jmp_patch(FREECAM_INJECT_ADDR_1, freecam_hook_1 as usize, 7) {
                Some(p) => p,
                None => {
                    eprintln!("Freecam: Failed to create patch 1");
                    return false;
                }
            };

            let patch_2 = match jmp_patch(FREECAM_INJECT_ADDR_2, freecam_hook_2 as usize, 6) {
                Some(p) => p,
                None => {
                    eprintln!("Freecam: Failed to create patch 2");
                    return false;
                }
            };

            // Apply patches
            eprintln!(
                "Freecam: Applying patch 1 at 0x{:08X}...",
                FREECAM_INJECT_ADDR_1
            );
            if !write_exec_patch(FREECAM_INJECT_ADDR_1, &patch_1) {
                eprintln!("Freecam: Failed to write patch 1");
                return false;
            }
            eprintln!("Freecam: Patch 1 applied successfully");

            eprintln!(
                "Freecam: Applying patch 2 at 0x{:08X}...",
                FREECAM_INJECT_ADDR_2
            );
            if !write_exec_patch(FREECAM_INJECT_ADDR_2, &patch_2) {
                eprintln!("Freecam: Failed to write patch 2");
                let _ = write_exec_patch(FREECAM_INJECT_ADDR_1, &FREECAM_ORIG_PATCH_1);
                return false;
            }
            eprintln!("Freecam: Patch 2 applied successfully");

            eprintln!("Freecam injection successful!");
            eprintln!(
                "  Hook 1: 0x{:08X} -> 0x{:08X}",
                FREECAM_INJECT_ADDR_1,
                freecam_hook_1 as *const () as usize
            );
            eprintln!(
                "  Hook 2: 0x{:08X} -> 0x{:08X}",
                FREECAM_INJECT_ADDR_2,
                freecam_hook_2 as *const () as usize
            );
            eprintln!("  Camera manager ptr addr: 0x{:08X}", FREECAM_CAM_MGR_PTR);
            eprintln!(
                "  Motion function addr: 0x{:08X} (safe={})",
                FREECAM_MOTION_FUNC, FREECAM_MOTION_CALL_SAFE
            );

            // Read and log what the camera manager pointer currently contains
            let cam_mgr_value = std::ptr::read(FREECAM_CAM_MGR_PTR as *const usize);
            eprintln!("  Camera manager value: 0x{:08X}", cam_mgr_value);

            FREECAM_INJECTION_SUCCESS = true;
            eprintln!("Freecam: Injection complete, waiting for game to call hook...");

            // Apply camera follow disable patch
            eprintln!("");
            eprintln!("Freecam: Disabling camera follow-player logic...");
        }

        // Call camera follow disable after successful injection
        if unsafe { FREECAM_INJECTION_SUCCESS } {
            self.disable_camera_follow();
        }

        true
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub(super) fn enable_free_cam_injection(&mut self) -> bool {
        false
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub(super) fn disable_free_cam_injection(&mut self) -> bool {
        unsafe {
            if FREECAM_INJECT_ADDR_1 == 0 || FREECAM_INJECT_ADDR_2 == 0 {
                return true; // Nothing to disable
            }

            // Restore original bytes
            let ok1 = write_exec_patch(FREECAM_INJECT_ADDR_1, &FREECAM_ORIG_PATCH_1);
            let ok2 = write_exec_patch(FREECAM_INJECT_ADDR_2, &FREECAM_ORIG_PATCH_2);

            // Reset hook state
            FREECAM_MODE = 0;
            FREECAM_L3R3_HELD = 0;
            FREECAM_XINPUT_HELD = 0;
            FREECAM_MOVE_DX = 0.0;
            FREECAM_MOVE_DY = 0.0;
            FREECAM_MOVE_DZ = 0.0;
            FREECAM_IN_HOOK = 0;
            FREECAM_INJECTION_SUCCESS = false;
            FREECAM_INJECTION_PENDING = false;

            if ok1 && ok2 {
                eprintln!("Freecam disabled successfully");
            }

            ok1 && ok2
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub(super) fn disable_free_cam_injection(&mut self) -> bool {
        true
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn try_pending_freecam_injection(&mut self) {
        // No longer needed - addresses are scanned at startup in refresh()
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn try_pending_freecam_injection(&mut self) {}
}
