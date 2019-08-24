pub mod fs_pcgcctl {
    pub mod stppclk {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000E00u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000E00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000E00u32 as *mut u32, reg);
            }
        }
    }
    pub mod gatehclk {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000E00u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000E00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000E00u32 as *mut u32, reg);
            }
        }
    }
    pub mod physusp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000E00u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000E00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000E00u32 as *mut u32, reg);
            }
        }
    }
}

