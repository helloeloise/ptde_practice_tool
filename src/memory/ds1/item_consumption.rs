use super::*;

impl Ds1 {
    pub fn get_infinite_magic(&mut self) -> bool {
        self.all_no_magic_quantity_consume.read_bool_rel(Some(0))
    }

    pub fn get_infinite_goods(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        (current_flags & CharFlags2::NO_GOODS_CONSUME as u32) != 0
    }

    pub fn set_no_mp_consume(&mut self) -> bool {
        let no_mp_consume = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_MPCONSUME));
        if !no_mp_consume {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_MPCONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_MPCONSUME), 0x0);
        }
        no_mp_consume
    }

    pub fn set_no_arrow_consume(&mut self) -> bool {
        let no_arrow_consume = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME));
        if !no_arrow_consume {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME), 0x0);
        }
        no_arrow_consume
    }

    pub fn set_all_no_magic_quantity_consume(&mut self) -> bool {
        let no_magic_qty_consume = self.get_infinite_magic();
        if !no_magic_qty_consume {
            self.all_no_magic_quantity_consume.write_u8_rel(Some(0), 0x1);
        } else {
            self.all_no_magic_quantity_consume.write_u8_rel(Some(0), 0x0);
        }
        no_magic_qty_consume
    }

    pub fn set_all_no_magic_quantity_consume_to(&mut self, value: bool) {
        self.all_no_magic_quantity_consume
            .write_u8_rel(Some(0), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_goods_consume(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        let no_goods_consume = (current_flags & CharFlags2::NO_GOODS_CONSUME as u32) != 0;

        if no_goods_consume {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::NO_GOODS_CONSUME as u32),
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::NO_GOODS_CONSUME as u32,
            );
        }

        no_goods_consume
    }

    pub fn set_no_goods_consume_to(&mut self, value: bool) {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        if value {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::NO_GOODS_CONSUME as u32,
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::NO_GOODS_CONSUME as u32),
            );
        }
    }
}
