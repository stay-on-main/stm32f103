pub mod imr {
    pub mod mr0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010400u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010400u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010400u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40010400u32 as *mut u32, reg);
            }
        }
    }
}

pub mod emr {
    pub mod mr0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010404u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010404u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010404u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40010404u32 as *mut u32, reg);
            }
        }
    }
}

pub mod rtsr {
    pub mod tr0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010408u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tr18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010408u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010408u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40010408u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ftsr {
    pub mod tr0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001040Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tr18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001040Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001040Cu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4001040Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod swier {
    pub mod swier0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010410u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
    pub mod swier18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010410u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010410u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40010410u32 as *mut u32, reg);
            }
        }
    }
}

pub mod pr {
    pub mod pr0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010414u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr16 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr17 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
    pub mod pr18 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010414u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010414u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40010414u32 as *mut u32, reg);
            }
        }
    }
}

