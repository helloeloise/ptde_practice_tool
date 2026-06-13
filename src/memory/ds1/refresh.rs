use super::*;

impl Ds1 {
    // Pointers are declared here
    pub fn refresh(&mut self) -> Result<(), String> {
        if !self.process.is_attached() {
            self.process.refresh()?;

            self.chr_dbg = self.process.scan_abs(
                "all_no_stamina_consume",
                &offsets::ALL_NO_STAMINA_CONSUME_AOB,
                offsets::ALL_NO_STAMINA_CONSUME_AOB_OFFSET,
                vec![0x0],
            )?;

            self.chr_data_1 = self.process.scan_abs(
                "chr_data_1",
                &offsets::CHAR_DATA_1_AOB,
                offsets::CHAR_DATA_1_AOB_OFFSET,
                vec![
                    0x0,
                    offsets::CHAR_DATA_1_OFFSET1,
                    offsets::CHAR_DATA_1_OFFSET2,
                    offsets::CHAR_DATA_1_OFFSET3,
                ],
            )?;

            self.target_bank = self.process.scan_abs(
                "target_bank",
                &offsets::TARGET_BANK_AOB,
                offsets::TARGET_BANK_AOB_OFFSET,
                vec![0x0, 0x0],
            )?;

            self.char_map_data = self.chr_data_1.clone();
            self.char_map_data.offsets.push(CharData1::CHAR_MAP_DATA_PTR);

            self.anim_data = self.char_map_data.clone();
            self.anim_data.offsets.push(CharMapData::ANIM_DATA_PTR);

            self.char_pos_data = self.char_map_data.clone();
            self.char_pos_data.offsets.push(CharMapData::CHAR_POS_DATA_PTR);

            self.chr_data_2 = self.process.scan_abs(
                "chr_data_2",
                &offsets::CHAR_DATA_2_AOB,
                offsets::CHAR_DATA_2_AOB_OFFSET,
                vec![0x0, offsets::CHAR_DATA_2_OFFSET1, offsets::CHAR_DATA_2_OFFSET2],
            )?;

            self.game_data_mgr = self.process.scan_abs(
                "game_data_mgr",
                &offsets::CHAR_DATA_2_AOB,
                offsets::CHAR_DATA_2_AOB_OFFSET,
                vec![0x0],
            )?;

            self.lock_on_mgr = self
                .process
                .scan_abs(
                    "lock_on_mgr",
                    &offsets::LOCK_ON_MGR_AOB,
                    offsets::LOCK_ON_MGR_AOB_OFFSET,
                    vec![0x0],
                )
                .unwrap_or_default();

            self.level_up =
                self.process
                    .scan_abs("level_up", &offsets::LEVEL_UP, 0x0, vec![0x0])?;

            self.bonfire_warp = self
                .process
                .scan_abs("bonfire_warp", &offsets::BONFIRE_WARP, 0x0, vec![0x0])
                .unwrap();
            self.bonfire_warp_2 = self.process.scan_abs(
                "bonfire_warp_2",
                &offsets::BONFIRE_WARP_2,
                offsets::BONFIRE_WARP_2_OFFSET1,
                vec![0x0],
            )?;

            self.item_drop_pointer = self.process.scan_abs(
                "item_drop_pointer",
                &offsets::ITEM_DROP_AOB,
                0x0,
                vec![0x0],
            )?;

            self.item_drop_unknown_1_pointer = self.process.scan_abs(
                "item_drop_unknown_1_pointer",
                &offsets::ITEM_DROP_UNKNOWN_1_AOB,
                offsets::ITEM_DROP_UNKNOWN_1_AOB_OFFSET,
                vec![0x0],
            )?;

            self.item_drop_unknown_2_pointer = self.process.scan_abs(
                "item_drop_unknown_2_pointer",
                &offsets::ITEM_DROP_UNKNOWN_2_AOB,
                offsets::ITEM_DROP_UNKNOWN_2_AOB_OFFSET,
                vec![0x0],
            )?;

            self.world_state = self.process.scan_abs(
                "world_state",
                &offsets::WORLD_STATE_AOB,
                offsets::WORLD_STATE_AOB_OFFSET,
                vec![0x0, offsets::WORLD_STATE_OFFSET1],
            )?;

            self.chr_flags_1 = self.process.scan_abs(
                "chr_flags_1",
                &offsets::FLAGS_AOB_1,
                offsets::FLAGS_AOB_1_OFFSET,
                vec![0x0],
            )?;

            self.input_state = self.process.scan_abs(
                "input_state",
                &offsets::INPUT_STATE_AOB,
                offsets::INPUT_STATE_AOB_OFFSET,
                vec![
                    0x0,
                    offsets::INPUT_STATE_OFFSET1,
                    offsets::INPUT_STATE_OFFSET2,
                    offsets::INPUT_STATE_OFFSET3,
                    offsets::INPUT_STATE_OFFSET4,
                ],
            )?;

            self.quitout = self.process.scan_abs(
                "quitout",
                &offsets::QUITOUT_AOB,
                offsets::QUITOUT_AOB_OFFSET,
                vec![0x0, offsets::QUITOUT_OFFSET1],
            )?;

            self.no_death_pointer = self.process.scan_abs(
                "no_death_pointer",
                &offsets::PLAYER_NO_DEAD_AOB,
                offsets::PLAYER_NO_DEAD_AOB_OFFSET,
                vec![0x0],
            )?;

            self.player_exterminate = self.process.scan_abs(
                "player_exterminate",
                &offsets::PLAYER_EXTERMINATE_AOB,
                offsets::PLAYER_EXTERMINATE_AOB_OFFSET,
                vec![0x0],
            )?;

            self.item_get_pointer =
                self.process
                    .scan_abs("item_get_pointer", &offsets::ITEM_GET_AOB, 0x0, vec![0x0])?;

            self.all_no_magic_quantity_consume = self.process.scan_abs(
                "all_no_magic_quantity_consume",
                &offsets::ALL_NO_MAGIC_QTY_CONSUME_AOB,
                offsets::ALL_NO_MAGIC_QTY_CONSUME_AOB_OFFSET,
                vec![0x0],
            )?;

            #[cfg(all(target_os = "windows", target_arch = "x86"))]
            self.scan_freecam_runtime_addresses();
        } else {
            self.process.refresh()?;
        }

        Ok(())
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    fn scan_freecam_runtime_addresses(&mut self) {
        unsafe {
            eprintln!("Scanning for freecam patterns...");
            eprintln!("  Pattern 1: {}", offsets::FREECAM_INJECT_1_AOB);
            eprintln!("  Pattern 2: {}", offsets::FREECAM_INJECT_2_AOB);
            eprintln!("  Pattern 3: {}", offsets::FREECAM_CAM_MGR_AOB);

            FREECAM_DEBUG_INJECT_MATCHES_1 = 0;
            FREECAM_DEBUG_INJECT_MATCHES_2 = 0;
            FREECAM_DEBUG_PAIR_DELTA = 0;
            FREECAM_DEBUG_PAIR_DELTA_ERR = 0;
            FREECAM_DEBUG_PAIR_CALL_REF = 0;

            let (pair, p1_count, p2_count) = find_freecam_inject_pair();
            FREECAM_DEBUG_INJECT_MATCHES_1 = p1_count as u32;
            FREECAM_DEBUG_INJECT_MATCHES_2 = p2_count as u32;
            eprintln!("  Freecam pattern match counts: p1={} p2={}", p1_count, p2_count);

            if let Some((pair_1, pair_2, _pair_delta, _pair_delta_err, pair_call_ref)) = pair {
                FREECAM_INJECT_ADDR_1 = pair_1;

                // Pattern found Hook2, but also try calculated release address
                let hook2_pattern_match = pair_2;
                let hook2_release_calc =
                    offsets::FREECAM_IMAGE_BASE + offsets::FREECAM_INJECT_2_RELEASE_RVA;
                let hook2_debug_calc =
                    offsets::FREECAM_IMAGE_BASE + offsets::FREECAM_INJECT_2_DEBUG_RVA;

                // Verify pattern bytes at calculated addresses
                let release_bytes_match =
                    is_readable_addr(hook2_release_calc, FREECAM_ORIG_PATCH_2.len()) && {
                        let mut buf = vec![0u8; FREECAM_ORIG_PATCH_2.len()];
                        std::ptr::copy_nonoverlapping(
                            hook2_release_calc as *const u8,
                            buf.as_mut_ptr(),
                            FREECAM_ORIG_PATCH_2.len(),
                        );
                        eprintln!(
                            "  Bytes at release calc 0x{:08X}: {:02X?} (expected: {:02X?})",
                            hook2_release_calc, buf, FREECAM_ORIG_PATCH_2
                        );
                        buf == FREECAM_ORIG_PATCH_2
                    };

                let debug_bytes_match =
                    is_readable_addr(hook2_debug_calc, FREECAM_ORIG_PATCH_2.len()) && {
                        let mut buf = vec![0u8; FREECAM_ORIG_PATCH_2.len()];
                        std::ptr::copy_nonoverlapping(
                            hook2_debug_calc as *const u8,
                            buf.as_mut_ptr(),
                            FREECAM_ORIG_PATCH_2.len(),
                        );
                        eprintln!(
                            "  Bytes at debug calc 0x{:08X}: {:02X?} (expected: {:02X?})",
                            hook2_debug_calc, buf, FREECAM_ORIG_PATCH_2
                        );
                        buf == FREECAM_ORIG_PATCH_2
                    };

                // Also check pattern-matched address
                let _pattern_bytes_match =
                    is_readable_addr(hook2_pattern_match, FREECAM_ORIG_PATCH_2.len()) && {
                        let mut buf = vec![0u8; FREECAM_ORIG_PATCH_2.len()];
                        std::ptr::copy_nonoverlapping(
                            hook2_pattern_match as *const u8,
                            buf.as_mut_ptr(),
                            FREECAM_ORIG_PATCH_2.len(),
                        );
                        eprintln!(
                            "  Bytes at pattern match 0x{:08X}: {:02X?} (expected: {:02X?})",
                            hook2_pattern_match, buf, FREECAM_ORIG_PATCH_2
                        );
                        buf == FREECAM_ORIG_PATCH_2
                    };

                // Use calculated addresses ONLY if bytes match, otherwise use pattern match
                FREECAM_INJECT_ADDR_2 = if release_bytes_match {
                    eprintln!(
                        "  ✓ Using calculated RELEASE Hook2 address: 0x{:08X}",
                        hook2_release_calc
                    );
                    hook2_release_calc
                } else if debug_bytes_match {
                    eprintln!(
                        "  ✓ Using calculated DEBUG Hook2 address: 0x{:08X}",
                        hook2_debug_calc
                    );
                    hook2_debug_calc
                } else {
                    eprintln!(
                        "  ! Using pattern-matched Hook2 address: 0x{:08X} (calc addresses had wrong bytes)",
                        hook2_pattern_match
                    );
                    hook2_pattern_match
                };

                FREECAM_DEBUG_PAIR_DELTA = FREECAM_INJECT_ADDR_2 - pair_1;
                FREECAM_DEBUG_PAIR_DELTA_ERR = FREECAM_DEBUG_PAIR_DELTA
                    .abs_diff(offsets::FREECAM_INJECT_2_DEBUG_RVA - offsets::FREECAM_INJECT_1_DEBUG_RVA);
                FREECAM_DEBUG_PAIR_CALL_REF = pair_call_ref;
                eprintln!(
                    "  ✓ Final inject pair: 0x{:08X} / 0x{:08X} delta=0x{:X} err=0x{:X} call_ref={}",
                    FREECAM_INJECT_ADDR_1,
                    FREECAM_INJECT_ADDR_2,
                    FREECAM_DEBUG_PAIR_DELTA,
                    FREECAM_DEBUG_PAIR_DELTA_ERR,
                    pair_call_ref
                );
            } else {
                // Scan for injection point 1
                match self.process.scan_abs("freecam_inject_1", &offsets::FREECAM_INJECT_1_AOB, 0, vec![]) {
                    Ok(ptr1) => {
                        FREECAM_INJECT_ADDR_1 = ptr1.base_address;
                        eprintln!("  ✓ Inject point 1 found: 0x{:08X}", ptr1.base_address);
                    }
                    Err(e) => {
                        eprintln!("  ✗ Inject point 1 NOT found: {}", e);
                        FREECAM_INJECT_ADDR_1 = 0;
                    }
                }

                // Scan for injection point 2
                match self.process.scan_abs("freecam_inject_2", &offsets::FREECAM_INJECT_2_AOB, 0, vec![]) {
                    Ok(ptr2) => {
                        FREECAM_INJECT_ADDR_2 = ptr2.base_address;
                        eprintln!("  ✓ Inject point 2 found: 0x{:08X}", ptr2.base_address);
                    }
                    Err(e) => {
                        eprintln!("  ✗ Inject point 2 NOT found: {}", e);
                        FREECAM_INJECT_ADDR_2 = 0;
                    }
                }

                if FREECAM_INJECT_ADDR_1 != 0 && FREECAM_INJECT_ADDR_2 > FREECAM_INJECT_ADDR_1 {
                    FREECAM_DEBUG_PAIR_DELTA = FREECAM_INJECT_ADDR_2 - FREECAM_INJECT_ADDR_1;
                    FREECAM_DEBUG_PAIR_DELTA_ERR = FREECAM_DEBUG_PAIR_DELTA
                        .abs_diff(offsets::FREECAM_INJECT_2_DEBUG_RVA - offsets::FREECAM_INJECT_1_DEBUG_RVA);
                }
            }

            if FREECAM_INJECT_ADDR_2 != 0 {
                FREECAM_HOOK2_SKIP_ADDR = resolve_hook2_skip_target(FREECAM_INJECT_ADDR_2);

                eprintln!("  ✓ Hook2 skip target: 0x{:08X}", FREECAM_HOOK2_SKIP_ADDR);

                let inject2_rva = FREECAM_INJECT_ADDR_2.saturating_sub(offsets::FREECAM_IMAGE_BASE);
                FREECAM_MOTION_FUNC = match inject2_rva {
                    offsets::FREECAM_INJECT_2_DEBUG_RVA => {
                        eprintln!("  ✓ Freecam variant: debug RVA set");
                        // Keep as diagnostic only; inline call path can crash in pause/menu states.
                        FREECAM_MOTION_CALL_SAFE = 0;
                        offsets::FREECAM_IMAGE_BASE + offsets::FREECAM_MOTION_FUNC_DEBUG_RVA
                    }
                    offsets::FREECAM_INJECT_2_RELEASE_RVA => {
                        eprintln!(
                            "  ✓ Freecam variant: release RVA set (debug - 0x{:X})",
                            offsets::FREECAM_DEBUG_TO_RELEASE_DELTA
                        );
                        // Keep as diagnostic only; inline call path can crash in pause/menu states.
                        FREECAM_MOTION_CALL_SAFE = 0;
                        offsets::FREECAM_IMAGE_BASE + offsets::FREECAM_MOTION_FUNC_RELEASE_RVA
                    }
                    _ => {
                        let legacy_candidate = FREECAM_INJECT_ADDR_2.saturating_sub(0x5D1);
                        let nearby_candidate = find_best_nearby_rel32_call_target(
                            FREECAM_INJECT_ADDR_2,
                            legacy_candidate,
                            0x4000,
                            0x400,
                        )
                        .unwrap_or(legacy_candidate);

                        // Keep unknown variants pinned to CT-derived baseline for stability.
                        // Nearby call target remains diagnostic only.
                        let candidate = legacy_candidate;
                        let executable = is_executable_addr(candidate, 16);
                        let has_call_ref = has_nearby_rel32_call_to_target(
                            FREECAM_INJECT_ADDR_2,
                            candidate,
                            0x2000,
                            0x200,
                        );
                        let strong_unique_pair = FREECAM_DEBUG_INJECT_MATCHES_1 == 1
                            && FREECAM_DEBUG_INJECT_MATCHES_2 == 1
                            && FREECAM_DEBUG_PAIR_DELTA_ERR <= 0x1000;
                        let near_legacy = nearby_candidate.abs_diff(legacy_candidate) <= 0x800;
                        eprintln!(
                            "  ! Inject2 RVA 0x{:X} unknown; legacy=0x{:08X} nearby=0x{:08X} chosen=0x{:08X} exec={} call_ref={} unique_pair={} near_legacy={}",
                            inject2_rva,
                            legacy_candidate,
                            nearby_candidate,
                            candidate,
                            executable,
                            has_call_ref,
                            strong_unique_pair,
                            near_legacy
                        );

                        // Unknown variant safety: keep target pinned to CT baseline and allow calls
                        // only when either structural call reference exists or inject pair is uniquely
                        // matched and close to CT delta.
                        FREECAM_MOTION_CALL_SAFE = if executable
                            && near_legacy
                            && (has_call_ref || strong_unique_pair)
                        {
                            0 // Disabled: motion call interferes with manual position writes
                        } else {
                            0
                        };
                        candidate
                    }
                };
                eprintln!(
                    "  ✓ Motion function resolved: 0x{:08X} (safe={})",
                    FREECAM_MOTION_FUNC, FREECAM_MOTION_CALL_SAFE
                );
            } else {
                FREECAM_MOTION_FUNC = 0;
                FREECAM_MOTION_CALL_SAFE = 0;
            }

            // Scan for camera manager pointer
            match self.process.scan_abs(
                "freecam_cam_mgr",
                &offsets::FREECAM_CAM_MGR_AOB,
                offsets::FREECAM_CAM_MGR_AOB_OFFSET,
                vec![],
            ) {
                Ok(cam_mgr_ptr) => {
                    // Store the ADDRESS where the pointer is stored (not the pointer value itself)
                    FREECAM_CAM_MGR_PTR = cam_mgr_ptr.base_address;
                    eprintln!(
                        "  ✓ Camera manager pointer address: 0x{:08X}",
                        cam_mgr_ptr.base_address
                    );
                }
                Err(e) => {
                    eprintln!("  ✗ Camera manager pointer NOT found: {}", e);
                    FREECAM_CAM_MGR_PTR = 0;
                }
            }

            // Scan for ChrFollowCam pointer (actual render camera)
            match self.process.scan_abs(
                "chr_follow_cam",
                &offsets::CHR_FOLLOW_CAM_AOB,
                offsets::CHR_FOLLOW_CAM_AOB_OFFSET,
                vec![],
            ) {
                Ok(chr_cam_ptr) => {
                    CHR_FOLLOW_CAM_PTR = chr_cam_ptr.base_address;
                    eprintln!(
                        "  ✓ ChrFollowCam pointer address: 0x{:08X}",
                        chr_cam_ptr.base_address
                    );
                }
                Err(e) => {
                    eprintln!("  ✗ ChrFollowCam pointer NOT found: {}", e);
                    CHR_FOLLOW_CAM_PTR = 0;
                }
            }

            // TODO: Camera follow disable patch - currently commented out due to freeze during injection
            // We'll add this as a separate function to run after injection is stable
            // Pattern: "80 7C 24 38 00 74 70 F3 0F 7E B4" - patch byte at offset +5 from 0x74 (je) to 0xEB (jmp)
        }
    }
}
