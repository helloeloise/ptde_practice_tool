use crate::config::Config;
use crate::memory::constants::CharData2;
use crate::memory::constants::{self, AnimData, CharData1, CharMapData, LevelUp};
use crate::memory::offsets;
use crate::memory::{Ds1, ds1};
use imgui::Condition;
use mem_rs::prelude::*;
use std::sync::{Arc, Mutex};

mod render_sections;
mod render_equipment;
mod update;

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
                    .border(false)
                    .build(|| {

                self.render_summary_section(ui, ds1);
                self.render_freecam_section(ui, ds1);

                self.render_equipment_section(ui, ds1);

                ui.separator();
                self.render_xinput_section(ui, ds1);
                ui.separator();
                self.render_keyboard_section(ui);

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
