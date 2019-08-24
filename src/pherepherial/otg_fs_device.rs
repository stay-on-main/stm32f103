pub mod fs_dcfg {
    pub mod dspd {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000800u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000800u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x50000800u32 as *mut u32, reg);
            }
        }
    }
    pub mod nzlsohsk {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000800u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000800u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x50000800u32 as *mut u32, reg);
            }
        }
    }
    pub mod dad {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000800u32 as *const u32) >> 4) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000800u32 as *const u32);
                reg &= 0xFFFFFF80u32;
                reg |= (val & 0x7F) << 4;
                core::ptr::write_volatile(0x50000800u32 as *mut u32, reg);
            }
        }
    }
    pub mod pfivl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000800u32 as *const u32) >> 11) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000800u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 11;
                core::ptr::write_volatile(0x50000800u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_dctl {
    pub mod rwusig {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000804u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000804u32 as *mut u32, reg);
            }
        }
    }
    pub mod sdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000804u32 as *mut u32, reg);
            }
        }
    }
    pub mod ginsts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod gonsts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 3) & 0x1
            }
        }

    }
    pub mod tctl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000804u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x50000804u32 as *mut u32, reg);
            }
        }
    }
    pub mod sginak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000804u32 as *mut u32, reg);
            }
        }
    }
    pub mod cginak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000804u32 as *mut u32, reg);
            }
        }
    }
    pub mod sgonak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x50000804u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgonak {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000804u32 as *mut u32, reg);
            }
        }
    }
    pub mod poprgdne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000804u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000804u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x50000804u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_dsts {
    pub mod suspsts {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000808u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000808u32 as *mut u32, reg);
            }
        }
    }
    pub mod enumspd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000808u32 as *const u32) >> 1) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000808u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 1;
                core::ptr::write_volatile(0x50000808u32 as *mut u32, reg);
            }
        }
    }
    pub mod eerr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000808u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000808u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000808u32 as *mut u32, reg);
            }
        }
    }
    pub mod fnsof {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000808u32 as *const u32) >> 8) & 0x3FFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000808u32 as *const u32);
                reg &= 0xFFFFC000u32;
                reg |= (val & 0x3FFF) << 8;
                core::ptr::write_volatile(0x50000808u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_diepmsk {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000810u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000810u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000810u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000810u32 as *mut u32, reg);
            }
        }
    }
    pub mod tom {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000810u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000810u32 as *mut u32, reg);
            }
        }
    }
    pub mod ittxfemsk {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000810u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000810u32 as *mut u32, reg);
            }
        }
    }
    pub mod inepnmm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000810u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000810u32 as *mut u32, reg);
            }
        }
    }
    pub mod inepnem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000810u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000810u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000810u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_doepmsk {
    pub mod xfrcm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000814u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000814u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000814u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000814u32 as *mut u32, reg);
            }
        }
    }
    pub mod stupm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000814u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000814u32 as *mut u32, reg);
            }
        }
    }
    pub mod otepdm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000814u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000814u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000814u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_daint {
    pub mod iepint {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000818u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000818u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000818u32 as *mut u32, reg);
            }
        }
    }
    pub mod oepint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000818u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000818u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x50000818u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_daintmsk {
    pub mod iepm {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000081Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000081Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x5000081Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod oepint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000081Cu32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000081Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x5000081Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dvbusdis {
    pub mod vbusdt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000828u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000828u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000828u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dvbuspulse {
    pub mod dvbusp {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000082Cu32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000082Cu32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x5000082Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod diepempmsk {
    pub mod ineptxfem {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000834u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000834u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000834u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_diepctl0 {
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000900u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000900u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x50000900u32 as *mut u32, reg);
            }
        }
    }
    pub mod usbaep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000900u32 as *const u32) >> 15) & 0x1
            }
        }

    }
    pub mod naksts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000900u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000900u32 as *const u32) >> 18) & 0x3
            }
        }

    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000900u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000900u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000900u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000900u32 as *const u32) >> 22) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000900u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 22;
                core::ptr::write_volatile(0x50000900u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000900u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000900u32 as *mut u32, reg);
            }
        }
    }
    pub mod snak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000900u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x50000900u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000900u32 as *const u32) >> 30) & 0x1
            }
        }

    }
    pub mod epena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000900u32 as *const u32) >> 31) & 0x1
            }
        }

    }
}

pub mod diepctl1 {
    pub mod epena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000920u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000920u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod soddfrm_sd1pid {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod sd0pid_sevnfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod snak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000920u32 as *const u32) >> 22) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 22;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000920u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000920u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod naksts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000920u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod eonum_dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000920u32 as *const u32) >> 16) & 0x1
            }
        }

    }
    pub mod usbaep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000920u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000920u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000920u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000920u32 as *mut u32, reg);
            }
        }
    }
}

pub mod diepctl2 {
    pub mod epena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000940u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000940u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod soddfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod sd0pid_sevnfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod snak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000940u32 as *const u32) >> 22) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 22;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000940u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000940u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod naksts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000940u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod eonum_dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000940u32 as *const u32) >> 16) & 0x1
            }
        }

    }
    pub mod usbaep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000940u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000940u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000940u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000940u32 as *mut u32, reg);
            }
        }
    }
}

pub mod diepctl3 {
    pub mod epena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000960u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000960u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod soddfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod sd0pid_sevnfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod snak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000960u32 as *const u32) >> 22) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 22;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000960u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000960u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod naksts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000960u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod eonum_dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000960u32 as *const u32) >> 16) & 0x1
            }
        }

    }
    pub mod usbaep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000960u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000960u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000960u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000960u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doepctl0 {
    pub mod epena {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000B00u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B00u32 as *const u32) >> 30) & 0x1
            }
        }

    }
    pub mod snak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x50000B00u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000B00u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B00u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000B00u32 as *mut u32, reg);
            }
        }
    }
    pub mod snpm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B00u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B00u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x50000B00u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B00u32 as *const u32) >> 18) & 0x3
            }
        }

    }
    pub mod naksts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B00u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod usbaep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B00u32 as *const u32) >> 15) & 0x1
            }
        }

    }
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B00u32 as *const u32) & 0x3
            }
        }

    }
}

pub mod doepctl1 {
    pub mod epena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B20u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B20u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod soddfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod sd0pid_sevnfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod snak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B20u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod snpm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B20u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B20u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod naksts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B20u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod eonum_dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B20u32 as *const u32) >> 16) & 0x1
            }
        }

    }
    pub mod usbaep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B20u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B20u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B20u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000B20u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doepctl2 {
    pub mod epena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B40u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B40u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod soddfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod sd0pid_sevnfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod snak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B40u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod snpm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B40u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B40u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod naksts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B40u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod eonum_dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B40u32 as *const u32) >> 16) & 0x1
            }
        }

    }
    pub mod usbaep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B40u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B40u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B40u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000B40u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doepctl3 {
    pub mod epena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B60u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B60u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod soddfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod sd0pid_sevnfrm {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod snak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnak {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod stall {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B60u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod snpm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B60u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod eptyp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B60u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod naksts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B60u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod eonum_dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B60u32 as *const u32) >> 16) & 0x1
            }
        }

    }
    pub mod usbaep {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B60u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
    pub mod mpsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B60u32 as *const u32) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B60u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= val & 0x7FF;
                core::ptr::write_volatile(0x50000B60u32 as *mut u32, reg);
            }
        }
    }
}

pub mod diepint0 {
    pub mod txfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000908u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod inepne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000908u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000908u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000908u32 as *mut u32, reg);
            }
        }
    }
    pub mod ittxfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000908u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000908u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000908u32 as *mut u32, reg);
            }
        }
    }
    pub mod toc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000908u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000908u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000908u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdisd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000908u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000908u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000908u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000908u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000908u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000908u32 as *mut u32, reg);
            }
        }
    }
}

pub mod diepint1 {
    pub mod txfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000928u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod inepne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000928u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000928u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000928u32 as *mut u32, reg);
            }
        }
    }
    pub mod ittxfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000928u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000928u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000928u32 as *mut u32, reg);
            }
        }
    }
    pub mod toc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000928u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000928u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000928u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdisd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000928u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000928u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000928u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000928u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000928u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000928u32 as *mut u32, reg);
            }
        }
    }
}

pub mod diepint2 {
    pub mod txfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000948u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod inepne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000948u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000948u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000948u32 as *mut u32, reg);
            }
        }
    }
    pub mod ittxfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000948u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000948u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000948u32 as *mut u32, reg);
            }
        }
    }
    pub mod toc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000948u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000948u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000948u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdisd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000948u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000948u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000948u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000948u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000948u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000948u32 as *mut u32, reg);
            }
        }
    }
}

pub mod diepint3 {
    pub mod txfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000968u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod inepne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000968u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000968u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000968u32 as *mut u32, reg);
            }
        }
    }
    pub mod ittxfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000968u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000968u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000968u32 as *mut u32, reg);
            }
        }
    }
    pub mod toc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000968u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000968u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000968u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdisd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000968u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000968u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000968u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000968u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000968u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000968u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doepint0 {
    pub mod b2bstup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B08u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000B08u32 as *mut u32, reg);
            }
        }
    }
    pub mod otepdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B08u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000B08u32 as *mut u32, reg);
            }
        }
    }
    pub mod stup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B08u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000B08u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdisd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B08u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000B08u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B08u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B08u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000B08u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doepint1 {
    pub mod b2bstup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B28u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B28u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000B28u32 as *mut u32, reg);
            }
        }
    }
    pub mod otepdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B28u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B28u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000B28u32 as *mut u32, reg);
            }
        }
    }
    pub mod stup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B28u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B28u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000B28u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdisd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B28u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B28u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000B28u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B28u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B28u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000B28u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doepint2 {
    pub mod b2bstup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B48u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000B48u32 as *mut u32, reg);
            }
        }
    }
    pub mod otepdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B48u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000B48u32 as *mut u32, reg);
            }
        }
    }
    pub mod stup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B48u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000B48u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdisd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B48u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000B48u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B48u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B48u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000B48u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doepint3 {
    pub mod b2bstup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B68u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000B68u32 as *mut u32, reg);
            }
        }
    }
    pub mod otepdis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B68u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000B68u32 as *mut u32, reg);
            }
        }
    }
    pub mod stup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B68u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000B68u32 as *mut u32, reg);
            }
        }
    }
    pub mod epdisd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B68u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000B68u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B68u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B68u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000B68u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dieptsiz0 {
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000910u32 as *const u32) >> 19) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000910u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 19;
                core::ptr::write_volatile(0x50000910u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000910u32 as *const u32) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000910u32 as *const u32);
                reg &= 0xFFFFFF80u32;
                reg |= val & 0x7F;
                core::ptr::write_volatile(0x50000910u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doeptsiz0 {
    pub mod stupcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B10u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B10u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000B10u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B10u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B10u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x50000B10u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B10u32 as *const u32) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B10u32 as *const u32);
                reg &= 0xFFFFFF80u32;
                reg |= val & 0x7F;
                core::ptr::write_volatile(0x50000B10u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dieptsiz1 {
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000930u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000930u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000930u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000930u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000930u32 as *const u32);
                reg &= 0xFFFFFC00u32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000930u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000930u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000930u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000930u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dieptsiz2 {
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000950u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000950u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000950u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000950u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000950u32 as *const u32);
                reg &= 0xFFFFFC00u32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000950u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000950u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000950u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000950u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dieptsiz3 {
    pub mod mcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000970u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000970u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000970u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000970u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000970u32 as *const u32);
                reg &= 0xFFFFFC00u32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000970u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000970u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000970u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000970u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dtxfsts0 {
    pub mod ineptfsav {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000918u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000918u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000918u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dtxfsts1 {
    pub mod ineptfsav {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000938u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000938u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000938u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dtxfsts2 {
    pub mod ineptfsav {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000958u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000958u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000958u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dtxfsts3 {
    pub mod ineptfsav {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000978u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000978u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000978u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doeptsiz1 {
    pub mod rxdpid_stupcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B30u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B30u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000B30u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B30u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B30u32 as *const u32);
                reg &= 0xFFFFFC00u32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000B30u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B30u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B30u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000B30u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doeptsiz2 {
    pub mod rxdpid_stupcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B50u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B50u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000B50u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B50u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B50u32 as *const u32);
                reg &= 0xFFFFFC00u32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000B50u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B50u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B50u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000B50u32 as *mut u32, reg);
            }
        }
    }
}

pub mod doeptsiz3 {
    pub mod rxdpid_stupcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B70u32 as *const u32) >> 29) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B70u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 29;
                core::ptr::write_volatile(0x50000B70u32 as *mut u32, reg);
            }
        }
    }
    pub mod pktcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000B70u32 as *const u32) >> 19) & 0x3FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B70u32 as *const u32);
                reg &= 0xFFFFFC00u32;
                reg |= (val & 0x3FF) << 19;
                core::ptr::write_volatile(0x50000B70u32 as *mut u32, reg);
            }
        }
    }
    pub mod xfrsiz {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000B70u32 as *const u32) & 0x7FFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000B70u32 as *const u32);
                reg &= 0xFFF80000u32;
                reg |= val & 0x7FFFF;
                core::ptr::write_volatile(0x50000B70u32 as *mut u32, reg);
            }
        }
    }
}

