pub mod mmccr {
    pub mod cr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028100u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028100u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40028100u32 as *mut u32, reg);
            }
        }
    }
    pub mod csr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028100u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028100u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40028100u32 as *mut u32, reg);
            }
        }
    }
    pub mod ror {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028100u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028100u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40028100u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028100u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028100u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40028100u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmcrir {
    pub mod rfces {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028104u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028104u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40028104u32 as *mut u32, reg);
            }
        }
    }
    pub mod rfaes {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028104u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028104u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40028104u32 as *mut u32, reg);
            }
        }
    }
    pub mod rgufs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028104u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028104u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40028104u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmctir {
    pub mod tgfscs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028108u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028108u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40028108u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgfmscs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028108u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028108u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40028108u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgfs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028108u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028108u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40028108u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmcrimr {
    pub mod rfcem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002810Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002810Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002810Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rfaem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002810Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002810Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002810Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rgufm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002810Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002810Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4002810Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmctimr {
    pub mod tgfscm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028110u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028110u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40028110u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgfmscm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028110u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028110u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40028110u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgfm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028110u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028110u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40028110u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmctgfsccr {
    pub mod tgfscc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002814Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002814Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002814Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmctgfmsccr {
    pub mod tgfmscc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028150u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028150u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028150u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmctgfcr {
    pub mod tgfc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028168u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028168u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028168u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmcrfcecr {
    pub mod rfcfc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028194u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028194u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028194u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmcrfaecr {
    pub mod rfaec {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028198u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028198u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028198u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmcrgufcr {
    pub mod rgufc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400281C4u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400281C4u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x400281C4u32 as *mut u32, reg);
            }
        }
    }
}

