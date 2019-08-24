pub mod sr {
    pub mod pe {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005000u32 as *const u32) & 0x1
            }
        }

    }
    pub mod fe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005000u32 as *const u32) >> 1) & 0x1
            }
        }

    }
    pub mod ne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005000u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod ore {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005000u32 as *const u32) >> 3) & 0x1
            }
        }

    }
    pub mod idle {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005000u32 as *const u32) >> 4) & 0x1
            }
        }

    }
    pub mod rxne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005000u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40005000u32 as *mut u32, reg);
            }
        }
    }
    pub mod tc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005000u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005000u32 as *mut u32, reg);
            }
        }
    }
    pub mod txe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005000u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod lbd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005000u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr {
    pub mod dr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005004u32 as *const u32) & 0x1FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005004u32 as *const u32);
                reg &= 0xFFFFFE00u32;
                reg |= val & 0x1FF;
                core::ptr::write_volatile(0x40005004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod brr {
    pub mod div_fraction {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005008u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005008u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005008u32 as *mut u32, reg);
            }
        }
    }
    pub mod div_mantissa {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005008u32 as *const u32) >> 4) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005008u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= (val & 0xFFF) << 4;
                core::ptr::write_volatile(0x40005008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr1 {
    pub mod sbk {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000500Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rwu {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod re {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod te {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod idleie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rxneie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txeie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod peie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ps {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pce {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod wake {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ue {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000500Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000500Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod add {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005010u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005010u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005010u32 as *mut u32, reg);
            }
        }
    }
    pub mod lbdl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005010u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40005010u32 as *mut u32, reg);
            }
        }
    }
    pub mod lbdie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005010u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005010u32 as *mut u32, reg);
            }
        }
    }
    pub mod stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005010u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005010u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005010u32 as *mut u32, reg);
            }
        }
    }
    pub mod linen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005010u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr3 {
    pub mod eie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005014u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40005014u32 as *mut u32, reg);
            }
        }
    }
    pub mod iren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005014u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40005014u32 as *mut u32, reg);
            }
        }
    }
    pub mod irlp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005014u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40005014u32 as *mut u32, reg);
            }
        }
    }
    pub mod hdsel {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005014u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40005014u32 as *mut u32, reg);
            }
        }
    }
    pub mod dmat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005014u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005014u32 as *mut u32, reg);
            }
        }
    }
}

