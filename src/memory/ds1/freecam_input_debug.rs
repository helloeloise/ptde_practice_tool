use super::*;

impl Ds1 {
    #[cfg(all(target_os = "windows", target_arch = "x86"))]
    pub fn get_freecam_input_debug(&self) -> (u32, u32, u32) {
        unsafe {
            // Replicate the pointer chain from the hook.
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
