pub mod kr {
    pub mod key {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003000u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003000u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40003000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod pr {
    pub mod pr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003004u32 as *const u32) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003004u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= val & 0x7;
                core::ptr::write_volatile(0x40003004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod rlr {
    pub mod rl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003008u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003008u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40003008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr {
    pub mod pvu {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000300Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000300Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000300Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rvu {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000300Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000300Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000300Cu32 as *mut u32, reg);
            }
        }
    }
}

