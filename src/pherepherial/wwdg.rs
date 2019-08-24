pub mod cr {
    pub mod t {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002C00u32 as *const u32) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002C00u32 as *const u32);
                reg &= 0xFFFFFF80u32;
                reg |= val & 0x7F;
                core::ptr::write_volatile(0x40002C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod wdga {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002C00u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002C00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40002C00u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cfr {
    pub mod w {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002C04u32 as *const u32) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002C04u32 as *const u32);
                reg &= 0xFFFFFF80u32;
                reg |= val & 0x7F;
                core::ptr::write_volatile(0x40002C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod wdgtb {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002C04u32 as *const u32) >> 7) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002C04u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 7;
                core::ptr::write_volatile(0x40002C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ewi {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002C04u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40002C04u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr {
    pub mod ewi {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002C08u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002C08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40002C08u32 as *mut u32, reg);
            }
        }
    }
}

