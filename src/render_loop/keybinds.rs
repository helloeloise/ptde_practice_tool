use mem_rs::memory::ReadWrite;

use crate::memory::constants::{AnimData, CharData2};
use crate::memory::Ds1;
use crate::render_loop::RenderLoop;
use crate::ui::Player;

impl RenderLoop {
    pub(super) fn process_keybinds(&mut self, ui: &imgui::Ui, ds1: &mut Ds1, ui_wants_input: bool) {
        // Skip keybind handling while user is typing or interacting with text fields.
        if ui_wants_input {
            return;
        }

        // Instantiate player once for all keybinds that need position data.
        // This remains lazy and only allocates when position hotkeys are used.
        let mut player_for_keybinds: Option<Player> = None;

        if let Some(key) = self.resolved_keybinds.quitout {
            if ui.is_key_pressed(key) {
                ds1.quitout.write_u32_rel(Some(0x0), 0x2);
            }
        }

        if let Some(key) = self.resolved_keybinds.moveswap {
            if ui.is_key_pressed(key) {
                if player_for_keybinds.is_none() {
                    let mut p = Player::new();
                    p.instantiate_position_only(ds1);
                    player_for_keybinds = Some(p);
                }
                player_for_keybinds.as_mut().unwrap().moveswap(ds1);
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_gravity {
            if ui.is_key_pressed(key) {
                ds1.set_no_gravity();
                self.no_gravity = !self.no_gravity;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_collision {
            if ui.is_key_pressed(key) {
                ds1.set_disable_collision();
                self.disable_collision = !self.disable_collision;
            }
        }

        if let Some(key) = self.resolved_keybinds.load_position_1 {
            if ui.is_key_pressed(key) {
                if let Some(pos) = self.stored_positions[0] {
                    ds1.teleport_player(pos.0, pos.1, pos.2, pos.3);
                    self.pending_angle_write = Some((pos.3, 10));
                    // Restore HP using chr_data_1 offset (current HP).
                    ds1.chr_data_1.write_i32_rel(Some(0x2D4), pos.4);
                }
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_update_ai {
            if ui.is_key_pressed(key) {
                ds1.set_no_update_ai();
                self.no_update_ai = !self.no_update_ai;
            }
        }

        if let Some(key) = self.resolved_keybinds.teleport_down {
            if ui.is_key_pressed(key) {
                if player_for_keybinds.is_none() {
                    let mut p = Player::new();
                    p.instantiate_position_only(ds1);
                    player_for_keybinds = Some(p);
                }
                let player = player_for_keybinds.as_ref().unwrap();
                ds1.teleport_player(player.x_pos, player.y_pos - 5.0, player.z_pos, player.angle);
                self.pending_angle_write = Some((player.angle, 10));
            }
        }

        if let Some(key) = self.resolved_keybinds.teleport_up {
            if ui.is_key_pressed(key) {
                if player_for_keybinds.is_none() {
                    let mut p = Player::new();
                    p.instantiate_position_only(ds1);
                    player_for_keybinds = Some(p);
                }
                let player = player_for_keybinds.as_ref().unwrap();
                ds1.teleport_player(player.x_pos, player.y_pos + 5.0, player.z_pos, player.angle);
                self.pending_angle_write = Some((player.angle, 10));
            }
        }

        if let Some(key) = self.resolved_keybinds.store_position_1 {
            if !self.menu_open && ui.is_key_pressed_no_repeat(key) {
                if player_for_keybinds.is_none() {
                    let mut p = Player::new();
                    p.instantiate_position_only(ds1);
                    player_for_keybinds = Some(p);
                }
                let player = player_for_keybinds.as_ref().unwrap();
                self.stored_positions[0] = Some((
                    player.x_pos,
                    player.y_pos,
                    player.z_pos,
                    player.angle,
                    player.hp,
                ));
            }
        }

        if let Some(key) = self.resolved_keybinds.store_position_1_angle_only {
            if !self.menu_open && ui.is_key_pressed_no_repeat(key) {
                if player_for_keybinds.is_none() {
                    let mut p = Player::new();
                    p.instantiate_position_only(ds1);
                    player_for_keybinds = Some(p);
                }
                let player = player_for_keybinds.as_ref().unwrap();
                match self.stored_positions[0] {
                    Some(ref mut pos) => pos.3 = player.angle,
                    None => {
                        self.stored_positions[0] = Some((
                            player.x_pos,
                            player.y_pos,
                            player.z_pos,
                            player.angle,
                            player.hp,
                        ))
                    }
                }
            }
        }

        if let Some(key) = self.resolved_keybinds.store_position_1_no_angle {
            if !self.menu_open && ui.is_key_pressed_no_repeat(key) {
                if player_for_keybinds.is_none() {
                    let mut p = Player::new();
                    p.instantiate_position_only(ds1);
                    player_for_keybinds = Some(p);
                }
                let player = player_for_keybinds.as_ref().unwrap();
                match self.stored_positions[0] {
                    Some(ref mut pos) => {
                        pos.0 = player.x_pos;
                        pos.1 = player.y_pos;
                        pos.2 = player.z_pos;
                        pos.4 = player.hp;
                    }
                    None => {
                        self.stored_positions[0] = Some((
                            player.x_pos,
                            player.y_pos,
                            player.z_pos,
                            player.angle,
                            player.hp,
                        ))
                    }
                }
            }
        }

        if let Some(key) = self.resolved_keybinds.restore_full_hp {
            if ui.is_key_pressed(key) {
                // Read max HP from CharData2 and write it to current HP in CharData1.
                let max_hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::MAX_HP));
                ds1.chr_data_1.write_i32_rel(Some(0x2D4), max_hp);
            }
        }

        if let Some(key) = self.resolved_keybinds.rtsr_range {
            if ui.is_key_pressed(key) {
                // Set HP to 19% of max HP (under 20% for RTSR activation).
                let max_hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::MAX_HP));
                let rtsr_hp = (max_hp as f32 * 0.19) as i32;
                ds1.chr_data_1.write_i32_rel(Some(0x2D4), rtsr_hp);
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_stamina {
            if ui.is_key_pressed(key) {
                ds1.set_no_stam_consume();
                self.no_stamina_consume = !self.no_stamina_consume;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_infinite_magic {
            if ui.is_key_pressed(key) {
                ds1.set_all_no_magic_quantity_consume();
                self.infinite_magic = !self.infinite_magic;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_infinite_goods {
            if ui.is_key_pressed(key) {
                ds1.set_no_goods_consume();
                self.infinite_goods = !self.infinite_goods;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_player_hide {
            if ui.is_key_pressed(key) {
                ds1.set_player_hide();
                self.player_hide = !self.player_hide;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_player_silence {
            if ui.is_key_pressed(key) {
                ds1.set_player_silence();
                self.player_silence = !self.player_silence;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_death {
            if ui.is_key_pressed(key) {
                ds1.set_no_death();
                self.no_death = !self.no_death;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_damage {
            if ui.is_key_pressed(key) {
                ds1.set_no_damage();
                self.no_damage = !self.no_damage;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_hit {
            if ui.is_key_pressed(key) {
                ds1.set_no_hit();
                self.no_hit = !self.no_hit;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_attack {
            if ui.is_key_pressed(key) {
                ds1.set_no_attack();
                self.no_attack = !self.no_attack;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_no_move {
            if ui.is_key_pressed(key) {
                ds1.set_no_move();
                self.no_move = !self.no_move;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_draw_direction {
            if ui.is_key_pressed(key) {
                ds1.set_draw_direction();
                self.draw_direction = !self.draw_direction;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_draw_counter {
            if ui.is_key_pressed(key) {
                ds1.set_draw_counter();
                self.draw_counter = !self.draw_counter;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_draw_stable_pos {
            if ui.is_key_pressed(key) {
                ds1.set_draw_stable_pos();
                self.draw_stable_pos = !self.draw_stable_pos;
            }
        }

        if let Some(key) = self.resolved_keybinds.toggle_debug_info {
            if ui.is_key_pressed(key) {
                self.debug_info.toggle();
            }
        }

        // Toggle animation speed (also a keybind, guard it here so it
        // fires only when the user is not typing in an input field).
        if ui.is_key_pressed(imgui::Key::Keypad2) {
            if self.anim_speed_toggled {
                self.anim_speed = self.saved_anim_speed;
                self.anim_speed_toggled = false;
            } else {
                self.saved_anim_speed = self.anim_speed;
                self.anim_speed = 1.0;
                self.anim_speed_toggled = true;
            }
            ds1.anim_data
                .write_f32_rel(Some(AnimData::PLAY_SPEED), self.anim_speed);
        }
    }
}
