pub mod bcr1 {
    pub mod cburstrw {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod asyncwait {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod extmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod waiten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod wren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod waitcfg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod waitpol {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod bursten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod faccen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mwid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mtyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFFFF3u32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod muxen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000000u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mbken {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000000u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xA0000000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod btr1 {
    pub mod accmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000004u32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000004u32 as *const u32);
                reg &= 0xCFFFFFFFu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0xA0000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod datlat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000004u32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000004u32 as *const u32);
                reg &= 0xF0FFFFFFu32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0xA0000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000004u32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000004u32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0xA0000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod busturn {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000004u32 as *const u32) >> 16) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000004u32 as *const u32);
                reg &= 0xFFF0FFFFu32;
                reg |= (val & 0xF) << 16;
                core::ptr::write_volatile(0xA0000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod datast {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000004u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000004u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA0000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod addhld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000004u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000004u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0xA0000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod addset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000004u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000004u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xA0000004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod bcr2 {
    pub mod cburstrw {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod asyncwait {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod extmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod waiten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod wren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod waitcfg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod wrapmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod waitpol {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod bursten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod faccen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod mwid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod mtyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFFFF3u32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod muxen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000008u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod mbken {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000008u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xA0000008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod btr2 {
    pub mod accmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000000Cu32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000000Cu32 as *const u32);
                reg &= 0xCFFFFFFFu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0xA000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datlat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000000Cu32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000000Cu32 as *const u32);
                reg &= 0xF0FFFFFFu32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0xA000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000000Cu32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000000Cu32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0xA000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod busturn {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000000Cu32 as *const u32) >> 16) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000000Cu32 as *const u32);
                reg &= 0xFFF0FFFFu32;
                reg |= (val & 0xF) << 16;
                core::ptr::write_volatile(0xA000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datast {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000000Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000000Cu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addhld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000000Cu32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000000Cu32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0xA000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA000000Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000000Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xA000000Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod bcr3 {
    pub mod cburstrw {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod asyncwait {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod extmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod waiten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod wren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod waitcfg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod wrapmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod waitpol {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bursten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod faccen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod mwid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod mtyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFFFF3u32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod muxen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000010u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod mbken {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xA0000010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod btr3 {
    pub mod accmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000014u32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000014u32 as *const u32);
                reg &= 0xCFFFFFFFu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0xA0000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod datlat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000014u32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000014u32 as *const u32);
                reg &= 0xF0FFFFFFu32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0xA0000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000014u32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000014u32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0xA0000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod busturn {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000014u32 as *const u32) >> 16) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000014u32 as *const u32);
                reg &= 0xFFF0FFFFu32;
                reg |= (val & 0xF) << 16;
                core::ptr::write_volatile(0xA0000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod datast {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000014u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000014u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA0000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod addhld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000014u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000014u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0xA0000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod addset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000014u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000014u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xA0000014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod bcr4 {
    pub mod cburstrw {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod asyncwait {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod extmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod waiten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod wren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod waitcfg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod wrapmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod waitpol {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod bursten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod faccen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod mwid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod mtyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFFFF3u32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod muxen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000018u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod mbken {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000018u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xA0000018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod btr4 {
    pub mod accmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000001Cu32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000001Cu32 as *const u32);
                reg &= 0xCFFFFFFFu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0xA000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datlat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000001Cu32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000001Cu32 as *const u32);
                reg &= 0xF0FFFFFFu32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0xA000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000001Cu32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000001Cu32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0xA000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod busturn {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000001Cu32 as *const u32) >> 16) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000001Cu32 as *const u32);
                reg &= 0xFFF0FFFFu32;
                reg |= (val & 0xF) << 16;
                core::ptr::write_volatile(0xA000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datast {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000001Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000001Cu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addhld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000001Cu32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000001Cu32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0xA000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA000001Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000001Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xA000001Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod pcr2 {
    pub mod eccps {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000060u32 as *const u32) >> 17) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000060u32 as *const u32);
                reg &= 0xFFF1FFFFu32;
                reg |= (val & 0x7) << 17;
                core::ptr::write_volatile(0xA0000060u32 as *mut u32, reg);
            }
        }
    }
    pub mod tar {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000060u32 as *const u32) >> 13) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000060u32 as *const u32);
                reg &= 0xFFFE1FFFu32;
                reg |= (val & 0xF) << 13;
                core::ptr::write_volatile(0xA0000060u32 as *mut u32, reg);
            }
        }
    }
    pub mod tclr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000060u32 as *const u32) >> 9) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000060u32 as *const u32);
                reg &= 0xFFFFE1FFu32;
                reg |= (val & 0xF) << 9;
                core::ptr::write_volatile(0xA0000060u32 as *mut u32, reg);
            }
        }
    }
    pub mod eccen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000060u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000060u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA0000060u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000060u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000060u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0xA0000060u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000060u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000060u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xA0000060u32 as *mut u32, reg);
            }
        }
    }
    pub mod pbken {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000060u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000060u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xA0000060u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwaiten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000060u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000060u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA0000060u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr2 {
    pub mod fempt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000064u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000064u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA0000064u32 as *mut u32, reg);
            }
        }
    }
    pub mod ifen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000064u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000064u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0xA0000064u32 as *mut u32, reg);
            }
        }
    }
    pub mod ilen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000064u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000064u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0xA0000064u32 as *mut u32, reg);
            }
        }
    }
    pub mod iren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000064u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000064u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xA0000064u32 as *mut u32, reg);
            }
        }
    }
    pub mod ifs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000064u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000064u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xA0000064u32 as *mut u32, reg);
            }
        }
    }
    pub mod ils {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000064u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000064u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA0000064u32 as *mut u32, reg);
            }
        }
    }
    pub mod irs {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000064u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000064u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xA0000064u32 as *mut u32, reg);
            }
        }
    }
}

pub mod pmem2 {
    pub mod memhizx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000068u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000068u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xA0000068u32 as *mut u32, reg);
            }
        }
    }
    pub mod memholdx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000068u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000068u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xA0000068u32 as *mut u32, reg);
            }
        }
    }
    pub mod memwaitx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000068u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000068u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA0000068u32 as *mut u32, reg);
            }
        }
    }
    pub mod memsetx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000068u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000068u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xA0000068u32 as *mut u32, reg);
            }
        }
    }
}

pub mod patt2 {
    pub mod atthizx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000006Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000006Cu32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xA000006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod attholdx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000006Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000006Cu32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xA000006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod attwaitx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000006Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000006Cu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA000006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod attsetx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA000006Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000006Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xA000006Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod eccr2 {
    pub mod eccx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000074u32 as *const u32) & 0xFFFFFFFF
            }
        }

    }
}

pub mod pcr3 {
    pub mod eccps {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000080u32 as *const u32) >> 17) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000080u32 as *const u32);
                reg &= 0xFFF1FFFFu32;
                reg |= (val & 0x7) << 17;
                core::ptr::write_volatile(0xA0000080u32 as *mut u32, reg);
            }
        }
    }
    pub mod tar {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000080u32 as *const u32) >> 13) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000080u32 as *const u32);
                reg &= 0xFFFE1FFFu32;
                reg |= (val & 0xF) << 13;
                core::ptr::write_volatile(0xA0000080u32 as *mut u32, reg);
            }
        }
    }
    pub mod tclr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000080u32 as *const u32) >> 9) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000080u32 as *const u32);
                reg &= 0xFFFFE1FFu32;
                reg |= (val & 0xF) << 9;
                core::ptr::write_volatile(0xA0000080u32 as *mut u32, reg);
            }
        }
    }
    pub mod eccen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000080u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000080u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA0000080u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000080u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000080u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0xA0000080u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000080u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000080u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xA0000080u32 as *mut u32, reg);
            }
        }
    }
    pub mod pbken {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000080u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000080u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xA0000080u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwaiten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000080u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000080u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA0000080u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr3 {
    pub mod fempt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000084u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000084u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA0000084u32 as *mut u32, reg);
            }
        }
    }
    pub mod ifen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000084u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000084u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0xA0000084u32 as *mut u32, reg);
            }
        }
    }
    pub mod ilen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000084u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000084u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0xA0000084u32 as *mut u32, reg);
            }
        }
    }
    pub mod iren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000084u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000084u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xA0000084u32 as *mut u32, reg);
            }
        }
    }
    pub mod ifs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000084u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000084u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xA0000084u32 as *mut u32, reg);
            }
        }
    }
    pub mod ils {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000084u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000084u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA0000084u32 as *mut u32, reg);
            }
        }
    }
    pub mod irs {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000084u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000084u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xA0000084u32 as *mut u32, reg);
            }
        }
    }
}

pub mod pmem3 {
    pub mod memhizx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000088u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000088u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xA0000088u32 as *mut u32, reg);
            }
        }
    }
    pub mod memholdx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000088u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000088u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xA0000088u32 as *mut u32, reg);
            }
        }
    }
    pub mod memwaitx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000088u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000088u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA0000088u32 as *mut u32, reg);
            }
        }
    }
    pub mod memsetx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000088u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000088u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xA0000088u32 as *mut u32, reg);
            }
        }
    }
}

pub mod patt3 {
    pub mod atthizx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000008Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000008Cu32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xA000008Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod attholdx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000008Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000008Cu32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xA000008Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod attwaitx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000008Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000008Cu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA000008Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod attsetx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA000008Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000008Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xA000008Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod eccr3 {
    pub mod eccx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000094u32 as *const u32) & 0xFFFFFFFF
            }
        }

    }
}

pub mod pcr4 {
    pub mod eccps {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A0u32 as *const u32) >> 17) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A0u32 as *const u32);
                reg &= 0xFFF1FFFFu32;
                reg |= (val & 0x7) << 17;
                core::ptr::write_volatile(0xA00000A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod tar {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A0u32 as *const u32) >> 13) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A0u32 as *const u32);
                reg &= 0xFFFE1FFFu32;
                reg |= (val & 0xF) << 13;
                core::ptr::write_volatile(0xA00000A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod tclr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A0u32 as *const u32) >> 9) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A0u32 as *const u32);
                reg &= 0xFFFFE1FFu32;
                reg |= (val & 0xF) << 9;
                core::ptr::write_volatile(0xA00000A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod eccen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A0u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A0u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA00000A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A0u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A0u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0xA00000A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A0u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A0u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xA00000A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod pbken {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A0u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A0u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xA00000A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwaiten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A0u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A0u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA00000A0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sr4 {
    pub mod fempt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A4u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A4u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0xA00000A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod ifen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A4u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A4u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0xA00000A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod ilen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A4u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A4u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0xA00000A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod iren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A4u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A4u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0xA00000A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod ifs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A4u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A4u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xA00000A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod ils {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A4u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A4u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xA00000A4u32 as *mut u32, reg);
            }
        }
    }
    pub mod irs {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA00000A4u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A4u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xA00000A4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod pmem4 {
    pub mod memhizx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A8u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A8u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xA00000A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod memholdx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A8u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A8u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xA00000A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod memwaitx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000A8u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A8u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA00000A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod memsetx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA00000A8u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000A8u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xA00000A8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod patt4 {
    pub mod atthizx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000ACu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000ACu32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xA00000ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod attholdx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000ACu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000ACu32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xA00000ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod attwaitx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000ACu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000ACu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA00000ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod attsetx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA00000ACu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000ACu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xA00000ACu32 as *mut u32, reg);
            }
        }
    }
}

pub mod pio4 {
    pub mod iohizx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000B0u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000B0u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xA00000B0u32 as *mut u32, reg);
            }
        }
    }
    pub mod ioholdx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000B0u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000B0u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xA00000B0u32 as *mut u32, reg);
            }
        }
    }
    pub mod iowaitx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA00000B0u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000B0u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA00000B0u32 as *mut u32, reg);
            }
        }
    }
    pub mod iosetx {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA00000B0u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA00000B0u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xA00000B0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod bwtr1 {
    pub mod accmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000104u32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000104u32 as *const u32);
                reg &= 0xCFFFFFFFu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0xA0000104u32 as *mut u32, reg);
            }
        }
    }
    pub mod datlat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000104u32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000104u32 as *const u32);
                reg &= 0xF0FFFFFFu32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0xA0000104u32 as *mut u32, reg);
            }
        }
    }
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000104u32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000104u32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0xA0000104u32 as *mut u32, reg);
            }
        }
    }
    pub mod datast {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000104u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000104u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA0000104u32 as *mut u32, reg);
            }
        }
    }
    pub mod addhld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000104u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000104u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0xA0000104u32 as *mut u32, reg);
            }
        }
    }
    pub mod addset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000104u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000104u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xA0000104u32 as *mut u32, reg);
            }
        }
    }
}

pub mod bwtr2 {
    pub mod accmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000010Cu32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000010Cu32 as *const u32);
                reg &= 0xCFFFFFFFu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0xA000010Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datlat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000010Cu32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000010Cu32 as *const u32);
                reg &= 0xF0FFFFFFu32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0xA000010Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000010Cu32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000010Cu32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0xA000010Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datast {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000010Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000010Cu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA000010Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addhld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000010Cu32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000010Cu32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0xA000010Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA000010Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000010Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xA000010Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod bwtr3 {
    pub mod accmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000114u32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000114u32 as *const u32);
                reg &= 0xCFFFFFFFu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0xA0000114u32 as *mut u32, reg);
            }
        }
    }
    pub mod datlat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000114u32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000114u32 as *const u32);
                reg &= 0xF0FFFFFFu32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0xA0000114u32 as *mut u32, reg);
            }
        }
    }
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000114u32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000114u32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0xA0000114u32 as *mut u32, reg);
            }
        }
    }
    pub mod datast {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000114u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000114u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA0000114u32 as *mut u32, reg);
            }
        }
    }
    pub mod addhld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA0000114u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000114u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0xA0000114u32 as *mut u32, reg);
            }
        }
    }
    pub mod addset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA0000114u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA0000114u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xA0000114u32 as *mut u32, reg);
            }
        }
    }
}

pub mod bwtr4 {
    pub mod accmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000011Cu32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000011Cu32 as *const u32);
                reg &= 0xCFFFFFFFu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0xA000011Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datlat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000011Cu32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000011Cu32 as *const u32);
                reg &= 0xF0FFFFFFu32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0xA000011Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000011Cu32 as *const u32) >> 20) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000011Cu32 as *const u32);
                reg &= 0xFF0FFFFFu32;
                reg |= (val & 0xF) << 20;
                core::ptr::write_volatile(0xA000011Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod datast {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000011Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000011Cu32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xA000011Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addhld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xA000011Cu32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000011Cu32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0xA000011Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xA000011Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xA000011Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xA000011Cu32 as *mut u32, reg);
            }
        }
    }
}

