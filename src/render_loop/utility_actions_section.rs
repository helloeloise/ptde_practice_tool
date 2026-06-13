use mem_rs::memory::ReadWrite;

use crate::memory::constants::CharData2;
use crate::memory::Ds1;
use crate::render_loop::RenderLoop;
use crate::ui::Player;

impl RenderLoop {
    pub(super) fn render_utility_actions_section(
        &mut self,
        ui: &imgui::Ui,
        ds1: &mut Ds1,
        player: &mut Player,
    ) {
        if ui.button("Moveswap") {
            player.moveswap(ds1);
        }

        ui.same_line();
        if ui.button("Swap Gender") {
            player.swap_gender(ds1);
        }

        ui.same_line();
        if ui.button("Restore Full HP") {
            // Read max HP from CharData2 and write it to current HP in CharData1.
            let max_hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::MAX_HP));
            ds1.chr_data_1.write_i32_rel(Some(0x2D4), max_hp);
        }

        ui.same_line();
        if ui.button("RTSR RANGE") {
            // Set HP to 19% of max HP (under 20% for RTSR activation).
            let max_hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::MAX_HP));
            let rtsr_hp = (max_hp as f32 * 0.19) as i32;
            ds1.chr_data_1.write_i32_rel(Some(0x2D4), rtsr_hp);
        }
    }
}
