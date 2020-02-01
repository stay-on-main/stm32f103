pub mod cr {
    pub mod hsion {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40021000u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hsirdy {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hsitrim {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 3) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFFFFFF07u32;
                reg |= (val & 0x1F) << 3;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hsical {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hseon {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hserdy {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hsebyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod csson {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod pllon {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
    pub mod pllrdy {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021000u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021000u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40021000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cfgr {
    pub mod sw {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40021004u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod sws {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFFFFFF3u32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod hpre {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ppre1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 8) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFFFF8FFu32;
                reg |= (val & 0x7) << 8;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ppre2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 11) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFFFC7FFu32;
                reg |= (val & 0x7) << 11;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod adcpre {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 14) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFFF3FFFu32;
                reg |= (val & 0x3) << 14;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod pllsrc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod pllxtpre {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod pllmul {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 18) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFC3FFFFu32;
                reg |= (val & 0xF) << 18;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod otgfspre {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
    pub mod mco {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021004u32 as *const u32) >> 24) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021004u32 as *const u32);
                reg &= 0xF8FFFFFFu32;
                reg |= (val & 0x7) << 24;
                core::ptr::write_volatile(0x40021004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cir {
    pub mod lsirdyf {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40021008u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod lserdyf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod hsirdyf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod hserdyf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod pllrdyf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod cssf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsirdyie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod lserdyie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod hsirdyie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod hserdyie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod pllrdyie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsirdyc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod lserdyc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod hsirdyc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod hserdyc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod pllrdyc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
    pub mod cssc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021008u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021008u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40021008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod apb2rstr {
    pub mod afiorst {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002100Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ioparst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod iopbrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod iopcrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod iopdrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ioperst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod iopfrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod iopgrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod adc1rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod adc2rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim1rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod spi1rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim8rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod usart1rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod adc3rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim9rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim10rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim11rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002100Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002100Cu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4002100Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod apb1rstr {
    pub mod tim2rst {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40021010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim3rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim4rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim5rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim6rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim7rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim12rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim13rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim14rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod wwdgrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod spi2rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod spi3rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod usart2rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod usart3rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod uart4rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod uart5rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod i2c1rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod i2c2rst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod usbrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod canrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bkprst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwrrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
    pub mod dacrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021010u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021010u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40021010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ahbenr {
    pub mod dma1en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40021014u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40021014u32 as *mut u32, reg);
            }
        }
    }
    pub mod dma2en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021014u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021014u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40021014u32 as *mut u32, reg);
            }
        }
    }
    pub mod sramen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021014u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021014u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40021014u32 as *mut u32, reg);
            }
        }
    }
    pub mod flitfen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021014u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021014u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40021014u32 as *mut u32, reg);
            }
        }
    }
    pub mod crcen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021014u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021014u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40021014u32 as *mut u32, reg);
            }
        }
    }
    pub mod fsmcen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021014u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021014u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40021014u32 as *mut u32, reg);
            }
        }
    }
    pub mod sdioen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021014u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021014u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40021014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod apb2enr {
    pub mod afioen {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40021018u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iopaen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iopben {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iopcen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iopden {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iopeen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iopfen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iopgen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod adc1en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod adc2en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim1en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod spi1en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim8en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod usart1en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod adc3en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim9en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim10en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim11en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021018u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021018u32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40021018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod apb1enr {
    pub mod tim2en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002101Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim3en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim4en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim5en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim6en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim7en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim12en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim13en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim14en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod wwdgen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod spi2en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod spi3en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod usart2en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod usart3en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod uart4en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod uart5en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod i2c1en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFDFFFFFu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod i2c2en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod usben {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFF7FFFFFu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod canen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bkpen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pwren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dacen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002101Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002101Cu32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x4002101Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod bdcr {
    pub mod lseon {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40021020u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021020u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40021020u32 as *mut u32, reg);
            }
        }
    }
    pub mod lserdy {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021020u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021020u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40021020u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsebyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021020u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021020u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40021020u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtcsel {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021020u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021020u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40021020u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtcen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021020u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021020u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40021020u32 as *mut u32, reg);
            }
        }
    }
    pub mod bdrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021020u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021020u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40021020u32 as *mut u32, reg);
            }
        }
    }
}

pub mod csr {
    pub mod lsion {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40021024u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsirdy {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021024u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
    pub mod rmvf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021024u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinrstf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021024u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
    pub mod porrstf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021024u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
    pub mod sftrstf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021024u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
    pub mod iwdgrstf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021024u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
    pub mod wwdgrstf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021024u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
    pub mod lpwrrstf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40021024u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40021024u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40021024u32 as *mut u32, reg);
            }
        }
    }
}

