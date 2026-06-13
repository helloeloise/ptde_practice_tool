use mem_rs::memory::ReadWrite;

use crate::memory::constants::{CharData1, CharData2};
use crate::memory::Ds1;
use crate::render_loop::RenderLoop;
use crate::ui::Player;

impl RenderLoop {
    pub(super) fn render_stats_section(
        &mut self,
        ui: &imgui::Ui,
        ds1: &mut Ds1,
        player: &mut Player,
    ) {
        let stats_flags = if self.stats_header_open {
            imgui::TreeNodeFlags::DEFAULT_OPEN
        } else {
            imgui::TreeNodeFlags::empty()
        };
        let stats_id = format!("Stats##reset{}", self.header_reset_counter);

        if ui.collapsing_header(&stats_id, stats_flags) {
            self.stats_header_open = true;
            // Instantiate player stats when Stats section is visible.
            player.instantiate(ds1);

            if ui.input_int("Vitality", &mut player.vitality).build() {
                player.vitality = player.vitality.max(1);
                player.set_player_stat(ds1, CharData2::VITALITY, player.vitality);
            }

            if ui.input_int("Attunement", &mut player.attunement).build() {
                player.attunement = player.attunement.max(1);
                player.set_player_stat(ds1, CharData2::ATTUNEMENT, player.attunement);
            }

            if ui.input_int("Endurance", &mut player.endurance).build() {
                player.endurance = player.endurance.max(1);
                player.set_player_stat(ds1, CharData2::ENDURANCE, player.endurance);
            }

            if ui.input_int("Strength", &mut player.strength).build() {
                player.strength = player.strength.max(1);
                player.set_player_stat(ds1, CharData2::STRENGTH, player.strength);
            }

            if ui.input_int("Dexterity", &mut player.dexterity).build() {
                player.dexterity = player.dexterity.max(1);
                player.set_player_stat(ds1, CharData2::DEXTERITY, player.dexterity);
            }

            if ui.input_int("Intelligence", &mut player.intelligence).build() {
                player.intelligence = player.intelligence.max(1);
                player.set_player_stat(ds1, CharData2::INTELLIGENCE, player.intelligence);
            }

            if ui.input_int("Faith", &mut player.faith).build() {
                player.faith = player.faith.max(1);
                player.set_player_stat(ds1, CharData2::FAITH, player.faith);
            }

            if ui
                .input_int("Souls", &mut player.souls)
                .step(100)
                .step_fast(1000)
                .build()
            {
                player.souls = player.souls.max(1);
                ds1.chr_data_2
                    .write_i32_rel(Some(CharData2::SOULS), player.souls);
            }

            if ui.input_int("Humanity", &mut player.humanity).build() {
                player.humanity = player.humanity.max(0);
                ds1.chr_data_2
                    .write_i32_rel(Some(CharData2::HUMANITY), player.humanity);
            }

            if ui
                .input_int("MP", &mut player.mp)
                .step(10)
                .step_fast(100)
                .build()
            {
                player.mp = player.mp.max(0);
                ds1.chr_data_1.write_i32_rel(Some(CharData1::MP), player.mp);
            }

            ui.text(format!(
                "Max MP: {}  |  Stamina Max: {}",
                player.max_mp, player.max_stamina
            ));

            let mut death_count = ds1.get_death_count();
            if ui.input_int("Deaths", &mut death_count).build() {
                death_count = death_count.max(0);
                ds1.set_death_count(death_count);
            }

            let mut ng_plus = ds1.get_ng_plus() as i32;
            if ui.input_int("NG+", &mut ng_plus).build() {
                ng_plus = ng_plus.clamp(0, 7);
                ds1.set_ng_plus(ng_plus as u8);
            }
        } else {
            self.stats_header_open = false;
        }
    }
}
