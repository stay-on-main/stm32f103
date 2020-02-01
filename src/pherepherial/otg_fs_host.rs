pub mod fs_hcfg {
    pub mod fslspcs {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000400u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000400u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x50000400u32 as *mut u32, reg);
            }
        }
    }
    pub mod fslss {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000400u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000400u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x50000400u32 as *mut u32, reg);
            }
        }
    }
}

pub mod hfir {
    pub mod frivl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000404u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000404u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000404u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hfnum {
    pub mod frnum {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000408u32 as *const u32) & 0xFFFF
            }
        }

    }
    pub mod ftrem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000408u32 as *const u32) >> 16) & 0xFFFF
            }
        }

    }
}

pub mod fs_hptxsts {
    pub mod ptxfsavl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000410u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000410u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000410u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptxqsav {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000410u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000410u32 as *const u32);
                reg &= 0xFF00FFFFu32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x50000410u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptxqtop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000410u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000410u32 as *const u32);
                reg &= 0xFFFFFFu32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0x50000410u32 as *mut u32, reg);
            }
        }
    }
}

pub mod haint {
    pub mod haint {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000414u32 as *const u32) & 0xFFFF
            }
        }

    }
}

pub mod haintmsk {
    pub mod haintm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000418u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000418u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000418u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hprt {
    pub mod pcsts {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000440u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod pcdet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod pena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod penchng {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod poca {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod pocchng {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod pres {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod psusp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod prst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod plsts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod ppwr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptctl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 13) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFFE1FFFu32;
                reg |= (val & 0xF) << 13;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
    pub mod pspd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000440u32 as *const u32) >> 17) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000440u32 as *const u32);
                reg &= 0xFFF9FFFFu32;
                reg |= (val & 0x3) << 17;
                core::ptr::write_volatile(0x50000440u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcchar0 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000500u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 11) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xFFFF87FFu32;
                reg |= (val & 0xF) << 11;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsdev {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xFFF3FFFFu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 22) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xE03FFFFFu32;
                reg |= (val & 0x7F) << 22;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod oddfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod chdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
    pub mod chena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000500u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000500u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000500u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcchar1 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000520u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 11) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xFFFF87FFu32;
                reg |= (val & 0xF) << 11;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsdev {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xFFF3FFFFu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 22) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xE03FFFFFu32;
                reg |= (val & 0x7F) << 22;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod oddfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod chdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
    pub mod chena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000520u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000520u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000520u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcchar2 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000540u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 11) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xFFFF87FFu32;
                reg |= (val & 0xF) << 11;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsdev {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xFFF3FFFFu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 22) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xE03FFFFFu32;
                reg |= (val & 0x7F) << 22;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod oddfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod chdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
    pub mod chena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000540u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000540u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000540u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcchar3 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000560u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 11) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xFFFF87FFu32;
                reg |= (val & 0xF) << 11;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsdev {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xFFF3FFFFu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 22) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xE03FFFFFu32;
                reg |= (val & 0x7F) << 22;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod oddfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod chdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
    pub mod chena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000560u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000560u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000560u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcchar4 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000580u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 11) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xFFFF87FFu32;
                reg |= (val & 0xF) << 11;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsdev {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xFFF3FFFFu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 22) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xE03FFFFFu32;
                reg |= (val & 0x7F) << 22;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod oddfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod chdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
    pub mod chena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000580u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000580u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000580u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcchar5 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005A0u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 11) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xFFFF87FFu32;
                reg |= (val & 0xF) << 11;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsdev {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xFFF3FFFFu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 22) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xE03FFFFFu32;
                reg |= (val & 0x7F) << 22;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod oddfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod chdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
    pub mod chena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A0u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A0u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x500005A0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcchar6 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005C0u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 11) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xFFFF87FFu32;
                reg |= (val & 0xF) << 11;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsdev {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xFFF3FFFFu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 22) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xE03FFFFFu32;
                reg |= (val & 0x7F) << 22;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod oddfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod chdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
    pub mod chena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C0u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C0u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x500005C0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcchar7 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005E0u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 11) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xFFFF87FFu32;
                reg |= (val & 0xF) << 11;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod lsdev {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xFFF3FFFFu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xFFCFFFFFu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 22) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xE03FFFFFu32;
                reg |= (val & 0x7F) << 22;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod oddfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xDFFFFFFFu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod chdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0xBFFFFFFFu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
    pub mod chena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E0u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E0u32 as *const u32);
                reg &= 0x7FFFFFFFu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x500005E0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcint0 {
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000508u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
    pub mod chh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000508u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000508u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
    pub mod nak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000508u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000508u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
    pub mod txerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000508u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
    pub mod bberr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000508u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
    pub mod frmor {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000508u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
    pub mod dterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000508u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000508u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000508u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcint1 {
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000528u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
    pub mod chh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000528u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000528u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
    pub mod nak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000528u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000528u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
    pub mod txerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000528u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
    pub mod bberr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000528u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
    pub mod frmor {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000528u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
    pub mod dterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000528u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000528u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000528u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcint2 {
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000548u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
    pub mod chh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000548u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000548u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
    pub mod nak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000548u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000548u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
    pub mod txerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000548u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
    pub mod bberr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000548u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
    pub mod frmor {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000548u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
    pub mod dterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000548u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000548u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000548u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcint3 {
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000568u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
    pub mod chh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000568u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000568u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
    pub mod nak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000568u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000568u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
    pub mod txerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000568u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
    pub mod bberr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000568u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
    pub mod frmor {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000568u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
    pub mod dterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000568u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000568u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000568u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcint4 {
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000588u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
    pub mod chh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000588u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000588u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
    pub mod nak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000588u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000588u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
    pub mod txerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000588u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
    pub mod bberr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000588u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
    pub mod frmor {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000588u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
    pub mod dterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000588u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000588u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000588u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcint5 {
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005A8u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod chh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A8u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A8u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod nak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A8u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A8u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod txerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A8u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod bberr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A8u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod frmor {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A8u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
    pub mod dterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005A8u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005A8u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x500005A8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcint6 {
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005C8u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod chh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C8u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C8u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod nak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C8u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C8u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod txerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C8u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod bberr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C8u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod frmor {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C8u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
    pub mod dterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005C8u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005C8u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x500005C8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcint7 {
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005E8u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
    pub mod chh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E8u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E8u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
    pub mod nak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E8u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
    pub mod ack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E8u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
    pub mod txerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E8u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
    pub mod bberr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E8u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
    pub mod frmor {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E8u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
    pub mod dterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005E8u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005E8u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x500005E8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcintmsk0 {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000050Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod chhm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stallm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nakm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ackm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nyet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txerrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bberrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmorm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dterrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000050Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000050Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x5000050Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcintmsk1 {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000052Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod chhm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stallm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nakm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ackm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nyet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txerrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bberrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmorm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dterrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000052Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000052Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x5000052Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcintmsk2 {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000054Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod chhm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stallm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nakm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ackm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nyet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txerrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bberrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmorm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dterrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000054Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000054Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x5000054Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcintmsk3 {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000056Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod chhm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stallm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nakm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ackm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nyet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txerrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bberrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmorm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dterrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000056Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000056Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x5000056Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcintmsk4 {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000058Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod chhm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stallm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nakm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ackm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nyet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txerrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bberrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmorm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dterrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000058Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000058Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x5000058Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcintmsk5 {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005ACu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod chhm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod stallm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod nakm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod ackm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod nyet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod txerrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod bberrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmorm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
    pub mod dterrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ACu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ACu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x500005ACu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcintmsk6 {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005CCu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod chhm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod stallm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod nakm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod ackm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod nyet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod txerrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod bberrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmorm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
    pub mod dterrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005CCu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005CCu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x500005CCu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hcintmsk7 {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005ECu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod chhm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod stallm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod nakm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod ackm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod nyet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod txerrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod bberrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmorm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
    pub mod dterrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005ECu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005ECu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x500005ECu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hctsiz0 {
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000510u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000510u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000510u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000510u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000510u32 as *const u32);
                reg &= 0xE007FFFFu32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000510u32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000510u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000510u32 as *const u32);
                reg &= 0x9FFFFFFFu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000510u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hctsiz1 {
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000530u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000530u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000530u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000530u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000530u32 as *const u32);
                reg &= 0xE007FFFFu32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000530u32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000530u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000530u32 as *const u32);
                reg &= 0x9FFFFFFFu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000530u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hctsiz2 {
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000550u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000550u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000550u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000550u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000550u32 as *const u32);
                reg &= 0xE007FFFFu32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000550u32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000550u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000550u32 as *const u32);
                reg &= 0x9FFFFFFFu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000550u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hctsiz3 {
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000570u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000570u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000570u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000570u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000570u32 as *const u32);
                reg &= 0xE007FFFFu32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000570u32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000570u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000570u32 as *const u32);
                reg &= 0x9FFFFFFFu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000570u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hctsiz4 {
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000590u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000590u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000590u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000590u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000590u32 as *const u32);
                reg &= 0xE007FFFFu32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000590u32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000590u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000590u32 as *const u32);
                reg &= 0x9FFFFFFFu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000590u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hctsiz5 {
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005B0u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005B0u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x500005B0u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005B0u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005B0u32 as *const u32);
                reg &= 0xE007FFFFu32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x500005B0u32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005B0u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005B0u32 as *const u32);
                reg &= 0x9FFFFFFFu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x500005B0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hctsiz6 {
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005D0u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005D0u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x500005D0u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005D0u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005D0u32 as *const u32);
                reg &= 0xE007FFFFu32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x500005D0u32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005D0u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005D0u32 as *const u32);
                reg &= 0x9FFFFFFFu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x500005D0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hctsiz7 {
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x500005F0u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005F0u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x500005F0u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005F0u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005F0u32 as *const u32);
                reg &= 0xE007FFFFu32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x500005F0u32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x500005F0u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x500005F0u32 as *const u32);
                reg &= 0x9FFFFFFFu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x500005F0u32 as *mut u32, reg);
            }
        }
    }
}

