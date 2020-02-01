pub mod can_mcr {
    pub mod dbf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod reset {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod ttcm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod abom {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod awum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod nart {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod rflm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod sleep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006400u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
    pub mod inrq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006400u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006400u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_msr {
    pub mod rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006404u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
    pub mod samp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006404u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006404u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
    pub mod txm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006404u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
    pub mod slaki {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006404u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
    pub mod wkui {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006404u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
    pub mod erri {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006404u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
    pub mod slak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006404u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
    pub mod inak {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006404u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006404u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006404u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tsr {
    pub mod low2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod low1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod low0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tme2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tme1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tme0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod code {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 24) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFCFFFFFFu32;
                reg |= (val & 0x3) << 24;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod abrq2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod terr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod alst2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod txok2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod rqcp2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod abrq1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod terr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod alst1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod txok1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod rqcp1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod abrq0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod terr0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod alst0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod txok0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006408u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
    pub mod rqcp0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006408u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006408u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006408u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rf0r {
    pub mod rfom0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000640Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000640Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000640Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fovr0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000640Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000640Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000640Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod full0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000640Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000640Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000640Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fmp0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000640Cu32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000640Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x4000640Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rf1r {
    pub mod rfom1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006410u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006410u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006410u32 as *mut u32, reg);
            }
        }
    }
    pub mod fovr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006410u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006410u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006410u32 as *mut u32, reg);
            }
        }
    }
    pub mod full1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006410u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006410u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006410u32 as *mut u32, reg);
            }
        }
    }
    pub mod fmp1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006410u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006410u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40006410u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ier {
    pub mod slkie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod wkuie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod errie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod lecie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod bofie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod epvie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod ewgie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod fovie1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffie1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod fmpie1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod fovie0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffie0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod fmpie0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006414u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
    pub mod tmeie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006414u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006414u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006414u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_esr {
    pub mod rec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006418u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006418u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x40006418u32 as *mut u32, reg);
            }
        }
    }
    pub mod tec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006418u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006418u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x40006418u32 as *mut u32, reg);
            }
        }
    }
    pub mod lec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006418u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006418u32 as *const u32);
                reg &= 0xFFFFFF8Fu32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40006418u32 as *mut u32, reg);
            }
        }
    }
    pub mod boff {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006418u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006418u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006418u32 as *mut u32, reg);
            }
        }
    }
    pub mod epvf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006418u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006418u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006418u32 as *mut u32, reg);
            }
        }
    }
    pub mod ewgf {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006418u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006418u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006418u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_btr {
    pub mod silm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000641Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000641Cu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4000641Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod lbkm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000641Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000641Cu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x4000641Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sjw {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000641Cu32 as *const u32) >> 24) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000641Cu32 as *const u32);
                reg &= 0xFCFFFFFFu32;
                reg |= (val & 0x3) << 24;
                core::ptr::write_volatile(0x4000641Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ts2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000641Cu32 as *const u32) >> 20) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000641Cu32 as *const u32);
                reg &= 0xFF8FFFFFu32;
                reg |= (val & 0x7) << 20;
                core::ptr::write_volatile(0x4000641Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ts1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000641Cu32 as *const u32) >> 16) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000641Cu32 as *const u32);
                reg &= 0xFFF0FFFFu32;
                reg |= (val & 0xF) << 16;
                core::ptr::write_volatile(0x4000641Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod brp {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000641Cu32 as *const u32) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000641Cu32 as *const u32);
                reg &= 0xFFFFFC00u32;
                reg |= val & 0x3FF;
                core::ptr::write_volatile(0x4000641Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ti0r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006580u32 as *const u32) >> 21) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006580u32 as *const u32);
                reg &= 0x1FFFFFu32;
                reg |= (val & 0x7FF) << 21;
                core::ptr::write_volatile(0x40006580u32 as *mut u32, reg);
            }
        }
    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006580u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006580u32 as *const u32);
                reg &= 0xFFE00007u32;
                reg |= (val & 0x3FFFF) << 3;
                core::ptr::write_volatile(0x40006580u32 as *mut u32, reg);
            }
        }
    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006580u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006580u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006580u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006580u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006580u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006580u32 as *mut u32, reg);
            }
        }
    }
    pub mod txrq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006580u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006580u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006580u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdt0r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006584u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006584u32 as *const u32);
                reg &= 0xFFFFu32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x40006584u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006584u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006584u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006584u32 as *mut u32, reg);
            }
        }
    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006584u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006584u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40006584u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdl0r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006588u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006588u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x40006588u32 as *mut u32, reg);
            }
        }
    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006588u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006588u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x40006588u32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006588u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006588u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x40006588u32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006588u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006588u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40006588u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdh0r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000658Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000658Cu32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x4000658Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000658Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000658Cu32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x4000658Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000658Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000658Cu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x4000658Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000658Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000658Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x4000658Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ti1r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006590u32 as *const u32) >> 21) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006590u32 as *const u32);
                reg &= 0x1FFFFFu32;
                reg |= (val & 0x7FF) << 21;
                core::ptr::write_volatile(0x40006590u32 as *mut u32, reg);
            }
        }
    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006590u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006590u32 as *const u32);
                reg &= 0xFFE00007u32;
                reg |= (val & 0x3FFFF) << 3;
                core::ptr::write_volatile(0x40006590u32 as *mut u32, reg);
            }
        }
    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006590u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006590u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006590u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006590u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006590u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006590u32 as *mut u32, reg);
            }
        }
    }
    pub mod txrq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006590u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006590u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006590u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdt1r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006594u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006594u32 as *const u32);
                reg &= 0xFFFFu32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x40006594u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006594u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006594u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006594u32 as *mut u32, reg);
            }
        }
    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006594u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006594u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40006594u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdl1r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006598u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006598u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x40006598u32 as *mut u32, reg);
            }
        }
    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006598u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006598u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x40006598u32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006598u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006598u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x40006598u32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006598u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006598u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40006598u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdh1r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000659Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000659Cu32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x4000659Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000659Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000659Cu32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x4000659Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000659Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000659Cu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x4000659Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000659Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000659Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x4000659Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ti2r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A0u32 as *const u32) >> 21) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A0u32 as *const u32);
                reg &= 0x1FFFFFu32;
                reg |= (val & 0x7FF) << 21;
                core::ptr::write_volatile(0x400065A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A0u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A0u32 as *const u32);
                reg &= 0xFFE00007u32;
                reg |= (val & 0x3FFFF) << 3;
                core::ptr::write_volatile(0x400065A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A0u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A0u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x400065A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A0u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A0u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x400065A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod txrq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065A0u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x400065A0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdt2r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A4u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A4u32 as *const u32);
                reg &= 0xFFFFu32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x400065A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A4u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A4u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x400065A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065A4u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A4u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x400065A4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdl2r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A8u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A8u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x400065A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A8u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A8u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x400065A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065A8u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A8u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400065A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065A8u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065A8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x400065A8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdh2r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065ACu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065ACu32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x400065ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065ACu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065ACu32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x400065ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065ACu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065ACu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400065ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065ACu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400065ACu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x400065ACu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ri0r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B0u32 as *const u32) >> 21) & 0x7FF
            }
        }

    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B0u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B0u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B0u32 as *const u32) >> 1) & 0x1
            }
        }

    }
}

pub mod can_rdt0r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B4u32 as *const u32) >> 16) & 0xFFFF
            }
        }

    }
    pub mod fmi {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B4u32 as *const u32) >> 8) & 0xFF
            }
        }

    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065B4u32 as *const u32) & 0xF
            }
        }

    }
}

pub mod can_rdl0r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B8u32 as *const u32) >> 24) & 0xFF
            }
        }

    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B8u32 as *const u32) >> 16) & 0xFF
            }
        }

    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065B8u32 as *const u32) >> 8) & 0xFF
            }
        }

    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065B8u32 as *const u32) & 0xFF
            }
        }

    }
}

pub mod can_rdh0r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065BCu32 as *const u32) >> 24) & 0xFF
            }
        }

    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065BCu32 as *const u32) >> 16) & 0xFF
            }
        }

    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065BCu32 as *const u32) >> 8) & 0xFF
            }
        }

    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065BCu32 as *const u32) & 0xFF
            }
        }

    }
}

pub mod can_ri1r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C0u32 as *const u32) >> 21) & 0x7FF
            }
        }

    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C0u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C0u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C0u32 as *const u32) >> 1) & 0x1
            }
        }

    }
}

pub mod can_rdt1r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C4u32 as *const u32) >> 16) & 0xFFFF
            }
        }

    }
    pub mod fmi {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C4u32 as *const u32) >> 8) & 0xFF
            }
        }

    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065C4u32 as *const u32) & 0xF
            }
        }

    }
}

pub mod can_rdl1r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C8u32 as *const u32) >> 24) & 0xFF
            }
        }

    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C8u32 as *const u32) >> 16) & 0xFF
            }
        }

    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065C8u32 as *const u32) >> 8) & 0xFF
            }
        }

    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065C8u32 as *const u32) & 0xFF
            }
        }

    }
}

pub mod can_rdh1r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065CCu32 as *const u32) >> 24) & 0xFF
            }
        }

    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065CCu32 as *const u32) >> 16) & 0xFF
            }
        }

    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400065CCu32 as *const u32) >> 8) & 0xFF
            }
        }

    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400065CCu32 as *const u32) & 0xFF
            }
        }

    }
}

pub mod can_fmr {
    pub mod finit {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006600u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006600u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006600u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_fm1r {
    pub mod fbm0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006604u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006604u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006604u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006604u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_fs1r {
    pub mod fsc0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000660Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000660Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000660Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000660Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ffa1r {
    pub mod ffa0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006614u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006614u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006614u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006614u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_fa1r {
    pub mod fact0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000661Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000661Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000661Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000661Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f0r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006640u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006640u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006640u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006640u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f0r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006644u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006644u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006644u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006644u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f1r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006648u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006648u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006648u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006648u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f1r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000664Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000664Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000664Cu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4000664Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f2r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006650u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006650u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006650u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006650u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f2r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006654u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006654u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006654u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006654u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f3r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006658u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006658u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006658u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006658u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f3r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000665Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000665Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000665Cu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4000665Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f4r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006660u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006660u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006660u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006660u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f4r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006664u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006664u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006664u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006664u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f5r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006668u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006668u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006668u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006668u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f5r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000666Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000666Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000666Cu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4000666Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f6r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006670u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006670u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006670u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006670u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f6r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006674u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006674u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006674u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006674u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f7r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006678u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006678u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006678u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006678u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f7r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000667Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000667Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000667Cu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4000667Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f8r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006680u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006680u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006680u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006680u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f8r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006684u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006684u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006684u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006684u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f9r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006688u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006688u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006688u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006688u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f9r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000668Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000668Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000668Cu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4000668Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f10r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006690u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006690u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006690u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006690u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f10r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006694u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006694u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006694u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006694u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f11r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006698u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006698u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006698u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006698u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f11r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000669Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000669Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000669Cu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4000669Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f12r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400066A0u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A0u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A0u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x400066A0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f12r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400066A4u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A4u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A4u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x400066A4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f13r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400066A8u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066A8u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066A8u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x400066A8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f13r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400066ACu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400066ACu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400066ACu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x400066ACu32 as *mut u32, reg);
            }
        }
    }
}

