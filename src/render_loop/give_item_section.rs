use crate::memory::Ds1;
use crate::render_loop::RenderLoop;
use crate::ui::Items;

impl RenderLoop {
    pub(super) fn render_give_item_section(&mut self, ui: &imgui::Ui, ds1: &mut Ds1) {
        let give_item_flags = if self.give_item_header_open {
            imgui::TreeNodeFlags::DEFAULT_OPEN
        } else {
            imgui::TreeNodeFlags::empty()
        };
        let give_item_id = format!("Give item##reset{}", self.header_reset_counter);

        if !ui.collapsing_header(&give_item_id, give_item_flags) {
            self.give_item_header_open = false;
            return;
        }

        self.give_item_header_open = true;
        let mut items_handler = Items::new();

        // Tab bar for selecting item type.
        if ui.radio_button("Items", &mut self.give_item_type, 0) {}
        ui.same_line();
        if ui.radio_button("Rings", &mut self.give_item_type, 1) {}
        ui.same_line();
        if ui.radio_button("Weapons", &mut self.give_item_type, 2) {}
        ui.same_line();
        if ui.radio_button("Armor", &mut self.give_item_type, 3) {}

        ui.separator();

        // Items tab.
        if self.give_item_type == 0 {
            ui.set_next_item_width(400.0);
            ui.input_text("Search", &mut self.item_search).build();

            let item_data = Items::get_item_data();
            let selected_item = &item_data[self.selected_item_index];
            let preview_text = format!("{} (ID: {})", selected_item.3, selected_item.0);

            ui.set_next_item_width(400.0);
            if let Some(_combo) = ui.begin_combo("##item_combo", &preview_text) {
                let search_lower = self.item_search.to_lowercase();

                for (index, item) in item_data.iter().enumerate() {
                    let item_name = item.3;

                    if !search_lower.is_empty() && !item_name.to_lowercase().contains(&search_lower) {
                        continue;
                    }

                    let is_selected = self.selected_item_index == index;
                    let label = format!("{} (ID: {})", item_name, item.0);

                    if ui.selectable_config(&label).selected(is_selected).build() {
                        self.selected_item_index = index;
                        self.item_quantity = item.1;
                    }

                    if is_selected {
                        ui.set_item_default_focus();
                    }
                }
            }

            if ui
                .input_int("Quantity", &mut self.item_quantity)
                .step(1)
                .step_fast(10)
                .build()
            {
                self.item_quantity = self.item_quantity.max(1);
            }

            if ui.button("Give Selected Item") {
                let selected = &item_data[self.selected_item_index];
                items_handler.execute_get_item(ds1, 0x40000000, selected.0, self.item_quantity);
            }
        }

        // Rings tab.
        if self.give_item_type == 1 {
            ui.set_next_item_width(400.0);
            ui.input_text("Search", &mut self.ring_search).build();

            let ring_data = Items::get_ring_data();
            let selected_ring = &ring_data[self.selected_ring_index];
            let preview_text = format!("{} (ID: {})", selected_ring.3, selected_ring.0);

            ui.set_next_item_width(400.0);
            if let Some(_combo) = ui.begin_combo("##ring_combo", &preview_text) {
                let search_lower = self.ring_search.to_lowercase();

                for (index, ring) in ring_data.iter().enumerate() {
                    let ring_name = ring.3;

                    if !search_lower.is_empty() && !ring_name.to_lowercase().contains(&search_lower) {
                        continue;
                    }

                    let is_selected = self.selected_ring_index == index;
                    let label = format!("{} (ID: {})", ring_name, ring.0);

                    if ui.selectable_config(&label).selected(is_selected).build() {
                        self.selected_ring_index = index;
                        self.ring_quantity = ring.1;
                    }
                    if is_selected {
                        ui.set_item_default_focus();
                    }
                }
            }

            if ui
                .input_int("Quantity", &mut self.ring_quantity)
                .step(1)
                .step_fast(10)
                .build()
            {
                self.ring_quantity = self.ring_quantity.max(1);
            }

            if ui.button("Give Selected Ring") {
                let selected = &ring_data[self.selected_ring_index];
                items_handler.execute_get_item(ds1, 0x20000000, selected.0, self.ring_quantity);
            }
        }

        // Weapons tab.
        if self.give_item_type == 2 {
            ui.set_next_item_width(400.0);
            ui.input_text("Search", &mut self.weapon_search).build();

            let weapon_data = Items::get_weapon_data();
            let selected_weapon = &weapon_data[self.selected_weapon_index];
            let preview_text = format!("{} (ID: {})", selected_weapon.3, selected_weapon.0);

            ui.set_next_item_width(400.0);
            if let Some(_combo) = ui.begin_combo("##weapon_combo", &preview_text) {
                let search_lower = self.weapon_search.to_lowercase();

                for (index, weapon) in weapon_data.iter().enumerate() {
                    let weapon_name = weapon.3;

                    if !search_lower.is_empty() && !weapon_name.to_lowercase().contains(&search_lower) {
                        continue;
                    }

                    let is_selected = self.selected_weapon_index == index;
                    let label = format!("{} (ID: {})", weapon_name, weapon.0);

                    if ui.selectable_config(&label).selected(is_selected).build() {
                        self.selected_weapon_index = index;
                        self.weapon_quantity = weapon.1;
                    }

                    if is_selected {
                        ui.set_item_default_focus();
                    }
                }
            }

            if ui
                .input_int("Quantity", &mut self.weapon_quantity)
                .step(1)
                .step_fast(10)
                .build()
            {
                self.weapon_quantity = self.weapon_quantity.max(1);
            }

            let infusion_data = Items::get_infusion_data();
            let selected_infusion = &infusion_data[self.selected_infusion_index];

            ui.set_next_item_width(200.0);
            if let Some(_combo) = ui.begin_combo("Infusion", selected_infusion.0) {
                for (index, infusion) in infusion_data.iter().enumerate() {
                    let is_selected = self.selected_infusion_index == index;

                    if ui.selectable_config(infusion.0).selected(is_selected).build() {
                        self.selected_infusion_index = index;
                        if self.weapon_upgrade_level > infusion.2 {
                            self.weapon_upgrade_level = infusion.2;
                        }
                    }

                    if is_selected {
                        ui.set_item_default_focus();
                    }
                }
            }

            ui.set_next_item_width(200.0);
            ui.slider(
                "Upgrade Level",
                0,
                selected_infusion.2,
                &mut self.weapon_upgrade_level,
            );

            if ui.button("Give Selected Weapon") {
                let selected = &weapon_data[self.selected_weapon_index];
                let upgraded_weapon_id = selected.0 + selected_infusion.1 + self.weapon_upgrade_level;
                items_handler.execute_get_item(ds1, 0x00000000, upgraded_weapon_id, self.weapon_quantity);
            }
        }

        // Armor tab.
        if self.give_item_type == 3 {
            ui.set_next_item_width(400.0);
            ui.input_text("Search", &mut self.armor_search).build();

            let armor_data = Items::get_armor_data();
            let selected_armor = &armor_data[self.selected_armor_index];
            let preview_text = format!("{} (ID: {})", selected_armor.3, selected_armor.0);

            ui.set_next_item_width(400.0);
            if let Some(_combo) = ui.begin_combo("##armor_combo", &preview_text) {
                let search_lower = self.armor_search.to_lowercase();

                for (index, armor) in armor_data.iter().enumerate() {
                    let armor_name = armor.3;

                    if !search_lower.is_empty() && !armor_name.to_lowercase().contains(&search_lower) {
                        continue;
                    }

                    let is_selected = self.selected_armor_index == index;
                    let label = format!("{} (ID: {})", armor_name, armor.0);

                    if ui.selectable_config(&label).selected(is_selected).build() {
                        self.selected_armor_index = index;
                        self.armor_quantity = armor.1;
                        // Reset upgrade level when selecting new armor.
                        let max_upgrade = match armor.2 {
                            0 => 0,
                            1 => 5,
                            2 => 10,
                            _ => 0,
                        };
                        if self.armor_upgrade_level > max_upgrade {
                            self.armor_upgrade_level = max_upgrade;
                        }
                    }

                    if is_selected {
                        ui.set_item_default_focus();
                    }
                }
            }

            if ui
                .input_int("Quantity", &mut self.armor_quantity)
                .step(1)
                .step_fast(10)
                .build()
            {
                self.armor_quantity = self.armor_quantity.max(1);
            }

            // Determine max upgrade level based on armor type (0=not upgradeable, 1=+5, 2=+10).
            let max_upgrade_level = match selected_armor.2 {
                0 => 0,
                1 => 5,
                2 => 10,
                _ => 0,
            };

            // Only show upgrade slider if armor can be upgraded.
            if max_upgrade_level > 0 {
                ui.set_next_item_width(200.0);
                ui.slider(
                    "Upgrade Level",
                    0,
                    max_upgrade_level,
                    &mut self.armor_upgrade_level,
                );
            }

            if ui.button("Give Selected Armor") {
                let selected = &armor_data[self.selected_armor_index];
                let upgraded_armor_id = selected.0 + self.armor_upgrade_level;
                items_handler.execute_get_item(ds1, 0x10000000, upgraded_armor_id, self.armor_quantity);
            }
        }
    }
}
