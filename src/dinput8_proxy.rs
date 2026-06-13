#![cfg(windows)]

use std::ffi::{CString, c_void};
use std::path::PathBuf;
use std::ptr;
use std::sync::{Once, OnceLock};

use windows_sys::Win32::Foundation::HMODULE;
use windows_sys::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows_sys::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, VirtualProtect};

const E_FAIL: i32 = 0x8000_4005u32 as i32;

type DirectInput8CreateFn = unsafe extern "system" fn(
    hinst: isize,
    version: u32,
    riidltf: *const c_void,
    out: *mut *mut c_void,
    punk_outer: *mut c_void,
) -> i32;

type DllCanUnloadNowFn = unsafe extern "system" fn() -> i32;
type DllGetClassObjectFn = unsafe extern "system" fn(
    rclsid: *const c_void,
    riid: *const c_void,
    ppv: *mut *mut c_void,
) -> i32;
type DllRegisterServerFn = unsafe extern "system" fn() -> i32;
type DllUnregisterServerFn = unsafe extern "system" fn() -> i32;
type GetdfDIJoystickFn = unsafe extern "system" fn() -> *const c_void;

static REAL_DINPUT8: OnceLock<HMODULE> = OnceLock::new();
static CHAINLOAD_ONCE: Once = Once::new();
static NO_LOGO_PATCH_ONCE: Once = Once::new();

struct BytePatch {
    rel_addr: usize,
    orig: &'static [u8],
    patch: &'static [u8],
}

const NO_LOGO_PATCHES: &[BytePatch] = &[
    // release
    BytePatch {
        rel_addr: 0x8320b0,
        orig: &[0xff, 0x24, 0x85, 0x24, 0x22, 0xc3, 0x00],
        patch: &[0xe9, 0x27, 0x01, 0x00, 0x00, 0x90, 0x90],
    },
    BytePatch {
        rel_addr: 0x8322b3,
        orig: &[0x74, 0x0d],
        patch: &[0x90, 0x90],
    },
    // debug
    BytePatch {
        rel_addr: 0x831b30,
        orig: &[0xff, 0x24, 0x85, 0xa4, 0x1c, 0xc3, 0x00],
        patch: &[0xe9, 0x27, 0x01, 0x00, 0x00, 0x90, 0x90],
    },
    BytePatch {
        rel_addr: 0x831d33,
        orig: &[0x74, 0x0d],
        patch: &[0x90, 0x90],
    },
    // steamworks
    BytePatch {
        rel_addr: 0x831180,
        orig: &[0xff, 0x24, 0x85, 0xf4, 0x12, 0xc3, 0x00],
        patch: &[0xe9, 0x27, 0x01, 0x00, 0x00, 0x90, 0x90],
    },
    BytePatch {
        rel_addr: 0x831383,
        orig: &[0x74, 0x0d],
        patch: &[0x90, 0x90],
    },
];

fn to_wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn get_system_dinput8_path() -> Option<String> {
    let windows_dir = std::env::var("WINDIR")
        .or_else(|_| std::env::var("SystemRoot"))
        .ok()?;

    let mut path = PathBuf::from(windows_dir);
    path.push("System32");
    path.push("dinput8.dll");
    Some(path.to_string_lossy().to_string())
}

fn real_module() -> Option<HMODULE> {
    if let Some(h) = REAL_DINPUT8.get().copied() {
        return Some(h);
    }

    let path = get_system_dinput8_path()?;
    let wide = to_wide_null(&path);
    let module = unsafe { LoadLibraryW(wide.as_ptr()) };
    if module == 0 {
        return None;
    }

    let _ = REAL_DINPUT8.set(module);
    Some(module)
}

unsafe fn resolve_fn<T>(name: &str) -> Option<T>
where
    T: Copy,
{
    let module = real_module()?;
    let c_name = CString::new(name).ok()?;
    let addr = GetProcAddress(module, c_name.as_ptr() as *const u8)?;
    Some(std::mem::transmute_copy(&addr))
}

fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn is_steam_dll(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.starts_with("steam")
        || lower == "csteamworks.dll"
        || lower == "sdkencryptedappticket.dll"
}

fn apply_patch_if_match(addr: usize, orig: &[u8], patch: &[u8]) -> bool {
    if orig.len() != patch.len() || orig.is_empty() {
        return false;
    }

    unsafe {
        let current = std::slice::from_raw_parts(addr as *const u8, orig.len());

        if current == patch {
            return true;
        }

        if current != orig {
            return false;
        }

        let mut old_protect = 0u32;
        if VirtualProtect(
            addr as *const c_void,
            patch.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) == 0
        {
            return false;
        }

        ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, patch.len());

        let mut restored = 0u32;
        let _ = VirtualProtect(
            addr as *const c_void,
            patch.len(),
            old_protect,
            &mut restored,
        );

        true
    }
}

fn apply_no_logo_patches_once() {
    NO_LOGO_PATCH_ONCE.call_once(|| {
        let base = 0x0040_0000usize;
        for p in NO_LOGO_PATCHES {
            let addr = base + p.rel_addr;
            let _ = apply_patch_if_match(addr, p.orig, p.patch);
        }
    });
}

fn chainload_other_dlls_once() {
    CHAINLOAD_ONCE.call_once(|| {
        apply_no_logo_patches_once();

        let dir = exe_dir();
        let entries = match std::fs::read_dir(&dir) {
            Ok(v) => v,
            Err(_) => return,
        };

        let mut dlls = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n.to_owned(),
                None => continue,
            };

            let is_dll = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("dll"))
                .unwrap_or(false);
            if !is_dll {
                continue;
            }

            if file_name.eq_ignore_ascii_case("dinput8.dll") {
                continue;
            }

            if is_steam_dll(&file_name) {
                continue;
            }

            dlls.push(path);
        }

        // Keep load order deterministic for easier debugging.
        dlls.sort();

        for dll in dlls {
            if let Some(path_str) = dll.to_str() {
                let wide = to_wide_null(path_str);
                unsafe {
                    let _ = LoadLibraryW(wide.as_ptr());
                }
            }
        }
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DirectInput8Create(
    hinst: isize,
    version: u32,
    riidltf: *const c_void,
    out: *mut *mut c_void,
    punk_outer: *mut c_void,
) -> i32 {
    chainload_other_dlls_once();
    let f: DirectInput8CreateFn = match resolve_fn("DirectInput8Create") {
        Some(v) => v,
        None => return E_FAIL,
    };
    f(hinst, version, riidltf, out, punk_outer)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllCanUnloadNow() -> i32 {
    chainload_other_dlls_once();
    let f: DllCanUnloadNowFn = match resolve_fn("DllCanUnloadNow") {
        Some(v) => v,
        None => return E_FAIL,
    };
    f()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllGetClassObject(
    rclsid: *const c_void,
    riid: *const c_void,
    ppv: *mut *mut c_void,
) -> i32 {
    chainload_other_dlls_once();
    let f: DllGetClassObjectFn = match resolve_fn("DllGetClassObject") {
        Some(v) => v,
        None => return E_FAIL,
    };
    f(rclsid, riid, ppv)
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllRegisterServer() -> i32 {
    chainload_other_dlls_once();
    let f: DllRegisterServerFn = match resolve_fn("DllRegisterServer") {
        Some(v) => v,
        None => return E_FAIL,
    };
    f()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllUnregisterServer() -> i32 {
    chainload_other_dlls_once();
    let f: DllUnregisterServerFn = match resolve_fn("DllUnregisterServer") {
        Some(v) => v,
        None => return E_FAIL,
    };
    f()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetdfDIJoystick() -> *const c_void {
    chainload_other_dlls_once();
    let f: GetdfDIJoystickFn = match resolve_fn("GetdfDIJoystick") {
        Some(v) => v,
        None => return ptr::null(),
    };
    f()
}
