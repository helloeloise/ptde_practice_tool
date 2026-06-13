use mem_rs::memory::ReadWrite;

use crate::memory::Ds1;
use crate::memory::constants::CharData1;
use crate::render_loop::RenderLoop;

impl RenderLoop {
    pub(super) fn sync_flags_if_needed(&mut self, ds1: &mut Ds1) {
        // Only sync flags every 3 seconds and only if at least one flag is enabled.
        let any_flag_enabled = self.no_stamina_consume
            || self.infinite_stamina
            || self.freeze_poise
            || self.infinite_magic
            || self.infinite_goods
            || self.player_hide
            || self.player_silence
            || self.no_death
            || self.player_exterminate
            || self.no_damage
            || self.no_hit
            || self.no_attack
            || self.no_move
            || self.no_update_ai
            || self.disable_collision
            || self.no_gravity
            || self.draw_direction
            || self.draw_counter
            || self.draw_stable_pos
            || self.disable_enemies
            || self.disable_events
            || self.auto_save_disabled
            || self.offline_mode
            || self.free_cam;

        if !any_flag_enabled || self.last_flag_sync_time.elapsed().as_secs() < 3 {
            return;
        }

        // Batch all flag writes together.
        if self.no_stamina_consume {
            ds1.set_no_stam_consume_to(true);
        }
        if self.infinite_stamina {
            let max_stamina = ds1.chr_data_1.read_i32_rel(Some(CharData1::MAX_STAMINA));
            ds1.chr_data_1
                .write_i32_rel(Some(CharData1::STAMINA), max_stamina.max(0));
        }
        if self.freeze_poise {
            ds1.set_freeze_poise_to(true);
        }
        if self.infinite_magic {
            ds1.set_all_no_magic_quantity_consume_to(true);
        }
        if self.infinite_goods {
            ds1.set_no_goods_consume_to(true);
        }
        if self.player_hide {
            ds1.set_player_hide_to(true);
        }
        if self.player_silence {
            ds1.set_player_silence_to(true);
        }
        if self.no_death {
            ds1.set_no_death_to(true);
        }
        if self.player_exterminate {
            ds1.set_player_exterminate(true);
        }
        if self.no_damage {
            ds1.set_no_damage_to(true);
        }
        if self.no_hit {
            ds1.set_no_hit_to(true);
        }
        if self.no_attack {
            ds1.set_no_attack_to(true);
        }
        if self.no_move {
            ds1.set_no_move_to(true);
        }
        if self.no_update_ai {
            ds1.set_no_update_ai_to(true);
        }
        if self.disable_collision {
            ds1.set_disable_collision_to(true);
        }
        if self.no_gravity {
            ds1.set_no_gravity_to(true);
        }
        if self.draw_direction {
            ds1.set_draw_direction_to(true);
        }
        if self.draw_counter {
            ds1.set_draw_counter_to(true);
        }
        if self.draw_stable_pos {
            ds1.set_draw_stable_pos_to(true);
        }
        if self.disable_enemies {
            ds1.set_disable_enemies_to(true);
        }
        if self.disable_events {
            ds1.set_disable_events_to(true);
        }
        if self.auto_save_disabled {
            ds1.set_auto_save_to(false);
        }
        if self.offline_mode {
            ds1.set_online_mode_to(false);
        }
        if self.free_cam {
            ds1.set_free_cam_to(true);
            self.free_cam = ds1.get_free_cam();
        }
        self.last_flag_sync_time = std::time::Instant::now();
    }
}
