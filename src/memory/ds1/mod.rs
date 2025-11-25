mod constants;
mod offsets;

use mem_rs::prelude::*;
use crate::memory::ds1::constants::*;

#[allow(dead_code)]
pub struct Ds1
{
    

    process: SendProcess,
    chr_dbg: SendPointer,
    pos_lock: SendPointer,                      // 0x16 0x27
    node_graph: SendPointer,                    // 0x12
    all_no_magic_quantity_consume: SendPointer, // 0x2
    player_no_dead: SendPointer,                // 0x22
    player_exterminate: SendPointer,            // 0x10
    all_no_stamina_consume: SendPointer,        // 0x18
    compass: SendPointer,                       // 0xC 0x15, 0x1E
    chr_data_1: SendPointer,                    // 0x2, 0x0, 0x4, 0x0
    char_map_data: SendPointer,                 // chr_data_1 (aob), 0x2, 0x0, 0x4, 0x0 0x2
    chr_data_2: SendPointer,
    char_pos_data: SendPointer, // 0x1, 0x0, 0x8
    no_stam_consume: bool,
}

struct SendProcess(Process);

unsafe impl Send for SendProcess {}
unsafe impl Sync for SendProcess {}

struct SendPointer(Pointer);
unsafe impl Sync for SendPointer {}

unsafe impl Send for SendPointer {}

impl Ds1 {
    pub fn new() -> Self {
        hudhook::alloc_console().ok();
        let mut ds1struct = Ds1 {

            

            process: SendProcess(Process::new("DARKSOULS.exe")),
            chr_dbg: SendPointer(Pointer::default()),
            pos_lock: SendPointer(Pointer::default()),
            node_graph: SendPointer(Pointer::default()),
            all_no_magic_quantity_consume: SendPointer(Pointer::default()),
            player_no_dead: SendPointer(Pointer::default()),
            player_exterminate: SendPointer(Pointer::default()),
            all_no_stamina_consume: SendPointer(Pointer::default()),
            compass: SendPointer(Pointer::default()),
            chr_data_1: SendPointer(Pointer::default()),
            char_map_data: SendPointer(Pointer::default()),
            chr_data_2: SendPointer(Pointer::default()),
            char_pos_data: SendPointer(Pointer::default()),
            no_stam_consume: false,
        };
        let _ = ds1struct.refresh();
        ds1struct
    }
    // Pointers are declared here
    pub fn refresh(&mut self) -> Result<(), String> {
        if !self.process.0.is_attached() {
            self.process.0.refresh()?;

            self.chr_data_2 = SendPointer(self.process.0.scan_abs(
                "chr_data_2",
                "8B 15 ? ? ? ? F3 0F 10 44 24 30 52",
                2,
                vec![0x0, 0x0, 0x4, 0x0],
            )?);
            self.chr_dbg = SendPointer(self.process.0.scan_abs(
                "all_no_stamina_consume",
                &offsets::all_no_stamina_consume_aob,
                offsets::all_no_stamina_consume_aob_offset,
                vec![0x0],
            )?);

            self.chr_data_1 = SendPointer(self.process.0.scan_abs(
                "chr_data_1",
                &offsets::char_data_1_aob,
                offsets::char_data_1_aob_offset,
                vec![
                    offsets::char_data_1_offset1,
                    offsets::char_data_1_offset2,
                    offsets::char_data_1_offset3,
                ],
            )?);

            self.char_map_data = SendPointer(self.process.0.scan_abs(
                "chr_map_data",
                &offsets::char_data_1_aob,
                offsets::char_data_1_aob_offset,
                vec![
                    offsets::char_data_1_offset1,
                    offsets::char_data_1_offset2,
                    offsets::char_data_1_offset3,
                    CharData1::CHAR_MAP_DATA_PTR,
                ],
            )?);

            self.char_pos_data = SendPointer(self.process.0.scan_abs(
                "chr_pos_data",
                &offsets::char_data_1_aob,
                offsets::char_data_1_aob_offset,
                vec![
                    0x0,
                    offsets::char_data_1_offset1,
                    offsets::char_data_1_offset2,
                    offsets::char_data_1_offset3,
                    CharData1::CHAR_MAP_DATA_PTR,
                    CharMapData::CHAR_POS_DATA_PTR,
                ],
            )?);

            self.char_pos_data.0.debug = true;
        } else {
            self.process.0.refresh()?;
        }

        Ok(())
    }

    pub fn get_hp(&self) -> u32 {
        let hp = self.chr_data_2.0.read_u32_rel(Some(0x2d4));
        print!("{:?}", hp);
        return hp;
    }

    pub fn get_stamina(&self) -> u32 {
        let stam = self.chr_data_2.0.read_u32_rel(Some(0x2e4));
        return stam;
    }


    pub fn get_x_pos(&self) ->f32 {
        let x_pos = self.char_pos_data.0.read_f32_rel(Some(CharPosData::POS_X));
        x_pos
    }

    pub fn get_y_pos(&self) ->f32 {
        let x_pos = self.char_pos_data.0.read_f32_rel(Some(CharPosData::POS_Y));
        x_pos
    }

    pub fn get_z_pos(&self) ->f32 {
        let x_pos = self.char_pos_data.0.read_f32_rel(Some(CharPosData::POS_Z));
        x_pos
    }
    


    pub fn get_no_stam_consume(&mut self) -> bool {
        let no_stamina_consume = self
            .chr_dbg
            .0
            .read_bool_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME));
        return no_stamina_consume;
    }

    pub fn get_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.chr_dbg.0.read_bool_rel(Some(ChrDbg::ALL_NO_UPDATE_AI));
        return no_update_ai;
    }

    pub fn set_no_stam_consume(&mut self) -> bool {
        let no_stamina_consume = self.get_no_stam_consume();
        if no_stamina_consume == false {
            self.chr_dbg
                .0
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x1);
        } else {
            self.chr_dbg
                .0
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x0);
        }
        no_stamina_consume
    }

    pub fn set_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.get_no_update_ai();
        if no_update_ai == false {
            self.chr_dbg
                .0
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x1);
        } else {
            self.chr_dbg
                .0
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x0);
        }
        no_update_ai
    }
}
