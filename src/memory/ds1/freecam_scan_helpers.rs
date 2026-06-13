use super::*;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub(super) fn has_nearby_rel32_call_to_target(
    anchor: usize,
    target: usize,
    range_before: usize,
    range_after: usize,
) -> bool {
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
pub(super) fn find_best_nearby_rel32_call_target(
    anchor: usize,
    expected: usize,
    range_before: usize,
    range_after: usize,
) -> Option<usize> {
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
pub(super) fn resolve_hook2_skip_target(inject2: usize) -> usize {
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
pub(super) fn find_executable_pattern_matches(pattern: &[u8], start: usize, max_addr: usize) -> Vec<usize> {
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
pub(super) fn find_freecam_inject_pair() -> (Option<(usize, usize, usize, usize, u8)>, usize, usize) {
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
                    best = Some((
                        *a1,
                        *a2,
                        delta,
                        delta_err,
                        if candidate_call_ref { 1 } else { 0 },
                        score,
                    ));
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
pub(super) fn jmp_patch(from: usize, to: usize, total_len: usize) -> Option<Vec<u8>> {
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
