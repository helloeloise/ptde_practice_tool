use super::*;

impl Ds1 {
    pub fn get_death_count(&self) -> i32 {
        self.game_data_mgr.read_i32_rel(Some(GameDataMgr::DEATH_COUNT))
    }

    pub fn set_death_count(&mut self, value: i32) {
        self.game_data_mgr.write_i32_rel(Some(GameDataMgr::DEATH_COUNT), value);
    }

    pub fn get_ng_plus(&self) -> u8 {
        self.game_data_mgr.read_u8_rel(Some(GameDataMgr::NG_PLUS))
    }

    pub fn set_ng_plus(&mut self, value: u8) {
        self.game_data_mgr.write_u8_rel(Some(GameDataMgr::NG_PLUS), value);
    }

    pub fn get_disable_enemies(&self) -> bool {
        self.world_state.read_bool_rel(Some(WorldState::DISABLE_ENEMIES))
    }

    pub fn set_disable_enemies_to(&mut self, value: bool) {
        self.world_state
            .write_u8_rel(Some(WorldState::DISABLE_ENEMIES), if value { 1 } else { 0 });
    }

    pub fn get_disable_events(&self) -> bool {
        self.world_state.read_bool_rel(Some(WorldState::DISABLE_EVENTS))
    }

    pub fn set_disable_events_to(&mut self, value: bool) {
        self.world_state
            .write_u8_rel(Some(WorldState::DISABLE_EVENTS), if value { 1 } else { 0 });
    }

    pub fn get_auto_save(&self) -> bool {
        self.world_state.read_bool_rel(Some(WorldState::AUTO_SAVE))
    }

    pub fn set_auto_save_to(&mut self, value: bool) {
        self.world_state
            .write_u8_rel(Some(WorldState::AUTO_SAVE), if value { 1 } else { 0 });
    }

    pub fn get_online_mode(&self) -> bool {
        self.world_state.read_bool_rel(Some(WorldState::ONLINE_MODE))
    }

    pub fn set_online_mode_to(&mut self, value: bool) {
        self.world_state
            .write_u8_rel(Some(WorldState::ONLINE_MODE), if value { 1 } else { 0 });
    }
}
