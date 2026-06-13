use crate::render_loop::RenderLoop;

impl RenderLoop {
    pub(super) fn handle_menu_toggle_input(&mut self, ui: &imgui::Ui) {
        // Check toggle key first, before acquiring the heavier ds1 lock.
        if let Some(key) = self.resolved_keybinds.toggle_menu {
            if ui.is_key_pressed(key) {
                let was_open = self.menu_open;
                self.menu_open = !self.menu_open;

                // Reset section headers when reopening to keep defaults predictable.
                if !was_open && self.menu_open {
                    self.positions_header_open = false;
                    self.stats_header_open = false;
                    self.give_item_header_open = false;
                    self.header_reset_counter = self.header_reset_counter.wrapping_add(1);
                }
            }
        }
    }
}
