use crate::memory::Ds1;
use crate::render_loop::RenderLoop;

impl RenderLoop {
    pub(super) fn run_pre_ui_frame_tasks(&mut self, ds1: &mut Ds1) {
        // Apply freecam behavior toggle from config to runtime hook state.
        {
            let config = self.config.lock().unwrap();
            ds1.set_freecam_force_mode2_active_only(config.freecam.force_mode2_active_only);
        }

        // Periodically refresh process handle to prevent it from going stale.
        if self.last_process_refresh_time.elapsed().as_secs() >= 30 {
            let _ = ds1.process.refresh();
            self.last_process_refresh_time = std::time::Instant::now();
        }

        // Keep freecam hooks/ticks updated as early as possible each frame.
        ds1.try_pending_freecam_injection();
        ds1.tick_freecam_mode_hotkey();
        ds1.tick_freecam_manual_movement();
    }
}
