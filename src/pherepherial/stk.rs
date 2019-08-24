pub mod ctrl {
    pub mod enable {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE000E010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tickint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E010u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE000E010u32 as *mut u32, reg);
            }
        }
    }
    pub mod clksource {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E010u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xE000E010u32 as *mut u32, reg);
            }
        }
    }
    pub mod countflag {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E010u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0xE000E010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod load_ {
    pub mod reload {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E014u32 as *const u32) & 0xFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E014u32 as *const u32);
                reg &= 0xFF000000u32;
                reg |= val & 0xFFFFFF;
                core::ptr::write_volatile(0xE000E014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod val {
    pub mod current {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E018u32 as *const u32) & 0xFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E018u32 as *const u32);
                reg &= 0xFF000000u32;
                reg |= val & 0xFFFFFF;
                core::ptr::write_volatile(0xE000E018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod calib {
    pub mod tenms {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E01Cu32 as *const u32) & 0xFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E01Cu32 as *const u32);
                reg &= 0xFF000000u32;
                reg |= val & 0xFFFFFF;
                core::ptr::write_volatile(0xE000E01Cu32 as *mut u32, reg);
            }
        }
    }
}

