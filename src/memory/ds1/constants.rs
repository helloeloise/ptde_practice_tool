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
    pub const CURRENT_POISE: usize = 0x1c0;
    pub const MAX_POISE: usize = 0x1c4;
    pub const POISE_RECOVERY_RATE: usize = 0x1cc;
    pub const AI_ID: usize = 0x41c;
}

pub struct CharFlags1;

#[allow(dead_code)]
impl CharFlags1 {
    pub const SET_DEAD_MODE: usize = 0x02000000;
    pub const DISABLE_DAMAGE: usize = 0x04000000; // Doesn't prevent healing
    pub const ENABLE_INVINCIBLE: usize = 0x08000000; // Super armor and disable damage, still die to killbox
    pub const FIRST_PERSON: usize = 0x00100000;
    pub const SET_DRAW_ENABLE: usize = 0x00800000;
    pub const SET_SUPER_ARMOR: usize = 0x00010000;
    pub const SET_DISABLE_GRAVITY: usize = 0x00004000;
    pub const FORCE_SET_OMISSION: usize = 0x00000100;
    pub const FORCE_UPDATE_NEXT_FRAME: usize = 0x00000200;
    pub const FORCE_PLAY_ANIMATION: usize = 0x00000100;
    pub const SET_EVENT_GENERATE: usize = 0x00000010;
    pub const DISABLE_HP_GAUGE: usize = 0x00000008;
}

pub struct CharFlags2;

#[allow(dead_code)]
impl CharFlags2 {
    pub const NO_GOODS_CONSUME: usize = 0x01000000;
    pub const DRAW_COUNTER: usize = 0x00200000;
    pub const DRAW_DIRECTION: usize = 0x00010000;
    pub const NO_UPDATE: usize = 0x00008000;
    pub const NO_ATTACK: usize = 0x00000100;
    pub const NO_MOVE: usize = 0x00000200;
    pub const NO_STAM_CONSUME: usize = 0x00000400;
    pub const NO_MP_CONSUME: usize = 0x00000800;
    pub const NO_DEAD: usize = 0x00000020;
    pub const NO_DAMAGE: usize = 0x00000040;
    pub const NO_HIT: usize = 0x00000080;
    pub const DRAW_STABLE_POS: usize = 0x6c8;
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
    pub const WARP_ANGLE: usize = 0xDC;
}

pub struct CharMapFlags;

#[allow(dead_code)]
impl CharMapFlags {
    pub const DISABLE_MAP_HIT: usize = 0x00000010;
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
    pub const MAX_HP: usize = 0x14;
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
    pub const INVENTORY_INDEX_START: usize = 0x1B8;
    pub const EQUIP_LEFT_1_IDX: usize = 0x1D4;
    pub const EQUIP_RIGHT_1_IDX: usize = 0x1D8;
    pub const EQUIP_LEFT_2_IDX: usize = 0x1DC;
    pub const EQUIP_RIGHT_2_IDX: usize = 0x1E0;
    pub const EQUIP_ARROW_1_IDX: usize = 0x1E4;
    pub const EQUIP_BOLT_1_IDX: usize = 0x1E8;
    pub const EQUIP_ARROW_2_IDX: usize = 0x1EC;
    pub const EQUIP_BOLT_2_IDX: usize = 0x1F0;
    pub const EQUIP_HELMET_IDX: usize = 0x1F4;
    pub const EQUIP_CHEST_IDX: usize = 0x1F8;
    pub const EQUIP_GLOVE_IDX: usize = 0x1FC;
    pub const EQUIP_PANTS_IDX: usize = 0x200;
    pub const EQUIP_HAIR_IDX: usize = 0x204;
    pub const EQUIP_RING_1_IDX: usize = 0x208;
    pub const EQUIP_RING_2_IDX: usize = 0x20C;
    pub const EQUIP_ITEM_1_IDX: usize = 0x210;
    pub const EQUIP_ITEM_2_IDX: usize = 0x214;
    pub const EQUIP_ITEM_3_IDX: usize = 0x218;
    pub const EQUIP_ITEM_4_IDX: usize = 0x21C;
    pub const EQUIP_ITEM_5_IDX: usize = 0x220;
    pub const STANCE: usize = 0x230;
    pub const EQUIP_LEFT_1_ID: usize = 0x24C;
    pub const EQUIP_RIGHT_1_ID: usize = 0x250;
    pub const EQUIP_LEFT_2_ID: usize = 0x254;
    pub const EQUIP_RIGHT_2_ID: usize = 0x258;
    pub const EQUIP_ARROW_1_ID: usize = 0x25C;
    pub const EQUIP_BOLT_1_ID: usize = 0x260;
    pub const EQUIP_ARROW_2_ID: usize = 0x264;
    pub const EQUIP_BOLT_2_ID: usize = 0x268;
    pub const EQUIP_HELMET_ID: usize = 0x26C;
    pub const EQUIP_CHEST_ID: usize = 0x270;
    pub const EQUIP_GLOVE_ID: usize = 0x274;
    pub const EQUIP_PANTS_ID: usize = 0x278;
    pub const EQUIP_HAIR_ID: usize = 0x27C;
    pub const EQUIP_RING_1_ID: usize = 0x280;
    pub const EQUIP_RING_2_ID: usize = 0x284;
    pub const EQUIP_ITEM_1_ID: usize = 0x288;
    pub const EQUIP_ITEM_2_ID: usize = 0x28C;
    pub const EQUIP_ITEM_3_ID: usize = 0x290;
    pub const EQUIP_ITEM_4_ID: usize = 0x294;
    pub const EQUIP_ITEM_5_ID: usize = 0x298;
    pub const GENDER: usize = 0xC2;
    pub const NEW_GAME: usize = 0x3C;
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
