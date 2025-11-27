pub mod constants;
pub mod offsets;

use crate::memory::ds1::constants::*;
use mem_rs::prelude::*;

#[allow(dead_code)]
pub struct Ds1 {
    pub process: Process,
    pub chr_dbg: Pointer,
    pub pos_lock: Pointer,                      // 0x16 0x27
    pub node_graph: Pointer,                    // 0x12
    pub all_no_magic_quantity_consume: Pointer, // 0x2
    pub player_no_dead: Pointer,                // 0x22
    pub player_exterminate: Pointer,            // 0x10
    pub all_no_stamina_consume: Pointer,        // 0x18
    pub compass: Pointer,                       // 0xC 0x15, 0x1E
    pub chr_data_1: Pointer,                    // 0x2, 0x0, 0x4, 0x0
    pub char_map_data: Pointer,                 // chr_data_1 (aob), 0x2, 0x0, 0x4, 0x0 0x2
    pub chr_data_2: Pointer,
    pub char_pos_data: Pointer, // 0x1, 0x0, 0x8
    pub no_stam_consume: bool,
}

impl Ds1 {
    pub fn new() -> Self {
        hudhook::alloc_console().ok();
        let mut ds1struct = Ds1 {
            process: Process::new("DARKSOULS.exe"),
            chr_dbg: Pointer::default(),
            pos_lock: Pointer::default(),
            node_graph: Pointer::default(),
            all_no_magic_quantity_consume: Pointer::default(),
            player_no_dead: Pointer::default(),
            player_exterminate: Pointer::default(),
            all_no_stamina_consume: Pointer::default(),
            compass: Pointer::default(),
            chr_data_1: Pointer::default(),
            char_map_data: Pointer::default(),
            chr_data_2: Pointer::default(),
            char_pos_data: Pointer::default(),
            no_stam_consume: false,
        };
        let _ = ds1struct.refresh();
        ds1struct
    }
    // Pointers are declared here
    pub fn refresh(&mut self) -> Result<(), String> {
        if !self.process.is_attached() {
            self.process.refresh()?;

            
            self.chr_dbg = self.process.scan_abs(
                "all_no_stamina_consume",
                &offsets::ALL_NO_STAMINA_CONSUME_AOB,
                offsets::ALL_NO_STAMINA_CONSUME_AOB_OFFSET,
                vec![0x0],
            )?;

            self.chr_data_1 = self.process.scan_abs(
                "chr_data_1",
                &offsets::CHAR_DATA_1_AOB,
                offsets::CHAR_DATA_1_AOB_OFFSET,
                vec![
                    0x0,
                    offsets::CHAR_DATA_1_OFFSET1,
                    offsets::CHAR_DATA_1_OFFSET2,
                    offsets::CHAR_DATA_1_OFFSET3,
                ],
            )?;

            self.char_map_data = self.chr_data_1.clone();
            self.char_map_data
                .offsets
                .push(CharData1::CHAR_MAP_DATA_PTR);

            self.char_pos_data = self.char_map_data.clone();
            self.char_pos_data
                .offsets
                .push(CharMapData::CHAR_POS_DATA_PTR);
            self.char_pos_data.debug = true;


            self.chr_data_2 = self.process.scan_abs(
                "chr_data_2",
                &offsets::CHAR_DATA_2_AOB,
                offsets::CHAR_DATA_2_AOB_OFFSET,
                vec![
                    0x0,
                    offsets::CHAR_DATA_2_OFFSET1,
                    offsets::CHAR_DATA_2_OFFSET2,
                ],
            )?;


        } else {
            self.process.refresh()?;
        }

        Ok(())
    }

    

    pub fn get_x_pos(&self) -> f32 {
        let x_pos = self.char_pos_data.read_f32_rel(Some(CharPosData::POS_X));
        x_pos
    }

    pub fn get_y_pos(&self) -> f32 {
        let x_pos = self.char_pos_data.read_f32_rel(Some(CharPosData::POS_Y));
        x_pos
    }

    pub fn get_z_pos(&self) -> f32 {
        let x_pos = self.char_pos_data.read_f32_rel(Some(CharPosData::POS_Z));
        x_pos
    }

    pub fn get_no_stam_consume(&mut self) -> bool {
        let no_stamina_consume = self
            .chr_dbg
            .read_bool_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME));
        return no_stamina_consume;
    }

    pub fn get_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_UPDATE_AI));
        return no_update_ai;
    }

    pub fn set_no_stam_consume(&mut self) -> bool {
        let no_stamina_consume = self.get_no_stam_consume();
        if no_stamina_consume == false {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x0);
        }
        no_stamina_consume
    }

    pub fn set_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.get_no_update_ai();
        if no_update_ai == false {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x0);
        }
        no_update_ai
    }

    pub fn teleport_player(&mut self, x: f32, y: f32, z: f32) {
        self.char_pos_data
            .write_f32_rel(Some(CharPosData::POS_X), x);
        self.char_pos_data
            .write_f32_rel(Some(CharPosData::POS_Y), y);
        self.char_pos_data
            .write_f32_rel(Some(CharPosData::POS_Z), z);
    }
}
