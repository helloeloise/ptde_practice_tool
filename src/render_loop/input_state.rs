use mem_rs::memory::ReadWrite;

use crate::memory::Ds1;
use crate::render_loop::RenderLoop;

impl RenderLoop {
    pub(super) fn update_game_input_state(&mut self, ds1: &mut Ds1, ui_wants_input: bool) {
        // Disable game input when menu is open or while UI is actively capturing keyboard/text.
        // Do not block input while TAS is running so TAS control remains uninterrupted.
        let tas_running = self.tas_runner.is_running();
        let should_disable_input = !tas_running && (ui_wants_input || self.menu_open);

        // Write only when the desired state changes and process is fully attached.
        if should_disable_input != self.input_was_disabled {
            if ds1.process.is_attached() {
                ds1.input_state
                    .write_u8_rel(None, if should_disable_input { 0x0 } else { 0x1 });
            }
            self.input_was_disabled = should_disable_input;
        }
    }
}
