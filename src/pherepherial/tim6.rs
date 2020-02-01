pub mod cr1 {
    pub mod arpe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40001000u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001000u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40001000u32 as *mut u32, reg);
            }
        }
    }
    pub mod opm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40001000u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001000u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40001000u32 as *mut u32, reg);
            }
        }
    }
    pub mod urs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40001000u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001000u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40001000u32 as *mut u32, reg);
            }
        }
    }
    pub mod udis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40001000u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001000u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40001000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cen {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40001000u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40001000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod mms {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40001004u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001004u32 as *const u32);
                reg &= 0xFFFFFF8Fu32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40001004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dier {
    pub mod ude {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000100Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000100Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod uie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000100Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000100Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr {
    pub mod uif {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40001010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40001010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod egr {
    pub mod ug {
        pub fn set(val: u32) {
            unsafe {
                let reg = val & 0x1;
                core::ptr::write_volatile(0x40001014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cnt {
    pub mod cnt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40001024u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001024u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40001024u32 as *mut u32, reg);
            }
        }
    }
}

pub mod psc {
    pub mod psc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40001028u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40001028u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40001028u32 as *mut u32, reg);
            }
        }
    }
}

pub mod arr {
    pub mod arr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000102Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000102Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4000102Cu32 as *mut u32, reg);
            }
        }
    }
}

