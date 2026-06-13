use crate::memory::Ds1;
use crate::render_loop::RenderLoop;

impl RenderLoop {
    pub(super) fn render_debug_flags_section(&mut self, ui: &imgui::Ui, ds1: &mut Ds1) {
        if !ui.collapsing_header("Debug Flags", imgui::TreeNodeFlags::DEFAULT_OPEN) {
            return;
        }

        if ui.checkbox("show console", &mut self.show_console) {
            if self.show_console {
                hudhook::alloc_console().ok();
            } else {
                hudhook::free_console().ok();
            }
        }

        if ui.checkbox("inf stam", &mut self.no_stamina_consume) {
            ds1.set_no_stam_consume();
        }

        ui.same_line();
        ui.checkbox("infinite stamina", &mut self.infinite_stamina);

        if ui.checkbox("infinite magic", &mut self.infinite_magic) {
            ds1.set_all_no_magic_quantity_consume();
        }

        if ui.checkbox("infinite goods", &mut self.infinite_goods) {
            ds1.set_no_goods_consume();
        }

        if ui.checkbox("player hide", &mut self.player_hide) {
            ds1.set_player_hide();
        }

        if ui.checkbox("player silence", &mut self.player_silence) {
            ds1.set_player_silence();
        }

        if ui.checkbox("no death", &mut self.no_death) {
            ds1.set_no_death();
        }

        if ui.checkbox("player exterminate", &mut self.player_exterminate) {
            ds1.set_player_exterminate(self.player_exterminate);
        }

        if ui.checkbox("no damage", &mut self.no_damage) {
            ds1.set_no_damage();
        }

        if ui.checkbox("no hit", &mut self.no_hit) {
            ds1.set_no_hit();
        }

        if ui.checkbox("no attack", &mut self.no_attack) {
            ds1.set_no_attack();
        }

        if ui.checkbox("no move", &mut self.no_move) {
            ds1.set_no_move();
        }

        if ui.checkbox("no update ai", &mut self.no_update_ai) {
            ds1.set_no_update_ai();
        }

        if ui.checkbox("disable collision", &mut self.disable_collision) {
            ds1.set_disable_collision();
        }

        if ui.checkbox("no gravity", &mut self.no_gravity) {
            ds1.set_no_gravity();
        }

        if ui.checkbox("draw direction", &mut self.draw_direction) {
            ds1.set_draw_direction();
        }

        if ui.checkbox("draw counter", &mut self.draw_counter) {
            ds1.set_draw_counter();
        }

        if ui.checkbox("draw stable pos", &mut self.draw_stable_pos) {
            ds1.set_draw_stable_pos();
        }

        ui.separator();
        if ui.checkbox("disable enemies", &mut self.disable_enemies) {
            ds1.set_disable_enemies_to(self.disable_enemies);
        }

        if ui.checkbox("disable events", &mut self.disable_events) {
            ds1.set_disable_events_to(self.disable_events);
        }

        if ui.checkbox("disable auto-save", &mut self.auto_save_disabled) {
            ds1.set_auto_save_to(!self.auto_save_disabled);
        }

        if ui.checkbox("offline mode", &mut self.offline_mode) {
            ds1.set_online_mode_to(!self.offline_mode);
        }

        if ui.checkbox("free cam", &mut self.free_cam) {
            ds1.set_free_cam_to(self.free_cam);
            self.free_cam = ds1.get_free_cam();
        }
    }
}
