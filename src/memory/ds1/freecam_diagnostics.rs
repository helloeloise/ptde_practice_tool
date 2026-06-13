use super::*;

impl Ds1 {
    // Freecam debug logging accessors
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_log_stats(&self) -> (i32, bool, usize, usize, usize) {
        unsafe {
            (
                FREECAM_MODE,
                FREECAM_INJECTION_SUCCESS,
                FREECAM_INJECT_ADDR_1,
                FREECAM_INJECT_ADDR_2,
                FREECAM_CAM_MGR_PTR,
            )
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_log_stats(&self) -> (i32, bool, usize, usize, usize) {
        (0, false, 0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_camera_values(&self) -> (usize, usize, i32) {
        unsafe {
            if FREECAM_CAM_MGR_PTR == 0 {
                return (0, 0, 0);
            }

            // Read camera manager pointer value
            let cam_mgr = std::ptr::read(FREECAM_CAM_MGR_PTR as *const usize);

            if cam_mgr == 0 {
                return (0, 0, 0);
            }

            // Read camera object pointer at [cam_mgr+0x6F8]
            let cam_obj_addr = (cam_mgr + 0x6F8) as *const usize;
            let cam_obj = std::ptr::read(cam_obj_addr);

            // Read mode value from camera object at +0x24C
            let mode = if cam_obj != 0 {
                std::ptr::read((cam_obj + 0x24C) as *const i32)
            } else {
                -1
            };

            (cam_mgr, cam_obj, mode)
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_camera_values(&self) -> (usize, usize, i32) {
        (0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_debug_values(&self) -> (usize, usize, u32, u32) {
        unsafe {
            (
                FREECAM_DEBUG_FUNC_ADDR,
                FREECAM_DEBUG_CAM_OBJ,
                FREECAM_DEBUG_CALLED_FUNC,
                FREECAM_DEBUG_HOOK_ENTRY,
            )
        }
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_input_hook_values(&self) -> (u32, u32) {
        unsafe { (FREECAM_DEBUG_INPUT_RAW_8, FREECAM_DEBUG_INPUT_RAW_C) }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_input_hook_values(&self) -> (u32, u32) {
        (0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_manual_debug_values(&self) -> (u32, u32, f32, f32, f32, usize) {
        unsafe {
            (
                FREECAM_DEBUG_MANUAL_TICKS,
                FREECAM_DEBUG_MANUAL_WRITES,
                FREECAM_DEBUG_MANUAL_AXES_X,
                FREECAM_DEBUG_MANUAL_AXES_Y,
                FREECAM_DEBUG_MANUAL_AXES_Z,
                FREECAM_DEBUG_LAST_WRITE_ADDR,
            )
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_manual_debug_values(&self) -> (u32, u32, f32, f32, f32, usize) {
        (0, 0, 0.0, 0.0, 0.0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_xinput_debug_values(&self) -> (i32, u16, i16, i16, i16, i16, u8, u8) {
        unsafe {
            (
                FREECAM_DEBUG_XINPUT_PAD,
                FREECAM_DEBUG_XINPUT_BUTTONS,
                FREECAM_DEBUG_XINPUT_LX,
                FREECAM_DEBUG_XINPUT_LY,
                FREECAM_DEBUG_XINPUT_RX,
                FREECAM_DEBUG_XINPUT_RY,
                FREECAM_DEBUG_XINPUT_LT,
                FREECAM_DEBUG_XINPUT_RT,
            )
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_xinput_debug_values(&self) -> (i32, u16, i16, i16, i16, i16, u8, u8) {
        (-1, 0, 0, 0, 0, 0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_motion_func_bytes(&self) -> (usize, Vec<u8>) {
        unsafe {
            // Instead of using calculated address, scan near injection point for calls
            if FREECAM_INJECT_ADDR_1 == 0 {
                return (0, vec![]);
            }

            // Read 256 bytes after our injection point to look for call instructions
            let scan_addr = FREECAM_INJECT_ADDR_1 + 0x20; // Start a bit after
            let mut bytes = vec![0u8; 128];

            for i in 0..128 {
                bytes[i] = std::ptr::read_volatile((scan_addr + i) as *const u8);
            }

            (scan_addr, bytes)
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_motion_func_bytes(&self) -> (usize, Vec<u8>) {
        (0, vec![])
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_debug_values(&self) -> (usize, usize, u32, u32) {
        (0, 0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_injection_info(&self) -> (bool, bool, bool, [u8; 7], [u8; 6], bool, u32) {
        unsafe {
            // Simplified - return injection success and addresses
            let addr1_valid = FREECAM_INJECT_ADDR_1 != 0;
            let addr2_valid = FREECAM_INJECT_ADDR_2 != 0;
            let found1 = if addr1_valid { FREECAM_ORIG_PATCH_1 } else { [0; 7] };
            let found2 = if addr2_valid { FREECAM_ORIG_PATCH_2 } else { [0; 6] };
            (
                FREECAM_INJECTION_SUCCESS,
                addr1_valid,
                addr2_valid,
                found1,
                found2,
                FREECAM_INJECTION_PENDING,
                0,
            )
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_injection_info(&self) -> (bool, bool, bool, [u8; 7], [u8; 6], bool, u32) {
        (false, false, false, [0; 7], [0; 6], false, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_camera_samples(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
        // Camera sampling removed in new implementation
        (0, 0, 0, 0, 0, 0, 0, 0)
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_camera_samples(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
        (0, 0, 0, 0, 0, 0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn clear_freecam_log(&mut self) {
        // No-op - logging removed
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn clear_freecam_log(&mut self) {}

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn save_freecam_log_to_file(&self) -> Result<String, String> {
        unsafe {
            use std::time::{SystemTime, UNIX_EPOCH};

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let filename = format!("freecam_status_{}.txt", timestamp);

            let (cam_mgr, cam_obj, cam_mode) = self.get_freecam_camera_values();
            let (dbg_a, dbg_b, dbg_c, dbg_d) = self.get_freecam_debug_values();
            let (hook_raw_8, hook_raw_c) = self.get_freecam_input_hook_values();
            let (esi_val, l3_raw, r3_raw) = self.get_freecam_input_debug();
            let (manual_ticks, manual_writes, mdx, mdy, mdz, last_addr) =
                self.get_freecam_manual_debug_values();
            let (xpad, xbtn, xlx, xly, xrx, xry, xlt, xrt) = self.get_freecam_xinput_debug_values();
            let x_l3 = (xbtn & 0x0040) != 0;
            let x_r3 = (xbtn & 0x0080) != 0;

            let info = format!(
                "Freecam Status Report\n\
                 ======================\n\
                 Timestamp (unix): {}\n\
                 Mode: {}\n\
                 Injection Success: {}\n\
                 Injection Pending: {}\n\
                 In Hook: {}\n\
                 \n\
                 Inject Addr 1: 0x{:08X}\n\
                 Inject Addr 2: 0x{:08X}\n\
                 Inject Match Count 1/2: {} / {}\n\
                 Inject Pair Delta: 0x{:X}\n\
                 Inject Pair Delta Err: 0x{:X}\n\
                 Inject Pair CallRef: {}\n\
                 Hook2 Skip Addr: 0x{:08X}\n\
                 Camera Manager Ptr Addr: 0x{:08X}\n\
                 Motion Function Addr: 0x{:08X}\n\
                 Motion Call Safe: {}\n\
                 \n\
                 Camera Manager Value: 0x{:08X}\n\
                 Camera Object Value: 0x{:08X}\n\
                 \n\
                 ChrFollowCam Ptr: 0x{:08X}\n\
                 ChrFollowCam Base [ptr]: 0x{:08X}\n\
                 ChrFollowCam L1 [base]: 0x{:08X}\n\
                 ChrFollowCam L2 [L1+3C]: 0x{:08X}\n\
                 ChrFollowCam L3 [L2+60]: 0x{:08X}\n\
                 Camera Mode In Memory: {}\n\
                 \n\
                 Debug Func Addr: 0x{:08X}\n\
                 Debug Cam Obj: 0x{:08X}\n\
                 Debug Called Counter: {}\n\
                 Debug Hook Entry Counter: {}\n\
                 Hook Input Raw [esi+0x8]: 0x{:08X}\n\
                 Hook Input Raw [esi+0xC]: 0x{:08X}\n\
                 Hook2 Entry/Skip/Orig Count: {} / {} / {}\n\
                 Cam Buffers [+18/+30/+254]: 0x{:08X} / 0x{:08X} / 0x{:08X}\n\
                 CamBuf+30 TA [1C/2C/3C]: {:.5} / {:.5} / {:.5}\n\
                 CamBuf+30 TB [40/44/48]: {:.5} / {:.5} / {:.5}\n\
                 Hook Apply Count: {}\n\
                 Hook Last Dst/Src: 0x{:08X} / 0x{:08X}\n\
                 Hook Move DX/Y/Z: {:.5} / {:.5} / {:.5}\n\
                 Hook Post [40/44/48]: {:.5} / {:.5} / {:.5}\n\
                 Manual Move Ticks: {}\n\
                 Manual Move Writes: {}\n\
                 Manual Delta X/Y/Z: {:.5} / {:.5} / {:.5}\n\
                 Manual Last Write Addr: 0x{:08X}\n\
                 XInput Active Pad: {}\n\
                 XInput Buttons: 0x{:04X}\n\
                 XInput L3/R3: {} / {}\n\
                 XInput LX/LY: {} / {}\n\
                 XInput RX/RY: {} / {}\n\
                 XInput LT/RT: {} / {}\n\
                 \n\
                 Input ESI: 0x{:08X}\n\
                 Input [esi+0x8]: 0x{:08X}\n\
                 Input [esi+0xC]: 0x{:08X}\n",
                timestamp,
                FREECAM_MODE,
                FREECAM_INJECTION_SUCCESS,
                FREECAM_INJECTION_PENDING,
                FREECAM_IN_HOOK,
                FREECAM_INJECT_ADDR_1,
                FREECAM_INJECT_ADDR_2,
                FREECAM_DEBUG_INJECT_MATCHES_1,
                FREECAM_DEBUG_INJECT_MATCHES_2,
                FREECAM_DEBUG_PAIR_DELTA,
                FREECAM_DEBUG_PAIR_DELTA_ERR,
                FREECAM_DEBUG_PAIR_CALL_REF,
                FREECAM_HOOK2_SKIP_ADDR,
                FREECAM_CAM_MGR_PTR,
                FREECAM_MOTION_FUNC,
                FREECAM_MOTION_CALL_SAFE,
                cam_mgr,
                cam_obj,
                FREECAM_DEBUG_CHR_FOLLOWCAM_PTR,
                FREECAM_DEBUG_CHR_BASE,
                FREECAM_DEBUG_CHR_L1,
                FREECAM_DEBUG_CHR_L2,
                FREECAM_DEBUG_CHR_FOLLOWCAM_OBJ,
                cam_mode,
                dbg_a,
                dbg_b,
                dbg_c,
                dbg_d,
                hook_raw_8,
                hook_raw_c,
                FREECAM_DEBUG_HOOK2_ENTRY_COUNT,
                FREECAM_DEBUG_HOOK2_SKIP_COUNT,
                FREECAM_DEBUG_HOOK2_ORIG_COUNT,
                FREECAM_DEBUG_BUF_18,
                FREECAM_DEBUG_BUF_30,
                FREECAM_DEBUG_BUF_254,
                FREECAM_DEBUG_BUF30_TA_X,
                FREECAM_DEBUG_BUF30_TA_Y,
                FREECAM_DEBUG_BUF30_TA_Z,
                FREECAM_DEBUG_BUF30_TB_X,
                FREECAM_DEBUG_BUF30_TB_Y,
                FREECAM_DEBUG_BUF30_TB_Z,
                FREECAM_DEBUG_HOOK_APPLY_COUNT,
                FREECAM_DEBUG_HOOK_LAST_DST,
                FREECAM_DEBUG_HOOK_LAST_SRC,
                FREECAM_DEBUG_HOOK_MOVE_DX,
                FREECAM_DEBUG_HOOK_MOVE_DY,
                FREECAM_DEBUG_HOOK_MOVE_DZ,
                FREECAM_DEBUG_HOOK_POST_40,
                FREECAM_DEBUG_HOOK_POST_44,
                FREECAM_DEBUG_HOOK_POST_48,
                manual_ticks,
                manual_writes,
                mdx,
                mdy,
                mdz,
                last_addr,
                xpad,
                xbtn,
                if x_l3 { 1 } else { 0 },
                if x_r3 { 1 } else { 0 },
                xlx,
                xly,
                xrx,
                xry,
                xlt,
                xrt,
                esi_val,
                l3_raw,
                r3_raw,
            );

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&filename)
                .map_err(|e| format!("Failed to create status file: {}", e))?;

            writeln!(file, "{}", info)
                .map_err(|e| format!("Failed to write status file: {}", e))?;
            file.flush()
                .map_err(|e| format!("Failed to flush status file: {}", e))?;

            Ok(filename)
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn save_freecam_log_to_file(&self) -> Result<String, String> {
        Err("Freecam logging not available on this platform".to_string())
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn scan_for_freecam_patterns(&self) -> Result<String, String> {
        use windows_sys::Win32::System::Memory::{
            VirtualQuery, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_EXECUTE_READ,
            PAGE_EXECUTE_READWRITE,
        };

        let pattern1 = &FREECAM_ORIG_PATCH_1;
        let pattern2 = &FREECAM_ORIG_PATCH_2;
        let mut results = Vec::new();
        let mut pattern1_addrs = Vec::new();
        let mut pattern2_addrs = Vec::new();
        let mut addr = 0x00400000usize; // Start of typical executable
        let max_addr = 0x20000000usize; // Limit to main module range (first ~512MB)

        eprintln!("Starting memory scan for freecam patterns...");
        eprintln!("Pattern 1: {:02X?}", pattern1);
        eprintln!("Pattern 2: {:02X?}", pattern2);
        eprintln!("Scanning range: 0x{:08X} - 0x{:08X}", addr, max_addr);

        unsafe {
            while addr < max_addr {
                let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
                let result = VirtualQuery(
                    addr as *const c_void,
                    &mut mbi,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                );

                if result == 0 {
                    break;
                }

                // Check if memory is committed and EXECUTABLE
                if mbi.State == MEM_COMMIT {
                    let is_executable = mbi.Protect == PAGE_EXECUTE_READ
                        || mbi.Protect == PAGE_EXECUTE_READWRITE;

                    if is_executable && mbi.RegionSize > 0 && mbi.RegionSize < 0x10000000 {
                        let region_start = mbi.BaseAddress as usize;
                        let region_size = mbi.RegionSize;

                        // Scan this region for patterns
                        for offset in 0..region_size.saturating_sub(pattern1.len()) {
                            let check_addr = region_start + offset;

                            // Try to read and match pattern 1
                            let slice1 = std::slice::from_raw_parts(check_addr as *const u8, pattern1.len());
                            if slice1 == pattern1 {
                                pattern1_addrs.push(check_addr);
                                eprintln!("Found pattern 1 at: 0x{:08X}", check_addr);
                            }

                            // Try to read and match pattern 2
                            if offset < region_size.saturating_sub(pattern2.len()) {
                                let slice2 = std::slice::from_raw_parts(check_addr as *const u8, pattern2.len());
                                if slice2 == pattern2 {
                                    pattern2_addrs.push(check_addr);
                                    eprintln!("Found pattern 2 at: 0x{:08X}", check_addr);
                                }
                            }
                        }
                    }
                }

                // Move to next region
                addr = (mbi.BaseAddress as usize).saturating_add(mbi.RegionSize.max(1));
                if addr <= mbi.BaseAddress as usize {
                    break; // Overflow protection
                }
            }
        }

        eprintln!(
            "Scan complete. Pattern 1: {} matches, Pattern 2: {} matches",
            pattern1_addrs.len(),
            pattern2_addrs.len()
        );

        if pattern1_addrs.is_empty() && pattern2_addrs.is_empty() {
            Ok("No patterns found in executable memory.".to_string())
        } else {
            unsafe {
                results.push(format!("=== Freecam Pattern Scan Results ==="));
                results.push(format!(""));
                results.push(format!("Current addresses being used:"));
                results.push(format!("  FREECAM_INJECT_ADDR_1 = 0x{:08X}", FREECAM_INJECT_ADDR_1));
                results.push(format!("  FREECAM_INJECT_ADDR_2 = 0x{:08X}", FREECAM_INJECT_ADDR_2));
                results.push(format!(""));
                results.push(format!("Pattern 1 (7 bytes): {:02X?}", pattern1));
                results.push(format!("Found {} match(es):", pattern1_addrs.len()));
                for (i, addr) in pattern1_addrs.iter().enumerate() {
                    let marker = if *addr == FREECAM_INJECT_ADDR_1 { " <- CURRENT" } else { "" };
                    results.push(format!("  [{}] 0x{:08X}{}", i, addr, marker));
                }
                results.push("".to_string());

                results.push(format!("Pattern 2 (6 bytes): {:02X?}", pattern2));
                results.push(format!("Found {} match(es):", pattern2_addrs.len()));
                for (i, addr) in pattern2_addrs.iter().enumerate() {
                    let marker = if *addr == FREECAM_INJECT_ADDR_2 { " <- CURRENT" } else { "" };
                    results.push(format!("  [{}] 0x{:08X}{}", i, addr, marker));
                }
                results.push("".to_string());
            }
            results.push(format!("If hook not working, try different address index."));
            results.push(format!("Update FREECAM_INJECT_ADDR_1/2 in src/memory/ds1/mod.rs"));

            // Save to file
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let filename = format!("freecam_scan_{}.txt", timestamp);

            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&filename)
            {
                for line in &results {
                    let _ = writeln!(file, "{}", line);
                }
                let _ = file.flush();
                eprintln!("Scan results saved to: {}", filename);
            }

            Ok(format!("Scan complete! Saved to {}", filename))
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn scan_for_freecam_patterns(&self) -> Result<String, String> {
        Err("Pattern scanning not available on this platform".to_string())
    }
}
