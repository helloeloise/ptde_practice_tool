pub mod constants;
pub mod offsets;
mod freecam_diagnostics;
mod freecam_hooks;
mod freecam_injection;
mod memory_access;
mod freecam_runtime;
mod freecam_scan_helpers;
mod freecam_state;
mod position_state;
mod refresh;
mod freecam_input_debug;
mod item_consumption;
mod player_flags;
mod world_flags;
mod xinput_glue;
mod xinput_hooks;
mod xinput_state;
mod xinput;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
use self::freecam_hooks::{freecam_hook_1, freecam_hook_2};
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use self::freecam_scan_helpers::{
    find_best_nearby_rel32_call_target, find_executable_pattern_matches, find_freecam_inject_pair,
    has_nearby_rel32_call_to_target, jmp_patch, resolve_hook2_skip_target,
};
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use self::freecam_state::*;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use self::memory_access::{
    is_executable_addr, is_readable_addr, is_writable_addr, write_exec_patch,
};
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use self::xinput_hooks::xinput_hook;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use self::xinput_state::*;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use self::xinput_glue::{
    apply_translation_deltas, key_down, normalize_stick_axis, xinput_get_state,
};

use crate::memory::ds1::constants::*;
use mem_rs::prelude::*;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use std::ffi::c_void;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE, PAGE_EXECUTE_READ,
    PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS, PAGE_READWRITE,
    PAGE_WRITECOPY, VirtualProtect, VirtualQuery, VirtualAlloc, VirtualFree, MEM_RESERVE,
};
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA};
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use windows_sys::Win32::UI::Input::XboxController::{
    XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB, XINPUT_GAMEPAD_RIGHT_SHOULDER,
    XINPUT_GAMEPAD_RIGHT_THUMB, XINPUT_GAMEPAD_A, XINPUT_STATE,
};
#[cfg(all(target_os = "windows", target_arch = "x86"))]
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_CONTROL, VK_DOWN, VK_LEFT, VK_RIGHT, VK_SHIFT, VK_SPACE, VK_UP,
};
use std::fs::OpenOptions;
use std::io::Write;

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
}

