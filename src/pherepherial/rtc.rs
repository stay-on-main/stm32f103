pub mod crh {
    pub mod secie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002800u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40002800u32 as *mut u32, reg);
            }
        }
    }
    pub mod alrie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002800u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40002800u32 as *mut u32, reg);
            }
        }
    }
    pub mod owie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002800u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40002800u32 as *mut u32, reg);
            }
        }
    }
}

pub mod crl {
    pub mod secf {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002804u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40002804u32 as *mut u32, reg);
            }
        }
    }
    pub mod alrf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002804u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40002804u32 as *mut u32, reg);
            }
        }
    }
    pub mod owf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002804u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40002804u32 as *mut u32, reg);
            }
        }
    }
    pub mod rsf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002804u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40002804u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002804u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40002804u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtoff {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40002804u32 as *const u32) >> 5) & 0x1
            }
        }

    }
}

pub mod prlh {
    pub mod prlh {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002808u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002808u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40002808u32 as *mut u32, reg);
            }
        }
    }
}

pub mod prll {
    pub mod prll {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000280Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000280Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4000280Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod divh {
    pub mod divh {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002810u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002810u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40002810u32 as *mut u32, reg);
            }
        }
    }
}

pub mod divl {
    pub mod divl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002814u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002814u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40002814u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cnth {
    pub mod cnth {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002818u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002818u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40002818u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cntl {
    pub mod cntl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000281Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000281Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4000281Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod alrh {
    pub mod alrh {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002820u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002820u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40002820u32 as *mut u32, reg);
            }
        }
    }
}

pub mod alrl {
    pub mod alrl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40002824u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40002824u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40002824u32 as *mut u32, reg);
            }
        }
    }
}

