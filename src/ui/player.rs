use crate::memory::constants::CharData2;
use crate::memory::constants::{self, LevelUp};
use crate::memory::offsets;
use crate::memory::{Ds1, ds1};
use mem_rs::prelude::*;
use std::alloc::{Layout, alloc, dealloc};
use std::arch::asm;
pub struct Player {
    pub x_stored_pos: f32,
    pub y_stored_pos: f32,
    pub z_stored_pos: f32,

    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,

    pub hp: i32,
    pub stamina: i32,

    pub vitality: i32,
    pub attunement: i32,
    pub endurance: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub faith: i32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x_stored_pos: 0.,
            y_stored_pos: 0.,
            z_stored_pos: 0.,

            x_pos: 0.,
            y_pos: 0.,
            z_pos: 0.,

            hp: 0,
            stamina: 0,

            vitality: 0,
            attunement: 0,
            endurance: 0,
            strength: 0,
            dexterity: 0,
            intelligence: 0,
            faith: 0,
        }
    }

    //Putting this in a separated function because i am unsure if i can directly get the values from ds1 in the ::new()
    pub fn instantiate(&mut self, ds1: &mut Ds1) {
        self.x_pos = ds1.get_x_pos();
        self.y_pos = ds1.get_y_pos();
        self.z_pos = ds1.get_z_pos();

        self.vitality = ds1.chr_data_2.read_i32_rel(Some(CharData2::VITALITY));
        self.attunement = ds1.chr_data_2.read_i32_rel(Some(CharData2::ATTUNEMENT));
        self.endurance = ds1.chr_data_2.read_i32_rel(Some(CharData2::ENDURANCE));
        self.strength = ds1.chr_data_2.read_i32_rel(Some(CharData2::STRENGTH));
        self.dexterity = ds1.chr_data_2.read_i32_rel(Some(CharData2::DEXTERITY));
        self.intelligence = ds1.chr_data_2.read_i32_rel(Some(CharData2::INTELLIGENCE));
        self.faith = ds1.chr_data_2.read_i32_rel(Some(CharData2::FAITH));
        self.hp = ds1.chr_data_2.read_i32_rel(Some(CharData2::HP));
        self.stamina = ds1.chr_data_2.read_i32_rel(Some(CharData2::STAMINA));
    }

    pub fn moveswap(&mut self, ds1: &mut Ds1){
        ds1.chr_data_2.write_i64_rel(Some(CharData2::STANCE), 2);
    }

    pub fn set_player_stat(&mut self, ds1: &mut Ds1, stat: usize, value: i32) {
        ds1.chr_data_2.write_i32_rel(Some(stat), value);
        self.inject_levelup_function(ds1);
    }

    pub fn inject_levelup_function(&mut self, ds1: &mut Ds1) {
        let stored_humanity = ds1.chr_data_2.read_i32_rel(Some(CharData2::HUMANITY));
        let level_up_fn_address = ds1.level_up.base_address;

        println!("Level up function address: {:X}", level_up_fn_address);
        let mut x: i32 = 25;
        let mut y: i32 = ds1.level_up.base_address as i32;
        let level_up_codecave: i32 = code_cave as *const () as i32;
        println!("Ptr address: {:X}", level_up_codecave);
        print!("Level up codecave address: {:X}", level_up_codecave);
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x0).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::VITALITY)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x4).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::ATTUNEMENT)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x8).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::ENDURANCE)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0xC).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::STRENGTH)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x10).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::DEXTERITY)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x14).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::RESISTANCE)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x18).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::INTELLIGENCE)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x1C).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::FAITH)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x16c).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::SOUL_LEVEL)),
        );
        ds1.process.write_i32_abs(
            (level_up_codecave + 0x178).try_into().unwrap(),
            ds1.chr_data_2.read_i32_rel(Some(CharData2::SOULS)),
        );

        unsafe {
            #[unsafe(no_mangle)]
            asm!(
                "mov eax, {x}",
                "mov ecx, {y}",
                "call {z}",x = in(reg) level_up_codecave as i32, y = in(reg)level_up_codecave - 328 ,z = in(reg) level_up_fn_address as i32,



            );
        }
        ds1.chr_data_2
            .write_i32_rel(Some(CharData2::HUMANITY), stored_humanity);
    }

    /*

    unsafe {
        asm!(
            "mov eax, {x}",
            "mov ecx, {x}",
            "call 0xC75DD0",
            "ret",
            x = in(reg) level_up_codecave as i32,

        )
    }
    */
}

#[unsafe(no_mangle)]
pub fn code_cave() {
    unsafe {
        core::arch::asm!(
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",
            "nop", "nop", "nop", "nop", "nop", "nop"
        );
    }
}
