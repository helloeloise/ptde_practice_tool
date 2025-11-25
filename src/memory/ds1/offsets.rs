pub const POS_LOCK_AOB: &str = "F3 0F 11 44 24 08 F3 0F 11 0C 24 F3 0F 11 54 24 04 F3 0F 7E 04 24";
pub const POS_LOCK_1_AOB_OFFSET: usize =  0x1;
pub const POS_LOCK_2_AOB_OFFSET: usize =  0x2;

pub const NODE_GRAPH_AOB: &str = "8B 4C 24 5C 8B 11 50 8B 42 34 FF D0 80 BB 90 00 00 00 ?";
pub const NODE_GRAPH_AOB_OFFSET: usize =  0x1;

pub const ALL_NO_MAGIC_QTY_CONSUME_AOB: &str = "38 1D ? ? ? ? 0F 94 C1 3A CB";
pub const ALL_NO_MAGIC_QTY_CONSUME_AOB_OFFSET: usize = 0x2;

pub const PLAYER_NO_DEAD_AOB: &str = "53 56 8B F0 8A 9E C4 03 00 00 8B 06 8B 90 A4 00 00 00 C0 EB 05 8B CE 80 E3 01 FF D2 84 C0 ? ? 80 3D ? ? ? ? 00";
pub const PLAYER_NO_DEAD_AOB_OFFSET: usize =  0x2;

pub const PLAYER_EXTERMINATE_AOB: &str = "8B 11 8B 82 A4 00 00 00 FF D0 84 C0 ? ? 80 3D ? ? ? ? 00";
pub const PLAYER_EXTERMINATE_AOB_OFFSET: usize =  0x10;

pub const all_no_stamina_consume_aob: &str = "51 8B 4C 24 08 3B 8A E4 02 00 00 ? ? F6 82 C5 03 00 00 04 ? ? 80 3D ? ? ? ? 00";
pub const all_no_stamina_consume_aob_offset: usize =  0x18;

pub const DRAW_MAP_AOB : &str = "80 3D ? ? ? ? 00 A1 ? ? ? ? 8B 48 08 8B 11 56 8B 72 28 B8 00 00 00 80";
pub const DRAW_MAP_AOB_OFFSET: usize =  0x2;

pub const char_data_1_aob : &str = "8B 15 ? ? ? ? F3 0F 10 44 24 30 52";
pub const char_data_1_aob_offset : usize =  0x2;
pub const char_data_1_offset1 : usize =  0x0;
pub const char_data_1_offset2 : usize =  0x4;
pub const char_data_1_offset3 : usize =  0x0; 