pub mod sr {
    pub mod strt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012400u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012400u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40012400u32 as *mut u32, reg);
            }
        }
    }
    pub mod jstrt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012400u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012400u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012400u32 as *mut u32, reg);
            }
        }
    }
    pub mod jeoc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012400u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012400u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012400u32 as *mut u32, reg);
            }
        }
    }
    pub mod eoc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012400u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012400u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40012400u32 as *mut u32, reg);
            }
        }
    }
    pub mod awd {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012400u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40012400u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr1 {
    pub mod awden {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod jawden {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod dualmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 16) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFF0FFFFu32;
                reg |= (val & 0xF) << 16;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod discnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 13) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFF1FFFu32;
                reg |= (val & 0x7) << 13;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod jdiscen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod discen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod jauto {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod awdsgl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod scan {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod jeocie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod awdie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod eocie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012404u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
    pub mod awdch {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012404u32 as *const u32) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012404u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= val & 0x1F;
                core::ptr::write_volatile(0x40012404u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod tsvrefe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod swstart {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod jswstart {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod exttrig {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod extsel {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 17) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFF1FFFFu32;
                reg |= (val & 0x7) << 17;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod jexttrig {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod jextsel {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 12) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFFF8FFFu32;
                reg |= (val & 0x7) << 12;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod align {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod dma {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod rstcal {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod cal {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod cont {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012408u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
    pub mod adon {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012408u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012408u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40012408u32 as *mut u32, reg);
            }
        }
    }
}

pub mod smpr1 {
    pub mod smp10 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001240Cu32 as *const u32) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001240Cu32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= val & 0x7;
                core::ptr::write_volatile(0x4001240Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod smp11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001240Cu32 as *const u32) >> 3) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001240Cu32 as *const u32);
                reg &= 0xFFFFFFC7u32;
                reg |= (val & 0x7) << 3;
                core::ptr::write_volatile(0x4001240Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod smp12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001240Cu32 as *const u32) >> 6) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001240Cu32 as *const u32);
                reg &= 0xFFFFFE3Fu32;
                reg |= (val & 0x7) << 6;
                core::ptr::write_volatile(0x4001240Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod smp13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001240Cu32 as *const u32) >> 9) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001240Cu32 as *const u32);
                reg &= 0xFFFFF1FFu32;
                reg |= (val & 0x7) << 9;
                core::ptr::write_volatile(0x4001240Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod smp14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001240Cu32 as *const u32) >> 12) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001240Cu32 as *const u32);
                reg &= 0xFFFF8FFFu32;
                reg |= (val & 0x7) << 12;
                core::ptr::write_volatile(0x4001240Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod smp15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001240Cu32 as *const u32) >> 15) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001240Cu32 as *const u32);
                reg &= 0xFFFC7FFFu32;
                reg |= (val & 0x7) << 15;
                core::ptr::write_volatile(0x4001240Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod smp16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001240Cu32 as *const u32) >> 18) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001240Cu32 as *const u32);
                reg &= 0xFFE3FFFFu32;
                reg |= (val & 0x7) << 18;
                core::ptr::write_volatile(0x4001240Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod smp17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001240Cu32 as *const u32) >> 21) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001240Cu32 as *const u32);
                reg &= 0xFF1FFFFFu32;
                reg |= (val & 0x7) << 21;
                core::ptr::write_volatile(0x4001240Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod smpr2 {
    pub mod smp0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012410u32 as *const u32) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= val & 0x7;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 3) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xFFFFFFC7u32;
                reg |= (val & 0x7) << 3;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 6) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xFFFFFE3Fu32;
                reg |= (val & 0x7) << 6;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 9) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xFFFFF1FFu32;
                reg |= (val & 0x7) << 9;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 12) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xFFFF8FFFu32;
                reg |= (val & 0x7) << 12;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 15) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xFFFC7FFFu32;
                reg |= (val & 0x7) << 15;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 18) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xFFE3FFFFu32;
                reg |= (val & 0x7) << 18;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 21) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xFF1FFFFFu32;
                reg |= (val & 0x7) << 21;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 24) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xF8FFFFFFu32;
                reg |= (val & 0x7) << 24;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
    pub mod smp9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012410u32 as *const u32) >> 27) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012410u32 as *const u32);
                reg &= 0xC7FFFFFFu32;
                reg |= (val & 0x7) << 27;
                core::ptr::write_volatile(0x40012410u32 as *mut u32, reg);
            }
        }
    }
}

pub mod jofr1 {
    pub mod joffset1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012414u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012414u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40012414u32 as *mut u32, reg);
            }
        }
    }
}

pub mod jofr2 {
    pub mod joffset2 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012418u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012418u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40012418u32 as *mut u32, reg);
            }
        }
    }
}

pub mod jofr3 {
    pub mod joffset3 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001241Cu32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001241Cu32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x4001241Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod jofr4 {
    pub mod joffset4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012420u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012420u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40012420u32 as *mut u32, reg);
            }
        }
    }
}

pub mod htr {
    pub mod ht {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012424u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012424u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40012424u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ltr {
    pub mod lt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012428u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012428u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40012428u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sqr1 {
    pub mod l {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001242Cu32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001242Cu32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0x4001242Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sq16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001242Cu32 as *const u32) >> 15) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001242Cu32 as *const u32);
                reg &= 0xFFF07FFFu32;
                reg |= (val & 0x1F) << 15;
                core::ptr::write_volatile(0x4001242Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sq15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001242Cu32 as *const u32) >> 10) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001242Cu32 as *const u32);
                reg &= 0xFFFF83FFu32;
                reg |= (val & 0x1F) << 10;
                core::ptr::write_volatile(0x4001242Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sq14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001242Cu32 as *const u32) >> 5) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001242Cu32 as *const u32);
                reg &= 0xFFFFFC1Fu32;
                reg |= (val & 0x1F) << 5;
                core::ptr::write_volatile(0x4001242Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sq13 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001242Cu32 as *const u32) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001242Cu32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= val & 0x1F;
                core::ptr::write_volatile(0x4001242Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod sqr2 {
    pub mod sq12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012430u32 as *const u32) >> 25) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012430u32 as *const u32);
                reg &= 0xC1FFFFFFu32;
                reg |= (val & 0x1F) << 25;
                core::ptr::write_volatile(0x40012430u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012430u32 as *const u32) >> 20) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012430u32 as *const u32);
                reg &= 0xFE0FFFFFu32;
                reg |= (val & 0x1F) << 20;
                core::ptr::write_volatile(0x40012430u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012430u32 as *const u32) >> 15) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012430u32 as *const u32);
                reg &= 0xFFF07FFFu32;
                reg |= (val & 0x1F) << 15;
                core::ptr::write_volatile(0x40012430u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012430u32 as *const u32) >> 10) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012430u32 as *const u32);
                reg &= 0xFFFF83FFu32;
                reg |= (val & 0x1F) << 10;
                core::ptr::write_volatile(0x40012430u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012430u32 as *const u32) >> 5) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012430u32 as *const u32);
                reg &= 0xFFFFFC1Fu32;
                reg |= (val & 0x1F) << 5;
                core::ptr::write_volatile(0x40012430u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq7 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012430u32 as *const u32) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012430u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= val & 0x1F;
                core::ptr::write_volatile(0x40012430u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sqr3 {
    pub mod sq6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012434u32 as *const u32) >> 25) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012434u32 as *const u32);
                reg &= 0xC1FFFFFFu32;
                reg |= (val & 0x1F) << 25;
                core::ptr::write_volatile(0x40012434u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012434u32 as *const u32) >> 20) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012434u32 as *const u32);
                reg &= 0xFE0FFFFFu32;
                reg |= (val & 0x1F) << 20;
                core::ptr::write_volatile(0x40012434u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012434u32 as *const u32) >> 15) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012434u32 as *const u32);
                reg &= 0xFFF07FFFu32;
                reg |= (val & 0x1F) << 15;
                core::ptr::write_volatile(0x40012434u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012434u32 as *const u32) >> 10) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012434u32 as *const u32);
                reg &= 0xFFFF83FFu32;
                reg |= (val & 0x1F) << 10;
                core::ptr::write_volatile(0x40012434u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012434u32 as *const u32) >> 5) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012434u32 as *const u32);
                reg &= 0xFFFFFC1Fu32;
                reg |= (val & 0x1F) << 5;
                core::ptr::write_volatile(0x40012434u32 as *mut u32, reg);
            }
        }
    }
    pub mod sq1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012434u32 as *const u32) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012434u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= val & 0x1F;
                core::ptr::write_volatile(0x40012434u32 as *mut u32, reg);
            }
        }
    }
}

pub mod jsqr {
    pub mod jl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012438u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012438u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x40012438u32 as *mut u32, reg);
            }
        }
    }
    pub mod jsq4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012438u32 as *const u32) >> 15) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012438u32 as *const u32);
                reg &= 0xFFF07FFFu32;
                reg |= (val & 0x1F) << 15;
                core::ptr::write_volatile(0x40012438u32 as *mut u32, reg);
            }
        }
    }
    pub mod jsq3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012438u32 as *const u32) >> 10) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012438u32 as *const u32);
                reg &= 0xFFFF83FFu32;
                reg |= (val & 0x1F) << 10;
                core::ptr::write_volatile(0x40012438u32 as *mut u32, reg);
            }
        }
    }
    pub mod jsq2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40012438u32 as *const u32) >> 5) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012438u32 as *const u32);
                reg &= 0xFFFFFC1Fu32;
                reg |= (val & 0x1F) << 5;
                core::ptr::write_volatile(0x40012438u32 as *mut u32, reg);
            }
        }
    }
    pub mod jsq1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012438u32 as *const u32) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40012438u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= val & 0x1F;
                core::ptr::write_volatile(0x40012438u32 as *mut u32, reg);
            }
        }
    }
}

pub mod jdr1 {
    pub mod jdata {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001243Cu32 as *const u32) & 0xFFFF
            }
        }

    }
}

pub mod jdr2 {
    pub mod jdata {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012440u32 as *const u32) & 0xFFFF
            }
        }

    }
}

pub mod jdr3 {
    pub mod jdata {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012444u32 as *const u32) & 0xFFFF
            }
        }

    }
}

pub mod jdr4 {
    pub mod jdata {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40012448u32 as *const u32) & 0xFFFF
            }
        }

    }
}

pub mod dr {
    pub mod data {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001244Cu32 as *const u32) & 0xFFFF
            }
        }

    }
    pub mod adc2data {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001244Cu32 as *const u32) >> 16) & 0xFFFF
            }
        }

    }
}

