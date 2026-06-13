use super::*;

impl Ds1 {
    pub fn get_free_cam(&mut self) -> bool {
        self.free_cam_enabled
    }

    pub fn set_free_cam_to(&mut self, value: bool) {
        if value {
            self.free_cam_enabled = self.enable_free_cam_injection();
        } else {
            let _ = self.disable_free_cam_injection();
            self.free_cam_enabled = false;
        }
    }

    pub fn get_no_stamina_consume(&mut self) -> bool {
        self.chr_dbg
            .read_bool_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME))
    }

    pub fn get_no_update_ai(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_UPDATE_AI))
    }

    pub fn get_player_hide(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::PLAYER_HIDE))
    }

    pub fn get_player_silence(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::PLAYER_SILENCE))
    }

    pub fn get_no_death(&mut self) -> bool {
        self.no_death_pointer.read_bool_rel(Some(0x0))
    }

    pub fn get_no_damage(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_DAMAGE))
    }

    pub fn get_no_hit(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_HIT))
    }

    pub fn get_no_attack(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_ATTACK))
    }

    pub fn get_no_move(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_MOVE))
    }

    pub fn get_disable_collision(&mut self) -> bool {
        let current_flags = self
            .char_map_data
            .read_u32_rel(Some(CharMapData::CHAR_MAP_FLAGS));
        (current_flags & CharMapFlags::DISABLE_MAP_HIT as u32) != 0
    }

    pub fn get_no_gravity(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_1));
        (current_flags & CharFlags1::SET_DISABLE_GRAVITY as u32) != 0
    }

    pub fn get_draw_direction(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        (current_flags & CharFlags2::DRAW_DIRECTION as u32) != 0
    }

    pub fn get_draw_counter(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        (current_flags & CharFlags2::DRAW_COUNTER as u32) != 0
    }

    pub fn get_draw_stable_pos(&mut self) -> bool {
        self.chr_data_1
            .read_bool_rel(Some(CharFlags2::DRAW_STABLE_POS))
    }

    pub fn set_no_stam_consume(&mut self) -> bool {
        let no_stamina_consume = self.get_no_stamina_consume();
        if !no_stamina_consume {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x0);
        }
        no_stamina_consume
    }

    pub fn set_no_stam_consume_to(&mut self, value: bool) {
        self.chr_dbg.write_u8_rel(
            Some(ChrDbg::ALL_NO_STAMINA_CONSUME),
            if value { 0x1 } else { 0x0 },
        );
    }

    pub fn set_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.get_no_update_ai();
        if !no_update_ai {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x0);
        }
        no_update_ai
    }

    pub fn set_no_update_ai_to(&mut self, value: bool) {
        self.chr_dbg.write_u8_rel(
            Some(ChrDbg::ALL_NO_UPDATE_AI),
            if value { 0x1 } else { 0x0 },
        );
    }

    pub fn set_no_death(&mut self) -> bool {
        let no_death = self.get_no_death();
        if !no_death {
            self.no_death_pointer.write_u8_rel(Some(0x0), 0x1);
        } else {
            self.no_death_pointer.write_u8_rel(Some(0x0), 0x0);
        }
        no_death
    }

    pub fn set_no_death_to(&mut self, value: bool) {
        self.no_death_pointer
            .write_u8_rel(Some(0x0), if value { 0x1 } else { 0x0 });
    }

    pub fn set_player_exterminate(&mut self, value: bool) {
        self.player_exterminate
            .write_u8_rel(Some(0x0), if value { 0x1 } else { 0x0 });
    }

    pub fn set_player_hide(&mut self) -> bool {
        let player_hide = self.get_player_hide();
        if !player_hide {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_HIDE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_HIDE), 0x0);
        }
        player_hide
    }

    pub fn set_player_hide_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::PLAYER_HIDE), if value { 0x1 } else { 0x0 });
    }

    pub fn set_player_silence(&mut self) -> bool {
        let player_silence = self.get_player_silence();
        if !player_silence {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_SILENCE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_SILENCE), 0x0);
        }
        player_silence
    }

    pub fn set_player_silence_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::PLAYER_SILENCE), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_damage(&mut self) -> bool {
        let no_damage = self.get_no_damage();
        if !no_damage {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_DAMAGE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_DAMAGE), 0x0);
        }
        no_damage
    }

    pub fn set_no_damage_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::ALL_NO_DAMAGE), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_hit(&mut self) -> bool {
        let no_hit = self.get_no_hit();
        if !no_hit {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_HIT), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_HIT), 0x0);
        }
        no_hit
    }

    pub fn set_no_hit_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::ALL_NO_HIT), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_attack(&mut self) -> bool {
        let no_attack = self.get_no_attack();
        if !no_attack {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_ATTACK), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_ATTACK), 0x0);
        }
        no_attack
    }

    pub fn set_no_attack_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::ALL_NO_ATTACK), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_move(&mut self) -> bool {
        let no_move = self.get_no_move();
        if !no_move {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_MOVE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_MOVE), 0x0);
        }
        no_move
    }

    pub fn set_no_move_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::ALL_NO_MOVE), if value { 0x1 } else { 0x0 });
    }

    pub fn teleport_player(&mut self, x: f32, y: f32, z: f32, angle: f32) {
        self.char_map_data.write_f32_rel(Some(CharMapData::WARP_X), x);
        self.char_map_data.write_f32_rel(Some(CharMapData::WARP_Y), y);
        self.char_map_data.write_f32_rel(Some(CharMapData::WARP_Z), z);
        self.char_map_data
            .write_f32_rel(Some(CharMapData::WARP_ANGLE), angle);
        self.char_pos_data
            .write_f32_rel(Some(CharPosData::POS_ANGLE), angle);
        self.char_map_data.write_u8_rel(Some(CharMapData::WARP), 1);
    }

    pub fn set_disable_collision(&mut self) -> bool {
        let current_flags = self
            .char_map_data
            .read_u32_rel(Some(CharMapData::CHAR_MAP_FLAGS));
        let collision_disabled = (current_flags & CharMapFlags::DISABLE_MAP_HIT as u32) != 0;

        if collision_disabled {
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags & !(CharMapFlags::DISABLE_MAP_HIT as u32),
            );
        } else {
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags | CharMapFlags::DISABLE_MAP_HIT as u32,
            );
        }

        collision_disabled
    }

    pub fn set_disable_collision_to(&mut self, value: bool) {
        let current_flags = self
            .char_map_data
            .read_u32_rel(Some(CharMapData::CHAR_MAP_FLAGS));
        if value {
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags | CharMapFlags::DISABLE_MAP_HIT as u32,
            );
        } else {
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags & !(CharMapFlags::DISABLE_MAP_HIT as u32),
            );
        }
    }

    pub fn set_no_gravity(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_1));
        let gravity_disabled = (current_flags & CharFlags1::SET_DISABLE_GRAVITY as u32) != 0;

        if gravity_disabled {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags & !(CharFlags1::SET_DISABLE_GRAVITY as u32),
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags | CharFlags1::SET_DISABLE_GRAVITY as u32,
            );
        }

        gravity_disabled
    }

    pub fn set_no_gravity_to(&mut self, value: bool) {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_1));
        if value {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags | CharFlags1::SET_DISABLE_GRAVITY as u32,
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags & !(CharFlags1::SET_DISABLE_GRAVITY as u32),
            );
        }
    }

    pub fn set_draw_direction(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        let draw_direction = (current_flags & CharFlags2::DRAW_DIRECTION as u32) != 0;

        if draw_direction {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::DRAW_DIRECTION as u32),
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::DRAW_DIRECTION as u32,
            );
        }

        draw_direction
    }

    pub fn set_draw_direction_to(&mut self, value: bool) {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        if value {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::DRAW_DIRECTION as u32,
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::DRAW_DIRECTION as u32),
            );
        }
    }

    pub fn set_draw_counter(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        let draw_counter = (current_flags & CharFlags2::DRAW_COUNTER as u32) != 0;

        if draw_counter {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::DRAW_COUNTER as u32),
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::DRAW_COUNTER as u32,
            );
        }

        draw_counter
    }

    pub fn set_draw_counter_to(&mut self, value: bool) {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        if value {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::DRAW_COUNTER as u32,
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::DRAW_COUNTER as u32),
            );
        }
    }

    pub fn set_draw_stable_pos(&mut self) -> bool {
        let draw_stable_pos = self.get_draw_stable_pos();

        if draw_stable_pos {
            self.chr_data_1
                .write_u8_rel(Some(CharFlags2::DRAW_STABLE_POS), 0x0);
        } else {
            self.chr_data_1
                .write_u8_rel(Some(CharFlags2::DRAW_STABLE_POS), 0x1);
        }

        draw_stable_pos
    }

    pub fn set_draw_stable_pos_to(&mut self, value: bool) {
        self.chr_data_1.write_u8_rel(
            Some(CharFlags2::DRAW_STABLE_POS),
            if value { 0x1 } else { 0x0 },
        );
    }
}
