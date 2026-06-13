use super::*;

impl Ds1 {
    pub fn get_x_pos(&self) -> f32 {
        self.char_pos_data.read_f32_rel(Some(CharPosData::POS_X))
    }

    pub fn get_y_pos(&self) -> f32 {
        self.char_pos_data.read_f32_rel(Some(CharPosData::POS_Y))
    }

    pub fn get_z_pos(&self) -> f32 {
        self.char_pos_data.read_f32_rel(Some(CharPosData::POS_Z))
    }

    pub fn get_angle(&self) -> f32 {
        self.char_pos_data
            .read_f32_rel(Some(CharPosData::POS_ANGLE))
    }
}
