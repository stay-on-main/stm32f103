pub mod acr {
    pub mod latency {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40022000u32 as *const u32) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022000u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= val & 0x7;
                core::ptr::write_volatile(0x40022000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hlfcya {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022000u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40022000u32 as *mut u32, reg);
            }
        }
    }
    pub mod prftbe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022000u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40022000u32 as *mut u32, reg);
            }
        }
    }
    pub mod prftbs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022000u32 as *const u32) >> 5) & 0x1
            }
        }

    }
}

pub mod keyr {
    pub mod key {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40022004u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022004u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40022004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod optkeyr {
    pub mod optkey {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40022008u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022008u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40022008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr {
    pub mod eop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002200Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002200Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002200Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod wrprterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002200Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002200Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002200Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pgerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002200Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002200Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002200Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bsy {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002200Cu32 as *const u32) & 0x1
            }
        }

    }
}

pub mod cr {
    pub mod pg {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40022010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod per {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod mer {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod optpg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod opter {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod strt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod lock {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod optwre {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod errie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
    pub mod eopie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40022010u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40022010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ar {
    pub mod far {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40022014u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022014u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40022014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod obr {
    pub mod opterr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002201Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002201Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002201Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rdprt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002201Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002201Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4002201Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod wdg_sw {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002201Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002201Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002201Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nrst_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002201Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002201Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002201Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nrst_stdby {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002201Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002201Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002201Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002201Cu32 as *const u32) >> 10) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002201Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 10;
                core::ptr::write_volatile(0x4002201Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002201Cu32 as *const u32) >> 18) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002201Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 18;
                core::ptr::write_volatile(0x4002201Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod wrpr {
    pub mod wrp {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40022020u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40022020u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40022020u32 as *mut u32, reg);
            }
        }
    }
}

