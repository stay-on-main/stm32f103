pub mod cr1 {
    pub mod ckd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C00u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C00u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40012C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod arpe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C00u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod cms {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C00u32 as *const u32) >> 5) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C00u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 5;
                core::ptr::write_volatile(0x40012C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C00u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40012C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod opm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C00u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod urs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C00u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod udis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C00u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40012C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod cen {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C00u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40012C00u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod ois4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ois3n {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ois3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ois2n {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ois2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ois1n {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ois1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ti1s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod mms {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ccds {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ccus {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C04u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ccpc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C04u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C04u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40012C04u32 as *mut u32, reg);
            }
        }
    }
}

pub mod smcr {
    pub mod etp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C08u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40012C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod ece {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C08u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40012C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod etps {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C08u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C08u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40012C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod etf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C08u32 as *const u32) >> 8) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C08u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 8;
                core::ptr::write_volatile(0x40012C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod msm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C08u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod ts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C08u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C08u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40012C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod sms {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C08u32 as *const u32) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C08u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= val & 0x7;
                core::ptr::write_volatile(0x40012C08u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dier {
    pub mod tde {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod comde {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc4de {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3de {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2de {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1de {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ude {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc4ie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3ie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2ie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1ie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod uie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C0Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod comie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C0Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40012C0Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr {
    pub mod cc4of {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3of {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2of {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1of {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod bif {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod tif {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod comif {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc4if {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3if {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2if {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1if {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C10u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod uif {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C10u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40012C10u32 as *mut u32, reg);
            }
        }
    }
}

pub mod egr {
    pub mod bg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C14u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod tg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C14u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40012C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod comg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C14u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40012C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc4g {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C14u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40012C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3g {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C14u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2g {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C14u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1g {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C14u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40012C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ug {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C14u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40012C14u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccmr1_output {
    pub mod oc2ce {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc2m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 12) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 12;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc2pe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc2fe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc1ce {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc1m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc1pe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc1fe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1s {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C18u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccmr1_input {
    pub mod ic2f {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 12) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 12;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ic2pcs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ic1f {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod icpcs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C18u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1s {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C18u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C18u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40012C18u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccmr2_output {
    pub mod oc4ce {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod oc4m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 12) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 12;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod oc4pe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod oc4fe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc4s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod oc3ce {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod oc3m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod oc3pe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod oc3fe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3s {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C1Cu32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccmr2_input {
    pub mod ic4f {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 12) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 12;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ic4psc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc4s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ic3f {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ic3psc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C1Cu32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3s {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C1Cu32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C1Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40012C1Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccer {
    pub mod cc4p {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc4e {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3np {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3ne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3p {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc3e {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2np {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2ne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2p {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2e {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1np {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1ne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1p {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C20u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1e {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C20u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40012C20u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cnt {
    pub mod cnt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C24u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C24u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40012C24u32 as *mut u32, reg);
            }
        }
    }
}

pub mod psc {
    pub mod psc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C28u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C28u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40012C28u32 as *mut u32, reg);
            }
        }
    }
}

pub mod arr {
    pub mod arr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C2Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C2Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40012C2Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr1 {
    pub mod ccr1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C34u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C34u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40012C34u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr2 {
    pub mod ccr2 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C38u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C38u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40012C38u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr3 {
    pub mod ccr3 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C3Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C3Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40012C3Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr4 {
    pub mod ccr4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C40u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C40u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40012C40u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dcr {
    pub mod dbl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C48u32 as *const u32) >> 8) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C48u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= (val & 0x1F) << 8;
                core::ptr::write_volatile(0x40012C48u32 as *mut u32, reg);
            }
        }
    }
    pub mod dba {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C48u32 as *const u32) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C48u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= val & 0x1F;
                core::ptr::write_volatile(0x40012C48u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmar {
    pub mod dmab {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C4Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C4Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40012C4Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod rcr {
    pub mod rep {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C30u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C30u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40012C30u32 as *mut u32, reg);
            }
        }
    }
}

pub mod bdtr {
    pub mod moe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C44u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40012C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod aoe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C44u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40012C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod bkp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C44u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40012C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod bke {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C44u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40012C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod ossr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C44u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod ossi {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C44u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C44u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40012C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod lock {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012C44u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C44u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40012C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtg {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012C44u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012C44u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40012C44u32 as *mut u32, reg);
            }
        }
    }
}

