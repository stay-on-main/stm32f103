pub mod cr1 {
    pub mod bidimode {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod bidioe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod crcen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod crcnext {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod dff {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxonly {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod ssm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod ssi {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsbfirst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod spe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod br {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 3) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 3;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod mstr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod cpol {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003800u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
    pub mod cpha {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003800u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40003800u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod txeie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003804u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40003804u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxneie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003804u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40003804u32 as *mut u32, reg);
            }
        }
    }
    pub mod errie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003804u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40003804u32 as *mut u32, reg);
            }
        }
    }
    pub mod ssoe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003804u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40003804u32 as *mut u32, reg);
            }
        }
    }
    pub mod txdmaen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003804u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40003804u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxdmaen {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003804u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40003804u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr {
    pub mod bsy {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003808u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod ovr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003808u32 as *const u32) >> 6) & 0x1
            }
        }

    }
    pub mod modf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003808u32 as *const u32) >> 5) & 0x1
            }
        }

    }
    pub mod crcerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003808u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40003808u32 as *mut u32, reg);
            }
        }
    }
    pub mod udr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003808u32 as *const u32) >> 3) & 0x1
            }
        }

    }
    pub mod chside {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003808u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod txe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003808u32 as *const u32) >> 1) & 0x1
            }
        }

    }
    pub mod rxne {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003808u32 as *const u32) & 0x1
            }
        }

    }
}

pub mod dr {
    pub mod dr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000380Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000380Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4000380Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod crcpr {
    pub mod crcpoly {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003810u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003810u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40003810u32 as *mut u32, reg);
            }
        }
    }
}

pub mod rxcrcr {
    pub mod rxcrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003814u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003814u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40003814u32 as *mut u32, reg);
            }
        }
    }
}

pub mod txcrcr {
    pub mod txcrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003818u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003818u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40003818u32 as *mut u32, reg);
            }
        }
    }
}

pub mod i2scfgr {
    pub mod i2smod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000381Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000381Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000381Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod i2se {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000381Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000381Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000381Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod i2scfg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000381Cu32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000381Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x4000381Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pcmsync {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000381Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000381Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000381Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod i2sstd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000381Cu32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000381Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x4000381Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ckpol {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000381Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000381Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000381Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datlen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000381Cu32 as *const u32) >> 1) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000381Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 1;
                core::ptr::write_volatile(0x4000381Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod chlen {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000381Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000381Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000381Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod i2spr {
    pub mod mckoe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003820u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003820u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40003820u32 as *mut u32, reg);
            }
        }
    }
    pub mod odd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40003820u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003820u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40003820u32 as *mut u32, reg);
            }
        }
    }
    pub mod i2sdiv {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40003820u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40003820u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40003820u32 as *mut u32, reg);
            }
        }
    }
}

