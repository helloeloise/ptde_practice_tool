use crate::render_loop::RenderLoop;

impl RenderLoop {
    pub(super) fn persist_main_window_layout_if_changed(
        &mut self,
        main_window_changed: bool,
        new_main_pos: [f32; 2],
        new_main_size: [f32; 2],
    ) {
        if !main_window_changed {
            return;
        }

        let mut config = self.config.lock().unwrap();
        let layout = &mut config.window_layout.main_window;
        layout.pos_x = new_main_pos[0];
        layout.pos_y = new_main_pos[1];
        layout.width = new_main_size[0];
        layout.height = new_main_size[1];

        // Throttle disk writes while preserving latest in-memory window placement.
        if self.last_main_window_save_time.elapsed().as_secs() >= 2 {
            let _ = config.save();
            self.last_main_window_save_time = std::time::Instant::now();
        }
    }
}
