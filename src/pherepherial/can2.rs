pub mod can_mcr {
    pub mod dbf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod reset {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod ttcm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod abom {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod awum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod nart {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod rflm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod sleep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006800u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
    pub mod inrq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006800u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006800u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_msr {
    pub mod rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006804u32 as *const u32) >> 11) & 0x1
            }
        }

    }
    pub mod samp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006804u32 as *const u32) >> 10) & 0x1
            }
        }

    }
    pub mod rxm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006804u32 as *const u32) >> 9) & 0x1
            }
        }

    }
    pub mod txm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006804u32 as *const u32) >> 8) & 0x1
            }
        }

    }
    pub mod slaki {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006804u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006804u32 as *mut u32, reg);
            }
        }
    }
    pub mod wkui {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006804u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006804u32 as *mut u32, reg);
            }
        }
    }
    pub mod erri {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006804u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006804u32 as *mut u32, reg);
            }
        }
    }
    pub mod slak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006804u32 as *const u32) >> 1) & 0x1
            }
        }

    }
    pub mod inak {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006804u32 as *const u32) & 0x1
            }
        }

    }
}

pub mod can_tsr {
    pub mod low2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 31) & 0x1
            }
        }

    }
    pub mod low1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 30) & 0x1
            }
        }

    }
    pub mod low0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 29) & 0x1
            }
        }

    }
    pub mod tme2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 28) & 0x1
            }
        }

    }
    pub mod tme1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 27) & 0x1
            }
        }

    }
    pub mod tme0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 26) & 0x1
            }
        }

    }
    pub mod code {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 24) & 0x3
            }
        }

    }
    pub mod abrq2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod terr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod alst2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod txok2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod rqcp2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod abrq1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod terr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod alst1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod txok1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod rqcp1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod abrq0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod terr0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod alst0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod txok0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006808u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
    pub mod rqcp0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006808u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006808u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rf0r {
    pub mod rfom0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000680Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000680Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000680Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fovr0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000680Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000680Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000680Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod full0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000680Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000680Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000680Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fmp0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000680Cu32 as *const u32) & 0x3
            }
        }

    }
}

pub mod can_rf1r {
    pub mod rfom1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006810u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006810u32 as *mut u32, reg);
            }
        }
    }
    pub mod fovr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006810u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006810u32 as *mut u32, reg);
            }
        }
    }
    pub mod full1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006810u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006810u32 as *mut u32, reg);
            }
        }
    }
    pub mod fmp1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006810u32 as *const u32) & 0x3
            }
        }

    }
}

pub mod can_ier {
    pub mod slkie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod wkuie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod errie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod lecie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod bofie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod epvie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod ewgie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod fovie1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffie1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod fmpie1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod fovie0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffie0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod fmpie0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006814u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
    pub mod tmeie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006814u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006814u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_esr {
    pub mod rec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006818u32 as *const u32) >> 24) & 0xFF
            }
        }

    }
    pub mod tec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006818u32 as *const u32) >> 16) & 0xFF
            }
        }

    }
    pub mod lec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006818u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006818u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40006818u32 as *mut u32, reg);
            }
        }
    }
    pub mod boff {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006818u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod epvf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006818u32 as *const u32) >> 1) & 0x1
            }
        }

    }
    pub mod ewgf {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006818u32 as *const u32) & 0x1
            }
        }

    }
}

pub mod can_btr {
    pub mod silm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000681Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000681Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4000681Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod lbkm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000681Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000681Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x4000681Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sjw {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000681Cu32 as *const u32) >> 24) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000681Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 24;
                core::ptr::write_volatile(0x4000681Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ts2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000681Cu32 as *const u32) >> 20) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000681Cu32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 20;
                core::ptr::write_volatile(0x4000681Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ts1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000681Cu32 as *const u32) >> 16) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000681Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 16;
                core::ptr::write_volatile(0x4000681Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod brp {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000681Cu32 as *const u32) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000681Cu32 as *const u32);
                reg &= 0xFFFFFC00u32;
                reg |= val & 0x3FF;
                core::ptr::write_volatile(0x4000681Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ti0r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006980u32 as *const u32) >> 21) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006980u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= (val & 0x7FF) << 21;
                core::ptr::write_volatile(0x40006980u32 as *mut u32, reg);
            }
        }
    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006980u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006980u32 as *const u32);
                reg &= 0xFFFC0000u32;
                reg |= (val & 0x3FFFF) << 3;
                core::ptr::write_volatile(0x40006980u32 as *mut u32, reg);
            }
        }
    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006980u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006980u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006980u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006980u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006980u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006980u32 as *mut u32, reg);
            }
        }
    }
    pub mod txrq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006980u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006980u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006980u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdt0r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006984u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006984u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x40006984u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006984u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006984u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006984u32 as *mut u32, reg);
            }
        }
    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006984u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006984u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40006984u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdl0r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006988u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006988u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x40006988u32 as *mut u32, reg);
            }
        }
    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006988u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006988u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x40006988u32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006988u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006988u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x40006988u32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006988u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006988u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40006988u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdh0r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000698Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000698Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x4000698Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000698Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000698Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x4000698Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000698Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000698Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x4000698Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000698Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000698Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x4000698Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ti1r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006990u32 as *const u32) >> 21) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006990u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= (val & 0x7FF) << 21;
                core::ptr::write_volatile(0x40006990u32 as *mut u32, reg);
            }
        }
    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006990u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006990u32 as *const u32);
                reg &= 0xFFFC0000u32;
                reg |= (val & 0x3FFFF) << 3;
                core::ptr::write_volatile(0x40006990u32 as *mut u32, reg);
            }
        }
    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006990u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006990u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006990u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006990u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006990u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006990u32 as *mut u32, reg);
            }
        }
    }
    pub mod txrq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006990u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006990u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006990u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdt1r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006994u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006994u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x40006994u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006994u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006994u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006994u32 as *mut u32, reg);
            }
        }
    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006994u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006994u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40006994u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdl1r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006998u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006998u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x40006998u32 as *mut u32, reg);
            }
        }
    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006998u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006998u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x40006998u32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006998u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006998u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x40006998u32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006998u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006998u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40006998u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdh1r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000699Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000699Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x4000699Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000699Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000699Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x4000699Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000699Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000699Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x4000699Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000699Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000699Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x4000699Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ti2r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A0u32 as *const u32) >> 21) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A0u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= (val & 0x7FF) << 21;
                core::ptr::write_volatile(0x400069A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A0u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A0u32 as *const u32);
                reg &= 0xFFFC0000u32;
                reg |= (val & 0x3FFFF) << 3;
                core::ptr::write_volatile(0x400069A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A0u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x400069A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A0u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x400069A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod txrq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069A0u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x400069A0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdt2r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A4u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A4u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x400069A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod tgt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A4u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x400069A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069A4u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A4u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x400069A4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdl2r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A8u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x400069A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A8u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x400069A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069A8u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400069A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069A8u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069A8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x400069A8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_tdh2r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069ACu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069ACu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x400069ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069ACu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069ACu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x400069ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069ACu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069ACu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400069ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069ACu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069ACu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x400069ACu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ri0r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B0u32 as *const u32) >> 21) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B0u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= (val & 0x7FF) << 21;
                core::ptr::write_volatile(0x400069B0u32 as *mut u32, reg);
            }
        }
    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B0u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B0u32 as *const u32);
                reg &= 0xFFFC0000u32;
                reg |= (val & 0x3FFFF) << 3;
                core::ptr::write_volatile(0x400069B0u32 as *mut u32, reg);
            }
        }
    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B0u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x400069B0u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B0u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x400069B0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rdt0r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B4u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B4u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x400069B4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fmi {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B4u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B4u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400069B4u32 as *mut u32, reg);
            }
        }
    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069B4u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B4u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x400069B4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rdl0r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B8u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x400069B8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B8u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x400069B8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069B8u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400069B8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069B8u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069B8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x400069B8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rdh0r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069BCu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069BCu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x400069BCu32 as *mut u32, reg);
            }
        }
    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069BCu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069BCu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x400069BCu32 as *mut u32, reg);
            }
        }
    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069BCu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069BCu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400069BCu32 as *mut u32, reg);
            }
        }
    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069BCu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069BCu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x400069BCu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ri1r {
    pub mod stid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C0u32 as *const u32) >> 21) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C0u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= (val & 0x7FF) << 21;
                core::ptr::write_volatile(0x400069C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod exid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C0u32 as *const u32) >> 3) & 0x3FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C0u32 as *const u32);
                reg &= 0xFFFC0000u32;
                reg |= (val & 0x3FFFF) << 3;
                core::ptr::write_volatile(0x400069C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod ide {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C0u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x400069C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C0u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x400069C0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rdt1r {
    pub mod time {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C4u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C4u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x400069C4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fmi {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C4u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C4u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400069C4u32 as *mut u32, reg);
            }
        }
    }
    pub mod dlc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069C4u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C4u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x400069C4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rdl1r {
    pub mod data3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C8u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x400069C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C8u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x400069C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069C8u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400069C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod data0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069C8u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069C8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x400069C8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_rdh1r {
    pub mod data7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069CCu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069CCu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x400069CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod data6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069CCu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069CCu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x400069CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod data5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x400069CCu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069CCu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x400069CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod data4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x400069CCu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x400069CCu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x400069CCu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_fmr {
    pub mod finit {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A00u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A00u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_fm1r {
    pub mod fbm0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A04u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbm13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A04u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A04u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_fs1r {
    pub mod fsc0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A0Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsc13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A0Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A0Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_ffa1r {
    pub mod ffa0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A14u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ffa13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A14u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A14u32 as *mut u32, reg);
            }
        }
    }
}

pub mod can_fa1r {
    pub mod fact0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A1Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fact13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A1Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A1Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f0r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A40u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A40u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A40u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f0r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A44u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A44u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A44u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f1r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A48u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A48u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A48u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f1r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A4Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A4Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A4Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A4Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f2r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A50u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A50u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A50u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A50u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f2r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A54u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A54u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A54u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A54u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f3r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A58u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A58u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A58u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A58u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f3r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A5Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A5Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A5Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A5Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f4r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A60u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A60u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A60u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f4r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A64u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A64u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A64u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A64u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f5r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A68u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A68u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A68u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f5r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A6Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A6Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A6Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A6Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f6r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A70u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A70u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A70u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A70u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f6r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A74u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A74u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A74u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A74u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f7r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A78u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A78u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A78u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A78u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f7r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A7Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A7Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A7Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A7Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f8r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A80u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A80u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A80u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A80u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f8r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A84u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A84u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A84u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A84u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f9r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A88u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A88u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A88u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A88u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f9r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A8Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A8Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A8Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A8Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f10r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A90u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A90u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A90u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f10r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A94u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A94u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A94u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f11r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A98u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A98u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A98u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A98u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f11r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006A9Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006A9Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006A9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006A9Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod f12r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006AA0u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA0u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006AA0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f12r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006AA4u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA4u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006AA4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f13r1 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006AA8u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AA8u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AA8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006AA8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod f13r2 {
    pub mod fb0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006AACu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb19 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb20 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb21 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb22 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb23 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb24 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb25 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb26 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb27 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb28 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb29 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb30 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
    pub mod fb31 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006AACu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006AACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40006AACu32 as *mut u32, reg);
            }
        }
    }
}

