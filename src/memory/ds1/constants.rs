pub struct ChrDbg;

#[allow(dead_code)]
impl ChrDbg {
    pub const ALL_NO_STAMINA_CONSUME: usize = 0;
    pub const ALL_NO_MPCONSUME: usize = 1;
    pub const ALL_NO_ARROW_CONSUME: usize = 2;
    pub const PLAYER_HIDE: usize = 3;
    pub const PLAYER_SILENCE: usize = 4;
    pub const ALL_NO_DEAD: usize = 5;
    pub const ALL_NO_DAMAGE: usize = 6;
    pub const ALL_NO_HIT: usize = 7;
    pub const ALL_NO_ATTACK: usize = 8;
    pub const ALL_NO_MOVE: usize = 9;
    pub const ALL_NO_UPDATE_AI: usize = 0xA;
}

pub struct DrawMap;

#[allow(dead_code)]
impl DrawMap {
    pub const DRAW_MAP: usize = 0;
    pub const DRAW_OBJECTS: usize = 0;
    pub const DRAW_CREATURES: usize = 0;
    pub const DRAW_SFX: usize = 0;
}

pub struct CharData1;

#[allow(dead_code)]
impl CharData1 {
    pub const CHAR_MAP_DATA_PTR: usize = 0x28;
    pub const CHR_TYPE: usize = 0x70;
    pub const TEAM_TYPE: usize = 0x74;
    pub const FORCE_PLAY_ANIMATION: usize = 0xFC;
    pub const CHAR_FLAGS_1: usize = 0x1FC;
    pub const PLAY_REGION: usize = 0x284;
    pub const HP: usize = 0x2D4;
    pub const STAMINA: usize = 0x2E4;
    pub const CHAR_FLAGS_2: usize = 0x3C4;
    pub const STORED_ITEM: usize = 0x628;
}

pub struct CharMapData;

#[allow(dead_code)]
impl CharMapData {
    pub const ANIM_DATA_PTR: usize = 0x14;
    pub const CHAR_POS_DATA_PTR: usize = 0x1C;
    pub const CHAR_MAP_FLAGS: usize = 0xC4;
    pub const WARP: usize = 0xC8;
    pub const WARP_X: usize = 0xD0;
    pub const WARP_Y: usize = 0xD4;
    pub const WARP_Z: usize = 0xD8;
    pub const WARP_ANGLE: usize = 0xD4;
}

pub struct AnimData;

#[allow(dead_code)]
impl AnimData {
    pub const PLAY_SPEED: usize = 0x64;
}

pub struct CharPosData;

#[allow(dead_code)]
impl CharPosData {
    pub const POS_ANGLE: usize = 0x4;
    pub const POS_X: usize = 0x10;
    pub const POS_Y: usize = 0x14;
    pub const POS_Z: usize = 0x18;
}

pub struct CharData2;

#[allow(dead_code)]
impl CharData2 {
    pub const HP: usize = 0xC;
    pub const STAMINA: usize = 0x28;
    pub const VITALITY: usize = 0x38;
    pub const ATTUNEMENT: usize = 0x40;
    pub const ENDURANCE: usize = 0x48;
    pub const STRENGTH: usize = 0x50;
    pub const DEXTERITY: usize = 0x58;
    pub const INTELLIGENCE: usize = 0x60;
    pub const FAITH: usize = 0x68;
    pub const RESISTANCE: usize = 0x80;
    pub const SOUL_LEVEL: usize = 0x88;
    pub const SOULS: usize = 0x8C;
    pub const HUMANITY: usize = 0x7C;
    pub const STANCE: usize = 0x230;
}

pub struct LevelUp;

#[allow(dead_code)]
impl LevelUp {
    pub const VITALITY: usize = 0x0;
    pub const ATTUNEMENT: usize = 0x4;
    pub const ENDURANCE: usize = 0x8;
    pub const STRENGTH: usize = 0xC;
    pub const DEXTERITY: usize = 0x10;
    pub const RESISTANCE: usize = 0x14;
    pub const INTELLIGENCE: usize = 0x18;
    pub const FAITH: usize = 0x1C;
    pub const SOUL_LEVEL: usize = 0x16C;
    pub const SOULS: usize = 0x178;
}


pub struct WorldState;

#[allow(dead_code)]
impl WorldState {
    pub const LAST_BONFIRE: usize = 0xB04;
    pub const POS_X_STABLE: usize = 0xB70;
    pub const POS_Y_STABLE: usize = 0xB74;
    pub const POS_Z_STABLE: usize = 0xB78;
    pub const POS_ANGLE_STABLE: usize = 0xB84;
}
