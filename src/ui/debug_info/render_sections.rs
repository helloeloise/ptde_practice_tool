use super::*;

impl DebugInfo {
    pub(super) fn render_summary_section(&mut self, ui: &imgui::Ui, ds1: &mut Ds1) {
        ui.text(format!(
            "Current Animation ID: {}",
            self.get_current_anim_id()
        ));
        ui.text(format!(
            "Position: ({:.9}, {:.9}, {:.9})",
            self.x_pos, self.y_pos, self.z_pos
        ));
        ui.text(format!("Angle: {:.9}", self.angle));
        if self.stored_pos_set {
            ui.text(format!(
                "Stored:   ({:.9}, {:.9}, {:.9}) | {:.9}",
                self.stored_x, self.stored_y, self.stored_z, self.stored_angle
            ));
        } else {
            ui.text("Stored:   (not set)");
        }
        ui.text(format!("Poise: {:.2}", self.current_poise));
        ui.text(format!("Poise Timer: {:.2}", self.poise_recovery_rate));
        ui.text(format!(
            "AI Timer: {:.2} | AI ID: {}",
            self.ai_timer, self.ai_id
        ));

        ui.text("New Game Cycle:");
        ui.same_line();
        ui.set_next_item_width(100.0);
        if ui.input_int("##new_game_cycle", &mut self.new_game_cycle).build() {
            self.new_game_cycle = self.new_game_cycle.clamp(0, 6);
            ds1.chr_data_2
                .write_i32_rel(Some(CharData2::NEW_GAME), self.new_game_cycle);
        }
        ui.separator();
        ui.text(format!(
            "Poison Resist: {} | Bleed Resist: {}",
            self.poison_resist, self.bleed_resist
        ));
        ui.text(format!(
            "Disease Resist: {} | Curse Resist: {}",
            self.disease_resist, self.curse_resist
        ));
        ui.separator();
    }

    pub(super) fn render_freecam_section(&mut self, ui: &imgui::Ui, ds1: &mut Ds1) {
        let freecam_status = if ds1.free_cam_enabled {
            "[OK] Freecam Hook: Active"
        } else {
            "Freecam Hook: Inactive"
        };
        let freecam_color = if ds1.free_cam_enabled {
            [0.0, 1.0, 0.0, 1.0]
        } else {
            [0.7, 0.7, 0.7, 1.0]
        };
        let _freecam_style = ui.push_style_color(imgui::StyleColor::Text, freecam_color);
        ui.text(freecam_status);
        drop(_freecam_style);

        let (injection_ok, addr1_valid, addr2_valid, found_addr1, found_addr2, pending, retry_count) =
            ds1.get_freecam_injection_info();
        let inject_color = if injection_ok {
            [0.0, 1.0, 0.0, 1.0]
        } else if pending {
            [1.0, 1.0, 0.0, 1.0]
        } else {
            [1.0, 0.0, 0.0, 1.0]
        };
        let _inject_style = ui.push_style_color(imgui::StyleColor::Text, inject_color);
        if pending {
            ui.text(format!(
                "Injection: Waiting for game... (retry {})",
                retry_count / 60
            ));
        } else {
            ui.text(format!("Injection Success: {}", injection_ok));
        }
        ui.text(format!("Addr1 (0x{:08X}) Valid: {}", 0x00EFD4A4, addr1_valid));
        ui.text(format!("Addr2 (0x{:08X}) Valid: {}", 0x00FF8224, addr2_valid));
        drop(_inject_style);

        if !addr1_valid {
            ui.text(format!(
                "  Expected: {:02X?}",
                &[0x89, 0x44, 0x24, 0x24, 0x8B, 0x43, 0x44]
            ));
            ui.text(format!("  Found:    {:02X?}", found_addr1));
        }
        if !addr2_valid {
            ui.text(format!(
                "  Expected: {:02X?}",
                &[0xC1, 0xEA, 0x14, 0xF6, 0xC2, 0x01]
            ));
            ui.text(format!("  Found:    {:02X?}", found_addr2));
        }

        if pending {
            let _hint_style =
                ui.push_style_color(imgui::StyleColor::Text, [0.7, 0.7, 0.7, 1.0]);
            ui.text_wrapped("Waiting for game to load... Load a save or start a new game.");
            drop(_hint_style);
        }

        if !injection_ok && !pending {
            ui.text("");
            if ui.small_button("Scan Memory for Patterns") {
                match ds1.scan_for_freecam_patterns() {
                    Ok(results) => {
                        self.freecam_log_save_message = Some(format!("Scan: {}", results));
                        self.freecam_log_save_time = std::time::Instant::now();
                    }
                    Err(e) => {
                        self.freecam_log_save_message = Some(format!("Scan Error: {}", e));
                        self.freecam_log_save_time = std::time::Instant::now();
                    }
                }
            }
            ui.text_wrapped("Click to scan memory for correct addresses");

            if let Some(ref msg) = self.freecam_log_save_message {
                if msg.starts_with("Scan:") && self.freecam_log_save_time.elapsed().as_secs() < 10 {
                    let _msg_style =
                        ui.push_style_color(imgui::StyleColor::Text, [1.0, 1.0, 0.0, 1.0]);
                    ui.text_wrapped(msg);
                    drop(_msg_style);
                } else if msg.starts_with("Scan Error:")
                    && self.freecam_log_save_time.elapsed().as_secs() < 5
                {
                    let _msg_style =
                        ui.push_style_color(imgui::StyleColor::Text, [1.0, 0.0, 0.0, 1.0]);
                    ui.text_wrapped(msg);
                    drop(_msg_style);
                }
            }
        }

        if ds1.free_cam_enabled {
            let (mode, injection_success, inject_addr1, inject_addr2, cam_mgr_ptr) =
                ds1.get_freecam_log_stats();
            ui.text(format!("Freecam Mode: {}", mode));
            ui.text(format!("Injection Success: {}", injection_success));
            ui.text(format!("Inject Addr 1: 0x{:08X}", inject_addr1));
            ui.text(format!("Inject Addr 2: 0x{:08X}", inject_addr2));
            ui.text(format!("Cam Mgr Ptr Addr: 0x{:08X}", cam_mgr_ptr));

            ui.separator();
            ui.text("Memory Values:");
            let (cam_mgr_value, cam_obj_value, cam_mode_value) = ds1.get_freecam_camera_values();
            ui.text(format!("Camera Manager: 0x{:08X}", cam_mgr_value));
            ui.text(format!("Camera Object: 0x{:08X}", cam_obj_value));
            ui.text(format!("Mode in Memory: {}", cam_mode_value));

            ui.separator();
            ui.text("Debug Info:");
            let (func_addr, cam_obj_dbg, call_count, hook_entry) = ds1.get_freecam_debug_values();
            ui.text(format!("Function Addr: 0x{:08X}", func_addr));
            ui.text(format!("Camera Obj (dbg): 0x{:08X}", cam_obj_dbg));
            ui.text(format!("Motion Call Count: {}", call_count));
            ui.text(format!("Hook Entry Count: {} times", hook_entry));
            let (manual_ticks, manual_writes, mdx, mdy, mdz, last_addr) =
                ds1.get_freecam_manual_debug_values();
            ui.text(format!("Manual Move Ticks: {}", manual_ticks));
            ui.text(format!("Manual Move Writes: {}", manual_writes));
            ui.text(format!("Manual Delta XYZ: {:.5} / {:.5} / {:.5}", mdx, mdy, mdz));
            ui.text(format!("Manual Last Write Addr: 0x{:08X}", last_addr));

            ui.separator();
            ui.text("Memory Scan (code after injection +0x20):");
            let (scan_addr, func_bytes) = ds1.get_freecam_motion_func_bytes();
            ui.text(format!("Scan Addr: 0x{:08X}", scan_addr));
            if !func_bytes.is_empty() {
                for (i, chunk) in func_bytes.chunks(16).enumerate() {
                    let offset = i * 16;
                    let bytes_str = chunk
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<_>>()
                        .join(" ");
                    ui.text(format!("+{:02X}: {}", offset, bytes_str));
                }
            }

            ui.separator();
            ui.text("Note: Simplified implementation.");
            ui.text("Press L3+R3 to cycle modes (0=Normal, 1=Conditional, 2=Free, 3=Reverse).");

            if ui.small_button("Save Status to File") {
                match ds1.save_freecam_log_to_file() {
                    Ok(filename) => {
                        self.freecam_log_save_message = Some(format!("Saved: {}", filename));
                        self.freecam_log_save_time = std::time::Instant::now();
                    }
                    Err(e) => {
                        self.freecam_log_save_message = Some(format!("Error: {}", e));
                        self.freecam_log_save_time = std::time::Instant::now();
                    }
                }
            }

            if let Some(ref msg) = self.freecam_log_save_message {
                if self.freecam_log_save_time.elapsed().as_secs() < 3 {
                    let color = if msg.starts_with("Saved:") {
                        [0.0, 1.0, 0.0, 1.0]
                    } else {
                        [1.0, 0.0, 0.0, 1.0]
                    };
                    let _msg_style = ui.push_style_color(imgui::StyleColor::Text, color);
                    ui.text(msg);
                    drop(_msg_style);
                } else {
                    self.freecam_log_save_message = None;
                }
            }
        }

        if ds1.free_cam_enabled {
            let (hook_raw_8, hook_raw_c) = ds1.get_freecam_input_hook_values();
            let (_pad, buttons, _lx, _ly, _rx, _ry, _lt, _rt) = ds1.get_freecam_xinput_debug_values();
            let l3_xinput = (buttons & 0x0040) != 0;
            let r3_xinput = (buttons & 0x0080) != 0;
            ui.text(format!("XInput L3 Bit: {}", if l3_xinput { 1 } else { 0 }));
            ui.text(format!("XInput R3 Bit: {}", if r3_xinput { 1 } else { 0 }));
            ui.text(format!("Hook raw [esi+0x8] (legacy): 0x{:08X}", hook_raw_8));
            ui.text(format!("Hook raw [esi+0xC] (legacy): 0x{:08X}", hook_raw_c));

            let (esi_val, l3_raw, r3_raw) = ds1.get_freecam_input_debug();
            ui.text(format!("ESI (input struct): 0x{:08X}", esi_val));
            ui.text(format!("[esi+0x8]:  0x{:08X}", l3_raw));
            ui.text(format!("[esi+0xC]: 0x{:08X}", r3_raw));
        }

        ui.separator();
    }

    pub(super) fn render_xinput_section(&mut self, ui: &imgui::Ui, ds1: &mut Ds1) {
        if ui.collapsing_header("XInput Input & Injection", imgui::TreeNodeFlags::empty()) {
            ui.text("Hooks XInputGetState at Windows API level");
            ui.text("Shows real-time button state + allows injection");
            ui.separator();
            static mut XINPUT_HOOK_ENABLED: bool = false;
            unsafe {
                if ui.checkbox("Enable XInput Hook", &mut XINPUT_HOOK_ENABLED) {
                    if XINPUT_HOOK_ENABLED {
                        if ds1.enable_xinput_hook() {
                            println!("[UI] XInput hook enabled successfully");
                        } else {
                            println!("[UI] Failed to enable XInput hook");
                            XINPUT_HOOK_ENABLED = false;
                        }
                    } else {
                        ds1.disable_xinput_hook();
                        println!("[UI] XInput hook disabled");
                    }
                }

                if XINPUT_HOOK_ENABLED {
                    ui.text_colored([0.0, 1.0, 0.0, 1.0], "✓ Hook Active");
                    let call_count = ds1.get_xinput_call_count();
                    let user_index = ds1.get_xinput_last_user_index();
                    ui.text(format!(
                        "XInputGetState calls: {} (controller #{})",
                        call_count,
                        if user_index == 0xFF {
                            "none".to_string()
                        } else {
                            user_index.to_string()
                        }
                    ));
                    if call_count == 0 {
                        ui.text_colored(
                            [1.0, 0.5, 0.0, 1.0],
                            "(Hook not being called - game may use DirectInput)",
                        );
                    } else {
                        ui.text_colored(
                            [0.0, 1.0, 0.0, 1.0],
                            "(Hook is active and being called)",
                        );
                    }

                    ui.separator();
                    ui.text("Current Buttons Pressed:");
                    let buttons = ds1.get_xinput_buttons();

                    if buttons == 0 {
                        ui.text("  (none)");
                    } else {
                        if (buttons & 0x1000) != 0 { ui.text("  ✓ A"); }
                        if (buttons & 0x2000) != 0 { ui.text("  ✓ B"); }
                        if (buttons & 0x4000) != 0 { ui.text("  ✓ X"); }
                        if (buttons & 0x8000) != 0 { ui.text("  ✓ Y"); }
                        if (buttons & 0x0001) != 0 { ui.text("  ✓ D-Up"); }
                        if (buttons & 0x0002) != 0 { ui.text("  ✓ D-Down"); }
                        if (buttons & 0x0004) != 0 { ui.text("  ✓ D-Left"); }
                        if (buttons & 0x0008) != 0 { ui.text("  ✓ D-Right"); }
                        if (buttons & 0x0100) != 0 { ui.text("  ✓ LB"); }
                        if (buttons & 0x0200) != 0 { ui.text("  ✓ RB"); }
                        if (buttons & 0x0040) != 0 { ui.text("  ✓ L3"); }
                        if (buttons & 0x0080) != 0 { ui.text("  ✓ R3"); }
                        if (buttons & 0x0010) != 0 { ui.text("  ✓ Start"); }
                        if (buttons & 0x0020) != 0 { ui.text("  ✓ Back"); }
                        ui.text(format!("Raw: 0x{:04X}", buttons));
                    }

                    ui.separator();
                    ui.text("Analog Values (Current):");
                    let (lt, rt) = ds1.get_xinput_triggers();
                    let (lx, ly) = ds1.get_xinput_left_stick();
                    let (rx, ry) = ds1.get_xinput_right_stick();
                    ui.text(format!("Triggers: LT={} RT={}", lt, rt));
                    ui.text(format!("Left Stick: X={} Y={}", lx, ly));
                    ui.text(format!("Right Stick: X={} Y={}", rx, ry));

                    ui.separator();
                    ui.text("Button Injection:");
                    static mut XINPUT_INJECT_ENABLED: bool = false;
                    if ui.checkbox("Enable Injection", &mut XINPUT_INJECT_ENABLED) {
                        ds1.set_xinput_injection(XINPUT_INJECT_ENABLED);
                    }

                    if XINPUT_INJECT_ENABLED {
                        ui.text_colored([0.0, 1.0, 0.0, 1.0], "Injection Active");

                        ui.text("Button Injection:");
                        if ui.button("Inject A Button") {
                            ds1.inject_xinput_buttons(0x1000);
                            println!("[XINPUT] Injecting A button");
                        }
                        ui.same_line();
                        if ui.button("Inject B") { ds1.inject_xinput_buttons(0x2000); }
                        ui.same_line();
                        if ui.button("Clear") { ds1.inject_xinput_buttons(0); }

                        ui.text("Quick Inject:");
                        if ui.button("X") { ds1.inject_xinput_buttons(0x4000); }
                        ui.same_line();
                        if ui.button("Y") { ds1.inject_xinput_buttons(0x8000); }
                        ui.same_line();
                        if ui.button("LB") { ds1.inject_xinput_buttons(0x0100); }
                        ui.same_line();
                        if ui.button("RB") { ds1.inject_xinput_buttons(0x0200); }

                        ui.text("Button Reference:");
                        ui.text("  A=0x1000 B=0x2000 X=0x4000 Y=0x8000");
                        ui.text("  D-Up=0x0001 D-Down=0x0002 D-Left=0x0004 D-Right=0x0008");
                        ui.text("  LB=0x0100 RB=0x0200 Start=0x0010 Back=0x0020");

                        ui.separator();
                        ui.text("Analog Injection:");
                        static mut INJECT_LT: i32 = 0;
                        static mut INJECT_RT: i32 = 0;
                        ui.text("Inject Triggers (0-255):");
                        if ui.slider("Left Trigger##inject", 0, 255, &mut INJECT_LT) {
                            ds1.inject_xinput_triggers(INJECT_LT as u8, INJECT_RT as u8);
                        }
                        if ui.slider("Right Trigger##inject", 0, 255, &mut INJECT_RT) {
                            ds1.inject_xinput_triggers(INJECT_LT as u8, INJECT_RT as u8);
                        }

                        static mut INJECT_LX: i32 = 0;
                        static mut INJECT_LY: i32 = 0;
                        ui.text("Inject Left Stick (-32768 to 32767):");
                        if ui.slider("Left X##inject", -32768, 32767, &mut INJECT_LX) {
                            ds1.inject_xinput_left_stick(INJECT_LX as i16, INJECT_LY as i16);
                        }
                        if ui.slider("Left Y##inject", -32768, 32767, &mut INJECT_LY) {
                            ds1.inject_xinput_left_stick(INJECT_LX as i16, INJECT_LY as i16);
                        }

                        static mut INJECT_RX: i32 = 0;
                        static mut INJECT_RY: i32 = 0;
                        ui.text("Inject Right Stick (-32768 to 32767):");
                        if ui.slider("Right X##inject", -32768, 32767, &mut INJECT_RX) {
                            ds1.inject_xinput_right_stick(INJECT_RX as i16, INJECT_RY as i16);
                        }
                        if ui.slider("Right Y##inject", -32768, 32767, &mut INJECT_RY) {
                            ds1.inject_xinput_right_stick(INJECT_RX as i16, INJECT_RY as i16);
                        }

                        if ui.button("Clear All Analog") {
                            INJECT_LT = 0;
                            INJECT_RT = 0;
                            INJECT_LX = 0;
                            INJECT_LY = 0;
                            INJECT_RX = 0;
                            INJECT_RY = 0;
                            ds1.inject_xinput_triggers(0, 0);
                            ds1.inject_xinput_left_stick(0, 0);
                            ds1.inject_xinput_right_stick(0, 0);
                        }
                    }
                } else {
                    ui.text("Enable hook to see button state and inject");
                }
            }
        }
    }

    pub(super) fn render_keyboard_section(&self, ui: &imgui::Ui) {
        if ui.collapsing_header("Keyboard Input (ImGui)", imgui::TreeNodeFlags::empty()) {
            ui.text("Shows keys detected by ImGui's input system");
            ui.text("(Same system used for tool keybinds)");
            ui.separator();
            ui.text("Currently Pressed Keys:");

            let mut any_key_pressed = false;
            let letters = [
                ('A', imgui::Key::A), ('B', imgui::Key::B), ('C', imgui::Key::C), ('D', imgui::Key::D),
                ('E', imgui::Key::E), ('F', imgui::Key::F), ('G', imgui::Key::G), ('H', imgui::Key::H),
                ('I', imgui::Key::I), ('J', imgui::Key::J), ('K', imgui::Key::K), ('L', imgui::Key::L),
                ('M', imgui::Key::M), ('N', imgui::Key::N), ('O', imgui::Key::O), ('P', imgui::Key::P),
                ('Q', imgui::Key::Q), ('R', imgui::Key::R), ('S', imgui::Key::S), ('T', imgui::Key::T),
                ('U', imgui::Key::U), ('V', imgui::Key::V), ('W', imgui::Key::W), ('X', imgui::Key::X),
                ('Y', imgui::Key::Y), ('Z', imgui::Key::Z),
            ];
            for (name, key) in &letters {
                if ui.is_key_down(*key) {
                    ui.text(format!("  ✓ {}", name));
                    any_key_pressed = true;
                }
            }

            let numbers = [
                ('0', imgui::Key::Alpha0), ('1', imgui::Key::Alpha1), ('2', imgui::Key::Alpha2),
                ('3', imgui::Key::Alpha3), ('4', imgui::Key::Alpha4), ('5', imgui::Key::Alpha5),
                ('6', imgui::Key::Alpha6), ('7', imgui::Key::Alpha7), ('8', imgui::Key::Alpha8),
                ('9', imgui::Key::Alpha9),
            ];
            for (name, key) in &numbers {
                if ui.is_key_down(*key) {
                    ui.text(format!("  ✓ {}", name));
                    any_key_pressed = true;
                }
            }

            let special_keys = [
                ("Space", imgui::Key::Space),
                ("Enter", imgui::Key::Enter),
                ("Escape", imgui::Key::Escape),
                ("Tab", imgui::Key::Tab),
                ("Backspace", imgui::Key::Backspace),
                ("Left Shift", imgui::Key::LeftShift),
                ("Right Shift", imgui::Key::RightShift),
                ("Left Ctrl", imgui::Key::LeftCtrl),
                ("Right Ctrl", imgui::Key::RightCtrl),
                ("Left Alt", imgui::Key::LeftAlt),
                ("Right Alt", imgui::Key::RightAlt),
                ("Up Arrow", imgui::Key::UpArrow),
                ("Down Arrow", imgui::Key::DownArrow),
                ("Left Arrow", imgui::Key::LeftArrow),
                ("Right Arrow", imgui::Key::RightArrow),
            ];
            for (name, key) in &special_keys {
                if ui.is_key_down(*key) {
                    ui.text(format!("  ✓ {}", name));
                    any_key_pressed = true;
                }
            }

            let f_keys = [
                ("F1", imgui::Key::F1), ("F2", imgui::Key::F2), ("F3", imgui::Key::F3),
                ("F4", imgui::Key::F4), ("F5", imgui::Key::F5), ("F6", imgui::Key::F6),
                ("F7", imgui::Key::F7), ("F8", imgui::Key::F8), ("F9", imgui::Key::F9),
                ("F10", imgui::Key::F10), ("F11", imgui::Key::F11), ("F12", imgui::Key::F12),
            ];
            for (name, key) in &f_keys {
                if ui.is_key_down(*key) {
                    ui.text(format!("  ✓ {}", name));
                    any_key_pressed = true;
                }
            }

            if !any_key_pressed {
                ui.text("  (none)");
            }

            ui.separator();
            ui.text_colored([0.7, 0.7, 0.7, 1.0], "Note: This uses the same input system as tool keybinds");
        }
    }
}
