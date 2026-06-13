use crate::memory::Ds1;
use crate::render_loop::RenderLoop;
use crate::ui::Bonfire;

impl RenderLoop {
    pub(super) fn render_bonfire_section(
        &mut self,
        ui: &imgui::Ui,
        ds1: &mut Ds1,
        bonfire: &mut Bonfire,
    ) {
        if !ui.collapsing_header("Select bonfire", imgui::TreeNodeFlags::empty()) {
            return;
        }

        ui.set_next_item_width(400.0);
        ui.input_text("Search", &mut self.bonfire_search).build();

        let selected_name = if self.selected_bonfire_id >= 0 {
            Bonfire::get_bonfires()
                .iter()
                .find(|(_, id)| *id == self.selected_bonfire_id)
                .map(|(name, _)| *name)
                .unwrap_or("Select a bonfire...")
        } else {
            "Select a bonfire..."
        };

        ui.set_next_item_width(400.0);
        if let Some(_combo) = ui.begin_combo("##bonfire_combo", selected_name) {
            let search_lower = self.bonfire_search.to_lowercase();

            for (bonfire_name, bonfire_id) in Bonfire::get_bonfires() {
                if !search_lower.is_empty() && !bonfire_name.to_lowercase().contains(&search_lower) {
                    continue;
                }

                let is_selected = self.selected_bonfire_id == bonfire_id;

                if ui
                    .selectable_config(bonfire_name)
                    .selected(is_selected)
                    .build()
                {
                    self.selected_bonfire_id = bonfire_id;
                }

                if is_selected {
                    ui.set_item_default_focus();
                }
            }
        }

        if ui.button("Warp to Selected Bonfire") && self.selected_bonfire_id >= 0 {
            bonfire.set_last_bonfire(ds1, self.selected_bonfire_id as u32);
            bonfire.inject_bonfire_function(ds1);
        }
    }
}
