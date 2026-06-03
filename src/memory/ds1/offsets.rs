pub const POS_LOCK_AOB: &str = "F3 0F 11 44 24 08 F3 0F 11 0C 24 F3 0F 11 54 24 04 F3 0F 7E 04 24";
pub const POS_LOCK_1_AOB_OFFSET: usize = 0x1;
pub const POS_LOCK_2_AOB_OFFSET: usize = 0x2;

pub const NODE_GRAPH_AOB: &str = "8B 4C 24 5C 8B 11 50 8B 42 34 FF D0 80 BB 90 00 00 00 ?";
pub const NODE_GRAPH_AOB_OFFSET: usize = 0x1;

pub const PLAYER_NO_DEAD_AOB: &str = "53 56 8B F0 8A 9E C4 03 00 00 8B 06 8B 90 A4 00 00 00 C0 EB 05 8B CE 80 E3 01 FF D2 84 C0 ? ? 80 3D ? ? ? ? 00";
pub const PLAYER_NO_DEAD_AOB_OFFSET: usize = 0x22;

pub const PLAYER_EXTERMINATE_AOB: &str = "8B 11 8B 82 A4 00 00 00 FF D0 84 C0 ? ? 80 3D ? ? ? ? 00";
pub const PLAYER_EXTERMINATE_AOB_OFFSET: usize = 0x10;

pub const ALL_NO_STAMINA_CONSUME_AOB: &str =
    "51 8B 4C 24 08 3B 8A E4 02 00 00 ? ? F6 82 C5 03 00 00 04 ? ? 80 3D ? ? ? ? 00";
pub const ALL_NO_STAMINA_CONSUME_AOB_OFFSET: usize = 0x18;

pub const DRAW_MAP_AOB: &str =
    "80 3D ? ? ? ? 00 A1 ? ? ? ? 8B 48 08 8B 11 56 8B 72 28 B8 00 00 00 80";
pub const DRAW_MAP_AOB_OFFSET: usize = 0x2;

pub const CHAR_DATA_1_AOB: &str = "8B 15 ? ? ? ? F3 0F 10 44 24 30 52"; // 0137DC70
pub const CHAR_DATA_1_AOB_OFFSET: usize = 0x2;
pub const CHAR_DATA_1_OFFSET1: usize = 0x0;
pub const CHAR_DATA_1_OFFSET2: usize = 0x4;
pub const CHAR_DATA_1_OFFSET3: usize = 0x0;

pub const TARGET_BANK_AOB: &str = "8B 15 ? ? ? ? F3 0F 10 44 24 30 52";
pub const TARGET_BANK_AOB_OFFSET: usize = 0x2;
pub const TARGET_BANK_OFFSET1: usize = 0x0;

pub const CHAR_DATA_2_AOB: &str = "A1 ? ? ? ? 8B 40 34 53 32 DB 85 C0";
pub const CHAR_DATA_2_AOB_OFFSET: usize = 0x1;
pub const CHAR_DATA_2_OFFSET1: usize = 0x0;
pub const CHAR_DATA_2_OFFSET2: usize = 0x8;

pub const LEVEL_UP: &str = "83 EC 08 8B 15 ? ? ? ? 56 57 8B 7A 08";

pub const BONFIRE_WARP: &str = "33 C0 39 46 04 ? ? 8B 0D ? ? ? ? 8B 15 ? ? ? ? C6 41 44 01";
pub const BONFIRE_WARP_2: &str =
    "89 73 44 C6 44 24 2C 00 C7 82 84 00 00 00 06 00 00 00 8B 3D ? ? ? ? 85 FF";
pub const BONFIRE_WARP_2_OFFSET1: usize = 0x14;

pub const WORLD_STATE_AOB: &str =
    "8B 54 24 10 8B C8 F7 D9 39 8A B8 0E 00 00 B3 01 0F 95 C2 8B 0D ? ? ? ? 80 B9 A5 0B 00 00 00";
pub const WORLD_STATE_AOB_OFFSET: usize = 0x15;
pub const WORLD_STATE_OFFSET1: usize = 0x0;

pub const FLAGS_AOB_1: &str = "8B 76 0C 89 35 ? ? ? ? 33 C0";
pub const FLAGS_AOB_1_OFFSET: usize = 0x5;

pub const INPUT_STATE_AOB: &str = "a1 ? ? ? ? 83 ec 28 53 c7 47 08 00 00 00 00 8b 58 3c";
pub const INPUT_STATE_AOB_OFFSET: usize = 0x1;
pub const INPUT_STATE_OFFSET1: usize = 0x0;
pub const INPUT_STATE_OFFSET2: usize = 0x3c;
pub const INPUT_STATE_OFFSET3: usize = 0x28;
pub const INPUT_STATE_OFFSET4: usize = 0xC0;

pub const QUITOUT_AOB: &str =
    "8B 70 3C 8B 2D ? ? ? ? 8B 45 00 8B 8E 48 01 00 00 89 4C 24 34 83 F8 01";
pub const QUITOUT_AOB_OFFSET: usize = 0x5;
pub const QUITOUT_OFFSET1: usize = 0x0;

pub const NO_DEATH_AOB: &str = "53 56 8B F0 8A 9E C4 03 00 00 8B 06 8B 90 A4 00 00 00 C0 EB 05 8B CE 80 E3 01 FF D2 84 C0 ? ? 80 3D ? ? ? ? 00";
pub const NO_DEATH_AOB_OFFSET: usize = 0x22;

pub const ITEM_GET_AOB: &str = "55 8B EC 83 E4 F8 83 EC 34 8B 4D 0C 53 8B 5D 08 56 83 C8 FF 33 F6 81 F9 00 00 00 20 57 89 44 24 1C 89 74 24 20 89 B3 8C 01 00 00 89 44 24 18";

pub const ITEM_DROP_AOB: &str = "8B 0D ? ? ? ? 83 EC 68 81 C1 28 08 00 00";
pub const ITEM_DROP_UNKNOWN_1_AOB: &str =
    "88 5D 3C 88 5D 3D F3 0F 10 61 08 F3 0F 11 65 40 39 1D ? ? ? ?";
pub const ITEM_DROP_UNKNOWN_1_AOB_OFFSET: usize = 0x12;
pub const ITEM_DROP_UNKNOWN_2_AOB: &str =
    "D9 E8 8B 1D ? ? ? ? 83 EC 08 D9 54 24 04 D9 1C 24 8D 44 24 20 6A 03 8B D3";
pub const ITEM_DROP_UNKNOWN_2_AOB_OFFSET: usize = 0x4;

pub const ALL_NO_MAGIC_QTY_CONSUME_AOB: &str = "38 1D ? ? ? ? 0F 94 C1 3A CB";
pub const ALL_NO_MAGIC_QTY_CONSUME_AOB_OFFSET: usize = 0x2;

pub const LOCK_ON_MGR_AOB: &str = "A1 ? ? ? ? 33 D2 89 93 40 01 00 00 89";
pub const LOCK_ON_MGR_AOB_OFFSET: usize = 0x1;

// Freecam injection points (matching Cheat Engine table)
pub const FREECAM_INJECT_1_AOB: &str = "89 44 24 24 8B 43 44";  // DARKSOULS.exe+B00AD4
pub const FREECAM_INJECT_2_AOB: &str = "C1 EA 14 F6 C2 01";     // DARKSOULS.exe+BFB431

// Debug/Release RVA mapping for PTDE freecam hooks.
// Release RVAs are debug RVAs minus 0x41C0.
pub const FREECAM_DEBUG_TO_RELEASE_DELTA: usize = 0x41C0;
pub const FREECAM_IMAGE_BASE: usize = 0x0040_0000;
pub const FREECAM_INJECT_1_DEBUG_RVA: usize = 0x00B0_0AD4;
pub const FREECAM_INJECT_2_DEBUG_RVA: usize = 0x00BF_B431;
pub const FREECAM_MOTION_FUNC_DEBUG_RVA: usize = 0x00BF_AE60;
pub const FREECAM_INJECT_1_RELEASE_RVA: usize = FREECAM_INJECT_1_DEBUG_RVA - FREECAM_DEBUG_TO_RELEASE_DELTA;
pub const FREECAM_INJECT_2_RELEASE_RVA: usize = FREECAM_INJECT_2_DEBUG_RVA - FREECAM_DEBUG_TO_RELEASE_DELTA;
pub const FREECAM_MOTION_FUNC_RELEASE_RVA: usize = FREECAM_MOTION_FUNC_DEBUG_RVA - FREECAM_DEBUG_TO_RELEASE_DELTA;

// Camera manager pointer (used for setting freecam mode in camera object)
pub const FREECAM_CAM_MGR_AOB: &str = "A1 ? ? ? ? 8B 80 EC 00 00 00 3B C6";
pub const FREECAM_CAM_MGR_AOB_OFFSET: usize = 0x1;

// ChrFollowCam - the actual render camera structure (from DSGadget)
pub const CHR_FOLLOW_CAM_AOB: &str = "D9 45 08 A1 ? ? ? ? 51 D9 1C 24 50";
pub const CHR_FOLLOW_CAM_AOB_OFFSET: usize = 0x4;

// Try to find a camera update function by scanning for common prologue patterns
// Pattern 1: Standard function prologue with stack frame
pub const CAMERA_UPDATE_FUNC_1_AOB: &str = "55 8B EC 83 EC ? 53 56 57";
// Pattern 2: Function that takes camera pointer as param
pub const CAMERA_UPDATE_FUNC_2_AOB: &str = "56 8B 74 24 08 85 F6 74";
// Pattern 3: Look near our injection point for calls
pub const CAMERA_UPDATE_FUNC_3_AOB: &str = "8B 43 44 85 C0 0F 9E C1 83 C0 FF";

pub const PHANTOM_TYPE_AOB: &str = "33 ? 83 ? ? 0F 87 ? ? ? ? FF ? ? ? ? ? ? 84 ? 0F 84 ? ? ? ? B8 ? ? ? ? C3";

// Camera follow disable - pattern around the je instruction that skips camera update
// We'll patch the je (74 70) at offset +5 to jmp (EB 70) to always skip
pub const CAMERA_FOLLOW_DISABLE_AOB: &str = "80 7C 24 38 00 74 70 F3 0F 7E B4";
pub const CAMERA_FOLLOW_DISABLE_PATTERN: &[u8] = &[
    0x80, 0x7C, 0x24, 0x38, 0x00,  // cmp byte ptr [esp+38], 0
    0x74, 0x70,                     // je +0x70 (THIS is what we patch to EB)
    0xF3, 0x0F, 0x7E, 0xB4          // movq xmm6, ...
];
pub const CAMERA_FOLLOW_DISABLE_PATCH_OFFSET: usize = 0x5; // Offset to the 74 byte

// Camera follow - NOP the camera position write instructions
// Location 1: Around B04183
pub const CAMERA_WRITE_1_XY_RVA: usize = 0x00B04183; // 7 bytes - movq2dq [ebx+100],mmx0
pub const CAMERA_WRITE_1_XY_SIZE: usize = 7;
pub const CAMERA_WRITE_1_Z_RVA: usize = 0x00B04190; // 8 bytes - movq [eax+108],xmm0
pub const CAMERA_WRITE_1_Z_SIZE: usize = 8;

// Location 2: Around B0EF77
pub const CAMERA_WRITE_2_XY_RVA: usize = 0x00B0EF77; // 8 bytes - movq [ebx+100],xmm0
pub const CAMERA_WRITE_2_XY_SIZE: usize = 8;
pub const CAMERA_WRITE_2_Z_RVA: usize = 0x00B0EF7F; // 8 bytes - movq [ebx+108],xmm1
pub const CAMERA_WRITE_2_Z_SIZE: usize = 8;