pub mod cr1 {
    pub mod swrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod alert {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod pec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod pos {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod start {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod nostretch {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod engc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod enpec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod enarp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod smbtype {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod smbus {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005800u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
    pub mod pe {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005800u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40005800u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod last {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005804u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005804u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40005804u32 as *mut u32, reg);
            }
        }
    }
    pub mod dmaen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005804u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005804u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005804u32 as *mut u32, reg);
            }
        }
    }
    pub mod itbufen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005804u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005804u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40005804u32 as *mut u32, reg);
            }
        }
    }
    pub mod itevten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005804u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005804u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40005804u32 as *mut u32, reg);
            }
        }
    }
    pub mod iterren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005804u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005804u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005804u32 as *mut u32, reg);
            }
        }
    }
    pub mod freq {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005804u32 as *const u32) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005804u32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= val & 0x3F;
                core::ptr::write_volatile(0x40005804u32 as *mut u32, reg);
            }
        }
    }
}

pub mod oar1 {
    pub mod addmode {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005808u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005808u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005808u32 as *mut u32, reg);
            }
        }
    }
    pub mod add10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005808u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005808u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40005808u32 as *mut u32, reg);
            }
        }
    }
    pub mod add7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005808u32 as *const u32) >> 1) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005808u32 as *const u32);
                reg &= 0xFFFFFF01u32;
                reg |= (val & 0x7F) << 1;
                core::ptr::write_volatile(0x40005808u32 as *mut u32, reg);
            }
        }
    }
    pub mod add0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005808u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40005808u32 as *mut u32, reg);
            }
        }
    }
}

pub mod oar2 {
    pub mod add2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000580Cu32 as *const u32) >> 1) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000580Cu32 as *const u32);
                reg &= 0xFFFFFF01u32;
                reg |= (val & 0x7F) << 1;
                core::ptr::write_volatile(0x4000580Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod endual {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000580Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000580Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000580Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr {
    pub mod dr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005810u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005810u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40005810u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr1 {
    pub mod smbalert {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod timeout {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod pecerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod ovr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod af {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod arlo {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod berr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod txe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod stopf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod add10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod btf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod addr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005814u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
    pub mod sb {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005814u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40005814u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr2 {
    pub mod pec {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005818u32 as *const u32) >> 8) & 0xFF
            }
        }

    }
    pub mod dualf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005818u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod smbhost {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005818u32 as *const u32) >> 6) & 0x1
            }
        }

    }
    pub mod smbdefault {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005818u32 as *const u32) >> 5) & 0x1
            }
        }

    }
    pub mod gencall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005818u32 as *const u32) >> 4) & 0x1
            }
        }

    }
    pub mod tra {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005818u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod busy {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005818u32 as *const u32) >> 1) & 0x1
            }
        }

    }
    pub mod msl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005818u32 as *const u32) & 0x1
            }
        }

    }
}

pub mod ccr {
    pub mod f_s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000581Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000581Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4000581Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod duty {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000581Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000581Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4000581Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ccr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000581Cu32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000581Cu32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x4000581Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod trise {
    pub mod trise {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005820u32 as *const u32) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005820u32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= val & 0x3F;
                core::ptr::write_volatile(0x40005820u32 as *mut u32, reg);
            }
        }
    }
}

