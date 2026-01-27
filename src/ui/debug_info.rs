use crate::memory::constants::CharData2;
use crate::memory::constants::{self, AnimData, CharData1, CharMapData, LevelUp};
use crate::memory::offsets;
use crate::memory::{Ds1, ds1};
use imgui::Condition;
use mem_rs::prelude::*;

pub struct DebugInfo {
    current_anim_id: i32,
    anim_speed: f32,
    equip_left_1_idx: i32,
    equip_left_1_id: i32,
    equip_right_1_idx: i32,
    equip_right_1_id: i32,
    equip_left_2_idx: i32,
    equip_left_2_id: i32,
    equip_right_2_idx: i32,
    equip_right_2_id: i32,
    equip_arrow_1_idx: i32,
    equip_arrow_1_id: i32,
    equip_bolt_1_idx: i32,
    equip_bolt_1_id: i32,
    equip_arrow_2_idx: i32,
    equip_arrow_2_id: i32,
    equip_bolt_2_idx: i32,
    equip_bolt_2_id: i32,
    equip_helmet_idx: i32,
    equip_helmet_id: i32,
    equip_chest_idx: i32,
    equip_chest_id: i32,
    equip_glove_idx: i32,
    equip_glove_id: i32,
    equip_pants_idx: i32,
    equip_pants_id: i32,
    equip_hair_idx: i32,
    equip_hair_id: i32,
    equip_ring_1_idx: i32,
    equip_ring_1_id: i32,
    equip_ring_2_idx: i32,
    equip_ring_2_id: i32,
    equip_item_1_idx: i32,
    equip_item_1_id: i32,
    equip_item_2_idx: i32,
    equip_item_2_id: i32,
    equip_item_3_idx: i32,
    equip_item_3_id: i32,
    equip_item_4_idx: i32,
    equip_item_4_id: i32,
    equip_item_5_idx: i32,
    equip_item_5_id: i32,
    stance: i32,
    current_poise: f32,
    poise_recovery_rate: f32,
    ai_timer: f32,
    is_open: bool,
}

impl DebugInfo {
    pub fn new() -> Self {
        DebugInfo {
            current_anim_id: 0,
            anim_speed: 1.0,
            equip_left_1_idx: 0,
            equip_left_1_id: 0,
            equip_right_1_idx: 0,
            equip_right_1_id: 0,
            equip_left_2_idx: 0,
            equip_left_2_id: 0,
            equip_right_2_idx: 0,
            equip_right_2_id: 0,
            equip_arrow_1_idx: 0,
            equip_arrow_1_id: 0,
            equip_bolt_1_idx: 0,
            equip_bolt_1_id: 0,
            equip_arrow_2_idx: 0,
            equip_arrow_2_id: 0,
            equip_bolt_2_idx: 0,
            equip_bolt_2_id: 0,
            equip_helmet_idx: 0,
            equip_helmet_id: 0,
            equip_chest_idx: 0,
            equip_chest_id: 0,
            equip_glove_idx: 0,
            equip_glove_id: 0,
            equip_pants_idx: 0,
            equip_pants_id: 0,
            equip_hair_idx: 0,
            equip_hair_id: 0,
            equip_ring_1_idx: 0,
            equip_ring_1_id: 0,
            equip_ring_2_idx: 0,
            equip_ring_2_id: 0,
            equip_item_1_idx: 0,
            equip_item_1_id: 0,
            equip_item_2_idx: 0,
            equip_item_2_id: 0,
            equip_item_3_idx: 0,
            equip_item_3_id: 0,
            equip_item_4_idx: 0,
            equip_item_4_id: 0,
            equip_item_5_idx: 0,
            equip_item_5_id: 0,
            stance: 0,
            current_poise: 0.0,
            poise_recovery_rate: 0.0,
            ai_timer: 0.0,
            is_open: false,
        }
    }

    pub fn update(&mut self, ds1: &Ds1) {
        self.current_anim_id = ds1
            .chr_data_1
            .read_i32_rel(Some(CharData1::FORCE_PLAY_ANIMATION));
        self.anim_speed = ds1.anim_data.read_f32_rel(Some(AnimData::PLAY_SPEED));
        self.equip_left_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_LEFT_1_IDX));
        self.equip_left_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_LEFT_1_ID));
        self.equip_right_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RIGHT_1_IDX));
        self.equip_right_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RIGHT_1_ID));
        self.equip_left_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_LEFT_2_IDX));
        self.equip_left_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_LEFT_2_ID));
        self.equip_right_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RIGHT_2_IDX));
        self.equip_right_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RIGHT_2_ID));
        self.equip_arrow_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ARROW_1_IDX));
        self.equip_arrow_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ARROW_1_ID));
        self.equip_bolt_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_BOLT_1_IDX));
        self.equip_bolt_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_BOLT_1_ID));
        self.equip_arrow_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ARROW_2_IDX));
        self.equip_arrow_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ARROW_2_ID));
        self.equip_bolt_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_BOLT_2_IDX));
        self.equip_bolt_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_BOLT_2_ID));
        self.equip_helmet_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_HELMET_IDX));
        self.equip_helmet_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_HELMET_ID));
        self.equip_chest_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_CHEST_IDX));
        self.equip_chest_id = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_CHEST_ID));
        self.equip_glove_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_GLOVE_IDX));
        self.equip_glove_id = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_GLOVE_ID));
        self.equip_pants_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_PANTS_IDX));
        self.equip_pants_id = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_PANTS_ID));
        self.equip_hair_idx = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_HAIR_IDX));
        self.equip_hair_id = ds1.chr_data_2.read_i32_rel(Some(CharData2::EQUIP_HAIR_ID));
        self.equip_ring_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RING_1_IDX));
        self.equip_ring_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RING_1_ID));
        self.equip_ring_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RING_2_IDX));
        self.equip_ring_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_RING_2_ID));
        self.equip_item_1_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_1_IDX));
        self.equip_item_1_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_1_ID));
        self.equip_item_2_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_2_IDX));
        self.equip_item_2_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_2_ID));
        self.equip_item_3_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_3_IDX));
        self.equip_item_3_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_3_ID));
        self.equip_item_4_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_4_IDX));
        self.equip_item_4_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_4_ID));
        self.equip_item_5_idx = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_5_IDX));
        self.equip_item_5_id = ds1
            .chr_data_2
            .read_i32_rel(Some(CharData2::EQUIP_ITEM_5_ID));
        self.stance = ds1.chr_data_2.read_i32_rel(Some(CharData2::STANCE));
        self.current_poise = ds1.chr_data_1.read_f32_rel(Some(CharData1::CURRENT_POISE));
        self.poise_recovery_rate = ds1
            .chr_data_1
            .read_f32_rel(Some(CharData1::POISE_RECOVERY_RATE));
        self.ai_timer = ds1.target_bank.read_f32_rel(Some(0x14));
    }

    pub fn get_current_anim_id(&self) -> i32 {
        self.current_anim_id
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn render_window(&mut self, ui: &imgui::Ui, ds1: &mut Ds1) {
        if !self.is_open {
            return;
        }

        let _text_style = ui.push_style_color(imgui::StyleColor::Text, [1.0, 1.0, 1.0, 1.0]);

        ui.window("Debug Info")
            .size([600.0, 700.0], Condition::FirstUseEver)
            .position([400.0, 16.0], Condition::FirstUseEver)
            .draw_background(false)
            .build(|| {
                ui.text(format!(
                    "Current Animation ID: {}",
                    self.get_current_anim_id()
                ));
                ui.text(format!("Poise: {:.2}", self.current_poise));
                ui.text(format!("Poise Timer: {:.2}", self.poise_recovery_rate));
                ui.text(format!("AI Timer: {:.2}", self.ai_timer));
                ui.separator();

                // Weapons
                if ui.collapsing_header("Weapons", imgui::TreeNodeFlags::empty()) {
                    ui.text("Left Hand 1:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##left1_idx", &mut self.equip_left_1_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_LEFT_1_IDX),
                            self.equip_left_1_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##left1_id", &mut self.equip_left_1_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_LEFT_1_ID), self.equip_left_1_id);
                    }

                    ui.text("Right Hand 1:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##right1_idx", &mut self.equip_right_1_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_RIGHT_1_IDX),
                            self.equip_right_1_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##right1_id", &mut self.equip_right_1_id)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_RIGHT_1_ID),
                            self.equip_right_1_id,
                        );
                    }

                    ui.text("Left Hand 2:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##left2_idx", &mut self.equip_left_2_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_LEFT_2_IDX),
                            self.equip_left_2_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##left2_id", &mut self.equip_left_2_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_LEFT_2_ID), self.equip_left_2_id);
                    }

                    ui.text("Right Hand 2:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##right2_idx", &mut self.equip_right_2_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_RIGHT_2_IDX),
                            self.equip_right_2_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##right2_id", &mut self.equip_right_2_id)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_RIGHT_2_ID),
                            self.equip_right_2_id,
                        );
                    }
                }

                // Ammo
                if ui.collapsing_header("Ammo", imgui::TreeNodeFlags::empty()) {
                    ui.text("Arrow 1:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##arrow1_idx", &mut self.equip_arrow_1_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ARROW_1_IDX),
                            self.equip_arrow_1_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##arrow1_id", &mut self.equip_arrow_1_id)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ARROW_1_ID),
                            self.equip_arrow_1_id,
                        );
                    }

                    ui.text("Bolt 1:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##bolt1_idx", &mut self.equip_bolt_1_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_BOLT_1_IDX),
                            self.equip_bolt_1_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##bolt1_id", &mut self.equip_bolt_1_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_BOLT_1_ID), self.equip_bolt_1_id);
                    }

                    ui.text("Arrow 2:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##arrow2_idx", &mut self.equip_arrow_2_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ARROW_2_IDX),
                            self.equip_arrow_2_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##arrow2_id", &mut self.equip_arrow_2_id)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ARROW_2_ID),
                            self.equip_arrow_2_id,
                        );
                    }

                    ui.text("Bolt 2:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##bolt2_idx", &mut self.equip_bolt_2_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_BOLT_2_IDX),
                            self.equip_bolt_2_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##bolt2_id", &mut self.equip_bolt_2_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_BOLT_2_ID), self.equip_bolt_2_id);
                    }
                }

                // Armor
                if ui.collapsing_header("Armor", imgui::TreeNodeFlags::empty()) {
                    ui.text("Helmet:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##helmet_idx", &mut self.equip_helmet_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_HELMET_IDX),
                            self.equip_helmet_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##helmet_id", &mut self.equip_helmet_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_HELMET_ID), self.equip_helmet_id);
                    }

                    ui.text("Chest:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##chest_idx", &mut self.equip_chest_idx)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_CHEST_IDX), self.equip_chest_idx);
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui.input_int("##chest_id", &mut self.equip_chest_id).build() {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_CHEST_ID), self.equip_chest_id);
                    }

                    ui.text("Gloves:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##glove_idx", &mut self.equip_glove_idx)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_GLOVE_IDX), self.equip_glove_idx);
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui.input_int("##glove_id", &mut self.equip_glove_id).build() {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_GLOVE_ID), self.equip_glove_id);
                    }

                    ui.text("Pants:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##pants_idx", &mut self.equip_pants_idx)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_PANTS_IDX), self.equip_pants_idx);
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui.input_int("##pants_id", &mut self.equip_pants_id).build() {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_PANTS_ID), self.equip_pants_id);
                    }

                    ui.text("Hair:");
                    ui.set_next_item_width(100.0);
                    if ui.input_int("##hair_idx", &mut self.equip_hair_idx).build() {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_HAIR_IDX), self.equip_hair_idx);
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui.input_int("##hair_id", &mut self.equip_hair_id).build() {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_HAIR_ID), self.equip_hair_id);
                    }
                }

                // Rings
                if ui.collapsing_header("Rings", imgui::TreeNodeFlags::empty()) {
                    ui.text("Ring 1:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##ring1_idx", &mut self.equip_ring_1_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_RING_1_IDX),
                            self.equip_ring_1_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##ring1_id", &mut self.equip_ring_1_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_RING_1_ID), self.equip_ring_1_id);
                    }

                    ui.text("Ring 2:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##ring2_idx", &mut self.equip_ring_2_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_RING_2_IDX),
                            self.equip_ring_2_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##ring2_id", &mut self.equip_ring_2_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_RING_2_ID), self.equip_ring_2_id);
                    }
                }

                // Items
                if ui.collapsing_header("Items", imgui::TreeNodeFlags::empty()) {
                    ui.text("Item 1:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item1_idx", &mut self.equip_item_1_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ITEM_1_IDX),
                            self.equip_item_1_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item1_id", &mut self.equip_item_1_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_ITEM_1_ID), self.equip_item_1_id);
                    }

                    ui.text("Item 2:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item2_idx", &mut self.equip_item_2_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ITEM_2_IDX),
                            self.equip_item_2_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item2_id", &mut self.equip_item_2_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_ITEM_2_ID), self.equip_item_2_id);
                    }

                    ui.text("Item 3:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item3_idx", &mut self.equip_item_3_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ITEM_3_IDX),
                            self.equip_item_3_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item3_id", &mut self.equip_item_3_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_ITEM_3_ID), self.equip_item_3_id);
                    }

                    ui.text("Item 4:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item4_idx", &mut self.equip_item_4_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ITEM_4_IDX),
                            self.equip_item_4_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item4_id", &mut self.equip_item_4_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_ITEM_4_ID), self.equip_item_4_id);
                    }

                    ui.text("Item 5:");
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item5_idx", &mut self.equip_item_5_idx)
                        .build()
                    {
                        ds1.chr_data_2.write_i32_rel(
                            Some(CharData2::EQUIP_ITEM_5_IDX),
                            self.equip_item_5_idx,
                        );
                    }
                    ui.same_line();
                    ui.set_next_item_width(100.0);
                    if ui
                        .input_int("##item5_id", &mut self.equip_item_5_id)
                        .build()
                    {
                        ds1.chr_data_2
                            .write_i32_rel(Some(CharData2::EQUIP_ITEM_5_ID), self.equip_item_5_id);
                    }
                }
            });
    }
}
