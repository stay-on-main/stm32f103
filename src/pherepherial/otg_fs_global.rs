pub mod fs_gotgctl {
    pub mod srqscs {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000000u32 as *const u32) & 0x1
            }
        }

    }
    pub mod srq {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hngscs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 8) & 0x1
            }
        }

    }
    pub mod hnprq {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x50000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod hshnpen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod dhnpen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x50000000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cidsts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 16) & 0x1
            }
        }

    }
    pub mod dbct {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod asvld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 18) & 0x1
            }
        }

    }
    pub mod bsvld {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000000u32 as *const u32) >> 19) & 0x1
            }
        }

    }
}

pub mod fs_gotgint {
    pub mod sedet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000004u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x50000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod srsschg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000004u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod hnsschg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000004u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x50000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod hngdet {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000004u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x50000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod adtochg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000004u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x50000004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbcdne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000004u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x50000004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_gahbcfg {
    pub mod gint {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000008u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfelvl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000008u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000008u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptxfelvl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000008u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x50000008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_gusbcfg {
    pub mod tocal {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000000Cu32 as *const u32) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000000Cu32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= val & 0x7;
                core::ptr::write_volatile(0x5000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod physel {
        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000000Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x5000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod srpcap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000000Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000000Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x5000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod hnpcap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000000Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000000Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x5000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod trdt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000000Cu32 as *const u32) >> 10) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000000Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 10;
                core::ptr::write_volatile(0x5000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fhmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000000Cu32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000000Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x5000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fdmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000000Cu32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000000Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x5000000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ctxpkt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000000Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000000Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x5000000Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_grstctl {
    pub mod csrst {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x50000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod hsrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000010u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod fcrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000010u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x50000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxfflsh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000010u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfflsh {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000010u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000010u32 as *const u32) >> 6) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000010u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= (val & 0x1F) << 6;
                core::ptr::write_volatile(0x50000010u32 as *mut u32, reg);
            }
        }
    }
    pub mod ahbidl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000010u32 as *const u32) >> 31) & 0x1
            }
        }

    }
}

pub mod fs_gintsts {
    pub mod cmod {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000014u32 as *const u32) & 0x1
            }
        }

    }
    pub mod mmis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod otgint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod sof {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxflvl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 4) & 0x1
            }
        }

    }
    pub mod nptxfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 5) & 0x1
            }
        }

    }
    pub mod ginakeff {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 6) & 0x1
            }
        }

    }
    pub mod goutnakeff {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod esusp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod usbsusp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod usbrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod enumdne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod isoodrp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod eopf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod iepint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 18) & 0x1
            }
        }

    }
    pub mod oepint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 19) & 0x1
            }
        }

    }
    pub mod iisoixfr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipxfr_incompisoout {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod hprtint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 24) & 0x1
            }
        }

    }
    pub mod hcint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 25) & 0x1
            }
        }

    }
    pub mod ptxfe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 26) & 0x1
            }
        }

    }
    pub mod cidschg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod discint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod srqint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
    pub mod wkupint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000014u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_gintmsk {
    pub mod mmism {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod otgint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod sofm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxflvlm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod nptxfem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod ginakeffm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod gonakeffm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod esuspm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod usbsuspm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod usbrst {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod enumdnem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod isoodrpm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod eopfm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod epmism {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iepint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod oepint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod iisoixfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipxfrm_iisooxfrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod prtim {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 24) & 0x1
            }
        }

    }
    pub mod hcim {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptxfem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod cidschgm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod discint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod srqim {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
    pub mod wuim {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000018u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x50000018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_grxstsr_device {
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000001Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000001Cu32 as *const u32) >> 4) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= (val & 0x7FF) << 4;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000001Cu32 as *const u32) >> 15) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 15;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pktsts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000001Cu32 as *const u32) >> 17) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 17;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000001Cu32 as *const u32) >> 21) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 21;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_grxstsr_host {
    pub mod epnum {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000001Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod bcnt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000001Cu32 as *const u32) >> 4) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= (val & 0x7FF) << 4;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dpid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000001Cu32 as *const u32) >> 15) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 15;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pktsts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000001Cu32 as *const u32) >> 17) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 17;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod frmnum {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000001Cu32 as *const u32) >> 21) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000001Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 21;
                core::ptr::write_volatile(0x5000001Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_grxfsiz {
    pub mod rxfd {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000024u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000024u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000024u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_gnptxfsiz_device {
    pub mod tx0fsa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000028u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000028u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000028u32 as *mut u32, reg);
            }
        }
    }
    pub mod tx0fd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000028u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000028u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x50000028u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_gnptxfsiz_host {
    pub mod nptxfsa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000028u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000028u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000028u32 as *mut u32, reg);
            }
        }
    }
    pub mod nptxfd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000028u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000028u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x50000028u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_gnptxsts {
    pub mod nptxfsav {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000002Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000002Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x5000002Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nptqxsav {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000002Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000002Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0x5000002Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nptxqtop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000002Cu32 as *const u32) >> 24) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000002Cu32 as *const u32);
                reg &= 0xFFFFFF80u32;
                reg |= (val & 0x7F) << 24;
                core::ptr::write_volatile(0x5000002Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_gccfg {
    pub mod pwrdwn {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000038u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x50000038u32 as *mut u32, reg);
            }
        }
    }
    pub mod vbusasen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000038u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x50000038u32 as *mut u32, reg);
            }
        }
    }
    pub mod vbusbsen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000038u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x50000038u32 as *mut u32, reg);
            }
        }
    }
    pub mod sofouten {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000038u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x50000038u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_cid {
    pub mod product_id {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000003Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000003Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x5000003Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_hptxfsiz {
    pub mod ptxsa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000100u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000100u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000100u32 as *mut u32, reg);
            }
        }
    }
    pub mod ptxfsiz {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000100u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000100u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x50000100u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_dieptxf1 {
    pub mod ineptxsa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000104u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000104u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000104u32 as *mut u32, reg);
            }
        }
    }
    pub mod ineptxfd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000104u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000104u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x50000104u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_dieptxf2 {
    pub mod ineptxsa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x50000108u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000108u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x50000108u32 as *mut u32, reg);
            }
        }
    }
    pub mod ineptxfd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x50000108u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x50000108u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x50000108u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fs_dieptxf3 {
    pub mod ineptxsa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x5000010Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000010Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x5000010Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ineptxfd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x5000010Cu32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x5000010Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0x5000010Cu32 as *mut u32, reg);
            }
        }
    }
}

