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

    pub fn set_player_vitality(&mut self, ds1: &mut Ds1, vitality: i32) {
        ds1.chr_data_2.write_i32_rel(Some(CharData2::VITALITY), vitality);
    }

    pub fn set_player_attunement(&mut self, ds1: &mut Ds1, attunement: i32) {
        ds1.chr_data_2.write_i32_rel(Some(CharData2::ATTUNEMENT), attunement);
    }
    pub fn set_player_endurance(&mut self, ds1: &mut Ds1, endurance: i32) {
        ds1.chr_data_2.write_i32_rel(Some(CharData2::ENDURANCE), endurance);
    }
    pub fn set_player_strength(&mut self, ds1: &mut Ds1, strength: i32) {
        ds1.chr_data_2.write_i32_rel(Some(CharData2::STRENGTH), strength);
    }
    pub fn set_player_dexterity(&mut self, ds1: &mut Ds1, dexterity: i32) {
        ds1.chr_data_2.write_i32_rel(Some(CharData2::DEXTERITY), dexterity);
    }
    pub fn set_player_intelligence(&mut self, ds1: &mut Ds1, intelligence: i32) {
        ds1.chr_data_2.write_i32_rel(Some(CharData2::INTELLIGENCE), intelligence);
    }
    pub fn set_player_faith(&mut self, ds1: &mut Ds1, faith: i32) {
        ds1.chr_data_2.write_i32_rel(Some(CharData2::FAITH), faith);
    }

}


