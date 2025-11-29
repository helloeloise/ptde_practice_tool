use crate::memory::constants::CharData2;
use crate::memory::constants::{self, LevelUp};
use crate::memory::offsets::{self, BONFIRE_WARP, BONFIRE_WARP_2_OFFSET1};
use crate::memory::{Ds1, ds1};
use crate::ui::bonfire;
use mem_rs::prelude::*;
use std::alloc::{Layout, alloc, dealloc};
use std::arch::asm;

pub struct Bonfire;

impl Bonfire {
    pub fn new() -> Self {
        Bonfire
    }


    pub fn inject_bonfire_function(&mut self, ds1: &mut Ds1) {


        
        

        println!("Bonfire selected ID: {}", self.get_last_bonfire(ds1));
        let bonfire_warp_address = ds1.bonfire_warp.base_address;
        let bonfire_warp_address_2 = ds1.bonfire_warp_2.read_i32_rel(Some(0x0));
        println!("Bonfire warp function injected at address: {:#x}", bonfire_warp_address);
        println!("Bonfire warp 2 function injected at address: {:#x}", bonfire_warp_address_2);

        /*
        
                
         */

        unsafe {
            #[unsafe(no_mangle)]
            asm!(
                "mov esi, {x}",
                "mov edi, 0x1",
                "push edi",
                "call {y}",
                x = in(reg) bonfire_warp_address_2 as i32,
                y = in(reg) bonfire_warp_address as i32,
            );
        }
        


    }

    pub fn get_last_bonfire(&mut self, ds1: &mut Ds1) -> i32 {
        ds1.world_state.read_i32_rel(Some(constants::WorldState::LAST_BONFIRE))
    }

    pub fn set_last_bonfire(&mut self, ds1: &mut Ds1, bonfire_id: u32) {
        ds1.world_state.write_i32_rel(Some(constants::WorldState::LAST_BONFIRE), bonfire_id as i32);
    }

    pub fn get_bonfires() -> Vec<(&'static str, i32)> {
        vec![
            ("Depths (Entrance)", 1002900),
            ("Depths (Gaping Dragon)", 1002950),
            ("Depths (Bonfire)", 1002960),
            ("Undead Burg (Pre-Dragon Scare)", 1012960),
            ("Undead Parish (Bonfire: Sunlight Altar)", 1012961),
            ("Undead Burg (Bonfire)", 1012962),
            ("Undead Parish (Bonfire)", 1012964),
            ("Undead Parish (Fog Gate Near Boar)", 1012965),
            ("Undead Parish (Lautrec's Cell)", 1012966),
            ("Firelink Shrine (Bonfire, Repeat)", 1020980),
            ("Firelink Shrine (Bonfire)", 1022960),
            ("Painted World of Ariamis (Rope Bridge)", 1102511),
            ("Painted World of Ariamis (Bonfire)", 1102960),
            ("Painted World of Ariamis (Before Bonfire)", 1102961),
            ("Darkroot Garden (Bonfire)", 1202961),
            ("Chasm of the Abyss (Manus)", 1212950),
            ("Oolacile Sanctuary (Bonfire)", 1212961),
            ("Oolacile Township (Bonfire)", 1212962),
            ("Sanctuary Garden (Bonfire)", 1212963),
            ("Oolacile Township Dungeon (Bonfire)", 1212964),
            ("Catacombs (Bonfire: Entrance)", 1302960),
            ("Catacombs (Bonfire: Hidden)", 1302961),
            ("Catacombs (Entrance)", 1302962),
            ("Catacombs (Nito)", 1312950),
            ("Tomb of the Giants (Bonfire: Lower)", 1312960),
            ("Tomb of the Giants (Bonfire: Upper)", 1312961),
            ("Tomb of the Giants (Entrance)", 1312962),
            ("Great Hollow (Bonfire, Repeat)", 1320980),
            ("Ash Lake (Bonfire: Entrance)", 1322960),
            ("Ash Lake (Bonfire: Stone Dragon)", 1322961),
            ("Great Hollow (Bonfire)", 1322962),
            ("Blighttown (Depths Entrance, Repeat)", 1400980),
            ("Quelaag's Domain (Bonfire)", 1402960),
            ("Blighttown (Bonfire: Swamp)", 1402961),
            ("Blighttown (Bonfire: Bridge)", 1402962),
            ("Blighttown (Depths Entrance)", 1402963),
            ("Lost Izalith (Bonfire: Bed of Chaos)", 1412950),
            ("Lost Izalith (Bonfire: Lava)", 1412960),
            ("Demon Ruins (Bonfire: Quelaag's Domain Entrance)", 1412961),
            ("Demon Ruins (Bonfire: Before Demon Firesage)", 1412962),
            ("Demon Ruins (Bonfire: Before Centipede Demon)", 1412963),
            ("Demon Ruins (Bonfire: Lost Izalith Entrance)", 1412964),
            ("Sen's Fortress (Entrance, Further)", 1502960),
            ("Sen's Fortress (Bonfire)", 1502961),
            ("Sen's Fortress (Entrance, Closer)", 1502962),
            ("Anor Londo (Entrance)", 1510980),
            ("Anor Londo (Bonfire: Chamber of the Princess)", 1512950),
            ("Anor Londo (Bonfire: Firekeeper)", 1512960),
            ("Anor Londo (Bonfire: Interior)", 1512961),
            ("Anor Londo (Bonfire: Darkmoon Tomb)", 1512962),
            ("New Londo Ruins (Entrance, Repeat)", 1600980),
            ("Abyss (Bonfire)", 1602950),
            ("New Londo Ruins (Entrance)", 1602951),
            ("New Londo Ruins (Pre-Ingward)", 1602960),
            ("Darkroot Basin (Bonfire)", 1602961),
            ("Duke's Archives (Cell)", 1702900),
            ("Crystal Cave (Seath)", 1702950),
            ("Duke's Archives (Bonfire: Balcony)", 1702960),
            ("Duke's Archives (Bonfire: Cell)", 1702961),
            ("Duke's Archives (Bonfire: Entrance)", 1702962),
            ("Kiln of the First Flame (Gwyn)", 1802130),
            ("Firelink Altar (Bonfire)", 1802960),
            ("Kiln of the First Flame (Entrance)", 1802961),
            ("Northern Undead Asylum (Cell)", 1812100),
            ("Northern Undead Asylum (Cell, Repeat)", 1812900),
            ("Northern Undead Asylum (Bonfire: Courtyard)", 1812960),
            ("Northern Undead Asylum (Bonfire: Hallway)", 1812961),
        ]
    }
}