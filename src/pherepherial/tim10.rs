pub mod cr1 {
    pub mod ckd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015000u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015000u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40015000u32 as *mut u32, reg);
            }
        }
    }
    pub mod arpe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015000u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015000u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40015000u32 as *mut u32, reg);
            }
        }
    }
    pub mod urs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015000u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015000u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40015000u32 as *mut u32, reg);
            }
        }
    }
    pub mod udis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015000u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015000u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40015000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cen {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40015000u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40015000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod mms {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015004u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015004u32 as *const u32);
                reg &= 0xFFFFFF8Fu32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40015004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dier {
    pub mod cc1ie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001500Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001500Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4001500Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod uie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001500Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001500Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4001500Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr {
    pub mod cc1of {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015010u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015010u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40015010u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1if {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015010u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015010u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40015010u32 as *mut u32, reg);
            }
        }
    }
    pub mod uif {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40015010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40015010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod egr {
    pub mod cc1g {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 1;
                core::ptr::write_volatile(0x40015014u32 as *mut u32, reg);
            }
        }
    }
    pub mod ug {
        pub fn set(val: u32) {
            unsafe {
                let reg = val & 0x1;
                core::ptr::write_volatile(0x40015014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccmr1_output {
    pub mod oc1m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015018u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015018u32 as *const u32);
                reg &= 0xFFFFFF8Fu32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40015018u32 as *mut u32, reg);
            }
        }
    }
    pub mod oc1pe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015018u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015018u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40015018u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1s {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40015018u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015018u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40015018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccmr1_input {
    pub mod ic1f {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015018u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015018u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x40015018u32 as *mut u32, reg);
            }
        }
    }
    pub mod ic1psc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015018u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015018u32 as *const u32);
                reg &= 0xFFFFFFF3u32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0x40015018u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1s {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40015018u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015018u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40015018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccer {
    pub mod cc1np {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015020u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015020u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40015020u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1p {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40015020u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015020u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40015020u32 as *mut u32, reg);
            }
        }
    }
    pub mod cc1e {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40015020u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015020u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40015020u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cnt {
    pub mod cnt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40015024u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015024u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40015024u32 as *mut u32, reg);
            }
        }
    }
}

pub mod psc {
    pub mod psc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40015028u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015028u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40015028u32 as *mut u32, reg);
            }
        }
    }
}

pub mod arr {
    pub mod arr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001502Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001502Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4001502Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr1 {
    pub mod ccr1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40015034u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40015034u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40015034u32 as *mut u32, reg);
            }
        }
    }
}

