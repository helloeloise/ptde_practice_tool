// Freecam-specific constants and mutable runtime state.
// Hook asm lives in freecam_hooks.rs and accesses these symbols through the parent ds1 module.

// Freecam: Original bytes at injection points (for verification)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) const FREECAM_ORIG_PATCH_1: [u8; 7] = [0x89, 0x44, 0x24, 0x24, 0x8B, 0x43, 0x44];
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) const FREECAM_ORIG_PATCH_2: [u8; 6] = [0xC1, 0xEA, 0x14, 0xF6, 0xC2, 0x01];

// Freecam: Dynamically scanned addresses (stored as statics for naked asm access)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_INJECT_ADDR_1: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_INJECT_ADDR_2: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_CAM_MGR_PTR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut CHR_FOLLOW_CAM_PTR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_HOOK2_SKIP_ADDR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_MOTION_FUNC: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_MOTION_CALL_SAFE: u8 = 0;

// Freecam: Hook state
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_MODE: i32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_IN_HOOK: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_L3R3_HELD: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_XINPUT_HELD: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_ACTIVE_PAD: i32 = -1;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_FORCE_MODE2_ACTIVE_ONLY: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_MOVE_DX: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_MOVE_DY: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_MOVE_DZ: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_INJECTION_PENDING: bool = false;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_INJECTION_SUCCESS: bool = false;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_FUNC_ADDR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_CAM_OBJ: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_CHR_FOLLOWCAM_PTR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_CHR_BASE: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_CHR_L1: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_CHR_L2: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_CHR_FOLLOWCAM_OBJ: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_CALLED_FUNC: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_ENTRY: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_INPUT_RAW_8: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_INPUT_RAW_C: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_MANUAL_TICKS: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_MANUAL_WRITES: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_CAMERA_FOLLOW_DISABLED: bool = false;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_MANUAL_AXES_X: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_MANUAL_AXES_Y: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_MANUAL_AXES_Z: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_LAST_WRITE_ADDR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_FALLBACK_BUF_BASE: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_XINPUT_PAD: i32 = -1;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_XINPUT_BUTTONS: u16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_XINPUT_LX: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_XINPUT_LY: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_XINPUT_RX: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_XINPUT_RY: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_XINPUT_LT: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_XINPUT_RT: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_INJECT_MATCHES_1: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_INJECT_MATCHES_2: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_PAIR_DELTA: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_PAIR_DELTA_ERR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_PAIR_CALL_REF: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK2_SKIP_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK2_ORIG_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK2_ENTRY_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF_18: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF_30: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF_254: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF30_TA_X: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF30_TA_Y: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF30_TA_Z: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF30_TB_X: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF30_TB_Y: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_BUF30_TB_Z: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_APPLY_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_LAST_DST: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_LAST_SRC: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_MOVE_DX: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_MOVE_DY: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_MOVE_DZ: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_POST_40: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_POST_44: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_DEBUG_HOOK_POST_48: f32 = 0.0;

// Freecam: Storage buffer for transform data (256 bytes, naturally aligned)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut FREECAM_STORAGE_BUFFER: [u32; 64] = [0; 64];
