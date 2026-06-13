use std::sync::OnceLock;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_HOOK_ENABLED: bool = false;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_HOOK_ADDR: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_ORIG_BYTES: [u8; 7] = [0; 7];
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_TRAMPOLINE: usize = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_INJECT_BUTTONS: u16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_INJECT_ENABLED: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_CURRENT_BUTTONS: u16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_CALL_COUNT: u32 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_LAST_USER_INDEX: u32 = 0xFF;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) type XInputGetStateFn = unsafe extern "system" fn(u32, *mut super::XINPUT_STATE) -> u32;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static XINPUT_GET_STATE_FN: OnceLock<Option<XInputGetStateFn>> = OnceLock::new();

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_INJECT_LEFT_TRIGGER: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_INJECT_RIGHT_TRIGGER: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_INJECT_THUMB_LX: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_INJECT_THUMB_LY: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_INJECT_THUMB_RX: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_INJECT_THUMB_RY: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_CURRENT_LEFT_TRIGGER: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_CURRENT_RIGHT_TRIGGER: u8 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_CURRENT_THUMB_LX: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_CURRENT_THUMB_LY: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_CURRENT_THUMB_RX: i16 = 0;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) static mut XINPUT_CURRENT_THUMB_RY: i16 = 0;
