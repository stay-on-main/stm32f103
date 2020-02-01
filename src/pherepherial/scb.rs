pub mod cpuid {
    pub mod revision {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED00u32 as *const u32) & 0xF
            }
        }

    }
    pub mod partno {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED00u32 as *const u32) >> 4) & 0xFFF
            }
        }

    }
    pub mod constant {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED00u32 as *const u32) >> 16) & 0xF
            }
        }

    }
    pub mod variant {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED00u32 as *const u32) >> 20) & 0xF
            }
        }

    }
    pub mod implementer {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED00u32 as *const u32) >> 24) & 0xFF
            }
        }

    }
}

pub mod icsr {
    pub mod vectactive {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED04u32 as *const u32) & 0x1FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0xFFFFFE00u32;
                reg |= val & 0x1FF;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
    pub mod rettobase {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED04u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
    pub mod vectpending {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED04u32 as *const u32) >> 12) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0xFFF80FFFu32;
                reg |= (val & 0x7F) << 12;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
    pub mod isrpending {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED04u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0xFFBFFFFFu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
    pub mod pendstclr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED04u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
    pub mod pendstset {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED04u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0xFBFFFFFFu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
    pub mod pendsvclr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED04u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0xF7FFFFFFu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
    pub mod pendsvset {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED04u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0xEFFFFFFFu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
    pub mod nmipendset {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED04u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED04u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0xE000ED04u32 as *mut u32, reg);
            }
        }
    }
}

pub mod vtor {
    pub mod tbloff {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED08u32 as *const u32) >> 9) & 0x1FFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED08u32 as *const u32);
                reg &= 0xC00001FFu32;
                reg |= (val & 0x1FFFFF) << 9;
                core::ptr::write_volatile(0xE000ED08u32 as *mut u32, reg);
            }
        }
    }
}

pub mod aircr {
    pub mod vectreset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED0Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED0Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE000ED0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod vectclractive {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED0Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED0Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE000ED0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sysresetreq {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED0Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED0Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xE000ED0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod prigroup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED0Cu32 as *const u32) >> 8) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED0Cu32 as *const u32);
                reg &= 0xFFFFF8FFu32;
                reg |= (val & 0x7) << 8;
                core::ptr::write_volatile(0xE000ED0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod endianess {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED0Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED0Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0xE000ED0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod vectkeystat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED0Cu32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED0Cu32 as *const u32);
                reg &= 0xFFFFu32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0xE000ED0Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod scr {
    pub mod sleeponexit {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED10u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED10u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE000ED10u32 as *mut u32, reg);
            }
        }
    }
    pub mod sleepdeep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED10u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED10u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xE000ED10u32 as *mut u32, reg);
            }
        }
    }
    pub mod seveonpend {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED10u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED10u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0xE000ED10u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr {
    pub mod nonbasethrdena {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED14u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED14u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE000ED14u32 as *mut u32, reg);
            }
        }
    }
    pub mod usersetmpend {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED14u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED14u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE000ED14u32 as *mut u32, reg);
            }
        }
    }
    pub mod unalign__trp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED14u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED14u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xE000ED14u32 as *mut u32, reg);
            }
        }
    }
    pub mod div_0_trp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED14u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED14u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0xE000ED14u32 as *mut u32, reg);
            }
        }
    }
    pub mod bfhfnmign {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED14u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED14u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0xE000ED14u32 as *mut u32, reg);
            }
        }
    }
    pub mod stkalign {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED14u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED14u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0xE000ED14u32 as *mut u32, reg);
            }
        }
    }
}

pub mod shpr1 {
    pub mod pri_4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED18u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED18u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000ED18u32 as *mut u32, reg);
            }
        }
    }
    pub mod pri_5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED18u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED18u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000ED18u32 as *mut u32, reg);
            }
        }
    }
    pub mod pri_6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED18u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED18u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000ED18u32 as *mut u32, reg);
            }
        }
    }
}

pub mod shpr2 {
    pub mod pri_11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED1Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED1Cu32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000ED1Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod shpr3 {
    pub mod pri_14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED20u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED20u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000ED20u32 as *mut u32, reg);
            }
        }
    }
    pub mod pri_15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED20u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED20u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000ED20u32 as *mut u32, reg);
            }
        }
    }
}

pub mod shcrs {
    pub mod memfaultact {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED24u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod busfaultact {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod usgfaultact {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod svcallact {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod monitoract {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod pendsvact {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod systickact {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod usgfaultpended {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod memfaultpended {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod busfaultpended {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod svcallpended {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod memfaultena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod busfaultena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
    pub mod usgfaultena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED24u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED24u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0xE000ED24u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cfsr_ufsr_bfsr_mmfsr {
    pub mod iaccviol {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED28u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod daccviol {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod munstkerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod mstkerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod mlsperr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod mmarvalid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod ibuserr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod preciserr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod impreciserr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod unstkerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod stkerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsperr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod bfarvalid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod undefinstr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod invstate {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod invpc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod nocp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod unaligned {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFEFFFFFFu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
    pub mod divbyzero {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED28u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED28u32 as *const u32);
                reg &= 0xFDFFFFFFu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0xE000ED28u32 as *mut u32, reg);
            }
        }
    }
}

pub mod hfsr {
    pub mod vecttbl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED2Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED2Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE000ED2Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod forced {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED2Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED2Cu32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0xE000ED2Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod debug_vt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED2Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED2Cu32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0xE000ED2Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod mmfar {
    pub mod mmfar {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED34u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED34u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000ED34u32 as *mut u32, reg);
            }
        }
    }
}

pub mod bfar {
    pub mod bfar {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED38u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED38u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000ED38u32 as *mut u32, reg);
            }
        }
    }
}

