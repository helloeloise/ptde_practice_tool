use crate::config::Config;
use crate::memory::constants::CharData2;
use crate::memory::constants::{self, AnimData, CharData1, CharMapData, LevelUp};
use crate::memory::offsets;
use crate::memory::{Ds1, ds1};
use imgui::Condition;
use mem_rs::prelude::*;
use std::sync::{Arc, Mutex};

pub struct DebugInfo {
    current_anim_id: i32,
    anim_speed: f32,
    equip_left_1_idx: i32,
    equip_left_1_id: i32,
    equip_right_1_idx: i32,
    equip_right_1_id: i32,
    equip_left_2_idx: i32,
    equip_left_2_id: i32,
    equip_right_2_idx: i32,
    equip_right_2_id: i32,
    equip_arrow_1_idx: i32,
    equip_arrow_1_id: i32,
    equip_bolt_1_idx: i32,
    equip_bolt_1_id: i32,
    equip_arrow_2_idx: i32,
    equip_arrow_2_id: i32,
    equip_bolt_2_idx: i32,
    equip_bolt_2_id: i32,
    equip_helmet_idx: i32,
    equip_helmet_id: i32,
    equip_chest_idx: i32,
    equip_chest_id: i32,
    equip_glove_idx: i32,
    equip_glove_id: i32,
    equip_pants_idx: i32,
    equip_pants_id: i32,
    equip_hair_idx: i32,
    equip_hair_id: i32,
    equip_ring_1_idx: i32,
    equip_ring_1_id: i32,
    equip_ring_2_idx: i32,
    equip_ring_2_id: i32,
    equip_item_1_idx: i32,
    equip_item_1_id: i32,
    equip_item_2_idx: i32,
    equip_item_2_id: i32,
    equip_item_3_idx: i32,
    equip_item_3_id: i32,
    equip_item_4_idx: i32,
    equip_item_4_id: i32,
    equip_item_5_idx: i32,
    equip_item_5_id: i32,
    x_pos: f32,
    y_pos: f32,
    z_pos: f32,
    angle: f32,
    stored_x: f32,
    stored_y: f32,
    stored_z: f32,
    stored_angle: f32,
    stored_pos_set: bool,
    stance: i32,
    current_poise: f32,
    poise_recovery_rate: f32,
    ai_timer: f32,
    ai_id: u32,
    new_game_cycle: i32,
    poison_resist: i32,
    bleed_resist: i32,
    disease_resist: i32,
    curse_resist: i32,
    is_open: bool,
    last_debug_window_save_time: std::time::Instant,
    freecam_log_save_message: Option<String>,
    freecam_log_save_time: std::time::Instant,
}

impl DebugInfo {
    pub fn new() -> Self {
        DebugInfo {
            current_anim_id: 0,
            anim_speed: 1.0,
            equip_left_1_idx: 0,
            equip_left_1_id: 0,
            equip_right_1_idx: 0,
            equip_right_1_id: 0,
            equip_left_2_idx: 0,
            equip_left_2_id: 0,
            equip_right_2_idx: 0,
            equip_right_2_id: 0,
            equip_arrow_1_idx: 0,
            equip_arrow_1_id: 0,
            equip_bolt_1_idx: 0,
            equip_bolt_1_id: 0,
            equip_arrow_2_idx: 0,
            equip_arrow_2_id: 0,
            equip_bolt_2_idx: 0,
            equip_bolt_2_id: 0,
            equip_helmet_idx: 0,
            equip_helmet_id: 0,
            equip_chest_idx: 0,
            equip_chest_id: 0,
            equip_glove_idx: 0,
            equip_glove_id: 0,
            equip_pants_idx: 0,
            equip_pants_id: 0,
            equip_hair_idx: 0,
            equip_hair_id: 0,
            equip_ring_1_idx: 0,
            equip_ring_1_id: 0,
            equip_ring_2_idx: 0,
            equip_ring_2_id: 0,
            equip_item_1_idx: 0,
            equip_item_1_id: 0,
            equip_item_2_idx: 0,
            equip_item_2_id: 0,
            equip_item_3_idx: 0,
            equip_item_3_id: 0,
            equip_item_4_idx: 0,
            equip_item_4_id: 0,
            equip_item_5_idx: 0,
            equip_item_5_id: 0,
            x_pos: 0.0,
            y_pos: 0.0,
            z_pos: 0.0,
            angle: 0.0,
            stored_x: 0.0,
            stored_y: 0.0,
            stored_z: 0.0,
            stored_angle: 0.0,
            stored_pos_set: false,
            stance: 0,
            current_poise: 0.0,
            poise_recovery_rate: 0.0,
            ai_timer: 0.0,
            ai_id: 0,
            new_game_cycle: 0,
            poison_resist: 0,
            bleed_resist: 0,
            disease_resist: 0,
            curse_resist: 0,
            is_open: false,
            last_debug_window_save_time: std::time::Instant::now(),
            freecam_log_save_message: None,
            freecam_log_save_time: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self, ds1: &Ds1) {
        self.current_anim_id = ds1
            .chr_data_1
            .read_i32_rel(Some(CharData1::FORCE_PLAY_ANIMATION));
        self.anim_speed = ds1.anim_data.read_f32_rel(Some(AnimData::PLAY_SPEED));
        self.equip_left_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_LEFT_1_IDX));
        self.equip_left_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_LEFT_1_ID));
        self.equip_right_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RIGHT_1_IDX));
        self.equip_right_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RIGHT_1_ID));
        self.equip_left_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_LEFT_2_IDX));
        self.equip_left_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_LEFT_2_ID));
        self.equip_right_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RIGHT_2_IDX));
        self.equip_right_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RIGHT_2_ID));
        self.equip_arrow_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ARROW_1_IDX));
        self.equip_arrow_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ARROW_1_ID));
        self.equip_bolt_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_BOLT_1_IDX));
        self.equip_bolt_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_BOLT_1_ID));
        self.equip_arrow_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ARROW_2_IDX));
        self.equip_arrow_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ARROW_2_ID));
        self.equip_bolt_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_BOLT_2_IDX));
        self.equip_bolt_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_BOLT_2_ID));
        self.equip_helmet_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_HELMET_IDX));
        self.equip_helmet_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_HELMET_ID));
        self.equip_chest_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_CHEST_IDX));
        self.equip_chest_id = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_CHEST_ID));
        self.equip_glove_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_GLOVE_IDX));
        self.equip_glove_id = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_GLOVE_ID));
        self.equip_pants_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_PANTS_IDX));
        self.equip_pants_id = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_PANTS_ID));
        self.equip_hair_idx = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_HAIR_IDX));
        self.equip_hair_id = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_HAIR_ID));
        self.equip_ring_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RING_1_IDX));
        self.equip_ring_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RING_1_ID));
        self.equip_ring_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RING_2_IDX));
        self.equip_ring_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RING_2_ID));
        self.equip_item_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_1_IDX));
        self.equip_item_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_1_ID));
        self.equip_item_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_2_IDX));
        self.equip_item_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_2_ID));
        self.equip_item_3_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_3_IDX));
        self.equip_item_3_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_3_ID));
        self.equip_item_4_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_4_IDX));
        self.equip_item_4_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_4_ID));
        self.equip_item_5_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_5_IDX));
        self.equip_item_5_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_5_ID));
        self.x_pos = ds1.get_x_pos();
        self.y_pos = ds1.get_y_pos();
        self.z_pos = ds1.get_z_pos();
        self.angle = ds1.get_angle();
        self.stance = ds1.chr_data_2.read_i32_rel(Some(CharData2::STANCE));
        self.current_poise = ds1.chr_data_1.read_f32_rel(Some(CharData1::CURRENT_POISE));
        self.poise_recovery_rate = ds1
            .chr_data_1
            .read_f32_rel(Some(CharData1::POISE_RECOVERY_RATE));
        self.ai_timer = ds1.target_bank.read_f32_rel(Some(0x14));
        self.ai_id = ds1.chr_data_1.read_u32_rel(Some(CharData1::AI_ID));
        self.new_game_cycle = ds1.chr_data_2.read_i32_rel(Some(CharData2::NEW_GAME));
        self.poison_resist = ds1.chr_data_2.read_i32_rel(Some(CharData2::POISON_RESIST));
        self.bleed_resist = ds1.chr_data_2.read_i32_rel(Some(CharData2::BLEED_RESIST));
        self.disease_resist = ds1.chr_data_2.read_i32_rel(Some(CharData2::DISEASE_RESIST));
        self.curse_resist = ds1.chr_data_2.read_i32_rel(Some(CharData2::CURSE_RESIST));
    }

    pub fn get_current_anim_id(&self) -> i32 {
        self.current_anim_id
    }

    pub fn set_stored_position(&mut self, pos: Option<(f32, f32, f32, f32)>) {
        match pos {
            Some((x, y, z, angle)) => {
                self.stored_x = x;
                self.stored_y = y;
                self.stored_z = z;
                self.stored_angle = angle;
                self.stored_pos_set = true;
            }
            None => {
                self.stored_pos_set = false;
            }
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn render_window(&mut self, ui: &imgui::Ui, ds1: &mut Ds1, config: &Arc<Mutex<Config>>) {
        if !self.is_open {
            return;
        }

        let _text_style = ui.push_style_color(imgui::StyleColor::Text, [1.0, 1.0, 1.0, 1.0]);

        let config_guard = config.lock().unwrap();
        let debug_window_layout = config_guard.window_layout.debug_window.clone();
        drop(config_guard);

        let mut debug_window_changed = false;
        let mut new_debug_pos = [0.0, 0.0];
        let mut new_debug_size = [0.0, 0.0];

        ui.window("Debug Info")
            .size(
                [debug_window_layout.width, debug_window_layout.height],
                Condition::FirstUseEver,
            )
            .position(
                [debug_window_layout.pos_x, debug_window_layout.pos_y],
                Condition::FirstUseEver,
            )
            .title_bar(false)
            .draw_background(false)
            .build(|| {
                // Capture window position/size at the start of the frame
                new_debug_pos = ui.window_pos();
                new_debug_size = ui.window_size();
                // Only mark as changed if values actually differ
                if (debug_window_layout.pos_x - new_debug_pos[0]).abs() > 1.0
                    || (debug_window_layout.pos_y - new_debug_pos[1]).abs() > 1.0
                    || (debug_window_layout.width - new_debug_size[0]).abs() > 1.0
                    || (debug_window_layout.height - new_debug_size[1]).abs() > 1.0
                {
                    debug_window_changed = true;
                }

                // Wrap content in scrollable child window
                ui.child_window("debug_content")
                    .size([0.0, 0.0]) // Take all available space
                    .build(|| {

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
                    ds1.chr_data_2.write_i32_rel(
                        Some(CharData2::NEW_GAME),
                        self.new_game_cycle,
                    );
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

                // Freecam hook status
                let freecam_status = if ds1.free_cam_enabled {
                    "[OK] Freecam Hook: Active"
                } else {
                    "Freecam Hook: Inactive"
                };
                let freecam_color = if ds1.free_cam_enabled {
                    [0.0, 1.0, 0.0, 1.0] // Green
                } else {
                    [0.7, 0.7, 0.7, 1.0] // Gray
                };
                let _freecam_style = ui.push_style_color(imgui::StyleColor::Text, freecam_color);
                ui.text(freecam_status);
                drop(_freecam_style);

                // Freecam injection diagnostics - always show to help debug why checkbox unchecks
                let (injection_ok, addr1_valid, addr2_valid, found_addr1, found_addr2, pending, retry_count) = ds1.get_freecam_injection_info();
                let inject_color = if injection_ok {
                    [0.0, 1.0, 0.0, 1.0] // Green
                } else if pending {
                    [1.0, 1.0, 0.0, 1.0] // Yellow - pending
                } else {
                    [1.0, 0.0, 0.0, 1.0] // Red - failed
                };
                let _inject_style = ui.push_style_color(imgui::StyleColor::Text, inject_color);
                if pending {
                    ui.text(format!("Injection: Waiting for game... (retry {})", retry_count / 60));
                } else {
                    ui.text(format!("Injection Success: {}", injection_ok));
                }
                ui.text(format!("Addr1 (0x{:08X}) Valid: {}", 0x00EFD4A4, addr1_valid));
                ui.text(format!("Addr2 (0x{:08X}) Valid: {}", 0x00FF8224, addr2_valid));
                drop(_inject_style);
                
                // Show expected vs found bytes if validation failed
                if !addr1_valid {
                    ui.text(format!("  Expected: {:02X?}", &[0x89, 0x44, 0x24, 0x24, 0x8B, 0x43, 0x44]));
                    ui.text(format!("  Found:    {:02X?}", found_addr1));
                }
                if !addr2_valid {
                    ui.text(format!("  Expected: {:02X?}", &[0xC1, 0xEA, 0x14, 0xF6, 0xC2, 0x01]));
                    ui.text(format!("  Found:    {:02X?}", found_addr2));
                }
                
                // Show hint if pending
                if pending {
                    let _hint_style = ui.push_style_color(imgui::StyleColor::Text, [0.7, 0.7, 0.7, 1.0]);
                    ui.text_wrapped("Waiting for game to load... Load a save or start a new game.");
                    drop(_hint_style);
                }
                
                // Add scan button to find correct addresses
                if !injection_ok && !pending {
                    ui.text(""); // Spacing
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
                    
                    // Display scan results
                    if let Some(ref msg) = self.freecam_log_save_message {
                        if msg.starts_with("Scan:") && self.freecam_log_save_time.elapsed().as_secs() < 10 {
                            let _msg_style = ui.push_style_color(imgui::StyleColor::Text, [1.0, 1.0, 0.0, 1.0]);
                            ui.text_wrapped(msg);
                            drop(_msg_style);
                        } else if msg.starts_with("Scan Error:") && self.freecam_log_save_time.elapsed().as_secs() < 5 {
                            let _msg_style = ui.push_style_color(imgui::StyleColor::Text, [1.0, 0.0, 0.0, 1.0]);
                            ui.text_wrapped(msg);
                            drop(_msg_style);
                        }
                    }
                }

                // Freecam debug logging
                if ds1.free_cam_enabled {
                    let (mode, injection_success, inject_addr1, inject_addr2, cam_mgr_ptr) = ds1.get_freecam_log_stats();
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
                    
                    // Display motion function info
                    ui.separator();
                    ui.text("Memory Scan (code after injection +0x20):");
                    let (scan_addr, func_bytes) = ds1.get_freecam_motion_func_bytes();
                    ui.text(format!("Scan Addr: 0x{:08X}", scan_addr));
                    if !func_bytes.is_empty() {
                        // Display in rows of 16 bytes
                        for (i, chunk) in func_bytes.chunks(16).enumerate() {
                            let offset = i * 16;
                            let bytes_str = chunk.iter()
                                .map(|b| format!("{:02X}", b))
                                .collect::<Vec<_>>()
                                .join(" ");
                            ui.text(format!("+{:02X}: {}", offset, bytes_str));
                        }
                    }
                    
                    // Camera sampling removed in new implementation
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
                    
                    // Display save message for 3 seconds
                    if let Some(ref msg) = self.freecam_log_save_message {
                        if self.freecam_log_save_time.elapsed().as_secs() < 3 {
                            let color = if msg.starts_with("Saved:") {
                                [0.0, 1.0, 0.0, 1.0] // Green for success
                            } else {
                                [1.0, 0.0, 0.0, 1.0] // Red for error
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
                    // Show L3+R3 debug bits
                    let (hook_raw_8, hook_raw_c) = ds1.get_freecam_input_hook_values();
                    let (_pad, buttons, _lx, _ly, _rx, _ry, _lt, _rt) = ds1.get_freecam_xinput_debug_values();
                    let l3_xinput = (buttons & 0x0040) != 0;
                    let r3_xinput = (buttons & 0x0080) != 0;
                    ui.text(format!("XInput L3 Bit: {}", if l3_xinput { 1 } else { 0 }));
                    ui.text(format!("XInput R3 Bit: {}", if r3_xinput { 1 } else { 0 }));
                    ui.text(format!("Hook raw [esi+0x8] (legacy): 0x{:08X}", hook_raw_8));
                    ui.text(format!("Hook raw [esi+0xC] (legacy): 0x{:08X}", hook_raw_c));

                    // Show ESI and raw input values for L3+R3 debug
                    let (esi_val, l3_raw, r3_raw) = ds1.get_freecam_input_debug();
                    ui.text(format!("ESI (input struct): 0x{:08X}", esi_val));
                    ui.text(format!("[esi+0x8]:  0x{:08X}", l3_raw));
                    ui.text(format!("[esi+0xC]: 0x{:08X}", r3_raw));
                }

                ui.separator();

                if ui.collapsing_header("Equipment", imgui::TreeNodeFlags::empty()) {
                    // Weapons
                    if ui.collapsing_header("Weapons", imgui::TreeNodeFlags::empty()) {
                        ui.text("Left Hand 1:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##left1_idx", &mut self.equip_left_1_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_LEFT_1_IDX),
                                self.equip_left_1_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##left1_id", &mut self.equip_left_1_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_LEFT_1_ID),
                                self.equip_left_1_id,
                            );
                        }

                        ui.text("Right Hand 1:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##right1_idx", &mut self.equip_right_1_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_RIGHT_1_IDX),
                                self.equip_right_1_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##right1_id", &mut self.equip_right_1_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_RIGHT_1_ID),
                                self.equip_right_1_id,
                            );
                        }

                        ui.text("Left Hand 2:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##left2_idx", &mut self.equip_left_2_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_LEFT_2_IDX),
                                self.equip_left_2_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##left2_id", &mut self.equip_left_2_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_LEFT_2_ID),
                                self.equip_left_2_id,
                            );
                        }

                        ui.text("Right Hand 2:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##right2_idx", &mut self.equip_right_2_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_RIGHT_2_IDX),
                                self.equip_right_2_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##right2_id", &mut self.equip_right_2_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_RIGHT_2_ID),
                                self.equip_right_2_id,
                            );
                        }
                    }

                    // Ammo
                    if ui.collapsing_header("Ammo", imgui::TreeNodeFlags::empty()) {
                        ui.text("Arrow 1:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##arrow1_idx", &mut self.equip_arrow_1_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ARROW_1_IDX),
                                self.equip_arrow_1_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##arrow1_id", &mut self.equip_arrow_1_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ARROW_1_ID),
                                self.equip_arrow_1_id,
                            );
                        }

                        ui.text("Bolt 1:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##bolt1_idx", &mut self.equip_bolt_1_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_BOLT_1_IDX),
                                self.equip_bolt_1_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##bolt1_id", &mut self.equip_bolt_1_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_BOLT_1_ID),
                                self.equip_bolt_1_id,
                            );
                        }

                        ui.text("Arrow 2:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##arrow2_idx", &mut self.equip_arrow_2_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ARROW_2_IDX),
                                self.equip_arrow_2_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##arrow2_id", &mut self.equip_arrow_2_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ARROW_2_ID),
                                self.equip_arrow_2_id,
                            );
                        }

                        ui.text("Bolt 2:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##bolt2_idx", &mut self.equip_bolt_2_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_BOLT_2_IDX),
                                self.equip_bolt_2_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##bolt2_id", &mut self.equip_bolt_2_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_BOLT_2_ID),
                                self.equip_bolt_2_id,
                            );
                        }
                    }

                    // Armor
                    if ui.collapsing_header("Armor", imgui::TreeNodeFlags::empty()) {
                        ui.text("Helmet:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##helmet_idx", &mut self.equip_helmet_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_HELMET_IDX),
                                self.equip_helmet_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##helmet_id", &mut self.equip_helmet_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_HELMET_ID),
                                self.equip_helmet_id,
                            );
                        }

                        ui.text("Chest:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##chest_idx", &mut self.equip_chest_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_CHEST_IDX),
                                self.equip_chest_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui.input_int("##chest_id", &mut self.equip_chest_id).build() {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_CHEST_ID),
                                self.equip_chest_id,
                            );
                        }

                        ui.text("Gloves:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##glove_idx", &mut self.equip_glove_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_GLOVE_IDX),
                                self.equip_glove_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui.input_int("##glove_id", &mut self.equip_glove_id).build() {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_GLOVE_ID),
                                self.equip_glove_id,
                            );
                        }

                        ui.text("Pants:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##pants_idx", &mut self.equip_pants_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_PANTS_IDX),
                                self.equip_pants_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui.input_int("##pants_id", &mut self.equip_pants_id).build() {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_PANTS_ID),
                                self.equip_pants_id,
                            );
                        }

                        ui.text("Hair:");
                        ui.set_next_item_width(100.0);
                        if ui.input_int("##hair_idx", &mut self.equip_hair_idx).build() {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_HAIR_IDX),
                                self.equip_hair_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui.input_int("##hair_id", &mut self.equip_hair_id).build() {
                            ds1.chr_data_2
                                .write_i32_rel(Some(CharData2::EQUIP_HAIR_ID), self.equip_hair_id);
                        }
                    }

                    // Rings
                    if ui.collapsing_header("Rings", imgui::TreeNodeFlags::empty()) {
                        ui.text("Ring 1:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##ring1_idx", &mut self.equip_ring_1_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_RING_1_IDX),
                                self.equip_ring_1_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##ring1_id", &mut self.equip_ring_1_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_RING_1_ID),
                                self.equip_ring_1_id,
                            );
                        }

                        ui.text("Ring 2:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##ring2_idx", &mut self.equip_ring_2_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_RING_2_IDX),
                                self.equip_ring_2_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##ring2_id", &mut self.equip_ring_2_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_RING_2_ID),
                                self.equip_ring_2_id,
                            );
                        }
                    }

                    // Items
                    if ui.collapsing_header("Items", imgui::TreeNodeFlags::empty()) {
                        ui.text("Item 1:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item1_idx", &mut self.equip_item_1_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_1_IDX),
                                self.equip_item_1_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item1_id", &mut self.equip_item_1_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_1_ID),
                                self.equip_item_1_id,
                            );
                        }

                        ui.text("Item 2:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item2_idx", &mut self.equip_item_2_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_2_IDX),
                                self.equip_item_2_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item2_id", &mut self.equip_item_2_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_2_ID),
                                self.equip_item_2_id,
                            );
                        }

                        ui.text("Item 3:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item3_idx", &mut self.equip_item_3_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_3_IDX),
                                self.equip_item_3_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item3_id", &mut self.equip_item_3_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_3_ID),
                                self.equip_item_3_id,
                            );
                        }

                        ui.text("Item 4:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item4_idx", &mut self.equip_item_4_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_4_IDX),
                                self.equip_item_4_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item4_id", &mut self.equip_item_4_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_4_ID),
                                self.equip_item_4_id,
                            );
                        }

                        ui.text("Item 5:");
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item5_idx", &mut self.equip_item_5_idx)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_5_IDX),
                                self.equip_item_5_idx,
                            );
                        }
                        ui.same_line();
                        ui.set_next_item_width(100.0);
                        if ui
                            .input_int("##item5_id", &mut self.equip_item_5_id)
                            .build()
                        {
                            ds1.chr_data_2.write_i32_rel(
                                Some(CharData2::EQUIP_ITEM_5_ID),
                                self.equip_item_5_id,
                            );
                        }
                    }
                }

                }); // End of scrollable child window

            });

        // Track debug window layout changes and persist to disk (throttled to 2s intervals)
        if debug_window_changed {
            let mut config_guard = config.lock().unwrap();
            let layout = &mut config_guard.window_layout.debug_window;
            layout.pos_x = new_debug_pos[0];
            layout.pos_y = new_debug_pos[1];
            layout.width = new_debug_size[0];
            layout.height = new_debug_size[1];
            if self.last_debug_window_save_time.elapsed().as_secs() >= 2 {
                let _ = config_guard.save();
                self.last_debug_window_save_time = std::time::Instant::now();
            }
        }
    }
}
