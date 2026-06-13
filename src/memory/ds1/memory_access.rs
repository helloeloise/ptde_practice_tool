use super::*;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) fn is_readable_addr(addr: usize, len: usize) -> bool {
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
pub(super) fn is_executable_addr(addr: usize, len: usize) -> bool {
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
pub(super) fn is_writable_addr(addr: usize, len: usize) -> bool {
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
pub(super) fn write_exec_patch(addr: usize, patch: &[u8]) -> bool {
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
