pub mod constants;
pub mod offsets;

use crate::memory::{ds1::constants::*, offsets::BONFIRE_WARP_2_OFFSET1};
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
    pub anim_data: Pointer,
    pub chr_data_2: Pointer,
    pub char_pos_data: Pointer, // 0x1, 0x0, 0x8
    pub no_stam_consume: bool,
    pub level_up: Pointer,
    pub bonfire_warp: Pointer,
    pub bonfire_warp_2: Pointer,
    pub world_state: Pointer,
    pub chr_flags_1: Pointer,
    pub input_state: Pointer,
    pub quitout: Pointer,
    pub no_death_pointer: Pointer,
    pub item_get_pointer: Pointer,
    pub item_drop_pointer: Pointer,
    pub item_drop_unknown_1_pointer: Pointer,
    pub item_drop_unknown_2_pointer: Pointer,
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
            anim_data: Pointer::default(),
            chr_data_2: Pointer::default(),
            char_pos_data: Pointer::default(),
            level_up: Pointer::default(),
            no_stam_consume: false,
            bonfire_warp: Pointer::default(),
            bonfire_warp_2: Pointer::default(),
            world_state: Pointer::default(),
            chr_flags_1: Pointer::default(),
            input_state: Pointer::default(),
            quitout: Pointer::default(),
            no_death_pointer: Pointer::default(),
            item_get_pointer: Pointer::default(),
            item_drop_pointer: Pointer::default(),
            item_drop_unknown_1_pointer: Pointer::default(),
            item_drop_unknown_2_pointer: Pointer::default(),
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

            self.anim_data = self.char_map_data.clone();
            self.anim_data
                .offsets
                .push(CharMapData::ANIM_DATA_PTR);

            self.char_pos_data = self.char_map_data.clone();
            self.char_pos_data
                .offsets
                .push(CharMapData::CHAR_POS_DATA_PTR);

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

            self.level_up =
                self.process
                    .scan_abs("level_up", &offsets::LEVEL_UP, 0x0, vec![0x0])?;

            self.bonfire_warp = self
                .process
                .scan_abs("bonfire_warp", &offsets::BONFIRE_WARP, 0x0, vec![0x0])
                .unwrap();
            self.bonfire_warp_2 = self.process.scan_abs(
                "bonfire_warp_2",
                &offsets::BONFIRE_WARP_2,
                BONFIRE_WARP_2_OFFSET1,
                vec![0x0],
            )?;

            self.item_drop_pointer = self.process.scan_abs(
                "item_drop_pointer",
                &offsets::ITEM_DROP_AOB,
                0x0,
                vec![0x0],
            )?;

            self.item_drop_unknown_1_pointer = self.process.scan_abs(
                "item_drop_unknown_1_pointer",
                &offsets::ITEM_DROP_UNKNOWN_1_AOB,
                offsets::ITEM_DROP_UNKNOWN_1_AOB_OFFSET,
                vec![0x0],
            )?;

            self.item_drop_unknown_2_pointer = self.process.scan_abs(
                "item_drop_unknown_2_pointer",
                &offsets::ITEM_DROP_UNKNOWN_2_AOB,
                offsets::ITEM_DROP_UNKNOWN_2_AOB_OFFSET,
                vec![0x0],
            )?;

            self.world_state = self.process.scan_abs(
                "world_state",
                &offsets::WORLD_STATE_AOB,
                offsets::WORLD_STATE_AOB_OFFSET,
                vec![0x0, offsets::WORLD_STATE_OFFSET1],
            )?;

            self.chr_flags_1 = self.process.scan_abs(
                "chr_flags_1",
                &offsets::FLAGS_AOB_1,
                offsets::FLAGS_AOB_1_OFFSET,
                vec![0x0],
            )?;

            self.input_state = self.process.scan_abs(
                "input_state",
                &offsets::INPUT_STATE_AOB,
                offsets::INPUT_STATE_AOB_OFFSET,
                vec![
                    0x0,
                    offsets::INPUT_STATE_OFFSET1,
                    offsets::INPUT_STATE_OFFSET2,
                    offsets::INPUT_STATE_OFFSET3,
                    offsets::INPUT_STATE_OFFSET4,
                ],
            )?;

            self.quitout = self.process.scan_abs(
                "quitout",
                &offsets::QUITOUT_AOB,
                offsets::QUITOUT_AOB_OFFSET,
                vec![0x0, offsets::QUITOUT_OFFSET1],
            )?;

            self.no_death_pointer = self.process.scan_abs(
                "no_death_pointer",
                &offsets::PLAYER_NO_DEAD_AOB,
                offsets::PLAYER_NO_DEAD_AOB_OFFSET,
                vec![0x0],
            )?;

            self.item_get_pointer = self.process.scan_abs(
                "item_get_pointer",
                &offsets::ITEM_GET_AOB,
                0x0,
                vec![0x0],
            )?;

            self.all_no_magic_quantity_consume = self.process.scan_abs(
                "all_no_magic_quantity_consume",
                &offsets::ALL_NO_MAGIC_QTY_CONSUME_AOB,
                offsets::ALL_NO_MAGIC_QTY_CONSUME_AOB_OFFSET,
                vec![0x0],
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

    pub fn set_no_death(&mut self) -> bool {
        let no_death = self.chr_dbg.read_bool_rel(Some(0x0));
        if no_death == false {
            self.no_death_pointer.write_u8_rel(Some(0x0), 0x1);
        } else {
            self.no_death_pointer.write_u8_rel(Some(0x0), 0x0);
        }
        no_death
    }

    pub fn set_no_mp_consume(&mut self) -> bool {
        let no_mp_consume = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_MPCONSUME));
        if no_mp_consume == false {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_MPCONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_MPCONSUME), 0x0);
        }
        no_mp_consume
    }

    pub fn set_no_arrow_consume(&mut self) -> bool {
        let no_arrow_consume = self
            .chr_dbg
            .read_bool_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME));
        if no_arrow_consume == false {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME), 0x0);
        }
        no_arrow_consume
    }

    pub fn set_player_hide(&mut self) -> bool {
        let player_hide = self.chr_dbg.read_bool_rel(Some(ChrDbg::PLAYER_HIDE));
        if player_hide == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_HIDE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_HIDE), 0x0);
        }
        player_hide
    }

    pub fn set_player_silence(&mut self) -> bool {
        let player_silence = self.chr_dbg.read_bool_rel(Some(ChrDbg::PLAYER_SILENCE));
        if player_silence == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_SILENCE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_SILENCE), 0x0);
        }
        player_silence
    }

    pub fn set_no_damage(&mut self) -> bool {
        let no_damage = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_DAMAGE));
        if no_damage == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_DAMAGE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_DAMAGE), 0x0);
        }
        no_damage
    }

    pub fn set_no_hit(&mut self) -> bool {
        let no_hit = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_HIT));
        if no_hit == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_HIT), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_HIT), 0x0);
        }
        no_hit
    }

    pub fn set_no_attack(&mut self) -> bool {
        let no_attack = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_ATTACK));
        if no_attack == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_ATTACK), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_ATTACK), 0x0);
        }
        no_attack
    }

    pub fn set_no_move(&mut self) -> bool {
        let no_move = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_MOVE));
        if no_move == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_MOVE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_MOVE), 0x0);
        }
        no_move
    }

    pub fn teleport_player(&mut self, x: f32, y: f32, z: f32) {
        self.char_map_data
            .write_f32_rel(Some(CharMapData::WARP_X), x);
        self.char_map_data
            .write_f32_rel(Some(CharMapData::WARP_Y), y);
        self.char_map_data
            .write_f32_rel(Some(CharMapData::WARP_Z), z);
        self.char_map_data.write_u32_rel(Some(CharMapData::WARP), 1);
    }

    pub fn set_disable_collision(&mut self) -> bool {
        let current_flags = self
            .char_map_data
            .read_u32_rel(Some(CharMapData::CHAR_MAP_FLAGS));
        let collision_disabled = (current_flags & CharMapFlags::DISABLE_MAP_HIT as u32) != 0;

        if collision_disabled {
            // Enable collision by clearing the flag
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags & !(CharMapFlags::DISABLE_MAP_HIT as u32),
            );
        } else {
            // Disable collision by setting the flag
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags | CharMapFlags::DISABLE_MAP_HIT as u32,
            );
        }

        collision_disabled
    }

    pub fn set_no_gravity(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_1));
        let gravity_disabled = (current_flags & CharFlags1::SET_DISABLE_GRAVITY as u32) != 0;

        if gravity_disabled {
            // Enable gravity by clearing the flag
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags & !(CharFlags1::SET_DISABLE_GRAVITY as u32),
            );
        } else {
            // Disable gravity by setting the flag
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags | CharFlags1::SET_DISABLE_GRAVITY as u32,
            );
        }

        gravity_disabled
    }

    pub fn set_all_no_magic_quantity_consume(&mut self) -> bool {
        let no_magic_qty_consume = self.all_no_magic_quantity_consume.read_bool_rel(Some(0));
        if no_magic_qty_consume == false {
            self.all_no_magic_quantity_consume
                .write_u8_rel(Some(0), 0x1);
        } else {
            self.all_no_magic_quantity_consume
                .write_u8_rel(Some(0), 0x0);
        }
        no_magic_qty_consume
    }

    pub fn set_no_goods_consume(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        let no_goods_consume = (current_flags & CharFlags2::NO_GOODS_CONSUME as u32) != 0;

        if no_goods_consume {
            // Disable infinite goods by clearing the flag
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::NO_GOODS_CONSUME as u32),
            );
        } else {
            // Enable infinite goods by setting the flag
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::NO_GOODS_CONSUME as u32,
            );
        }

        no_goods_consume
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

    pub fn set_draw_stable_pos(&mut self) -> bool {
        let draw_stable_pos = self.chr_data_1.read_bool_rel(Some(CharFlags2::DRAW_STABLE_POS));

        if draw_stable_pos {
            self.chr_data_1.write_u8_rel(Some(CharFlags2::DRAW_STABLE_POS), 0x0);
        } else {
            self.chr_data_1.write_u8_rel(Some(CharFlags2::DRAW_STABLE_POS), 0x1);
        }

        draw_stable_pos
    }
}
