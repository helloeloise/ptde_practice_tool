use mem_rs::memory::ReadWrite;

use crate::memory::Ds1;
use crate::memory::constants::{CharMapData, CharPosData};
use crate::render_loop::RenderLoop;

impl RenderLoop {
    pub(super) fn apply_pending_angle_write(&mut self, ds1: &mut Ds1) {
        // Re-apply angle after the warp byte clears (game has finished processing the warp).
        // Writing before the warp clears is pointless because the warp routine resets it.
        if let Some((angle, frames)) = self.pending_angle_write {
            let warp_active = ds1.char_map_data.read_bool_rel(Some(CharMapData::WARP));
            if !warp_active {
                ds1.char_pos_data
                    .write_f32_rel(Some(CharPosData::POS_ANGLE), angle);
                if frames == 0 {
                    self.pending_angle_write = None;
                } else {
                    self.pending_angle_write = Some((angle, frames - 1));
                }
            }
        }
    }
}
