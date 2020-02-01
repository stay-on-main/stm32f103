pub mod cr1 {
    pub mod ckd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C00u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C00u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40014C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod arpe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C00u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C00u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40014C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod opm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C00u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C00u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40014C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod urs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C00u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C00u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40014C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod udis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C00u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C00u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40014C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod cen {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C00u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40014C00u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod mms {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C04u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C04u32 as *const u32);
                reg &= 0xFFFFFF8Fu32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40014C04u32 as *mut u32, reg);
            }
        }
    }
}

pub mod smcr {
    pub mod msm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C08u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C08u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40014C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod ts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C08u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C08u32 as *const u32);
                reg &= 0xFFFFFF8Fu32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40014C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod sms {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C08u32 as *const u32) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C08u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= val & 0x7;
                core::ptr::write_volatile(0x40014C08u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dier {
    pub mod tie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C0Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C0Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40014C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2ie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C0Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C0Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40014C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1ie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C0Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C0Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40014C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod uie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C0Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40014C0Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr {
    pub mod cc2of {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C10u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C10u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40014C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1of {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C10u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C10u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40014C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod tif {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C10u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C10u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40014C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2if {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C10u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C10u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40014C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1if {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C10u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C10u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40014C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod uif {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C10u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40014C10u32 as *mut u32, reg);
            }
        }
    }
}

pub mod egr {
    pub mod tg {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 6;
                core::ptr::write_volatile(0x40014C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2g {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 2;
                core::ptr::write_volatile(0x40014C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1g {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 1;
                core::ptr::write_volatile(0x40014C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ug {
        pub fn set(val: u32) {
            unsafe {
                let reg = val & 0x1;
                core::ptr::write_volatile(0x40014C14u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccmr1_output {
    pub mod oc2m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 12) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFF8FFFu32;
                reg |= (val & 0x7) << 12;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc2pe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc2fe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc1m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFF8Fu32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc1pe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc1fe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1s {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C18u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccmr1_input {
    pub mod ic2f {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 12) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFF0FFFu32;
                reg |= (val & 0xF) << 12;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ic2psc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ic1f {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ic1psc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C18u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFFF3u32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1s {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C18u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C18u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40014C18u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccer {
    pub mod cc2np {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C20u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C20u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40014C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2p {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C20u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C20u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40014C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc2e {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C20u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C20u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40014C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1np {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C20u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C20u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40014C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1p {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40014C20u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C20u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40014C20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1e {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C20u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40014C20u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cnt {
    pub mod cnt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C24u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C24u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40014C24u32 as *mut u32, reg);
            }
        }
    }
}

pub mod psc {
    pub mod psc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C28u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C28u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40014C28u32 as *mut u32, reg);
            }
        }
    }
}

pub mod arr {
    pub mod arr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C2Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C2Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40014C2Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr1 {
    pub mod ccr1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C34u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C34u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40014C34u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr2 {
    pub mod ccr2 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40014C38u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40014C38u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40014C38u32 as *mut u32, reg);
            }
        }
    }
}

