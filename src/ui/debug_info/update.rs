use super::*;

impl DebugInfo {
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
        self.x_pos = ds1.get_x_pos();
        self.y_pos = ds1.get_y_pos();
        self.z_pos = ds1.get_z_pos();
        self.angle = ds1.get_angle();
        self.stance = ds1.chr_data_2.read_i32_rel(Some(CharData2::STANCE));
        self.current_poise = ds1.chr_data_1.read_f32_rel(Some(CharData1::CURRENT_POISE));
        self.poise_recovery_rate = ds1
            .chr_data_1
            .read_f32_rel(Some(CharData1::POISE_RECOVERY_RATE));
        self.ai_timer = ds1.target_bank.read_f32_rel(Some(0x14));
        self.ai_id = ds1.chr_data_1.read_u32_rel(Some(CharData1::AI_ID));
        self.new_game_cycle = ds1.chr_data_2.read_i32_rel(Some(CharData2::NEW_GAME));
        self.poison_resist = ds1.chr_data_2.read_i32_rel(Some(CharData2::POISON_RESIST));
        self.bleed_resist = ds1.chr_data_2.read_i32_rel(Some(CharData2::BLEED_RESIST));
        self.disease_resist = ds1.chr_data_2.read_i32_rel(Some(CharData2::DISEASE_RESIST));
        self.curse_resist = ds1.chr_data_2.read_i32_rel(Some(CharData2::CURSE_RESIST));
    }
}
