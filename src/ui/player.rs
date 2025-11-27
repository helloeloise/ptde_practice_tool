use crate::memory::constants::CharData2;
use crate::memory::{Ds1, ds1};
use crate::memory::offsets;
use crate::memory::constants;

use mem_rs::prelude::*;
pub struct Player {
    pub x_stored_pos: f32,
    pub y_stored_pos: f32,
    pub z_stored_pos: f32,

    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,

    pub hp: u32,
    pub stamina: u32,

    pub vitality: u32,
    pub attunement: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
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


        self.vitality = ds1.chr_data_2.read_u32_rel(Some(CharData2::VITALITY));
        self.attunement = ds1.chr_data_2.read_u32_rel(Some(CharData2::ATTUNEMENT));
        self.endurance = ds1.chr_data_2.read_u32_rel(Some(CharData2::ENDURANCE));
        self.strength = ds1.chr_data_2.read_u32_rel(Some(CharData2::STRENGTH));
        self.dexterity = ds1.chr_data_2.read_u32_rel(Some(CharData2::DEXTERITY));
        self.intelligence = ds1.chr_data_2.read_u32_rel(Some(CharData2::INTELLIGENCE));
        self.faith = ds1.chr_data_2.read_u32_rel(Some(CharData2::FAITH));
        self.hp = ds1.chr_data_2.read_u32_rel(Some(CharData2::HP));
        self.stamina = ds1.chr_data_2.read_u32_rel(Some(CharData2::STAMINA));
    }
}
