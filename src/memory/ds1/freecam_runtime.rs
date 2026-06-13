use super::*;

impl Ds1 {
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn tick_freecam_mode_hotkey(&mut self) {
        unsafe {
            if !self.free_cam_enabled {
                FREECAM_XINPUT_HELD = 0;
                FREECAM_ACTIVE_PAD = -1;
                return;
            }

            let mut both_pressed = false;
            let mut pressed_idx: i32 = -1;

            for user_idx in 0..4u32 {
                let mut state: XINPUT_STATE = std::mem::zeroed();
                if xinput_get_state(user_idx, &mut state) == 0 {
                    let buttons = state.Gamepad.wButtons;
                    let l3 = (buttons & XINPUT_GAMEPAD_LEFT_THUMB) != 0;
                    let r3 = (buttons & XINPUT_GAMEPAD_RIGHT_THUMB) != 0;
                    if l3 && r3 {
                        both_pressed = true;
                        pressed_idx = user_idx as i32;
                        FREECAM_DEBUG_XINPUT_PAD = user_idx as i32;
                        FREECAM_DEBUG_XINPUT_BUTTONS = buttons;
                        FREECAM_DEBUG_XINPUT_LX = state.Gamepad.sThumbLX;
                        FREECAM_DEBUG_XINPUT_LY = state.Gamepad.sThumbLY;
                        FREECAM_DEBUG_XINPUT_RX = state.Gamepad.sThumbRX;
                        FREECAM_DEBUG_XINPUT_RY = state.Gamepad.sThumbRY;
                        FREECAM_DEBUG_XINPUT_LT = state.Gamepad.bLeftTrigger;
                        FREECAM_DEBUG_XINPUT_RT = state.Gamepad.bRightTrigger;
                        break;
                    }
                }
            }

            if both_pressed {
                FREECAM_ACTIVE_PAD = pressed_idx;
                if FREECAM_XINPUT_HELD == 0 {
                    // Fallback mode cycle for builds where game-native button bit layout differs.
                    FREECAM_MODE += 1;
                    if FREECAM_MODE >= 4 {
                        FREECAM_MODE = 0;
                    }
                    FREECAM_XINPUT_HELD = 1;
                }
            } else {
                FREECAM_XINPUT_HELD = 0;
            }
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn tick_freecam_mode_hotkey(&mut self) {}

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn tick_freecam_manual_movement(&mut self) {
        unsafe {
            FREECAM_DEBUG_MANUAL_TICKS = FREECAM_DEBUG_MANUAL_TICKS.wrapping_add(1);

            if !self.free_cam_enabled {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }

            // Apply manual movement in all non-zero modes.
            if FREECAM_MODE == 0 {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }

            // Keep manual movement active in freecam modes so stick/trigger mapping can drive
            // hook-time deltas on release variants where game-native input bits differ.

            if FREECAM_CAM_MGR_PTR == 0 || !is_readable_addr(FREECAM_CAM_MGR_PTR, 4) {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }
            let cam_mgr = std::ptr::read_volatile(FREECAM_CAM_MGR_PTR as *const usize);
            if cam_mgr == 0 || !is_readable_addr(cam_mgr + 0x6F8, 4) {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }
            let cam_obj = std::ptr::read_volatile((cam_mgr + 0x6F8) as *const usize);
            if cam_obj == 0 {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }

            // Some PTDE variants never execute our secondary constraint hook path.
            // Reinforce freecam state directly on the camera object each tick.
            if is_writable_addr(cam_obj + 0x24C, 4) {
                let mode_write = if FREECAM_MODE == 0 {
                    0
                } else if FREECAM_FORCE_MODE2_ACTIVE_ONLY != 0 {
                    2
                } else {
                    FREECAM_MODE
                };
                std::ptr::write_volatile((cam_obj + 0x24C) as *mut i32, mode_write);
            }
            // Mirror flag bytes touched by the original script path around the frame-camera state.
            if is_writable_addr(cam_obj + 0x58, 1) {
                std::ptr::write_volatile((cam_obj + 0x58) as *mut u8, 0);
            }
            if is_writable_addr(cam_obj + 0x59, 1) {
                std::ptr::write_volatile((cam_obj + 0x59) as *mut u8, 0);
            }
            if is_writable_addr(cam_obj + 0x44, 1) {
                std::ptr::write_volatile((cam_obj + 0x44) as *mut u8, 1);
            }

            let mut cam_target = if is_readable_addr(cam_obj + 0x254, 4) {
                std::ptr::read_volatile((cam_obj + 0x254) as *const usize)
            } else {
                0
            };

            if cam_target == 0 {
                cam_target = std::ptr::addr_of_mut!(FREECAM_STORAGE_BUFFER) as *mut u32 as usize;
            }

            let cam_buf_18 = if is_readable_addr(cam_obj + 0x18, 4) {
                std::ptr::read_volatile((cam_obj + 0x18) as *const usize)
            } else {
                0
            };

            let cam_buf_30 = if is_readable_addr(cam_obj + 0x30, 4) {
                std::ptr::read_volatile((cam_obj + 0x30) as *const usize)
            } else {
                0
            };

            if cam_buf_30 != 0 {
                // Keep render-camera control bytes aligned with active freecam movement.
                if is_writable_addr(cam_buf_30 + 0x58, 1) {
                    std::ptr::write_volatile((cam_buf_30 + 0x58) as *mut u8, 0);
                }
                if is_writable_addr(cam_buf_30 + 0x59, 1) {
                    std::ptr::write_volatile((cam_buf_30 + 0x59) as *mut u8, 0);
                }
                if is_writable_addr(cam_buf_30 + 0x44, 1) {
                    std::ptr::write_volatile((cam_buf_30 + 0x44) as *mut u8, 1);
                }
            }

            if cam_buf_30 != 0 {
                if is_readable_addr(cam_buf_30 + 0x3C, 4) {
                    FREECAM_DEBUG_BUF30_TA_X = std::ptr::read_volatile((cam_buf_30 + 0x1C) as *const f32);
                    FREECAM_DEBUG_BUF30_TA_Y = std::ptr::read_volatile((cam_buf_30 + 0x2C) as *const f32);
                    FREECAM_DEBUG_BUF30_TA_Z = std::ptr::read_volatile((cam_buf_30 + 0x3C) as *const f32);
                }
                if is_readable_addr(cam_buf_30 + 0x48, 4) {
                    FREECAM_DEBUG_BUF30_TB_X = std::ptr::read_volatile((cam_buf_30 + 0x40) as *const f32);
                    FREECAM_DEBUG_BUF30_TB_Y = std::ptr::read_volatile((cam_buf_30 + 0x44) as *const f32);
                    FREECAM_DEBUG_BUF30_TB_Z = std::ptr::read_volatile((cam_buf_30 + 0x48) as *const f32);
                }
            }

            let mut state = std::mem::zeroed::<XINPUT_STATE>();
            let mut found = false;
            if FREECAM_ACTIVE_PAD >= 0 && xinput_get_state(FREECAM_ACTIVE_PAD as u32, &mut state) == 0 {
                found = true;
                FREECAM_DEBUG_XINPUT_PAD = FREECAM_ACTIVE_PAD;
            } else {
                for user_idx in 0..4u32 {
                    if xinput_get_state(user_idx, &mut state) == 0 {
                        found = true;
                        FREECAM_ACTIVE_PAD = user_idx as i32;
                        FREECAM_DEBUG_XINPUT_PAD = user_idx as i32;
                        break;
                    }
                }
            }

            if !found {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }

            FREECAM_DEBUG_XINPUT_BUTTONS = state.Gamepad.wButtons;
            FREECAM_DEBUG_XINPUT_LX = state.Gamepad.sThumbLX;
            FREECAM_DEBUG_XINPUT_LY = state.Gamepad.sThumbLY;
            FREECAM_DEBUG_XINPUT_RX = state.Gamepad.sThumbRX;
            FREECAM_DEBUG_XINPUT_RY = state.Gamepad.sThumbRY;
            FREECAM_DEBUG_XINPUT_LT = state.Gamepad.bLeftTrigger;
            FREECAM_DEBUG_XINPUT_RT = state.Gamepad.bRightTrigger;

            let lx = normalize_stick_axis(state.Gamepad.sThumbLX, 250);
            let ly = normalize_stick_axis(state.Gamepad.sThumbLY, 250);
            let rx = normalize_stick_axis(state.Gamepad.sThumbRX, 250);
            let ry = normalize_stick_axis(state.Gamepad.sThumbRY, 250);

            // Support both stick layouts by selecting the stronger input per axis.
            let move_x = if lx.abs() >= rx.abs() { lx } else { rx };
            let move_z = if ly.abs() >= ry.abs() { ly } else { ry };

            let buttons = state.Gamepad.wButtons;
            let up_trig = (state.Gamepad.bRightTrigger as f32 / 255.0).clamp(0.0, 1.0);
            let down_trig = (state.Gamepad.bLeftTrigger as f32 / 255.0).clamp(0.0, 1.0);
            let up_shoulder = if (buttons & XINPUT_GAMEPAD_RIGHT_SHOULDER) != 0 { 1.0 } else { 0.0 };
            let down_shoulder = if (buttons & XINPUT_GAMEPAD_LEFT_SHOULDER) != 0 { 1.0 } else { 0.0 };
            let up = up_trig.max(up_shoulder);
            let down = down_trig.max(down_shoulder);
            let mut speed = 2.50f32;

            if (buttons & XINPUT_GAMEPAD_RIGHT_SHOULDER) != 0 {
                speed = 6.00;
            } else if (buttons & XINPUT_GAMEPAD_LEFT_SHOULDER) != 0 {
                speed = 0.80;
            }

            let dx = move_x * speed;
            let dz = move_z * speed;
            let dy = (up - down) * speed;

            // Keyboard fallback for setups where the game/controller path is not exposed via XInput.
            let key_left = key_down('A' as i32) || key_down(VK_LEFT as i32);
            let key_right = key_down('D' as i32) || key_down(VK_RIGHT as i32);
            let key_forward = key_down('W' as i32) || key_down(VK_UP as i32);
            let key_back = key_down('S' as i32) || key_down(VK_DOWN as i32);
            let key_up = key_down(VK_SPACE as i32);
            let key_downward = key_down(VK_CONTROL as i32);
            let key_fast = key_down(VK_SHIFT as i32);

            let mut kx = 0.0f32;
            let mut ky = 0.0f32;
            let mut kz = 0.0f32;
            if key_right {
                kx += 1.0;
            }
            if key_left {
                kx -= 1.0;
            }
            if key_forward {
                kz += 1.0;
            }
            if key_back {
                kz -= 1.0;
            }
            if key_up {
                ky += 1.0;
            }
            if key_downward {
                ky -= 1.0;
            }

            let key_speed = if key_fast { 6.00 } else { 2.50 };
            let dx = if kx != 0.0 { kx * key_speed } else { dx };
            let dy = if ky != 0.0 { ky * key_speed } else { dy };
            let dz = if kz != 0.0 { kz * key_speed } else { dz };

            FREECAM_DEBUG_MANUAL_AXES_X = dx;
            FREECAM_DEBUG_MANUAL_AXES_Y = dy;
            FREECAM_DEBUG_MANUAL_AXES_Z = dz;
            FREECAM_MOVE_DX = dx;
            FREECAM_MOVE_DY = dy;
            FREECAM_MOVE_DZ = dz;

            if dx.abs() < 0.0001 && dy.abs() < 0.0001 && dz.abs() < 0.0001 {
                return;
            }

            // Apply to primary camera object buffers first (most reliable for this variant).
            let mut wrote_any = false;
            let mut last_write = 0usize;

            if cam_target != 0 && apply_translation_deltas(cam_target, dx, dy, dz) {
                wrote_any = true;
                last_write = cam_target + 0x40;
            }

            if cam_buf_30 != 0 && apply_translation_deltas(cam_buf_30, dx, dy, dz) {
                if !wrote_any {
                    last_write = cam_buf_30 + 0x40;
                }
                wrote_any = true;
            }

            if cam_buf_18 != 0 && apply_translation_deltas(cam_buf_18, dx, dy, dz) {
                if !wrote_any {
                    last_write = cam_buf_18 + 0x40;
                }
                wrote_any = true;
            }

            // Try writing to ChrFollowCam (actual render camera from DSGadget)
            FREECAM_DEBUG_CHR_FOLLOWCAM_PTR = CHR_FOLLOW_CAM_PTR;
            FREECAM_DEBUG_CHR_BASE = 0;
            FREECAM_DEBUG_CHR_L1 = 0;
            FREECAM_DEBUG_CHR_L2 = 0;
            FREECAM_DEBUG_CHR_FOLLOWCAM_OBJ = 0;

            if CHR_FOLLOW_CAM_PTR != 0 && is_readable_addr(CHR_FOLLOW_CAM_PTR, 4) {
                let chr_cam_base = std::ptr::read_volatile(CHR_FOLLOW_CAM_PTR as *const usize);
                FREECAM_DEBUG_CHR_BASE = chr_cam_base;
                if chr_cam_base != 0 && is_readable_addr(chr_cam_base, 4) {
                    let chr_cam_l1 = std::ptr::read_volatile(chr_cam_base as *const usize);
                    FREECAM_DEBUG_CHR_L1 = chr_cam_l1;
                    if chr_cam_l1 != 0 && is_readable_addr(chr_cam_l1 + 0x3C, 4) {
                        let chr_cam_l2 = std::ptr::read_volatile((chr_cam_l1 + 0x3C) as *const usize);
                        FREECAM_DEBUG_CHR_L2 = chr_cam_l2;
                        if chr_cam_l2 != 0 && is_readable_addr(chr_cam_l2 + 0x60, 4) {
                            let chr_cam_l3 = std::ptr::read_volatile((chr_cam_l2 + 0x60) as *const usize);
                            FREECAM_DEBUG_CHR_FOLLOWCAM_OBJ = chr_cam_l3; // Store for debugging
                            if chr_cam_l3 != 0 && is_writable_addr(chr_cam_l3 + 0x108, 4) {
                                // Write to PosX/Y/Z at offsets 0x100, 0x104, 0x108
                                let pos_x = std::ptr::read_volatile((chr_cam_l3 + 0x100) as *const f32);
                                let pos_y = std::ptr::read_volatile((chr_cam_l3 + 0x104) as *const f32);
                                let pos_z = std::ptr::read_volatile((chr_cam_l3 + 0x108) as *const f32);
                                std::ptr::write_volatile((chr_cam_l3 + 0x100) as *mut f32, pos_x + dx);
                                std::ptr::write_volatile((chr_cam_l3 + 0x104) as *mut f32, pos_y + dy);
                                std::ptr::write_volatile((chr_cam_l3 + 0x108) as *mut f32, pos_z + dz);

                                // ALSO write player position so camera follows our freecam instead
                                // This tricks the follow camera into following where we are
                                if is_writable_addr(chr_cam_l3 + 0xE8, 4) {
                                    // RotX/Y/Z (what player is) - write same as camera pos
                                    std::ptr::write_volatile((chr_cam_l3 + 0xE0) as *mut f32, pos_x + dx);
                                    std::ptr::write_volatile((chr_cam_l3 + 0xE4) as *mut f32, pos_y + dy);
                                    std::ptr::write_volatile((chr_cam_l3 + 0xE8) as *mut f32, pos_z + dz);
                                }

                                if !wrote_any {
                                    last_write = chr_cam_l3 + 0x104;
                                }
                                wrote_any = true;
                            }
                        }
                    }
                }
            }

            if wrote_any {
                FREECAM_DEBUG_MANUAL_WRITES = FREECAM_DEBUG_MANUAL_WRITES.wrapping_add(1);
                FREECAM_DEBUG_LAST_WRITE_ADDR = last_write;
                FREECAM_FALLBACK_BUF_BASE = last_write.saturating_sub(0x40);
            } else {
                FREECAM_FALLBACK_BUF_BASE = 0;
            }
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn tick_freecam_manual_movement(&mut self) {}
}
