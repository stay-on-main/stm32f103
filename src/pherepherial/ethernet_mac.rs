pub mod maccr {
    pub mod re {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod te {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod dc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod bl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 5) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 5;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod apcs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod rd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipco {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod dm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod lm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod rod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod fes {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod csd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod ifg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 17) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 17;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod jd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
    pub mod wd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028000u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40028000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod macffr {
    pub mod pm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028004u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod hu {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod hm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod daif {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod pam {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod bfd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod pcf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 6) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 6;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod saif {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod saf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod hpf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ra {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028004u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40028004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod machthr {
    pub mod hth {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028008u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028008u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod machtlr {
    pub mod htl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002800Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002800Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002800Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod macmiiar {
    pub mod mb {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40028010u32 as *mut u32, reg);
            }
        }
    }
    pub mod mw {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028010u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40028010u32 as *mut u32, reg);
            }
        }
    }
    pub mod cr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028010u32 as *const u32) >> 2) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028010u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 2;
                core::ptr::write_volatile(0x40028010u32 as *mut u32, reg);
            }
        }
    }
    pub mod mr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028010u32 as *const u32) >> 6) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028010u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= (val & 0x1F) << 6;
                core::ptr::write_volatile(0x40028010u32 as *mut u32, reg);
            }
        }
    }
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028010u32 as *const u32) >> 11) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028010u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= (val & 0x1F) << 11;
                core::ptr::write_volatile(0x40028010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod macmiidr {
    pub mod md {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028014u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028014u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40028014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod macfcr {
    pub mod fcb_bpa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028018u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40028018u32 as *mut u32, reg);
            }
        }
    }
    pub mod tfce {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028018u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40028018u32 as *mut u32, reg);
            }
        }
    }
    pub mod rfce {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028018u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40028018u32 as *mut u32, reg);
            }
        }
    }
    pub mod upfd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028018u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40028018u32 as *mut u32, reg);
            }
        }
    }
    pub mod plt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028018u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028018u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40028018u32 as *mut u32, reg);
            }
        }
    }
    pub mod zqpd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028018u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40028018u32 as *mut u32, reg);
            }
        }
    }
    pub mod pt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028018u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028018u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x40028018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod macvlantr {
    pub mod vlanti {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002801Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002801Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4002801Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod vlantc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002801Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002801Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4002801Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod macrwuffr {
}

pub mod macpmtcsr {
    pub mod pd {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002802Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod mpe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002802Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4002802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod wfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002802Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod mpr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002802Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod wfr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002802Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod gu {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002802Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4002802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod wffrpr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002802Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4002802Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod macsr {
    pub mod pmts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028038u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40028038u32 as *mut u32, reg);
            }
        }
    }
    pub mod mmcs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028038u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40028038u32 as *mut u32, reg);
            }
        }
    }
    pub mod mmcrs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028038u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40028038u32 as *mut u32, reg);
            }
        }
    }
    pub mod mmcts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028038u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40028038u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028038u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40028038u32 as *mut u32, reg);
            }
        }
    }
}

pub mod macimr {
    pub mod pmtim {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002803Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tstim {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002803Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4002803Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod maca0hr {
    pub mod maca0h {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028040u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028040u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40028040u32 as *mut u32, reg);
            }
        }
    }
    pub mod mo {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028040u32 as *const u32) >> 31) & 0x1
            }
        }

    }
}

pub mod maca0lr {
    pub mod maca0l {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028044u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028044u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028044u32 as *mut u32, reg);
            }
        }
    }
}

pub mod maca1hr {
    pub mod maca1h {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028048u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028048u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40028048u32 as *mut u32, reg);
            }
        }
    }
    pub mod mbc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028048u32 as *const u32) >> 24) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028048u32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= (val & 0x3F) << 24;
                core::ptr::write_volatile(0x40028048u32 as *mut u32, reg);
            }
        }
    }
    pub mod sa {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028048u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028048u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40028048u32 as *mut u32, reg);
            }
        }
    }
    pub mod ae {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028048u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028048u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40028048u32 as *mut u32, reg);
            }
        }
    }
}

pub mod maca1lr {
    pub mod maca1l {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002804Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002804Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002804Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod maca2hr {
    pub mod eth_maca2hr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028050u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028050u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40028050u32 as *mut u32, reg);
            }
        }
    }
    pub mod mbc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028050u32 as *const u32) >> 24) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028050u32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= (val & 0x3F) << 24;
                core::ptr::write_volatile(0x40028050u32 as *mut u32, reg);
            }
        }
    }
    pub mod sa {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028050u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028050u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40028050u32 as *mut u32, reg);
            }
        }
    }
    pub mod ae {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028050u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028050u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40028050u32 as *mut u32, reg);
            }
        }
    }
}

pub mod maca2lr {
    pub mod maca2l {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028054u32 as *const u32) & 0x7FFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028054u32 as *const u32);
                reg &= 0x80000000u32;
                reg |= val & 0x7FFFFFFF;
                core::ptr::write_volatile(0x40028054u32 as *mut u32, reg);
            }
        }
    }
}

pub mod maca3hr {
    pub mod maca3h {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028058u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028058u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40028058u32 as *mut u32, reg);
            }
        }
    }
    pub mod mbc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028058u32 as *const u32) >> 24) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028058u32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= (val & 0x3F) << 24;
                core::ptr::write_volatile(0x40028058u32 as *mut u32, reg);
            }
        }
    }
    pub mod sa {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028058u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40028058u32 as *mut u32, reg);
            }
        }
    }
    pub mod ae {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028058u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40028058u32 as *mut u32, reg);
            }
        }
    }
}

pub mod maca3lr {
    pub mod mbca3l {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002805Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002805Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002805Cu32 as *mut u32, reg);
            }
        }
    }
}

