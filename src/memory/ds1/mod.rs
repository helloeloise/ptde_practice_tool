pub mod constants;
pub mod offsets;

use crate::memory::{ds1::constants::*, offsets::BONFIRE_WARP_2_OFFSET1};
use mem_rs::prelude::*;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use std::ffi::c_void;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE, PAGE_EXECUTE_READ,
    PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS, PAGE_READWRITE,
    PAGE_WRITECOPY, VirtualProtect, VirtualQuery,
};
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use windows_sys::Win32::UI::Input::XboxController::{
    XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB, XINPUT_GAMEPAD_RIGHT_SHOULDER,
    XINPUT_GAMEPAD_RIGHT_THUMB, XINPUT_STATE, XInputGetState,
};
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_CONTROL, VK_DOWN, VK_LEFT, VK_RIGHT, VK_SHIFT, VK_SPACE, VK_UP,
};
use std::fs::OpenOptions;
use std::io::Write;

// Freecam: Original bytes at injection points (for verification)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
const FREECAM_ORIG_PATCH_1: [u8; 7] = [0x89, 0x44, 0x24, 0x24, 0x8B, 0x43, 0x44];
#[cfg(all(target_os = "windows", target_arch = "x86"))]
const FREECAM_ORIG_PATCH_2: [u8; 6] = [0xC1, 0xEA, 0x14, 0xF6, 0xC2, 0x01];

// Freecam: Dynamically scanned addresses (stored as statics for naked asm access)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_INJECT_ADDR_1: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_INJECT_ADDR_2: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_CAM_MGR_PTR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut CHR_FOLLOW_CAM_PTR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_HOOK2_SKIP_ADDR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_MOTION_FUNC: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_MOTION_CALL_SAFE: u8 = 0;

// Freecam: Hook state
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_MODE: i32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_IN_HOOK: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_L3R3_HELD: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_XINPUT_HELD: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_ACTIVE_PAD: i32 = -1;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_FORCE_MODE2_ACTIVE_ONLY: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_MOVE_DX: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_MOVE_DY: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_MOVE_DZ: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_INJECTION_PENDING: bool = false;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_INJECTION_SUCCESS: bool = false;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_FUNC_ADDR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_CAM_OBJ: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_CHR_FOLLOWCAM_PTR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_CHR_BASE: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_CHR_L1: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_CHR_L2: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_CHR_FOLLOWCAM_OBJ: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_CALLED_FUNC: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_ENTRY: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_INPUT_RAW_8: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_INPUT_RAW_C: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_MANUAL_TICKS: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_MANUAL_WRITES: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_CAMERA_FOLLOW_DISABLED: bool = false;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_MANUAL_AXES_X: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_MANUAL_AXES_Y: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_MANUAL_AXES_Z: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_LAST_WRITE_ADDR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_FALLBACK_BUF_BASE: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_XINPUT_PAD: i32 = -1;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_XINPUT_BUTTONS: u16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_XINPUT_LX: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_XINPUT_LY: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_XINPUT_RX: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_XINPUT_RY: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_XINPUT_LT: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_XINPUT_RT: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_INJECT_MATCHES_1: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_INJECT_MATCHES_2: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_PAIR_DELTA: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_PAIR_DELTA_ERR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_PAIR_CALL_REF: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK2_SKIP_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK2_ORIG_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK2_ENTRY_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF_18: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF_30: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF_254: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF30_TA_X: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF30_TA_Y: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF30_TA_Z: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF30_TB_X: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF30_TB_Y: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_BUF30_TB_Z: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_APPLY_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_LAST_DST: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_LAST_SRC: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_MOVE_DX: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_MOVE_DY: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_MOVE_DZ: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_POST_40: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_POST_44: f32 = 0.0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_DEBUG_HOOK_POST_48: f32 = 0.0;

// Freecam: Storage buffer for transform data (256 bytes, naturally aligned)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
static mut FREECAM_STORAGE_BUFFER: [u32; 64] = [0; 64];  // u32 array for 4-byte alignment

// Freecam hook 1: Main camera logic (matches CT hook at DARKSOULS.exe+B00AD4)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
#[unsafe(naked)]
unsafe extern "C" fn freecam_hook_1() {
    std::arch::naked_asm!(
        // Increment hook entry counter
        "inc dword ptr [{hook_entry}]",
        
        // Original instruction 1: mov [esp+0x24], eax
        "mov dword ptr [esp+0x24], eax",
        
        // Save registers (match CT exactly)
        "push ebp",
        "push esi",
        "push edi",
        "push ecx",
        "push edx",
        "sub esp, 0xC",
        
        // Set in-hook flag
        "mov byte ptr [{in_hook}], 1",
        // Release variant note: game-native button bits differ from CT mapping on this build
        // (observed L3+X false positives). Keep mode cycling in Rust XInput path only.
        "mov byte ptr [{l3r3_held}], 0",
        "mov dword ptr [{debug_input_8}], 0",
        "mov dword ptr [{debug_input_c}], 0",
        "afterModeCycle:",
        "mov ecx, dword ptr [{mode}]",
        
        // Get camera manager (read pointer then dereference)
        "mov esi, dword ptr [{cam_mgr_ptr}]",
        "test esi, esi",
        "je freeCamFinish",
        "mov esi, dword ptr [esi]",
        "test esi, esi",
        "je freeCamFinish",
        // Get camera object
        "mov eax, dword ptr [esi+0x6F8]",
        "test eax, eax",
        "je freeCamFinish",
        // Save camera object address for debug
        "push eax",
        "mov dword ptr [{debug_cam_obj}], eax",
        "pop eax",
        // Capture camera transform buffer pointers for diagnostics
        "mov ecx, dword ptr [eax+0x18]",
        "mov dword ptr [{debug_buf_18}], ecx",
        "mov ecx, dword ptr [eax+0x30]",
        "mov dword ptr [{debug_buf_30}], ecx",
        "mov ecx, dword ptr [eax+0x254]",
        "mov dword ptr [{debug_buf_254}], ecx",
        // Write mode
        "mov ecx, dword ptr [{mode}]",
        "cmp ecx, 0",
        "je modeWrite",
        "cmp byte ptr [{force_mode2_active_only}], 1",
        "jne modeWrite",
        "mov ecx, 2",
        "modeWrite:",
        "mov dword ptr [eax+0x24C], ecx",
        // Jump based on mode
        "cmp ecx, 1",
        "je freeCamMode1",
        "cmp ecx, 2",
        "je freeCamMode2",
        "cmp ecx, 3",
        "je freeCamMode3",
        // Default: mode 0
        "jmp freeCamMode0",
        
        // Mode 0: Copy current camera to storage (initialize)
        "freeCamMode0:",
        "mov edi, dword ptr [eax+0x254]",
        "mov esi, dword ptr [eax+0x18]",

        // Release variant: +0x18 can be NULL. Fall back to +0x30 (render buffer).
        "test esi, esi",
        "jne mode0_have_src",
        "mov esi, dword ptr [eax+0x30]",
        "test esi, esi",
        "je freeCamFinish",
        "mode0_have_src:",

        // If +0x254 is unavailable, use our static storage buffer as persistent freecam state.
        "test edi, edi",
        "jne mode0_do_copy",
        "lea edi, [{storage_buf}]",
        
        "mode0_do_copy:",
        "movups xmm0, [esi+0x10]",
        "movups [edi+0x10], xmm0",
        "movups xmm0, [esi+0x20]",
        "movups [edi+0x20], xmm0",
        "movups xmm0, [esi+0x30]",
        "movups [edi+0x30], xmm0",
        "movups xmm0, [esi+0x40]",
        "movups [edi+0x40], xmm0",
        "mov byte ptr [ebx+0x58], 0",
        "mov byte ptr [ebx+0x59], 0",
        "jmp freeCamFinish",
        
        // Mode 1: Check freeze state, else jump to update motion (matches CE)
        "freeCamMode1:",
        "mov ebp, [edx+4]",
        "mov ebp, [ebp+0x28]",
        "mov ebp, [ebp+0xC]",
        "shr ebp, 3",
        "test ebp, 1",
        "je frozen",
        "jmp freeCamMode2",
        "frozen:",
        "mov byte ptr [ebx+0x58], 1",
        "mov byte ptr [ebx+0x59], 1",
        "jmp freeCamUpdateMotion",
        // Mode 2: Always jump to update motion
        "freeCamMode2:",
        "mov byte ptr [ebx+0x58], 0",
        "mov byte ptr [ebx+0x59], 0",
        "jmp freeCamUpdateMotion",
        // Shared update motion logic for mode 1 (not frozen) and mode 2
        "freeCamUpdateMotion:",
        "mov byte ptr [ebx+0x44], 1",
        "mov esi, dword ptr [eax+0x30]",
        "test esi, esi",
        "je freeCamFinish",
        "mov dword ptr [{hook_last_src}], esi",
        "mov edi, dword ptr [eax+0x254]",
        // +0x254 can be null on some PTDE variants. Keep an independent storage source.
        "test edi, edi",
        "jne update_dst_ready",
        "lea edi, [{storage_buf}]",
        "update_dst_ready:",
        "mov dword ptr [{hook_last_dst}], edi",
        "movups xmm0, [edi+0x10]",
        "movups [esi+0x10], xmm0",
        "movups xmm0, [edi+0x20]",
        "movups [esi+0x20], xmm0",
        "movups xmm0, [edi+0x30]",
        "movups [esi+0x30], xmm0",
        "movups xmm0, [edi+0x40]",
        "movups [esi+0x40], xmm0",
        "mov ebp, [{motion_func}]",
        "mov dword ptr [{debug_func_addr}], ebp",
        "cmp byte ptr [{motion_call_safe}], 1",
        "jne skipMotionCall",
        "push edx",
        "push edi",
        "inc dword ptr [{debug_called}]",
        "call ebp",
        "skipMotionCall:",

        // Enforce movement directly on live render/source buffers.
        "fld dword ptr [esi+0x40]",
        "fadd dword ptr [{move_dx}]",
        "fstp dword ptr [esi+0x40]",
        "fld dword ptr [esi+0x44]",
        "fadd dword ptr [{move_dy}]",
        "fstp dword ptr [esi+0x44]",
        "fld dword ptr [esi+0x48]",
        "fadd dword ptr [{move_dz}]",
        "fstp dword ptr [esi+0x48]",
        "fld dword ptr [esi+0x1C]",
        "fadd dword ptr [{move_dx}]",
        "fstp dword ptr [esi+0x1C]",
        "fld dword ptr [esi+0x2C]",
        "fadd dword ptr [{move_dy}]",
        "fstp dword ptr [esi+0x2C]",
        "fld dword ptr [esi+0x3C]",
        "fadd dword ptr [{move_dz}]",
        "fstp dword ptr [esi+0x3C]",
        "fld dword ptr [esi+0x30]",
        "fadd dword ptr [{move_dx}]",
        "fstp dword ptr [esi+0x30]",
        "fld dword ptr [esi+0x34]",
        "fadd dword ptr [{move_dy}]",
        "fstp dword ptr [esi+0x34]",
        "fld dword ptr [esi+0x38]",
        "fadd dword ptr [{move_dz}]",
        "fstp dword ptr [esi+0x38]",
        "cmp edi, esi",
        "je postMoveDone",
        "fld dword ptr [edi+0x40]",
        "fadd dword ptr [{move_dx}]",
        "fstp dword ptr [edi+0x40]",
        "fld dword ptr [edi+0x44]",
        "fadd dword ptr [{move_dy}]",
        "fstp dword ptr [edi+0x44]",
        "fld dword ptr [edi+0x48]",
        "fadd dword ptr [{move_dz}]",
        "fstp dword ptr [edi+0x48]",
        "fld dword ptr [edi+0x1C]",
        "fadd dword ptr [{move_dx}]",
        "fstp dword ptr [edi+0x1C]",
        "fld dword ptr [edi+0x2C]",
        "fadd dword ptr [{move_dy}]",
        "fstp dword ptr [edi+0x2C]",
        "fld dword ptr [edi+0x3C]",
        "fadd dword ptr [{move_dz}]",
        "fstp dword ptr [edi+0x3C]",
        "postMoveDone:",

        // Hook-side telemetry only. All movement writes are done in Rust-side validated code
        // to avoid crashes when pause/emote/menu states change camera pointer lifetimes.
        "inc dword ptr [{hook_apply_count}]",
        "fld dword ptr [{move_dx}]",
        "fstp dword ptr [{hook_move_dx}]",
        "fld dword ptr [{move_dy}]",
        "fstp dword ptr [{hook_move_dy}]",
        "fld dword ptr [{move_dz}]",
        "fstp dword ptr [{hook_move_dz}]",
        "fld dword ptr [esi+0x40]",
        "fstp dword ptr [{hook_post_40}]",
        "fld dword ptr [esi+0x44]",
        "fstp dword ptr [{hook_post_44}]",
        "fld dword ptr [esi+0x48]",
        "fstp dword ptr [{hook_post_48}]",
        "jmp freeCamFinish",
        
        // Mode 3: Copy stored back to current
        "freeCamMode3:",
        "mov esi, dword ptr [eax+0x30]",
        "mov edi, dword ptr [eax+0x254]",

        // Release variant: +0x254 can be NULL. Fall back to static storage if needed.
        "test esi, esi",
        "je freeCamFinish",
        "test edi, edi",
        "jne mode3_do_copy",
        "lea edi, [{storage_buf}]",
        "mode3_do_copy:",
        "movups xmm0, [edi+0x10]",
        "movups [esi+0x10], xmm0",
        "movups xmm0, [edi+0x20]",
        "movups [esi+0x20], xmm0",
        "movups xmm0, [edi+0x30]",
        "movups [esi+0x30], xmm0",
        "movups xmm0, [edi+0x40]",
        "movups [esi+0x40], xmm0",
        "mov byte ptr [ebx+0x58], 0",
        "mov byte ptr [ebx+0x59], 0",
        "jmp freeCamFinish",
        
        "freeCamFinish:",
        "add esp, 0xC",
        "pop edx",
        "pop ecx",
        "pop edi",
        "pop esi",
        "pop ebp",
        "mov byte ptr [{in_hook}], 0",
        
        // Original instruction 2
        "mov eax, dword ptr [ebx+0x44]",
        
        // Return
        "mov ecx, [{inject1}]",
        "add ecx, 7",
        "jmp ecx",
        
        hook_entry = sym FREECAM_DEBUG_HOOK_ENTRY,
        inject1 = sym FREECAM_INJECT_ADDR_1,
        motion_call_safe = sym FREECAM_MOTION_CALL_SAFE,
        motion_func = sym FREECAM_MOTION_FUNC,
        move_dx = sym FREECAM_MOVE_DX,
        move_dy = sym FREECAM_MOVE_DY,
        move_dz = sym FREECAM_MOVE_DZ,
        force_mode2_active_only = sym FREECAM_FORCE_MODE2_ACTIVE_ONLY,
        in_hook = sym FREECAM_IN_HOOK,
        l3r3_held = sym FREECAM_L3R3_HELD,
        mode = sym FREECAM_MODE,
        cam_mgr_ptr = sym FREECAM_CAM_MGR_PTR,
        debug_cam_obj = sym FREECAM_DEBUG_CAM_OBJ,
        debug_buf_18 = sym FREECAM_DEBUG_BUF_18,
        debug_buf_30 = sym FREECAM_DEBUG_BUF_30,
        debug_buf_254 = sym FREECAM_DEBUG_BUF_254,
        hook_apply_count = sym FREECAM_DEBUG_HOOK_APPLY_COUNT,
        hook_last_dst = sym FREECAM_DEBUG_HOOK_LAST_DST,
        hook_last_src = sym FREECAM_DEBUG_HOOK_LAST_SRC,
        hook_move_dx = sym FREECAM_DEBUG_HOOK_MOVE_DX,
        hook_move_dy = sym FREECAM_DEBUG_HOOK_MOVE_DY,
        hook_move_dz = sym FREECAM_DEBUG_HOOK_MOVE_DZ,
        hook_post_40 = sym FREECAM_DEBUG_HOOK_POST_40,
        hook_post_44 = sym FREECAM_DEBUG_HOOK_POST_44,
        hook_post_48 = sym FREECAM_DEBUG_HOOK_POST_48,
        debug_func_addr = sym FREECAM_DEBUG_FUNC_ADDR,
        debug_called = sym FREECAM_DEBUG_CALLED_FUNC,
        debug_input_8 = sym FREECAM_DEBUG_INPUT_RAW_8,
        debug_input_c = sym FREECAM_DEBUG_INPUT_RAW_C,
        storage_buf = sym FREECAM_STORAGE_BUFFER,
    );
}

// Freecam hook 2: Skip camera constraint check when in freecam (matches CT hook at DARKSOULS.exe+BFB431)
#[cfg(all(target_os = "windows", target_arch = "x86"))]
#[unsafe(naked)]
unsafe extern "C" fn freecam_hook_2() {
    std::arch::naked_asm!(
        "inc dword ptr [{hook2_entry}]",
        "cmp byte ptr [{in_hook}], 1",
        "je skip_original",
        "do_original:",
        "inc dword ptr [{hook2_orig}]",
        // Original instructions
        "shr edx, 0x14",
        "test dl, 1",
        "jmp ret_hook2",
        "skip_original:",
        "inc dword ptr [{hook2_skip}]",
        // Jump to the original conditional-branch target when bypassing this check.
        "mov ecx, [{hook2_skip_addr}]",
        "test ecx, ecx",
        "jne jump_skip_target",
        "ret_hook2:",
        // Return to original code (addr + 6)
        "mov ecx, [{inject2}]",
        "add ecx, 6",
        "jmp ecx",
        "jump_skip_target:",
        "jmp ecx",
        in_hook = sym FREECAM_IN_HOOK,
        inject2 = sym FREECAM_INJECT_ADDR_2,
        hook2_skip_addr = sym FREECAM_HOOK2_SKIP_ADDR,
        hook2_entry = sym FREECAM_DEBUG_HOOK2_ENTRY_COUNT,
        hook2_skip = sym FREECAM_DEBUG_HOOK2_SKIP_COUNT,
        hook2_orig = sym FREECAM_DEBUG_HOOK2_ORIG_COUNT,
    );
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn is_readable_addr(addr: usize, len: usize) -> bool {
    if addr == 0 || len == 0 {
        return false;
    }

    // Query page metadata before dereferencing raw pointers in hook-debug paths.
    let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let result = unsafe {
        VirtualQuery(
            addr as *const c_void,
            &mut mbi,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    };

    if result == 0 || mbi.State != MEM_COMMIT {
        return false;
    }

    if (mbi.Protect & PAGE_NOACCESS) != 0 || (mbi.Protect & PAGE_GUARD) != 0 {
        return false;
    }

    let base = mbi.BaseAddress as usize;
    let end = match base.checked_add(mbi.RegionSize) {
        Some(v) => v,
        None => return false,
    };
    let req_end = match addr.checked_add(len) {
        Some(v) => v,
        None => return false,
    };

    req_end <= end
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn is_executable_addr(addr: usize, len: usize) -> bool {
    if addr == 0 || len == 0 {
        return false;
    }

    let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let result = unsafe {
        VirtualQuery(
            addr as *const c_void,
            &mut mbi,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    };

    if result == 0 || mbi.State != MEM_COMMIT {
        return false;
    }

    if (mbi.Protect & PAGE_NOACCESS) != 0 || (mbi.Protect & PAGE_GUARD) != 0 {
        return false;
    }

    let exec = (mbi.Protect & PAGE_EXECUTE) != 0
        || (mbi.Protect & PAGE_EXECUTE_READ) != 0
        || (mbi.Protect & PAGE_EXECUTE_READWRITE) != 0
        || (mbi.Protect & PAGE_EXECUTE_WRITECOPY) != 0;
    if !exec {
        return false;
    }

    let base = mbi.BaseAddress as usize;
    let end = match base.checked_add(mbi.RegionSize) {
        Some(v) => v,
        None => return false,
    };
    let req_end = match addr.checked_add(len) {
        Some(v) => v,
        None => return false,
    };

    req_end <= end
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn is_writable_addr(addr: usize, len: usize) -> bool {
    if addr == 0 || len == 0 {
        return false;
    }

    let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let result = unsafe {
        VirtualQuery(
            addr as *const c_void,
            &mut mbi,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    };

    if result == 0 || mbi.State != MEM_COMMIT {
        return false;
    }

    if (mbi.Protect & PAGE_NOACCESS) != 0 || (mbi.Protect & PAGE_GUARD) != 0 {
        return false;
    }

    let writable = (mbi.Protect & PAGE_READWRITE) != 0
        || (mbi.Protect & PAGE_WRITECOPY) != 0
        || (mbi.Protect & PAGE_EXECUTE_READWRITE) != 0
        || (mbi.Protect & PAGE_EXECUTE_WRITECOPY) != 0;
    if !writable {
        return false;
    }

    let base = mbi.BaseAddress as usize;
    let end = match base.checked_add(mbi.RegionSize) {
        Some(v) => v,
        None => return false,
    };
    let req_end = match addr.checked_add(len) {
        Some(v) => v,
        None => return false,
    };

    req_end <= end
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn normalize_stick_axis(raw: i16, deadzone: i16) -> f32 {
    let v = raw as f32;
    let d = deadzone as f32;
    if v.abs() <= d {
        0.0
    } else {
        let sign = if v.is_sign_negative() { -1.0 } else { 1.0 };
        let mag = (v.abs() - d) / (32767.0 - d);
        (mag * sign).clamp(-1.0, 1.0)
    }
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn apply_translation_deltas(buf: usize, dx: f32, dy: f32, dz: f32) -> bool {
    if buf == 0 {
        return false;
    }
    if !is_readable_addr(buf, 0x4C) || !is_writable_addr(buf, 0x4C) {
        return false;
    }

    unsafe {
        let clamp = |v: f32| v.clamp(-10000.0, 10000.0);

        // Layout A
        let a_x = clamp(std::ptr::read_volatile((buf + 0x1C) as *const f32) + dx);
        let a_y = clamp(std::ptr::read_volatile((buf + 0x2C) as *const f32) + dy);
        let a_z = clamp(std::ptr::read_volatile((buf + 0x3C) as *const f32) + dz);
        std::ptr::write_volatile((buf + 0x1C) as *mut f32, a_x);
        std::ptr::write_volatile((buf + 0x2C) as *mut f32, a_y);
        std::ptr::write_volatile((buf + 0x3C) as *mut f32, a_z);

        // Layout C
        let c_x = clamp(std::ptr::read_volatile((buf + 0x30) as *const f32) + dx);
        let c_y = clamp(std::ptr::read_volatile((buf + 0x34) as *const f32) + dy);
        let c_z = clamp(std::ptr::read_volatile((buf + 0x38) as *const f32) + dz);
        std::ptr::write_volatile((buf + 0x30) as *mut f32, c_x);
        std::ptr::write_volatile((buf + 0x34) as *mut f32, c_y);
        std::ptr::write_volatile((buf + 0x38) as *mut f32, c_z);

        // Layout B
        let b_x = clamp(std::ptr::read_volatile((buf + 0x40) as *const f32) + dx);
        let b_y = clamp(std::ptr::read_volatile((buf + 0x44) as *const f32) + dy);
        let b_z = clamp(std::ptr::read_volatile((buf + 0x48) as *const f32) + dz);
        std::ptr::write_volatile((buf + 0x40) as *mut f32, b_x);
        std::ptr::write_volatile((buf + 0x44) as *mut f32, b_y);
        std::ptr::write_volatile((buf + 0x48) as *mut f32, b_z);

        // Layout D (legacy column-major translation slots)
        let d_x = clamp(std::ptr::read_volatile((buf + 0x0C) as *const f32) + dx);
        let d_y = clamp(std::ptr::read_volatile((buf + 0x1C) as *const f32) + dy);
        let d_z = clamp(std::ptr::read_volatile((buf + 0x2C) as *const f32) + dz);
        std::ptr::write_volatile((buf + 0x0C) as *mut f32, d_x);
        std::ptr::write_volatile((buf + 0x1C) as *mut f32, d_y);
        std::ptr::write_volatile((buf + 0x2C) as *mut f32, d_z);
    }

    true
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn key_down(vk: i32) -> bool {
    unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn has_nearby_rel32_call_to_target(anchor: usize, target: usize, range_before: usize, range_after: usize) -> bool {
    let start = anchor.saturating_sub(range_before);
    let end = anchor.saturating_add(range_after);

    for call_site in start..end {
        if !is_readable_addr(call_site, 5) {
            continue;
        }
        let op = unsafe { std::ptr::read_volatile(call_site as *const u8) };
        if op != 0xE8 {
            continue;
        }

        let rel = unsafe { std::ptr::read_unaligned((call_site + 1) as *const i32) } as isize;
        let resolved = ((call_site + 5) as isize).wrapping_add(rel) as usize;
        if resolved == target {
            return true;
        }
    }

    false
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn find_best_nearby_rel32_call_target(anchor: usize, expected: usize, range_before: usize, range_after: usize) -> Option<usize> {
    let start = anchor.saturating_sub(range_before);
    let end = anchor.saturating_add(range_after);

    let mut best: Option<usize> = None;
    let mut best_score: usize = usize::MAX;

    for call_site in start..end {
        if !is_readable_addr(call_site, 5) {
            continue;
        }
        let op = unsafe { std::ptr::read_volatile(call_site as *const u8) };
        if op != 0xE8 {
            continue;
        }

        let rel = unsafe { std::ptr::read_unaligned((call_site + 1) as *const i32) } as isize;
        let target = ((call_site + 5) as isize).wrapping_add(rel) as usize;

        if !is_executable_addr(target, 16) {
            continue;
        }

        let mut score = target.abs_diff(expected);
        if target > anchor {
            score = score.saturating_add(0x2000);
        }

        if score < best_score {
            best_score = score;
            best = Some(target);
        }
    }

    best
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn resolve_hook2_skip_target(inject2: usize) -> usize {
    let start = inject2.saturating_add(6);
    let end = start.saturating_add(0x40);
    let mut addr = start;

    while addr < end {
        if !is_readable_addr(addr, 2) {
            addr = addr.saturating_add(1);
            continue;
        }

        let op = unsafe { std::ptr::read_volatile(addr as *const u8) };

        // short Jcc (0x70..0x7F)
        if (0x70..=0x7F).contains(&op) {
            let rel = unsafe { std::ptr::read_volatile((addr + 1) as *const i8) } as isize;
            let target = ((addr + 2) as isize).wrapping_add(rel) as usize;
            if is_executable_addr(target, 4) {
                return target;
            }
        }

        // near Jcc (0F 80..8F)
        if op == 0x0F && is_readable_addr(addr, 6) {
            let op2 = unsafe { std::ptr::read_volatile((addr + 1) as *const u8) };
            if (0x80..=0x8F).contains(&op2) {
                let rel = unsafe { std::ptr::read_unaligned((addr + 2) as *const i32) } as isize;
                let target = ((addr + 6) as isize).wrapping_add(rel) as usize;
                if is_executable_addr(target, 4) {
                    return target;
                }
            }
        }

        addr = addr.saturating_add(1);
    }

    0
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn find_executable_pattern_matches(pattern: &[u8], start: usize, max_addr: usize) -> Vec<usize> {
    let mut matches = Vec::new();
    if pattern.is_empty() {
        return matches;
    }

    let mut addr = start;
    unsafe {
        while addr < max_addr {
            let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
            let result = VirtualQuery(
                addr as *const c_void,
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );
            if result == 0 {
                break;
            }

            let protect = mbi.Protect;
            let is_exec = (protect & PAGE_EXECUTE) != 0
                || (protect & PAGE_EXECUTE_READ) != 0
                || (protect & PAGE_EXECUTE_READWRITE) != 0
                || (protect & PAGE_EXECUTE_WRITECOPY) != 0;

            if mbi.State == MEM_COMMIT && is_exec && mbi.RegionSize >= pattern.len() {
                let base = mbi.BaseAddress as usize;
                let size = mbi.RegionSize;
                let end = size.saturating_sub(pattern.len());
                for off in 0..=end {
                    let check = base + off;
                    let slice = std::slice::from_raw_parts(check as *const u8, pattern.len());
                    if slice == pattern {
                        matches.push(check);
                    }
                }
            }

            addr = (mbi.BaseAddress as usize).saturating_add(mbi.RegionSize.max(1));
            if addr <= mbi.BaseAddress as usize {
                break;
            }
        }
    }

    matches
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn find_freecam_inject_pair() -> (Option<(usize, usize, usize, usize, u8)>, usize, usize) {
    let expected_delta = offsets::FREECAM_INJECT_2_DEBUG_RVA - offsets::FREECAM_INJECT_1_DEBUG_RVA;
    let p1 = find_executable_pattern_matches(&FREECAM_ORIG_PATCH_1, 0x0040_0000, 0x2000_0000);
    let p2 = find_executable_pattern_matches(&FREECAM_ORIG_PATCH_2, 0x0040_0000, 0x2000_0000);

    let p1_count = p1.len();
    let p2_count = p2.len();
    let mut best: Option<(usize, usize, usize, usize, u8, usize)> = None;

    for a2 in &p2 {
        let candidate = a2.saturating_sub(0x5D1);
        let candidate_exec = is_executable_addr(candidate, 16);
        let candidate_call_ref = has_nearby_rel32_call_to_target(*a2, candidate, 0x2000, 0x200);
        let rva = a2.saturating_sub(offsets::FREECAM_IMAGE_BASE);
        let rva_err = rva
            .abs_diff(offsets::FREECAM_INJECT_2_DEBUG_RVA)
            .min(rva.abs_diff(offsets::FREECAM_INJECT_2_RELEASE_RVA));

        for a1 in &p1 {
            if *a2 <= *a1 {
                continue;
            }

            let delta = *a2 - *a1;
            let delta_err = delta.abs_diff(expected_delta);

            let mut score = delta_err;
            if !candidate_exec {
                score = score.saturating_add(0x20_000);
            }
            if !candidate_call_ref {
                score = score.saturating_add(0x10_000);
            }
            score = score.saturating_add(rva_err / 4);

            match best {
                Some((_, _, _, _, _, best_score)) if score >= best_score => {}
                _ => {
                    best = Some((*a1, *a2, delta, delta_err, if candidate_call_ref { 1 } else { 0 }, score));
                }
            }
        }
    }

    (
        best.map(|(a1, a2, delta, delta_err, call_ref, _)| (a1, a2, delta, delta_err, call_ref)),
        p1_count,
        p2_count,
    )
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn jmp_patch(from: usize, to: usize, total_len: usize) -> Option<Vec<u8>> {
    if total_len < 5 {
        return None;
    }

    let rel = (to as isize).wrapping_sub((from + 5) as isize);
    if rel < i32::MIN as isize || rel > i32::MAX as isize {
        return None;
    }

    let mut patch = vec![0x90u8; total_len];
    patch[0] = 0xE9;
    patch[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
    Some(patch)
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
fn write_exec_patch(addr: usize, patch: &[u8]) -> bool {
    if addr == 0 || patch.is_empty() {
        return false;
    }
    if !is_executable_addr(addr, patch.len()) {
        return false;
    }

    unsafe {
        let mut old_protect: u32 = 0;
        if VirtualProtect(
            addr as *const c_void,
            patch.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) == 0
        {
            return false;
        }

        std::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, patch.len());

        let mut restored_protect: u32 = 0;
        let _ = VirtualProtect(
            addr as *const c_void,
            patch.len(),
            old_protect,
            &mut restored_protect,
        );
    }

    true
}

#[allow(dead_code)]
pub struct Ds1 {
    pub process: Process,
    pub chr_dbg: Pointer,
    pub pos_lock: Pointer,                      // 0x16 0x27
    pub node_graph: Pointer,                    // 0x12
    pub all_no_magic_quantity_consume: Pointer, // 0x2
    pub player_no_dead: Pointer,                // 0x22
    pub player_exterminate: Pointer,            // 0x10
    pub all_no_stamina_consume: Pointer,        // 0x18
    pub compass: Pointer,                       // 0xC 0x15, 0x1E
    pub chr_data_1: Pointer,                    // 0x2, 0x0, 0x4, 0x0
    pub char_map_data: Pointer,                 // chr_data_1 (aob), 0x2, 0x0, 0x4, 0x0 0x2
    pub anim_data: Pointer,
    pub chr_data_2: Pointer,
    pub game_data_mgr: Pointer,
    pub lock_on_mgr: Pointer,
    pub char_pos_data: Pointer, // 0x1, 0x0, 0x8
    pub no_stam_consume: bool,
    pub level_up: Pointer,
    pub bonfire_warp: Pointer,
    pub bonfire_warp_2: Pointer,
    pub world_state: Pointer,
    pub chr_flags_1: Pointer,
    pub input_state: Pointer,
    pub quitout: Pointer,
    pub no_death_pointer: Pointer,
    pub item_get_pointer: Pointer,
    pub item_drop_pointer: Pointer,
    pub item_drop_unknown_1_pointer: Pointer,
    pub item_drop_unknown_2_pointer: Pointer,
    pub target_bank: Pointer,
    pub free_cam_enabled: bool,
}

impl Ds1 {
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn set_freecam_force_mode2_active_only(&mut self, enabled: bool) {
        unsafe {
            FREECAM_FORCE_MODE2_ACTIVE_ONLY = if enabled { 1 } else { 0 };
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn set_freecam_force_mode2_active_only(&mut self, _enabled: bool) {}

    pub fn new() -> Self {
        let mut ds1struct = Ds1 {
            process: Process::new("DARKSOULS.exe"),
            chr_dbg: Pointer::default(),
            pos_lock: Pointer::default(),
            node_graph: Pointer::default(),
            all_no_magic_quantity_consume: Pointer::default(),
            player_no_dead: Pointer::default(),
            player_exterminate: Pointer::default(),
            all_no_stamina_consume: Pointer::default(),
            compass: Pointer::default(),
            chr_data_1: Pointer::default(),
            char_map_data: Pointer::default(),
            anim_data: Pointer::default(),
            chr_data_2: Pointer::default(),
            game_data_mgr: Pointer::default(),
            lock_on_mgr: Pointer::default(),
            char_pos_data: Pointer::default(),
            level_up: Pointer::default(),
            no_stam_consume: false,
            bonfire_warp: Pointer::default(),
            bonfire_warp_2: Pointer::default(),
            world_state: Pointer::default(),
            chr_flags_1: Pointer::default(),
            input_state: Pointer::default(),
            quitout: Pointer::default(),
            no_death_pointer: Pointer::default(),
            item_get_pointer: Pointer::default(),
            item_drop_pointer: Pointer::default(),
            item_drop_unknown_1_pointer: Pointer::default(),
            item_drop_unknown_2_pointer: Pointer::default(),
            target_bank: Pointer::default(),
            free_cam_enabled: false,
        };
        let _ = ds1struct.refresh();
        ds1struct
    }
    // Pointers are declared here
    pub fn refresh(&mut self) -> Result<(), String> {
        if !self.process.is_attached() {
            self.process.refresh()?;

            self.chr_dbg = self.process.scan_abs(
                "all_no_stamina_consume",
                &offsets::ALL_NO_STAMINA_CONSUME_AOB,
                offsets::ALL_NO_STAMINA_CONSUME_AOB_OFFSET,
                vec![0x0],
            )?;

            self.chr_data_1 = self.process.scan_abs(
                "chr_data_1",
                &offsets::CHAR_DATA_1_AOB,
                offsets::CHAR_DATA_1_AOB_OFFSET,
                vec![
                    0x0,
                    offsets::CHAR_DATA_1_OFFSET1,
                    offsets::CHAR_DATA_1_OFFSET2,
                    offsets::CHAR_DATA_1_OFFSET3,
                ],
            )?;

            self.target_bank = self.process.scan_abs(
                "target_bank",
                &offsets::TARGET_BANK_AOB,
                offsets::TARGET_BANK_AOB_OFFSET,
                vec![0x0, 0x0],
            )?;

            self.char_map_data = self.chr_data_1.clone();
            self.char_map_data
                .offsets
                .push(CharData1::CHAR_MAP_DATA_PTR);

            self.anim_data = self.char_map_data.clone();
            self.anim_data.offsets.push(CharMapData::ANIM_DATA_PTR);

            self.char_pos_data = self.char_map_data.clone();
            self.char_pos_data
                .offsets
                .push(CharMapData::CHAR_POS_DATA_PTR);

            self.chr_data_2 = self.process.scan_abs(
                "chr_data_2",
                &offsets::CHAR_DATA_2_AOB,
                offsets::CHAR_DATA_2_AOB_OFFSET,
                vec![
                    0x0,
                    offsets::CHAR_DATA_2_OFFSET1,
                    offsets::CHAR_DATA_2_OFFSET2,
                ],
            )?;

            self.game_data_mgr = self.process.scan_abs(
                "game_data_mgr",
                &offsets::CHAR_DATA_2_AOB,
                offsets::CHAR_DATA_2_AOB_OFFSET,
                vec![0x0],
            )?;

            self.lock_on_mgr = self.process.scan_abs(
                "lock_on_mgr",
                &offsets::LOCK_ON_MGR_AOB,
                offsets::LOCK_ON_MGR_AOB_OFFSET,
                vec![0x0],
            )
            .unwrap_or_default();

            self.level_up =
                self.process
                    .scan_abs("level_up", &offsets::LEVEL_UP, 0x0, vec![0x0])?;

            self.bonfire_warp = self
                .process
                .scan_abs("bonfire_warp", &offsets::BONFIRE_WARP, 0x0, vec![0x0])
                .unwrap();
            self.bonfire_warp_2 = self.process.scan_abs(
                "bonfire_warp_2",
                &offsets::BONFIRE_WARP_2,
                BONFIRE_WARP_2_OFFSET1,
                vec![0x0],
            )?;

            self.item_drop_pointer = self.process.scan_abs(
                "item_drop_pointer",
                &offsets::ITEM_DROP_AOB,
                0x0,
                vec![0x0],
            )?;

            self.item_drop_unknown_1_pointer = self.process.scan_abs(
                "item_drop_unknown_1_pointer",
                &offsets::ITEM_DROP_UNKNOWN_1_AOB,
                offsets::ITEM_DROP_UNKNOWN_1_AOB_OFFSET,
                vec![0x0],
            )?;

            self.item_drop_unknown_2_pointer = self.process.scan_abs(
                "item_drop_unknown_2_pointer",
                &offsets::ITEM_DROP_UNKNOWN_2_AOB,
                offsets::ITEM_DROP_UNKNOWN_2_AOB_OFFSET,
                vec![0x0],
            )?;

            self.world_state = self.process.scan_abs(
                "world_state",
                &offsets::WORLD_STATE_AOB,
                offsets::WORLD_STATE_AOB_OFFSET,
                vec![0x0, offsets::WORLD_STATE_OFFSET1],
            )?;

            self.chr_flags_1 = self.process.scan_abs(
                "chr_flags_1",
                &offsets::FLAGS_AOB_1,
                offsets::FLAGS_AOB_1_OFFSET,
                vec![0x0],
            )?;

            self.input_state = self.process.scan_abs(
                "input_state",
                &offsets::INPUT_STATE_AOB,
                offsets::INPUT_STATE_AOB_OFFSET,
                vec![
                    0x0,
                    offsets::INPUT_STATE_OFFSET1,
                    offsets::INPUT_STATE_OFFSET2,
                    offsets::INPUT_STATE_OFFSET3,
                    offsets::INPUT_STATE_OFFSET4,
                ],
            )?;

            self.quitout = self.process.scan_abs(
                "quitout",
                &offsets::QUITOUT_AOB,
                offsets::QUITOUT_AOB_OFFSET,
                vec![0x0, offsets::QUITOUT_OFFSET1],
            )?;

            self.no_death_pointer = self.process.scan_abs(
                "no_death_pointer",
                &offsets::PLAYER_NO_DEAD_AOB,
                offsets::PLAYER_NO_DEAD_AOB_OFFSET,
                vec![0x0],
            )?;

            self.player_exterminate = self.process.scan_abs(
                "player_exterminate",
                &offsets::PLAYER_EXTERMINATE_AOB,
                offsets::PLAYER_EXTERMINATE_AOB_OFFSET,
                vec![0x0],
            )?;

            self.item_get_pointer = self.process.scan_abs(
                "item_get_pointer",
                &offsets::ITEM_GET_AOB,
                0x0,
                vec![0x0],
            )?;

            self.all_no_magic_quantity_consume = self.process.scan_abs(
                "all_no_magic_quantity_consume",
                &offsets::ALL_NO_MAGIC_QTY_CONSUME_AOB,
                offsets::ALL_NO_MAGIC_QTY_CONSUME_AOB_OFFSET,
                vec![0x0],
            )?;

            // Scan for freecam injection addresses and camera manager pointer
            #[cfg(all(target_os = "windows", target_arch = "x86"))]
            unsafe {
                eprintln!("Scanning for freecam patterns...");
                eprintln!("  Pattern 1: {}", offsets::FREECAM_INJECT_1_AOB);
                eprintln!("  Pattern 2: {}", offsets::FREECAM_INJECT_2_AOB);
                eprintln!("  Pattern 3: {}", offsets::FREECAM_CAM_MGR_AOB);

                FREECAM_DEBUG_INJECT_MATCHES_1 = 0;
                FREECAM_DEBUG_INJECT_MATCHES_2 = 0;
                FREECAM_DEBUG_PAIR_DELTA = 0;
                FREECAM_DEBUG_PAIR_DELTA_ERR = 0;
                FREECAM_DEBUG_PAIR_CALL_REF = 0;

                let (pair, p1_count, p2_count) = find_freecam_inject_pair();
                FREECAM_DEBUG_INJECT_MATCHES_1 = p1_count as u32;
                FREECAM_DEBUG_INJECT_MATCHES_2 = p2_count as u32;
                eprintln!(
                    "  Freecam pattern match counts: p1={} p2={}",
                    p1_count,
                    p2_count
                );

                if let Some((pair_1, pair_2, pair_delta, pair_delta_err, pair_call_ref)) = pair {
                    FREECAM_INJECT_ADDR_1 = pair_1;
                    
                    // Pattern found Hook2, but also try calculated release address
                    let hook2_pattern_match = pair_2;
                    let hook2_release_calc = offsets::FREECAM_IMAGE_BASE + offsets::FREECAM_INJECT_2_RELEASE_RVA;
                    let hook2_debug_calc = offsets::FREECAM_IMAGE_BASE + offsets::FREECAM_INJECT_2_DEBUG_RVA;
                    
                    // Verify pattern bytes at calculated addresses
                    let release_bytes_match = is_readable_addr(hook2_release_calc, FREECAM_ORIG_PATCH_2.len())
                        && {
                            let mut buf = vec![0u8; FREECAM_ORIG_PATCH_2.len()];
                            std::ptr::copy_nonoverlapping(
                                hook2_release_calc as *const u8,
                                buf.as_mut_ptr(),
                                FREECAM_ORIG_PATCH_2.len()
                            );
                            eprintln!("  Bytes at release calc 0x{:08X}: {:02X?} (expected: {:02X?})", 
                                hook2_release_calc, buf, FREECAM_ORIG_PATCH_2);
                            buf == FREECAM_ORIG_PATCH_2
                        };
                    
                    let debug_bytes_match = is_readable_addr(hook2_debug_calc, FREECAM_ORIG_PATCH_2.len())
                        && {
                            let mut buf = vec![0u8; FREECAM_ORIG_PATCH_2.len()];
                            std::ptr::copy_nonoverlapping(
                                hook2_debug_calc as *const u8,
                                buf.as_mut_ptr(),
                                FREECAM_ORIG_PATCH_2.len()
                            );
                            eprintln!("  Bytes at debug calc 0x{:08X}: {:02X?} (expected: {:02X?})", 
                                hook2_debug_calc, buf, FREECAM_ORIG_PATCH_2);
                            buf == FREECAM_ORIG_PATCH_2
                        };
                    
                    // Also check pattern-matched address
                    let pattern_bytes_match = is_readable_addr(hook2_pattern_match, FREECAM_ORIG_PATCH_2.len())
                        && {
                            let mut buf = vec![0u8; FREECAM_ORIG_PATCH_2.len()];
                            std::ptr::copy_nonoverlapping(
                                hook2_pattern_match as *const u8,
                                buf.as_mut_ptr(),
                                FREECAM_ORIG_PATCH_2.len()
                            );
                            eprintln!("  Bytes at pattern match 0x{:08X}: {:02X?} (expected: {:02X?})", 
                                hook2_pattern_match, buf, FREECAM_ORIG_PATCH_2);
                            buf == FREECAM_ORIG_PATCH_2
                        };
                    
                    // Use calculated addresses ONLY if bytes match, otherwise use pattern match
                    FREECAM_INJECT_ADDR_2 = if release_bytes_match {
                        eprintln!("  ✓ Using calculated RELEASE Hook2 address: 0x{:08X}", hook2_release_calc);
                        hook2_release_calc
                    } else if debug_bytes_match {
                        eprintln!("  ✓ Using calculated DEBUG Hook2 address: 0x{:08X}", hook2_debug_calc);
                        hook2_debug_calc
                    } else {
                        eprintln!("  ! Using pattern-matched Hook2 address: 0x{:08X} (calc addresses had wrong bytes)", 
                            hook2_pattern_match);
                        hook2_pattern_match
                    };
                    
                    FREECAM_DEBUG_PAIR_DELTA = FREECAM_INJECT_ADDR_2 - pair_1;
                    FREECAM_DEBUG_PAIR_DELTA_ERR = FREECAM_DEBUG_PAIR_DELTA
                        .abs_diff(offsets::FREECAM_INJECT_2_DEBUG_RVA - offsets::FREECAM_INJECT_1_DEBUG_RVA);
                    FREECAM_DEBUG_PAIR_CALL_REF = pair_call_ref;
                    eprintln!(
                        "  ✓ Final inject pair: 0x{:08X} / 0x{:08X} delta=0x{:X} err=0x{:X} call_ref={}",
                        FREECAM_INJECT_ADDR_1,
                        FREECAM_INJECT_ADDR_2,
                        FREECAM_DEBUG_PAIR_DELTA,
                        FREECAM_DEBUG_PAIR_DELTA_ERR,
                        pair_call_ref
                    );
                } else {
                    // Scan for injection point 1
                    match self.process.scan_abs(
                        "freecam_inject_1",
                        &offsets::FREECAM_INJECT_1_AOB,
                        0,
                        vec![],
                    ) {
                        Ok(ptr1) => {
                            FREECAM_INJECT_ADDR_1 = ptr1.base_address;
                            eprintln!("  ✓ Inject point 1 found: 0x{:08X}", ptr1.base_address);
                        }
                        Err(e) => {
                            eprintln!("  ✗ Inject point 1 NOT found: {}", e);
                            FREECAM_INJECT_ADDR_1 = 0;
                        }
                    }

                    // Scan for injection point 2
                    match self.process.scan_abs(
                        "freecam_inject_2",
                        &offsets::FREECAM_INJECT_2_AOB,
                        0,
                        vec![],
                    ) {
                        Ok(ptr2) => {
                            FREECAM_INJECT_ADDR_2 = ptr2.base_address;
                            eprintln!("  ✓ Inject point 2 found: 0x{:08X}", ptr2.base_address);
                        }
                        Err(e) => {
                            eprintln!("  ✗ Inject point 2 NOT found: {}", e);
                            FREECAM_INJECT_ADDR_2 = 0;
                        }
                    }

                    if FREECAM_INJECT_ADDR_1 != 0 && FREECAM_INJECT_ADDR_2 > FREECAM_INJECT_ADDR_1 {
                        FREECAM_DEBUG_PAIR_DELTA = FREECAM_INJECT_ADDR_2 - FREECAM_INJECT_ADDR_1;
                        FREECAM_DEBUG_PAIR_DELTA_ERR = FREECAM_DEBUG_PAIR_DELTA
                            .abs_diff(offsets::FREECAM_INJECT_2_DEBUG_RVA - offsets::FREECAM_INJECT_1_DEBUG_RVA);
                    }
                }

                if FREECAM_INJECT_ADDR_2 != 0 {
                    FREECAM_HOOK2_SKIP_ADDR = resolve_hook2_skip_target(FREECAM_INJECT_ADDR_2);

                    eprintln!(
                        "  ✓ Hook2 skip target: 0x{:08X}",
                        FREECAM_HOOK2_SKIP_ADDR
                    );

                    let inject2_rva = FREECAM_INJECT_ADDR_2.saturating_sub(offsets::FREECAM_IMAGE_BASE);
                    FREECAM_MOTION_FUNC = match inject2_rva {
                        offsets::FREECAM_INJECT_2_DEBUG_RVA => {
                            eprintln!("  ✓ Freecam variant: debug RVA set");
                            // Keep as diagnostic only; inline call path can crash in pause/menu states.
                            FREECAM_MOTION_CALL_SAFE = 0;
                            offsets::FREECAM_IMAGE_BASE + offsets::FREECAM_MOTION_FUNC_DEBUG_RVA
                        }
                        offsets::FREECAM_INJECT_2_RELEASE_RVA => {
                            eprintln!("  ✓ Freecam variant: release RVA set (debug - 0x{:X})", offsets::FREECAM_DEBUG_TO_RELEASE_DELTA);
                            // Keep as diagnostic only; inline call path can crash in pause/menu states.
                            FREECAM_MOTION_CALL_SAFE = 0;
                            offsets::FREECAM_IMAGE_BASE + offsets::FREECAM_MOTION_FUNC_RELEASE_RVA
                        }
                        _ => {
                            let legacy_candidate = FREECAM_INJECT_ADDR_2.saturating_sub(0x5D1);
                            let nearby_candidate = find_best_nearby_rel32_call_target(
                                FREECAM_INJECT_ADDR_2,
                                legacy_candidate,
                                0x4000,
                                0x400,
                            )
                            .unwrap_or(legacy_candidate);

                            // Keep unknown variants pinned to CT-derived baseline for stability.
                            // Nearby call target remains diagnostic only.
                            let candidate = legacy_candidate;
                            let executable = is_executable_addr(candidate, 16);
                            let has_call_ref = has_nearby_rel32_call_to_target(
                                FREECAM_INJECT_ADDR_2,
                                candidate,
                                0x2000,
                                0x200,
                            );
                            let strong_unique_pair = FREECAM_DEBUG_INJECT_MATCHES_1 == 1
                                && FREECAM_DEBUG_INJECT_MATCHES_2 == 1
                                && FREECAM_DEBUG_PAIR_DELTA_ERR <= 0x1000;
                            let near_legacy = nearby_candidate.abs_diff(legacy_candidate) <= 0x800;
                            eprintln!(
                                "  ! Inject2 RVA 0x{:X} unknown; legacy=0x{:08X} nearby=0x{:08X} chosen=0x{:08X} exec={} call_ref={} unique_pair={} near_legacy={}",
                                inject2_rva,
                                legacy_candidate,
                                nearby_candidate,
                                candidate,
                                executable,
                                has_call_ref,
                                strong_unique_pair,
                                near_legacy
                            );

                            // Unknown variant safety: keep target pinned to CT baseline and allow calls
                            // only when either structural call reference exists or inject pair is uniquely
                            // matched and close to CT delta.
                            FREECAM_MOTION_CALL_SAFE = if executable
                                && near_legacy
                                && (has_call_ref || strong_unique_pair)
                            {
                                0  // Disabled: motion call interferes with manual position writes
                            } else {
                                0
                            };
                            candidate
                        }
                    };
                    eprintln!(
                        "  ✓ Motion function resolved: 0x{:08X} (safe={})",
                        FREECAM_MOTION_FUNC,
                        FREECAM_MOTION_CALL_SAFE
                    );
                } else {
                    FREECAM_MOTION_FUNC = 0;
                    FREECAM_MOTION_CALL_SAFE = 0;
                }

                // Scan for camera manager pointer
                match self.process.scan_abs(
                    "freecam_cam_mgr",
                    &offsets::FREECAM_CAM_MGR_AOB,
                    offsets::FREECAM_CAM_MGR_AOB_OFFSET,
                    vec![],
                ) {
                    Ok(cam_mgr_ptr) => {
                        // Store the ADDRESS where the pointer is stored (not the pointer value itself)
                        FREECAM_CAM_MGR_PTR = cam_mgr_ptr.base_address;
                        eprintln!("  ✓ Camera manager pointer address: 0x{:08X}", cam_mgr_ptr.base_address);
                    }
                    Err(e) => {
                        eprintln!("  ✗ Camera manager pointer NOT found: {}", e);
                        FREECAM_CAM_MGR_PTR = 0;
                    }
                }

                // Scan for ChrFollowCam pointer (actual render camera)
                match self.process.scan_abs(
                    "chr_follow_cam",
                    &offsets::CHR_FOLLOW_CAM_AOB,
                    offsets::CHR_FOLLOW_CAM_AOB_OFFSET,
                    vec![],
                ) {
                    Ok(chr_cam_ptr) => {
                        CHR_FOLLOW_CAM_PTR = chr_cam_ptr.base_address;
                        eprintln!("  ✓ ChrFollowCam pointer address: 0x{:08X}", chr_cam_ptr.base_address);
                    }
                    Err(e) => {
                        eprintln!("  ✗ ChrFollowCam pointer NOT found: {}", e);
                        CHR_FOLLOW_CAM_PTR = 0;
                    }
                }

                // TODO: Camera follow disable patch - currently commented out due to freeze during injection
                // We'll add this as a separate function to run after injection is stable
                // Pattern: "80 7C 24 38 00 74 70 F3 0F 7E B4" - patch byte at offset +5 from 0x74 (je) to 0xEB (jmp)
            }
        } else {
            self.process.refresh()?;
        }

        Ok(())
    }

    pub fn get_x_pos(&self) -> f32 {
        let x_pos = self.char_pos_data.read_f32_rel(Some(CharPosData::POS_X));
        x_pos
    }

    pub fn get_y_pos(&self) -> f32 {
        let x_pos = self.char_pos_data.read_f32_rel(Some(CharPosData::POS_Y));
        x_pos
    }

    pub fn get_z_pos(&self) -> f32 {
        let x_pos = self.char_pos_data.read_f32_rel(Some(CharPosData::POS_Z));
        x_pos
    }

    pub fn get_angle(&self) -> f32 {
        self.char_pos_data
            .read_f32_rel(Some(CharPosData::POS_ANGLE))
    }

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
        self.world_state.write_u8_rel(Some(WorldState::DISABLE_ENEMIES), if value { 1 } else { 0 });
    }

    pub fn get_disable_events(&self) -> bool {
        self.world_state.read_bool_rel(Some(WorldState::DISABLE_EVENTS))
    }

    pub fn set_disable_events_to(&mut self, value: bool) {
        self.world_state.write_u8_rel(Some(WorldState::DISABLE_EVENTS), if value { 1 } else { 0 });
    }

    pub fn get_auto_save(&self) -> bool {
        self.world_state.read_bool_rel(Some(WorldState::AUTO_SAVE))
    }

    pub fn set_auto_save_to(&mut self, value: bool) {
        self.world_state.write_u8_rel(Some(WorldState::AUTO_SAVE), if value { 1 } else { 0 });
    }

    pub fn get_online_mode(&self) -> bool {
        self.world_state.read_bool_rel(Some(WorldState::ONLINE_MODE))
    }

    pub fn set_online_mode_to(&mut self, value: bool) {
        self.world_state.write_u8_rel(Some(WorldState::ONLINE_MODE), if value { 1 } else { 0 });
    }

    pub fn get_free_cam(&mut self) -> bool {
        self.free_cam_enabled
    }

    pub fn set_free_cam_to(&mut self, value: bool) {
        if value {
            self.free_cam_enabled = self.enable_free_cam_injection();
        } else {
            let _ = self.disable_free_cam_injection();
            self.free_cam_enabled = false;
        }
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn tick_freecam_mode_hotkey(&mut self) {
        unsafe {
            if !self.free_cam_enabled {
                FREECAM_XINPUT_HELD = 0;
                FREECAM_ACTIVE_PAD = -1;
                return;
            }

            let mut both_pressed = false;
            let mut pressed_idx: i32 = -1;

            for user_idx in 0..4u32 {
                let mut state: XINPUT_STATE = std::mem::zeroed();
                if XInputGetState(user_idx, &mut state) == 0 {
                    let buttons = state.Gamepad.wButtons;
                    let l3 = (buttons & XINPUT_GAMEPAD_LEFT_THUMB) != 0;
                    let r3 = (buttons & XINPUT_GAMEPAD_RIGHT_THUMB) != 0;
                    if l3 && r3 {
                        both_pressed = true;
                        pressed_idx = user_idx as i32;
                        FREECAM_DEBUG_XINPUT_PAD = user_idx as i32;
                        FREECAM_DEBUG_XINPUT_BUTTONS = buttons;
                        FREECAM_DEBUG_XINPUT_LX = state.Gamepad.sThumbLX;
                        FREECAM_DEBUG_XINPUT_LY = state.Gamepad.sThumbLY;
                        FREECAM_DEBUG_XINPUT_RX = state.Gamepad.sThumbRX;
                        FREECAM_DEBUG_XINPUT_RY = state.Gamepad.sThumbRY;
                        FREECAM_DEBUG_XINPUT_LT = state.Gamepad.bLeftTrigger;
                        FREECAM_DEBUG_XINPUT_RT = state.Gamepad.bRightTrigger;
                        break;
                    }
                }
            }

            if both_pressed {
                FREECAM_ACTIVE_PAD = pressed_idx;
                if FREECAM_XINPUT_HELD == 0 {
                    // Fallback mode cycle for builds where game-native button bit layout differs.
                    FREECAM_MODE += 1;
                    if FREECAM_MODE >= 4 {
                        FREECAM_MODE = 0;
                    }
                    FREECAM_XINPUT_HELD = 1;
                }
            } else {
                FREECAM_XINPUT_HELD = 0;
            }
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn tick_freecam_mode_hotkey(&mut self) {}

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn tick_freecam_manual_movement(&mut self) {
        unsafe {
            FREECAM_DEBUG_MANUAL_TICKS = FREECAM_DEBUG_MANUAL_TICKS.wrapping_add(1);

            if !self.free_cam_enabled {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }

            // Apply manual movement in all non-zero modes.
            if FREECAM_MODE == 0 {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }

            // Keep manual movement active in freecam modes so stick/trigger mapping can drive
            // hook-time deltas on release variants where game-native input bits differ.

            if FREECAM_CAM_MGR_PTR == 0 || !is_readable_addr(FREECAM_CAM_MGR_PTR, 4) {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }
            let cam_mgr = std::ptr::read_volatile(FREECAM_CAM_MGR_PTR as *const usize);
            if cam_mgr == 0 || !is_readable_addr(cam_mgr + 0x6F8, 4) {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }
            let cam_obj = std::ptr::read_volatile((cam_mgr + 0x6F8) as *const usize);
            if cam_obj == 0 {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }

            // Some PTDE variants never execute our secondary constraint hook path.
            // Reinforce freecam state directly on the camera object each tick.
            if is_writable_addr(cam_obj + 0x24C, 4) {
                let mode_write = if FREECAM_MODE == 0 {
                    0
                } else if FREECAM_FORCE_MODE2_ACTIVE_ONLY != 0 {
                    2
                } else {
                    FREECAM_MODE
                };
                std::ptr::write_volatile((cam_obj + 0x24C) as *mut i32, mode_write);
            }
            // Mirror flag bytes touched by the original script path around the frame-camera state.
            if is_writable_addr(cam_obj + 0x58, 1) {
                std::ptr::write_volatile((cam_obj + 0x58) as *mut u8, 0);
            }
            if is_writable_addr(cam_obj + 0x59, 1) {
                std::ptr::write_volatile((cam_obj + 0x59) as *mut u8, 0);
            }
            if is_writable_addr(cam_obj + 0x44, 1) {
                std::ptr::write_volatile((cam_obj + 0x44) as *mut u8, 1);
            }

            let mut cam_target = if is_readable_addr(cam_obj + 0x254, 4) {
                std::ptr::read_volatile((cam_obj + 0x254) as *const usize)
            } else {
                0
            };

            if cam_target == 0 {
                cam_target = std::ptr::addr_of_mut!(FREECAM_STORAGE_BUFFER) as *mut u32 as usize;
            }

            let cam_buf_18 = if is_readable_addr(cam_obj + 0x18, 4) {
                std::ptr::read_volatile((cam_obj + 0x18) as *const usize)
            } else {
                0
            };

            let cam_buf_30 = if is_readable_addr(cam_obj + 0x30, 4) {
                std::ptr::read_volatile((cam_obj + 0x30) as *const usize)
            } else {
                0
            };

            if cam_buf_30 != 0 {
                // Keep render-camera control bytes aligned with active freecam movement.
                if is_writable_addr(cam_buf_30 + 0x58, 1) {
                    std::ptr::write_volatile((cam_buf_30 + 0x58) as *mut u8, 0);
                }
                if is_writable_addr(cam_buf_30 + 0x59, 1) {
                    std::ptr::write_volatile((cam_buf_30 + 0x59) as *mut u8, 0);
                }
                if is_writable_addr(cam_buf_30 + 0x44, 1) {
                    std::ptr::write_volatile((cam_buf_30 + 0x44) as *mut u8, 1);
                }
            }

            if cam_buf_30 != 0 {
                if is_readable_addr(cam_buf_30 + 0x3C, 4) {
                    FREECAM_DEBUG_BUF30_TA_X = std::ptr::read_volatile((cam_buf_30 + 0x1C) as *const f32);
                    FREECAM_DEBUG_BUF30_TA_Y = std::ptr::read_volatile((cam_buf_30 + 0x2C) as *const f32);
                    FREECAM_DEBUG_BUF30_TA_Z = std::ptr::read_volatile((cam_buf_30 + 0x3C) as *const f32);
                }
                if is_readable_addr(cam_buf_30 + 0x48, 4) {
                    FREECAM_DEBUG_BUF30_TB_X = std::ptr::read_volatile((cam_buf_30 + 0x40) as *const f32);
                    FREECAM_DEBUG_BUF30_TB_Y = std::ptr::read_volatile((cam_buf_30 + 0x44) as *const f32);
                    FREECAM_DEBUG_BUF30_TB_Z = std::ptr::read_volatile((cam_buf_30 + 0x48) as *const f32);
                }
            }

            let mut state = std::mem::zeroed::<XINPUT_STATE>();
            let mut found = false;
            if FREECAM_ACTIVE_PAD >= 0 && XInputGetState(FREECAM_ACTIVE_PAD as u32, &mut state) == 0 {
                found = true;
                FREECAM_DEBUG_XINPUT_PAD = FREECAM_ACTIVE_PAD;
            } else {
                for user_idx in 0..4u32 {
                    if XInputGetState(user_idx, &mut state) == 0 {
                        found = true;
                        FREECAM_ACTIVE_PAD = user_idx as i32;
                        FREECAM_DEBUG_XINPUT_PAD = user_idx as i32;
                        break;
                    }
                }
            }

            if !found {
                FREECAM_MOVE_DX = 0.0;
                FREECAM_MOVE_DY = 0.0;
                FREECAM_MOVE_DZ = 0.0;
                return;
            }

            FREECAM_DEBUG_XINPUT_BUTTONS = state.Gamepad.wButtons;
            FREECAM_DEBUG_XINPUT_LX = state.Gamepad.sThumbLX;
            FREECAM_DEBUG_XINPUT_LY = state.Gamepad.sThumbLY;
            FREECAM_DEBUG_XINPUT_RX = state.Gamepad.sThumbRX;
            FREECAM_DEBUG_XINPUT_RY = state.Gamepad.sThumbRY;
            FREECAM_DEBUG_XINPUT_LT = state.Gamepad.bLeftTrigger;
            FREECAM_DEBUG_XINPUT_RT = state.Gamepad.bRightTrigger;

            let lx = normalize_stick_axis(state.Gamepad.sThumbLX, 250);
            let ly = normalize_stick_axis(state.Gamepad.sThumbLY, 250);
            let rx = normalize_stick_axis(state.Gamepad.sThumbRX, 250);
            let ry = normalize_stick_axis(state.Gamepad.sThumbRY, 250);

            // Support both stick layouts by selecting the stronger input per axis.
            let move_x = if lx.abs() >= rx.abs() { lx } else { rx };
            let move_z = if ly.abs() >= ry.abs() { ly } else { ry };

            let buttons = state.Gamepad.wButtons;
            let up_trig = (state.Gamepad.bRightTrigger as f32 / 255.0).clamp(0.0, 1.0);
            let down_trig = (state.Gamepad.bLeftTrigger as f32 / 255.0).clamp(0.0, 1.0);
            let up_shoulder = if (buttons & XINPUT_GAMEPAD_RIGHT_SHOULDER) != 0 { 1.0 } else { 0.0 };
            let down_shoulder = if (buttons & XINPUT_GAMEPAD_LEFT_SHOULDER) != 0 { 1.0 } else { 0.0 };
            let up = up_trig.max(up_shoulder);
            let down = down_trig.max(down_shoulder);
            let mut speed = 2.50f32;

            if (buttons & XINPUT_GAMEPAD_RIGHT_SHOULDER) != 0 {
                speed = 6.00;
            } else if (buttons & XINPUT_GAMEPAD_LEFT_SHOULDER) != 0 {
                speed = 0.80;
            }

            let dx = move_x * speed;
            let dz = move_z * speed;
            let dy = (up - down) * speed;

            // Keyboard fallback for setups where the game/controller path is not exposed via XInput.
            let key_left = key_down('A' as i32) || key_down(VK_LEFT as i32);
            let key_right = key_down('D' as i32) || key_down(VK_RIGHT as i32);
            let key_forward = key_down('W' as i32) || key_down(VK_UP as i32);
            let key_back = key_down('S' as i32) || key_down(VK_DOWN as i32);
            let key_up = key_down(VK_SPACE as i32);
            let key_downward = key_down(VK_CONTROL as i32);
            let key_fast = key_down(VK_SHIFT as i32);

            let mut kx = 0.0f32;
            let mut ky = 0.0f32;
            let mut kz = 0.0f32;
            if key_right {
                kx += 1.0;
            }
            if key_left {
                kx -= 1.0;
            }
            if key_forward {
                kz += 1.0;
            }
            if key_back {
                kz -= 1.0;
            }
            if key_up {
                ky += 1.0;
            }
            if key_downward {
                ky -= 1.0;
            }

            let key_speed = if key_fast { 6.00 } else { 2.50 };
            let dx = if kx != 0.0 { kx * key_speed } else { dx };
            let dy = if ky != 0.0 { ky * key_speed } else { dy };
            let dz = if kz != 0.0 { kz * key_speed } else { dz };

            FREECAM_DEBUG_MANUAL_AXES_X = dx;
            FREECAM_DEBUG_MANUAL_AXES_Y = dy;
            FREECAM_DEBUG_MANUAL_AXES_Z = dz;
            FREECAM_MOVE_DX = dx;
            FREECAM_MOVE_DY = dy;
            FREECAM_MOVE_DZ = dz;

            if dx.abs() < 0.0001 && dy.abs() < 0.0001 && dz.abs() < 0.0001 {
                return;
            }

            // Apply to primary camera object buffers first (most reliable for this variant).
            let mut wrote_any = false;
            let mut last_write = 0usize;

            if cam_target != 0 && apply_translation_deltas(cam_target, dx, dy, dz) {
                wrote_any = true;
                last_write = cam_target + 0x40;
            }

            if cam_buf_30 != 0 && apply_translation_deltas(cam_buf_30, dx, dy, dz) {
                if !wrote_any {
                    last_write = cam_buf_30 + 0x40;
                }
                wrote_any = true;
            }

            if cam_buf_18 != 0 && apply_translation_deltas(cam_buf_18, dx, dy, dz) {
                if !wrote_any {
                    last_write = cam_buf_18 + 0x40;
                }
                wrote_any = true;
            }

            // Try writing to ChrFollowCam (actual render camera from DSGadget)
            FREECAM_DEBUG_CHR_FOLLOWCAM_PTR = CHR_FOLLOW_CAM_PTR;
            FREECAM_DEBUG_CHR_BASE = 0;
            FREECAM_DEBUG_CHR_L1 = 0;
            FREECAM_DEBUG_CHR_L2 = 0;
            FREECAM_DEBUG_CHR_FOLLOWCAM_OBJ = 0;
            
            if CHR_FOLLOW_CAM_PTR != 0 && is_readable_addr(CHR_FOLLOW_CAM_PTR, 4) {
                let chr_cam_base = std::ptr::read_volatile(CHR_FOLLOW_CAM_PTR as *const usize);
                FREECAM_DEBUG_CHR_BASE = chr_cam_base;
                if chr_cam_base != 0 && is_readable_addr(chr_cam_base, 4) {
                    let chr_cam_l1 = std::ptr::read_volatile(chr_cam_base as *const usize);
                    FREECAM_DEBUG_CHR_L1 = chr_cam_l1;
                    if chr_cam_l1 != 0 && is_readable_addr(chr_cam_l1 + 0x3C, 4) {
                        let chr_cam_l2 = std::ptr::read_volatile((chr_cam_l1 + 0x3C) as *const usize);
                        FREECAM_DEBUG_CHR_L2 = chr_cam_l2;
                        if chr_cam_l2 != 0 && is_readable_addr(chr_cam_l2 + 0x60, 4) {
                            let chr_cam_l3 = std::ptr::read_volatile((chr_cam_l2 + 0x60) as *const usize);
                            FREECAM_DEBUG_CHR_FOLLOWCAM_OBJ = chr_cam_l3;  // Store for debugging
                            if chr_cam_l3 != 0 && is_writable_addr(chr_cam_l3 + 0x108, 4) {
                                // Write to PosX/Y/Z at offsets 0x100, 0x104, 0x108
                                let pos_x = std::ptr::read_volatile((chr_cam_l3 + 0x100) as *const f32);
                                let pos_y = std::ptr::read_volatile((chr_cam_l3 + 0x104) as *const f32);
                                let pos_z = std::ptr::read_volatile((chr_cam_l3 + 0x108) as *const f32);
                                std::ptr::write_volatile((chr_cam_l3 + 0x100) as *mut f32, pos_x + dx);
                                std::ptr::write_volatile((chr_cam_l3 + 0x104) as *mut f32, pos_y + dy);
                                std::ptr::write_volatile((chr_cam_l3 + 0x108) as *mut f32, pos_z + dz);
                                
                                // ALSO write player position so camera follows our freecam instead
                                // This tricks the follow camera into following where we are
                                if is_writable_addr(chr_cam_l3 + 0xE8, 4) {
                                    // RotX/Y/Z (what player is) - write same as camera pos
                                    std::ptr::write_volatile((chr_cam_l3 + 0xE0) as *mut f32, pos_x + dx);
                                    std::ptr::write_volatile((chr_cam_l3 + 0xE4) as *mut f32, pos_y + dy);
                                    std::ptr::write_volatile((chr_cam_l3 + 0xE8) as *mut f32, pos_z + dz);
                                }
                                
                                if !wrote_any {
                                    last_write = chr_cam_l3 + 0x104;
                                }
                                wrote_any = true;
                            }
                        }
                    }
                }
            }

            if wrote_any {
                FREECAM_DEBUG_MANUAL_WRITES = FREECAM_DEBUG_MANUAL_WRITES.wrapping_add(1);
                FREECAM_DEBUG_LAST_WRITE_ADDR = last_write;
                FREECAM_FALLBACK_BUF_BASE = last_write.saturating_sub(0x40);
            } else {
                FREECAM_FALLBACK_BUF_BASE = 0;
            }
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn tick_freecam_manual_movement(&mut self) {}

    /// Disable camera follow logic by patching je to jmp
    /// Call this AFTER freecam injection is complete and stable
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn disable_camera_follow(&mut self) -> bool {
        use crate::memory::ds1::offsets;
        
        unsafe {
            eprintln!("[CAMERA FOLLOW DISABLE] NOPing camera write instructions...");
            
            let base = 0x00400000usize;
            
            // Location 1 - X/Y write (7 bytes)
            let addr1_xy = base + offsets::CAMERA_WRITE_1_XY_RVA;
            let patch1_xy = vec![0x90u8; offsets::CAMERA_WRITE_1_XY_SIZE];
            let ok1_xy = write_exec_patch(addr1_xy, &patch1_xy);
            eprintln!("[WRITE 1 X/Y] 0x{:08X} ({} bytes) -> {}", addr1_xy, offsets::CAMERA_WRITE_1_XY_SIZE, if ok1_xy { "OK" } else { "FAIL" });
            
            // Location 1 - Z write (8 bytes)
            let addr1_z = base + offsets::CAMERA_WRITE_1_Z_RVA;
            let patch1_z = vec![0x90u8; offsets::CAMERA_WRITE_1_Z_SIZE];
            let ok1_z = write_exec_patch(addr1_z, &patch1_z);
            eprintln!("[WRITE 1 Z] 0x{:08X} ({} bytes) -> {}", addr1_z, offsets::CAMERA_WRITE_1_Z_SIZE, if ok1_z { "OK" } else { "FAIL" });
            
            // Location 2 - X/Y write (8 bytes)
            let addr2_xy = base + offsets::CAMERA_WRITE_2_XY_RVA;
            let patch2_xy = vec![0x90u8; offsets::CAMERA_WRITE_2_XY_SIZE];
            let ok2_xy = write_exec_patch(addr2_xy, &patch2_xy);
            eprintln!("[WRITE 2 X/Y] 0x{:08X} ({} bytes) -> {}", addr2_xy, offsets::CAMERA_WRITE_2_XY_SIZE, if ok2_xy { "OK" } else { "FAIL" });
            
            // Location 2 - Z write (8 bytes)
            let addr2_z = base + offsets::CAMERA_WRITE_2_Z_RVA;
            let patch2_z = vec![0x90u8; offsets::CAMERA_WRITE_2_Z_SIZE];
            let ok2_z = write_exec_patch(addr2_z, &patch2_z);
            eprintln!("[WRITE 2 Z] 0x{:08X} ({} bytes) -> {}", addr2_z, offsets::CAMERA_WRITE_2_Z_SIZE, if ok2_z { "OK" } else { "FAIL" });
            
            let success_count = [ok1_xy, ok1_z, ok2_xy, ok2_z].iter().filter(|&&x| x).count();
            eprintln!("[CAMERA FOLLOW DISABLE] {}/4 writes NOPed successfully", success_count);
            
            success_count == 4
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn disable_camera_follow(&mut self) -> bool {
        false
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    fn enable_free_cam_injection(&mut self) -> bool {
        if self.free_cam_enabled {
            return true;
        }

        unsafe {
            // Check if addresses were scanned
            if FREECAM_INJECT_ADDR_1 == 0 || FREECAM_INJECT_ADDR_2 == 0 {
                eprintln!("Freecam: Injection addresses not found. Make sure the game is loaded.");
                return false;
            }

            if FREECAM_CAM_MGR_PTR == 0 {
                eprintln!("Freecam: Camera manager pointer not found.");
                return false;
            }

            if FREECAM_MOTION_CALL_SAFE == 1 && FREECAM_MOTION_FUNC == 0 {
                eprintln!("Freecam: Motion update function not resolved for safe call path.");
                return false;
            }

            // Reset hook state
            FREECAM_MODE = 0;
            FREECAM_L3R3_HELD = 0;
            FREECAM_XINPUT_HELD = 0;
            FREECAM_MOVE_DX = 0.0;
            FREECAM_MOVE_DY = 0.0;
            FREECAM_MOVE_DZ = 0.0;
            FREECAM_IN_HOOK = 0;
            FREECAM_INJECTION_SUCCESS = false;
            FREECAM_INJECTION_PENDING = false;
            
            // Reset debug counters
            FREECAM_DEBUG_FUNC_ADDR = 0;
            FREECAM_DEBUG_CAM_OBJ = 0;
            FREECAM_DEBUG_CALLED_FUNC = 0;
            FREECAM_DEBUG_HOOK_ENTRY = 0;
            FREECAM_DEBUG_INPUT_RAW_8 = 0;
            FREECAM_DEBUG_INPUT_RAW_C = 0;
            FREECAM_DEBUG_HOOK2_ENTRY_COUNT = 0;
            FREECAM_DEBUG_HOOK2_SKIP_COUNT = 0;
            FREECAM_DEBUG_HOOK2_ORIG_COUNT = 0;
            FREECAM_DEBUG_BUF_18 = 0;
            FREECAM_DEBUG_BUF_30 = 0;
            FREECAM_DEBUG_BUF_254 = 0;
            FREECAM_DEBUG_BUF30_TA_X = 0.0;
            FREECAM_DEBUG_BUF30_TA_Y = 0.0;
            FREECAM_DEBUG_BUF30_TA_Z = 0.0;
            FREECAM_DEBUG_BUF30_TB_X = 0.0;
            FREECAM_DEBUG_BUF30_TB_Y = 0.0;
            FREECAM_DEBUG_BUF30_TB_Z = 0.0;
            FREECAM_DEBUG_HOOK_APPLY_COUNT = 0;
            FREECAM_DEBUG_HOOK_LAST_DST = 0;
            FREECAM_DEBUG_HOOK_LAST_SRC = 0;
            FREECAM_DEBUG_HOOK_MOVE_DX = 0.0;
            FREECAM_DEBUG_HOOK_MOVE_DY = 0.0;
            FREECAM_DEBUG_HOOK_MOVE_DZ = 0.0;
            FREECAM_DEBUG_HOOK_POST_40 = 0.0;
            FREECAM_DEBUG_HOOK_POST_44 = 0.0;
            FREECAM_DEBUG_HOOK_POST_48 = 0.0;
            FREECAM_FALLBACK_BUF_BASE = 0;

            // Verify addresses contain the expected original opcodes
            let addr1_bytes = std::slice::from_raw_parts(FREECAM_INJECT_ADDR_1 as *const u8, 7);
            let addr2_bytes = std::slice::from_raw_parts(FREECAM_INJECT_ADDR_2 as *const u8, 6);

            if addr1_bytes != FREECAM_ORIG_PATCH_1 {
                eprintln!("Freecam: Addr1 bytes don't match expected pattern");
                eprintln!("  Expected: {:02X?}", FREECAM_ORIG_PATCH_1);
                eprintln!("  Found:    {:02X?}", addr1_bytes);
                return false;
            }

            if addr2_bytes != FREECAM_ORIG_PATCH_2 {
                eprintln!("Freecam: Addr2 bytes don't match expected pattern");
                eprintln!("  Expected: {:02X?}", FREECAM_ORIG_PATCH_2);
                eprintln!("  Found:    {:02X?}", addr2_bytes);
                return false;
            }

            // Create jump patches
            let patch_1 = match jmp_patch(FREECAM_INJECT_ADDR_1, freecam_hook_1 as usize, 7) {
                Some(p) => p,
                None => {
                    eprintln!("Freecam: Failed to create patch 1");
                    return false;
                }
            };

            let patch_2 = match jmp_patch(FREECAM_INJECT_ADDR_2, freecam_hook_2 as usize, 6) {
                Some(p) => p,
                None => {
                    eprintln!("Freecam: Failed to create patch 2");
                    return false;
                }
            };

            // Apply patches
            eprintln!("Freecam: Applying patch 1 at 0x{:08X}...", FREECAM_INJECT_ADDR_1);
            if !write_exec_patch(FREECAM_INJECT_ADDR_1, &patch_1) {
                eprintln!("Freecam: Failed to write patch 1");
                return false;
            }
            eprintln!("Freecam: Patch 1 applied successfully");

            eprintln!("Freecam: Applying patch 2 at 0x{:08X}...", FREECAM_INJECT_ADDR_2);
            if !write_exec_patch(FREECAM_INJECT_ADDR_2, &patch_2) {
                eprintln!("Freecam: Failed to write patch 2");
                let _ = write_exec_patch(FREECAM_INJECT_ADDR_1, &FREECAM_ORIG_PATCH_1);
                return false;
            }
            eprintln!("Freecam: Patch 2 applied successfully");

            eprintln!("Freecam injection successful!");
            eprintln!("  Hook 1: 0x{:08X} -> 0x{:08X}", FREECAM_INJECT_ADDR_1, freecam_hook_1 as *const () as usize);
            eprintln!("  Hook 2: 0x{:08X} -> 0x{:08X}", FREECAM_INJECT_ADDR_2, freecam_hook_2 as *const () as usize);
            eprintln!("  Camera manager ptr addr: 0x{:08X}", FREECAM_CAM_MGR_PTR);
            eprintln!(
                "  Motion function addr: 0x{:08X} (safe={})",
                FREECAM_MOTION_FUNC,
                FREECAM_MOTION_CALL_SAFE
            );
            
            // Read and log what the camera manager pointer currently contains
            let cam_mgr_value = std::ptr::read(FREECAM_CAM_MGR_PTR as *const usize);
            eprintln!("  Camera manager value: 0x{:08X}", cam_mgr_value);
            
            FREECAM_INJECTION_SUCCESS = true;
            eprintln!("Freecam: Injection complete, waiting for game to call hook...");
            
            // Apply camera follow disable patch
            eprintln!("");
            eprintln!("Freecam: Disabling camera follow-player logic...");
        }
        
        // Call camera follow disable after successful injection
        if unsafe { FREECAM_INJECTION_SUCCESS } {
            self.disable_camera_follow();
        }
        
        true
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    fn enable_free_cam_injection(&mut self) -> bool {
        false
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    fn disable_free_cam_injection(&mut self) -> bool {
        unsafe {
            if FREECAM_INJECT_ADDR_1 == 0 || FREECAM_INJECT_ADDR_2 == 0 {
                return true; // Nothing to disable
            }

            // Restore original bytes
            let ok1 = write_exec_patch(FREECAM_INJECT_ADDR_1, &FREECAM_ORIG_PATCH_1);
            let ok2 = write_exec_patch(FREECAM_INJECT_ADDR_2, &FREECAM_ORIG_PATCH_2);

            // Reset hook state
            FREECAM_MODE = 0;
            FREECAM_L3R3_HELD = 0;
            FREECAM_XINPUT_HELD = 0;
            FREECAM_MOVE_DX = 0.0;
            FREECAM_MOVE_DY = 0.0;
            FREECAM_MOVE_DZ = 0.0;
            FREECAM_IN_HOOK = 0;
            FREECAM_INJECTION_SUCCESS = false;
            FREECAM_INJECTION_PENDING = false;

            if ok1 && ok2 {
                eprintln!("Freecam disabled successfully");
            }

            ok1 && ok2
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    fn disable_free_cam_injection(&mut self) -> bool {
        true
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn try_pending_freecam_injection(&mut self) {
        // No longer needed - addresses are scanned at startup in refresh()
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn try_pending_freecam_injection(&mut self) {}

    pub fn get_no_stamina_consume(&mut self) -> bool {
        let no_stamina_consume = self
            .chr_dbg
            .read_bool_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME));
        return no_stamina_consume;
    }

    pub fn get_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_UPDATE_AI));
        return no_update_ai;
    }

    pub fn get_infinite_magic(&mut self) -> bool {
        self.all_no_magic_quantity_consume.read_bool_rel(Some(0))
    }

    pub fn get_infinite_goods(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        (current_flags & CharFlags2::NO_GOODS_CONSUME as u32) != 0
    }

    pub fn get_player_hide(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::PLAYER_HIDE))
    }

    pub fn get_player_silence(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::PLAYER_SILENCE))
    }

    pub fn get_no_death(&mut self) -> bool {
        self.no_death_pointer.read_bool_rel(Some(0x0))
    }

    pub fn get_no_damage(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_DAMAGE))
    }

    pub fn get_no_hit(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_HIT))
    }

    pub fn get_no_attack(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_ATTACK))
    }

    pub fn get_no_move(&mut self) -> bool {
        self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_MOVE))
    }

    pub fn get_disable_collision(&mut self) -> bool {
        let current_flags = self
            .char_map_data
            .read_u32_rel(Some(CharMapData::CHAR_MAP_FLAGS));
        (current_flags & CharMapFlags::DISABLE_MAP_HIT as u32) != 0
    }

    pub fn get_no_gravity(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_1));
        (current_flags & CharFlags1::SET_DISABLE_GRAVITY as u32) != 0
    }

    pub fn get_draw_direction(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        (current_flags & CharFlags2::DRAW_DIRECTION as u32) != 0
    }

    pub fn get_draw_counter(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        (current_flags & CharFlags2::DRAW_COUNTER as u32) != 0
    }

    pub fn get_draw_stable_pos(&mut self) -> bool {
        self.chr_data_1
            .read_bool_rel(Some(CharFlags2::DRAW_STABLE_POS))
    }

    pub fn set_no_stam_consume(&mut self) -> bool {
        let no_stamina_consume = self.get_no_stamina_consume();
        if no_stamina_consume == false {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x0);
        }
        no_stamina_consume
    }

    pub fn set_no_stam_consume_to(&mut self, value: bool) {
        self.chr_dbg.write_u8_rel(
            Some(ChrDbg::ALL_NO_STAMINA_CONSUME),
            if value { 0x1 } else { 0x0 },
        );
    }

    pub fn set_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.get_no_update_ai();
        if no_update_ai == false {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x0);
        }
        no_update_ai
    }

    pub fn set_no_update_ai_to(&mut self, value: bool) {
        self.chr_dbg.write_u8_rel(
            Some(ChrDbg::ALL_NO_UPDATE_AI),
            if value { 0x1 } else { 0x0 },
        );
    }

    pub fn set_no_death(&mut self) -> bool {
        let no_death = self.get_no_death();
        if no_death == false {
            self.no_death_pointer.write_u8_rel(Some(0x0), 0x1);
        } else {
            self.no_death_pointer.write_u8_rel(Some(0x0), 0x0);
        }
        no_death
    }

    pub fn set_no_death_to(&mut self, value: bool) {
        self.no_death_pointer
            .write_u8_rel(Some(0x0), if value { 0x1 } else { 0x0 });
    }

    pub fn set_player_exterminate(&mut self, value: bool) {
        self.player_exterminate
            .write_u8_rel(Some(0x0), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_mp_consume(&mut self) -> bool {
        let no_mp_consume = self.chr_dbg.read_bool_rel(Some(ChrDbg::ALL_NO_MPCONSUME));
        if no_mp_consume == false {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_MPCONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_MPCONSUME), 0x0);
        }
        no_mp_consume
    }

    pub fn set_no_arrow_consume(&mut self) -> bool {
        let no_arrow_consume = self
            .chr_dbg
            .read_bool_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME));
        if no_arrow_consume == false {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME), 0x1);
        } else {
            self.chr_dbg
                .write_u8_rel(Some(ChrDbg::ALL_NO_ARROW_CONSUME), 0x0);
        }
        no_arrow_consume
    }

    pub fn set_player_hide(&mut self) -> bool {
        let player_hide = self.get_player_hide();
        if player_hide == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_HIDE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_HIDE), 0x0);
        }
        player_hide
    }

    pub fn set_player_hide_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::PLAYER_HIDE), if value { 0x1 } else { 0x0 });
    }

    pub fn set_player_silence(&mut self) -> bool {
        let player_silence = self.get_player_silence();
        if player_silence == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_SILENCE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::PLAYER_SILENCE), 0x0);
        }
        player_silence
    }

    pub fn set_player_silence_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::PLAYER_SILENCE), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_damage(&mut self) -> bool {
        let no_damage = self.get_no_damage();
        if no_damage == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_DAMAGE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_DAMAGE), 0x0);
        }
        no_damage
    }

    pub fn set_no_damage_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::ALL_NO_DAMAGE), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_hit(&mut self) -> bool {
        let no_hit = self.get_no_hit();
        if no_hit == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_HIT), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_HIT), 0x0);
        }
        no_hit
    }

    pub fn set_no_hit_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::ALL_NO_HIT), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_attack(&mut self) -> bool {
        let no_attack = self.get_no_attack();
        if no_attack == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_ATTACK), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_ATTACK), 0x0);
        }
        no_attack
    }

    pub fn set_no_attack_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::ALL_NO_ATTACK), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_move(&mut self) -> bool {
        let no_move = self.get_no_move();
        if no_move == false {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_MOVE), 0x1);
        } else {
            self.chr_dbg.write_u8_rel(Some(ChrDbg::ALL_NO_MOVE), 0x0);
        }
        no_move
    }

    pub fn set_no_move_to(&mut self, value: bool) {
        self.chr_dbg
            .write_u8_rel(Some(ChrDbg::ALL_NO_MOVE), if value { 0x1 } else { 0x0 });
    }

    pub fn teleport_player(&mut self, x: f32, y: f32, z: f32, angle: f32) {
        self.char_map_data
            .write_f32_rel(Some(CharMapData::WARP_X), x);
        self.char_map_data
            .write_f32_rel(Some(CharMapData::WARP_Y), y);
        self.char_map_data
            .write_f32_rel(Some(CharMapData::WARP_Z), z);
        self.char_map_data
            .write_f32_rel(Some(CharMapData::WARP_ANGLE), angle);
        // Pre-write the live angle so it's correct when the warp routine reads it
        self.char_pos_data
            .write_f32_rel(Some(CharPosData::POS_ANGLE), angle);
        self.char_map_data.write_u8_rel(Some(CharMapData::WARP), 1);
    }

    pub fn set_disable_collision(&mut self) -> bool {
        let current_flags = self
            .char_map_data
            .read_u32_rel(Some(CharMapData::CHAR_MAP_FLAGS));
        let collision_disabled = (current_flags & CharMapFlags::DISABLE_MAP_HIT as u32) != 0;

        if collision_disabled {
            // Enable collision by clearing the flag
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags & !(CharMapFlags::DISABLE_MAP_HIT as u32),
            );
        } else {
            // Disable collision by setting the flag
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags | CharMapFlags::DISABLE_MAP_HIT as u32,
            );
        }

        collision_disabled
    }

    pub fn set_disable_collision_to(&mut self, value: bool) {
        let current_flags = self
            .char_map_data
            .read_u32_rel(Some(CharMapData::CHAR_MAP_FLAGS));
        if value {
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags | CharMapFlags::DISABLE_MAP_HIT as u32,
            );
        } else {
            self.char_map_data.write_u32_rel(
                Some(CharMapData::CHAR_MAP_FLAGS),
                current_flags & !(CharMapFlags::DISABLE_MAP_HIT as u32),
            );
        }
    }

    pub fn set_no_gravity(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_1));
        let gravity_disabled = (current_flags & CharFlags1::SET_DISABLE_GRAVITY as u32) != 0;

        if gravity_disabled {
            // Enable gravity by clearing the flag
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags & !(CharFlags1::SET_DISABLE_GRAVITY as u32),
            );
        } else {
            // Disable gravity by setting the flag
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags | CharFlags1::SET_DISABLE_GRAVITY as u32,
            );
        }

        gravity_disabled
    }

    pub fn set_no_gravity_to(&mut self, value: bool) {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_1));
        if value {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags | CharFlags1::SET_DISABLE_GRAVITY as u32,
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_1),
                current_flags & !(CharFlags1::SET_DISABLE_GRAVITY as u32),
            );
        }
    }

    pub fn set_all_no_magic_quantity_consume(&mut self) -> bool {
        let no_magic_qty_consume = self.get_infinite_magic();
        if no_magic_qty_consume == false {
            self.all_no_magic_quantity_consume
                .write_u8_rel(Some(0), 0x1);
        } else {
            self.all_no_magic_quantity_consume
                .write_u8_rel(Some(0), 0x0);
        }
        no_magic_qty_consume
    }

    pub fn set_all_no_magic_quantity_consume_to(&mut self, value: bool) {
        self.all_no_magic_quantity_consume
            .write_u8_rel(Some(0), if value { 0x1 } else { 0x0 });
    }

    pub fn set_no_goods_consume(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        let no_goods_consume = (current_flags & CharFlags2::NO_GOODS_CONSUME as u32) != 0;

        if no_goods_consume {
            // Disable infinite goods by clearing the flag
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::NO_GOODS_CONSUME as u32),
            );
        } else {
            // Enable infinite goods by setting the flag
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::NO_GOODS_CONSUME as u32,
            );
        }

        no_goods_consume
    }

    pub fn set_no_goods_consume_to(&mut self, value: bool) {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        if value {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::NO_GOODS_CONSUME as u32,
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::NO_GOODS_CONSUME as u32),
            );
        }
    }

    pub fn set_draw_direction(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        let draw_direction = (current_flags & CharFlags2::DRAW_DIRECTION as u32) != 0;

        if draw_direction {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::DRAW_DIRECTION as u32),
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::DRAW_DIRECTION as u32,
            );
        }

        draw_direction
    }

    pub fn set_draw_direction_to(&mut self, value: bool) {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        if value {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::DRAW_DIRECTION as u32,
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::DRAW_DIRECTION as u32),
            );
        }
    }

    pub fn set_draw_counter(&mut self) -> bool {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        let draw_counter = (current_flags & CharFlags2::DRAW_COUNTER as u32) != 0;

        if draw_counter {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::DRAW_COUNTER as u32),
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::DRAW_COUNTER as u32,
            );
        }

        draw_counter
    }

    pub fn set_draw_counter_to(&mut self, value: bool) {
        let current_flags = self.chr_data_1.read_u32_rel(Some(CharData1::CHAR_FLAGS_2));
        if value {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags | CharFlags2::DRAW_COUNTER as u32,
            );
        } else {
            self.chr_data_1.write_u32_rel(
                Some(CharData1::CHAR_FLAGS_2),
                current_flags & !(CharFlags2::DRAW_COUNTER as u32),
            );
        }
    }

    pub fn set_draw_stable_pos(&mut self) -> bool {
        let draw_stable_pos = self.get_draw_stable_pos();

        if draw_stable_pos {
            self.chr_data_1
                .write_u8_rel(Some(CharFlags2::DRAW_STABLE_POS), 0x0);
        } else {
            self.chr_data_1
                .write_u8_rel(Some(CharFlags2::DRAW_STABLE_POS), 0x1);
        }

        draw_stable_pos
    }

    pub fn set_draw_stable_pos_to(&mut self, value: bool) {
        self.chr_data_1.write_u8_rel(
            Some(CharFlags2::DRAW_STABLE_POS),
            if value { 0x1 } else { 0x0 },
        );
    }

    // Freecam debug logging accessors
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_log_stats(&self) -> (i32, bool, usize, usize, usize) {
        unsafe {
            (
                FREECAM_MODE,
                FREECAM_INJECTION_SUCCESS,
                FREECAM_INJECT_ADDR_1,
                FREECAM_INJECT_ADDR_2,
                FREECAM_CAM_MGR_PTR,
            )
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_log_stats(&self) -> (i32, bool, usize, usize, usize) {
        (0, false, 0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_camera_values(&self) -> (usize, usize, i32) {
        unsafe {
            if FREECAM_CAM_MGR_PTR == 0 {
                return (0, 0, 0);
            }
            
            // Read camera manager pointer value
            let cam_mgr = std::ptr::read(FREECAM_CAM_MGR_PTR as *const usize);
            
            if cam_mgr == 0 {
                return (0, 0, 0);
            }
            
            // Read camera object pointer at [cam_mgr+0x6F8]
            let cam_obj_addr = (cam_mgr + 0x6F8) as *const usize;
            let cam_obj = std::ptr::read(cam_obj_addr);
            
            // Read mode value from camera object at +0x24C
            let mode = if cam_obj != 0 {
                std::ptr::read((cam_obj + 0x24C) as *const i32)
            } else {
                -1
            };
            
            (cam_mgr, cam_obj, mode)
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_camera_values(&self) -> (usize, usize, i32) {
        (0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_debug_values(&self) -> (usize, usize, u32, u32) {
        unsafe {
            (
                FREECAM_DEBUG_FUNC_ADDR,
                FREECAM_DEBUG_CAM_OBJ,
                FREECAM_DEBUG_CALLED_FUNC,
                FREECAM_DEBUG_HOOK_ENTRY,
            )
        }
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_input_hook_values(&self) -> (u32, u32) {
        unsafe { (FREECAM_DEBUG_INPUT_RAW_8, FREECAM_DEBUG_INPUT_RAW_C) }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_input_hook_values(&self) -> (u32, u32) {
        (0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_manual_debug_values(&self) -> (u32, u32, f32, f32, f32, usize) {
        unsafe {
            (
                FREECAM_DEBUG_MANUAL_TICKS,
                FREECAM_DEBUG_MANUAL_WRITES,
                FREECAM_DEBUG_MANUAL_AXES_X,
                FREECAM_DEBUG_MANUAL_AXES_Y,
                FREECAM_DEBUG_MANUAL_AXES_Z,
                FREECAM_DEBUG_LAST_WRITE_ADDR,
            )
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_manual_debug_values(&self) -> (u32, u32, f32, f32, f32, usize) {
        (0, 0, 0.0, 0.0, 0.0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_xinput_debug_values(&self) -> (i32, u16, i16, i16, i16, i16, u8, u8) {
        unsafe {
            (
                FREECAM_DEBUG_XINPUT_PAD,
                FREECAM_DEBUG_XINPUT_BUTTONS,
                FREECAM_DEBUG_XINPUT_LX,
                FREECAM_DEBUG_XINPUT_LY,
                FREECAM_DEBUG_XINPUT_RX,
                FREECAM_DEBUG_XINPUT_RY,
                FREECAM_DEBUG_XINPUT_LT,
                FREECAM_DEBUG_XINPUT_RT,
            )
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_xinput_debug_values(&self) -> (i32, u16, i16, i16, i16, i16, u8, u8) {
        (-1, 0, 0, 0, 0, 0, 0, 0)
    }
    
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_motion_func_bytes(&self) -> (usize, Vec<u8>) {
        unsafe {
            // Instead of using calculated address, scan near injection point for calls
            if FREECAM_INJECT_ADDR_1 == 0 {
                return (0, vec![]);
            }
            
            // Read 256 bytes after our injection point to look for call instructions
            let scan_addr = FREECAM_INJECT_ADDR_1 + 0x20;  // Start a bit after
            let mut bytes = vec![0u8; 128];
            
            for i in 0..128 {
                bytes[i] = std::ptr::read_volatile((scan_addr + i) as *const u8);
            }
            
            (scan_addr, bytes)
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_motion_func_bytes(&self) -> (usize, Vec<u8>) {
        (0, vec![])
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_debug_values(&self) -> (usize, usize, u32, u32) {
        (0, 0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_injection_info(&self) -> (bool, bool, bool, [u8; 7], [u8; 6], bool, u32) {
        unsafe {
            // Simplified - return injection success and addresses
            let addr1_valid = FREECAM_INJECT_ADDR_1 != 0;
            let addr2_valid = FREECAM_INJECT_ADDR_2 != 0;
            let found1 = if addr1_valid { FREECAM_ORIG_PATCH_1 } else { [0; 7] };
            let found2 = if addr2_valid { FREECAM_ORIG_PATCH_2 } else { [0; 6] };
            (
                FREECAM_INJECTION_SUCCESS,
                addr1_valid,
                addr2_valid,
                found1,
                found2,
                FREECAM_INJECTION_PENDING,
                0,
            )
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_injection_info(&self) -> (bool, bool, bool, [u8; 7], [u8; 6], bool, u32) {
        (false, false, false, [0; 7], [0; 6], false, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_camera_samples(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
        // Camera sampling removed in new implementation
        (0, 0, 0, 0, 0, 0, 0, 0)
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_camera_samples(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
        (0, 0, 0, 0, 0, 0, 0, 0)
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn clear_freecam_log(&mut self) {
        // No-op - logging removed
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn clear_freecam_log(&mut self) {}

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn save_freecam_log_to_file(&self) -> Result<String, String> {
        unsafe {
            use std::time::{SystemTime, UNIX_EPOCH};

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let filename = format!("freecam_status_{}.txt", timestamp);

            let (cam_mgr, cam_obj, cam_mode) = self.get_freecam_camera_values();
            let (dbg_a, dbg_b, dbg_c, dbg_d) = self.get_freecam_debug_values();
            let (hook_raw_8, hook_raw_c) = self.get_freecam_input_hook_values();
            let (esi_val, l3_raw, r3_raw) = self.get_freecam_input_debug();
            let (manual_ticks, manual_writes, mdx, mdy, mdz, last_addr) =
                self.get_freecam_manual_debug_values();
            let (xpad, xbtn, xlx, xly, xrx, xry, xlt, xrt) = self.get_freecam_xinput_debug_values();
            let x_l3 = (xbtn & 0x0040) != 0;
            let x_r3 = (xbtn & 0x0080) != 0;

            let info = format!(
                "Freecam Status Report\n\
                 ======================\n\
                 Timestamp (unix): {}\n\
                 Mode: {}\n\
                 Injection Success: {}\n\
                 Injection Pending: {}\n\
                 In Hook: {}\n\
                 \n\
                 Inject Addr 1: 0x{:08X}\n\
                 Inject Addr 2: 0x{:08X}\n\
                 Inject Match Count 1/2: {} / {}\n\
                 Inject Pair Delta: 0x{:X}\n\
                 Inject Pair Delta Err: 0x{:X}\n\
                 Inject Pair CallRef: {}\n\
                 Hook2 Skip Addr: 0x{:08X}\n\
                 Camera Manager Ptr Addr: 0x{:08X}\n\
                 Motion Function Addr: 0x{:08X}\n\
                 Motion Call Safe: {}\n\
                 \n\
                 Camera Manager Value: 0x{:08X}\n\
                 Camera Object Value: 0x{:08X}\n\
                 \n\
                 ChrFollowCam Ptr: 0x{:08X}\n\
                 ChrFollowCam Base [ptr]: 0x{:08X}\n\
                 ChrFollowCam L1 [base]: 0x{:08X}\n\
                 ChrFollowCam L2 [L1+3C]: 0x{:08X}\n\
                 ChrFollowCam L3 [L2+60]: 0x{:08X}\n\
                 Camera Mode In Memory: {}\n\
                 \n\
                 Debug Func Addr: 0x{:08X}\n\
                 Debug Cam Obj: 0x{:08X}\n\
                 Debug Called Counter: {}\n\
                 Debug Hook Entry Counter: {}\n\
                 Hook Input Raw [esi+0x8]: 0x{:08X}\n\
                 Hook Input Raw [esi+0xC]: 0x{:08X}\n\
                 Hook2 Entry/Skip/Orig Count: {} / {} / {}\n\
                 Cam Buffers [+18/+30/+254]: 0x{:08X} / 0x{:08X} / 0x{:08X}\n\
                 CamBuf+30 TA [1C/2C/3C]: {:.5} / {:.5} / {:.5}\n\
                 CamBuf+30 TB [40/44/48]: {:.5} / {:.5} / {:.5}\n\
                 Hook Apply Count: {}\n\
                 Hook Last Dst/Src: 0x{:08X} / 0x{:08X}\n\
                 Hook Move DX/Y/Z: {:.5} / {:.5} / {:.5}\n\
                 Hook Post [40/44/48]: {:.5} / {:.5} / {:.5}\n\
                 Manual Move Ticks: {}\n\
                 Manual Move Writes: {}\n\
                 Manual Delta X/Y/Z: {:.5} / {:.5} / {:.5}\n\
                 Manual Last Write Addr: 0x{:08X}\n\
                 XInput Active Pad: {}\n\
                 XInput Buttons: 0x{:04X}\n\
                 XInput L3/R3: {} / {}\n\
                 XInput LX/LY: {} / {}\n\
                 XInput RX/RY: {} / {}\n\
                 XInput LT/RT: {} / {}\n\
                 \n\
                 Input ESI: 0x{:08X}\n\
                 Input [esi+0x8]: 0x{:08X}\n\
                 Input [esi+0xC]: 0x{:08X}\n",
                timestamp,
                FREECAM_MODE,
                FREECAM_INJECTION_SUCCESS,
                FREECAM_INJECTION_PENDING,
                FREECAM_IN_HOOK,
                FREECAM_INJECT_ADDR_1,
                FREECAM_INJECT_ADDR_2,
                FREECAM_DEBUG_INJECT_MATCHES_1,
                FREECAM_DEBUG_INJECT_MATCHES_2,
                FREECAM_DEBUG_PAIR_DELTA,
                FREECAM_DEBUG_PAIR_DELTA_ERR,
                FREECAM_DEBUG_PAIR_CALL_REF,
                FREECAM_HOOK2_SKIP_ADDR,
                FREECAM_CAM_MGR_PTR,
                FREECAM_MOTION_FUNC,
                FREECAM_MOTION_CALL_SAFE,
                cam_mgr,
                cam_obj,
                FREECAM_DEBUG_CHR_FOLLOWCAM_PTR,
                FREECAM_DEBUG_CHR_BASE,
                FREECAM_DEBUG_CHR_L1,
                FREECAM_DEBUG_CHR_L2,
                FREECAM_DEBUG_CHR_FOLLOWCAM_OBJ,
                cam_mode,
                dbg_a,
                dbg_b,
                dbg_c,
                dbg_d,
                hook_raw_8,
                hook_raw_c,
                FREECAM_DEBUG_HOOK2_ENTRY_COUNT,
                FREECAM_DEBUG_HOOK2_SKIP_COUNT,
                FREECAM_DEBUG_HOOK2_ORIG_COUNT,
                FREECAM_DEBUG_BUF_18,
                FREECAM_DEBUG_BUF_30,
                FREECAM_DEBUG_BUF_254,
                FREECAM_DEBUG_BUF30_TA_X,
                FREECAM_DEBUG_BUF30_TA_Y,
                FREECAM_DEBUG_BUF30_TA_Z,
                FREECAM_DEBUG_BUF30_TB_X,
                FREECAM_DEBUG_BUF30_TB_Y,
                FREECAM_DEBUG_BUF30_TB_Z,
                FREECAM_DEBUG_HOOK_APPLY_COUNT,
                FREECAM_DEBUG_HOOK_LAST_DST,
                FREECAM_DEBUG_HOOK_LAST_SRC,
                FREECAM_DEBUG_HOOK_MOVE_DX,
                FREECAM_DEBUG_HOOK_MOVE_DY,
                FREECAM_DEBUG_HOOK_MOVE_DZ,
                FREECAM_DEBUG_HOOK_POST_40,
                FREECAM_DEBUG_HOOK_POST_44,
                FREECAM_DEBUG_HOOK_POST_48,
                manual_ticks,
                manual_writes,
                mdx,
                mdy,
                mdz,
                last_addr,
                xpad,
                xbtn,
                if x_l3 { 1 } else { 0 },
                if x_r3 { 1 } else { 0 },
                xlx,
                xly,
                xrx,
                xry,
                xlt,
                xrt,
                esi_val,
                l3_raw,
                r3_raw,
            );

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&filename)
                .map_err(|e| format!("Failed to create status file: {}", e))?;

            writeln!(file, "{}", info)
                .map_err(|e| format!("Failed to write status file: {}", e))?;
            file.flush()
                .map_err(|e| format!("Failed to flush status file: {}", e))?;

            Ok(filename)
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn save_freecam_log_to_file(&self) -> Result<String, String> {
        Err("Freecam logging not available on this platform".to_string())
    }

    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn scan_for_freecam_patterns(&self) -> Result<String, String> {
        use windows_sys::Win32::System::Memory::{
            VirtualQuery, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_EXECUTE_READ, 
            PAGE_EXECUTE_READWRITE, PAGE_READONLY, PAGE_READWRITE
        };
        
        let pattern1 = &FREECAM_ORIG_PATCH_1;
        let pattern2 = &FREECAM_ORIG_PATCH_2;
        let mut results = Vec::new();
        let mut pattern1_addrs = Vec::new();
        let mut pattern2_addrs = Vec::new();
        let mut addr = 0x00400000usize; // Start of typical executable
        let max_addr = 0x20000000usize; // Limit to main module range (first ~512MB)
        
        eprintln!("Starting memory scan for freecam patterns...");
        eprintln!("Pattern 1: {:02X?}", pattern1);
        eprintln!("Pattern 2: {:02X?}", pattern2);
        eprintln!("Scanning range: 0x{:08X} - 0x{:08X}", addr, max_addr);
        
        unsafe {
            while addr < max_addr {
                let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
                let result = VirtualQuery(
                    addr as *const c_void,
                    &mut mbi,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                );
                
                if result == 0 {
                    break;
                }
                
                // Check if memory is committed and EXECUTABLE
                if mbi.State == MEM_COMMIT {
                    let is_executable = mbi.Protect == PAGE_EXECUTE_READ
                        || mbi.Protect == PAGE_EXECUTE_READWRITE;
                    
                    if is_executable && mbi.RegionSize > 0 && mbi.RegionSize < 0x10000000 {
                        let region_start = mbi.BaseAddress as usize;
                        let region_size = mbi.RegionSize;
                        
                        // Scan this region for patterns
                        for offset in 0..region_size.saturating_sub(pattern1.len()) {
                            let check_addr = region_start + offset;
                            
                            // Try to read and match pattern 1
                            let slice1 = std::slice::from_raw_parts(check_addr as *const u8, pattern1.len());
                            if slice1 == pattern1 {
                                pattern1_addrs.push(check_addr);
                                eprintln!("Found pattern 1 at: 0x{:08X}", check_addr);
                            }
                            
                            // Try to read and match pattern 2
                            if offset < region_size.saturating_sub(pattern2.len()) {
                                let slice2 = std::slice::from_raw_parts(check_addr as *const u8, pattern2.len());
                                if slice2 == pattern2 {
                                    pattern2_addrs.push(check_addr);
                                    eprintln!("Found pattern 2 at: 0x{:08X}", check_addr);
                                }
                            }
                        }
                    }
                }
                
                // Move to next region
                addr = (mbi.BaseAddress as usize).saturating_add(mbi.RegionSize.max(1));
                if addr <= mbi.BaseAddress as usize {
                    break; // Overflow protection
                }
            }
        }
        
        eprintln!("Scan complete. Pattern 1: {} matches, Pattern 2: {} matches", 
                  pattern1_addrs.len(), pattern2_addrs.len());
        
        if pattern1_addrs.is_empty() && pattern2_addrs.is_empty() {
            Ok("No patterns found in executable memory.".to_string())
        } else {
            unsafe {
                results.push(format!("=== Freecam Pattern Scan Results ==="));
                results.push(format!(""));
                results.push(format!("Current addresses being used:"));
                results.push(format!("  FREECAM_INJECT_ADDR_1 = 0x{:08X}", FREECAM_INJECT_ADDR_1));
                results.push(format!("  FREECAM_INJECT_ADDR_2 = 0x{:08X}", FREECAM_INJECT_ADDR_2));
                results.push(format!(""));
                results.push(format!("Pattern 1 (7 bytes): {:02X?}", pattern1));
                results.push(format!("Found {} match(es):", pattern1_addrs.len()));
                for (i, addr) in pattern1_addrs.iter().enumerate() {
                    let marker = if *addr == FREECAM_INJECT_ADDR_1 { " <- CURRENT" } else { "" };
                    results.push(format!("  [{}] 0x{:08X}{}", i, addr, marker));
                }
                results.push("".to_string());
                
                results.push(format!("Pattern 2 (6 bytes): {:02X?}", pattern2));
                results.push(format!("Found {} match(es):", pattern2_addrs.len()));
                for (i, addr) in pattern2_addrs.iter().enumerate() {
                    let marker = if *addr == FREECAM_INJECT_ADDR_2 { " <- CURRENT" } else { "" };
                    results.push(format!("  [{}] 0x{:08X}{}", i, addr, marker));
                }
                results.push("".to_string());
            }
            results.push(format!("If hook not working, try different address index."));
            results.push(format!("Update FREECAM_INJECT_ADDR_1/2 in src/memory/ds1/mod.rs"));
            
            // Save to file
            use std::time::{SystemTime, UNIX_EPOCH};
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let filename = format!("freecam_scan_{}.txt", timestamp);
            
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&filename)
            {
                for line in &results {
                    let _ = writeln!(file, "{}", line);
                }
                let _ = file.flush();
                eprintln!("Scan results saved to: {}", filename);
            }
            
            Ok(format!("Scan complete! Saved to {}", filename))
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn scan_for_freecam_patterns(&self) -> Result<String, String> {
        Err("Pattern scanning not available on this platform".to_string())
    }
}

impl Ds1 {
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_input_debug(&self) -> (u32, u32, u32) {
        unsafe {
            // Replicate the pointer chain from the hook
            let edx = self.input_state.base_address as usize;
            if edx == 0 {
                return (0, 0, 0);
            }

            if !is_readable_addr(edx + 0x4, 4) {
                return (0, 0, 0);
            }
            let eax = std::ptr::read_volatile((edx + 0x4) as *const u32) as usize; // [edx+0x4]
            if eax == 0 {
                return (0, 0, 0);
            }

            if !is_readable_addr(eax + 0x28, 4) {
                return (0, 0, 0);
            }
            let esi = std::ptr::read_volatile((eax + 0x28) as *const u32) as usize; // [eax+0x28]
            if esi == 0 {
                return (0, 0, 0);
            }

            if !is_readable_addr(esi + 0x8, 8) {
                return (0, 0, 0);
            }
            let l3_raw = std::ptr::read_volatile((esi + 0x8) as *const u32); // [esi+0x8]
            let r3_raw = std::ptr::read_volatile((esi + 0xC) as *const u32); // [esi+0xC]
            (esi as u32, l3_raw, r3_raw)
        }
    }

    #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
    pub fn get_freecam_input_debug(&self) -> (u32, u32, u32) {
        (0, 0, 0)
    }
}
