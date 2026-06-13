use mem_rs::memory::ReadWrite;

use crate::memory::constants::WorldState;
use crate::memory::Ds1;
use crate::render_loop::RenderLoop;
use crate::ui::Player;

impl RenderLoop {
    pub(super) fn render_positions_section(
        &mut self,
        ui: &imgui::Ui,
        ds1: &mut Ds1,
        player: &mut Player,
    ) {
        let positions_flags = if self.positions_header_open {
            imgui::TreeNodeFlags::DEFAULT_OPEN
        } else {
            imgui::TreeNodeFlags::empty()
        };

        let positions_id = format!("Positions##reset{}", self.header_reset_counter);
        if ui.collapsing_header(&positions_id, positions_flags) {
            self.positions_header_open = true;
            // Update player position data (lightweight operation).
            player.instantiate_position_only(ds1);

            // Read-only display of current and stable positions.
            ui.text(format!(
                "Current: X:{:.4} Y:{:.4} Z:{:.4} A:{:.4}",
                player.x_pos, player.y_pos, player.z_pos, player.angle
            ));
            let stable_x = ds1.world_state.read_f32_rel(Some(WorldState::POS_X_STABLE));
            let stable_y = ds1.world_state.read_f32_rel(Some(WorldState::POS_Y_STABLE));
            let stable_z = ds1.world_state.read_f32_rel(Some(WorldState::POS_Z_STABLE));
            let stable_angle = ds1
                .world_state
                .read_f32_rel(Some(WorldState::POS_ANGLE_STABLE));
            ui.text(format!(
                "Stable:  X:{:.4} Y:{:.4} Z:{:.4} A:{:.4}",
                stable_x, stable_y, stable_z, stable_angle
            ));
            ui.separator();

            for i in 0..3 {
                ui.text(format!("Slot {}", i + 1));
                ui.same_line();
                if ui.button(format!("Store##{}", i)) {
                    self.stored_positions[i] = Some((
                        player.x_pos,
                        player.y_pos,
                        player.z_pos,
                        player.angle,
                        player.hp,
                    ));
                }

                ui.same_line();
                {
                    let _d = ui.begin_disabled(self.stored_positions[i].is_none());
                    if ui.button(format!("Restore##{}", i)) {
                        if let Some(pos) = self.stored_positions[i] {
                            ds1.teleport_player(pos.0, pos.1, pos.2, pos.3);
                            self.pending_angle_write = Some((pos.3, 10));
                            ds1.chr_data_1.write_i32_rel(Some(0x2D4), pos.4);
                        }
                    }
                }

                if let Some(ref mut pos) = self.stored_positions[i] {
                    ui.set_next_item_width(180.0);
                    ui.input_float(&format!("X##{}", i), &mut pos.0)
                        .display_format("%.9f")
                        .build();
                    ui.same_line();

                    ui.set_next_item_width(180.0);
                    ui.input_float(&format!("Y##{}", i), &mut pos.1)
                        .display_format("%.9f")
                        .build();
                    ui.same_line();

                    ui.set_next_item_width(180.0);
                    ui.input_float(&format!("Z##{}", i), &mut pos.2)
                        .display_format("%.9f")
                        .build();
                    ui.same_line();

                    ui.set_next_item_width(180.0);
                    ui.input_float(&format!("Angle##{}", i), &mut pos.3)
                        .display_format("%.9f")
                        .build();
                }

                ui.separator();
            }
        } else {
            self.positions_header_open = false;
        }
    }
}
