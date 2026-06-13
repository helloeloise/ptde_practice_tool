use mem_rs::memory::ReadWrite;

use crate::memory::Ds1;
use crate::memory::constants::AnimData;
use crate::render_loop::RenderLoop;

impl RenderLoop {
    pub(super) fn handle_eject_action(&mut self, ui: &imgui::Ui, ds1: &mut Ds1) {
        if !ui.button("Eject") {
            return;
        }

        println!("Eject button pressed!");

        // Restore all modified memory values to their original state before ejecting.
        println!("Restoring memory values to default state...");

        // Re-enable player input.
        ds1.input_state.write_u8_rel(None, 0x1);

        // Disable all debug flags.
        ds1.set_no_stam_consume_to(false);
        ds1.set_freeze_poise_to(false);
        ds1.set_all_no_magic_quantity_consume_to(false);
        ds1.set_no_goods_consume_to(false);
        ds1.set_player_hide_to(false);
        ds1.set_player_silence_to(false);
        ds1.set_no_death_to(false);
        ds1.set_player_exterminate(false);
        ds1.set_no_damage_to(false);
        ds1.set_no_hit_to(false);
        ds1.set_no_attack_to(false);
        ds1.set_no_move_to(false);
        ds1.set_no_update_ai_to(false);
        ds1.set_disable_collision_to(false);
        ds1.set_no_gravity_to(false);
        ds1.set_draw_direction_to(false);
        ds1.set_draw_counter_to(false);
        ds1.set_draw_stable_pos_to(false);
        ds1.set_disable_enemies_to(false);
        ds1.set_disable_events_to(false);
        ds1.set_auto_save_to(true);
        ds1.set_online_mode_to(true);
        ds1.set_free_cam_to(false);

        println!("Memory cleanup complete. Ejecting...");
        self.tas_runner.stop();
        hudhook::eject();
    }

    pub(super) fn render_animation_speed_control(&mut self, ui: &imgui::Ui, ds1: &mut Ds1) {
        if ui
            .input_float("Animation Speed", &mut self.anim_speed)
            .step(0.1)
            .step_fast(1.0)
            .build()
        {
            self.anim_speed = self.anim_speed.max(0.0);
            ds1.anim_data
                .write_f32_rel(Some(AnimData::PLAY_SPEED), self.anim_speed);
        }
    }
}
