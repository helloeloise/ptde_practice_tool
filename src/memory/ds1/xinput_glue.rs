use super::*;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) fn normalize_stick_axis(raw: i16, deadzone: i16) -> f32 {
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
fn resolve_xinput_get_state() -> Option<XInputGetStateFn> {
    *XINPUT_GET_STATE_FN.get_or_init(|| unsafe {
        let func_name = b"XInputGetState\0";
        let module_candidates: [&[u8]; 4] = [
            b"xinput1_4.dll\0",
            b"xinput1_3.dll\0",
            b"xinput9_1_0.dll\0",
            b"xinput1_2.dll\0",
        ];

        for module_name in module_candidates {
            let mut module = GetModuleHandleA(module_name.as_ptr());
            if module == 0 {
                module = LoadLibraryA(module_name.as_ptr());
            }
            if module == 0 {
                continue;
            }

            if let Some(addr) = GetProcAddress(module, func_name.as_ptr()) {
                let f: XInputGetStateFn = std::mem::transmute(addr as usize);
                return Some(f);
            }
        }

        None
    })
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) fn xinput_get_state(user_idx: u32, state: &mut XINPUT_STATE) -> u32 {
    match resolve_xinput_get_state() {
        Some(f) => unsafe { f(user_idx, state as *mut XINPUT_STATE) },
        None => 0x48F, // ERROR_DEVICE_NOT_CONNECTED
    }
}

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) fn apply_translation_deltas(buf: usize, dx: f32, dy: f32, dz: f32) -> bool {
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
pub(super) fn key_down(vk: i32) -> bool {
    unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
}
