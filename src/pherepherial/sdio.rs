pub mod power {
    pub mod pwrctrl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018000u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40018000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod clkcr {
    pub mod clkdiv {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018004u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018004u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40018004u32 as *mut u32, reg);
            }
        }
    }
    pub mod clken {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018004u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40018004u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwrsav {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018004u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40018004u32 as *mut u32, reg);
            }
        }
    }
    pub mod bypass {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018004u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40018004u32 as *mut u32, reg);
            }
        }
    }
    pub mod widbus {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018004u32 as *const u32) >> 11) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 11;
                core::ptr::write_volatile(0x40018004u32 as *mut u32, reg);
            }
        }
    }
    pub mod negedge {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018004u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40018004u32 as *mut u32, reg);
            }
        }
    }
    pub mod hwfc_en {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018004u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40018004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod arg {
    pub mod cmdarg {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018008u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018008u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40018008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmd {
    pub mod cmdindex {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001800Cu32 as *const u32) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= val & 0x3F;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod waitresp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001800Cu32 as *const u32) >> 6) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 6;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod waitint {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001800Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod waitpend {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001800Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cpsmen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001800Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sdiosuspend {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001800Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod encmdcompl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001800Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nien {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001800Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ce_atacmd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001800Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001800Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4001800Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod respcmd {
    pub mod respcmd {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018010u32 as *const u32) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018010u32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= val & 0x3F;
                core::ptr::write_volatile(0x40018010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod respi1 {
    pub mod cardstatus1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018014u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018014u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40018014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod resp2 {
    pub mod cardstatus2 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018018u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018018u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40018018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod resp3 {
    pub mod cardstatus3 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001801Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001801Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4001801Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod resp4 {
    pub mod cardstatus4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018020u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018020u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40018020u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dtimer {
    pub mod datatime {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018024u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018024u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40018024u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dlen {
    pub mod datalength {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018028u32 as *const u32) & 0x1FFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018028u32 as *const u32);
                reg &= 0xFE000000u32;
                reg |= val & 0x1FFFFFF;
                core::ptr::write_volatile(0x40018028u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dctrl {
    pub mod dten {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001802Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dtdir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001802Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dtmode {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001802Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dmaen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001802Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dblocksize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001802Cu32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pwstart {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001802Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pwstop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001802Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rwmod {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001802Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sdioen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001802Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001802Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4001802Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dcount {
    pub mod datacount {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018030u32 as *const u32) & 0x1FFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018030u32 as *const u32);
                reg &= 0xFE000000u32;
                reg |= val & 0x1FFFFFF;
                core::ptr::write_volatile(0x40018030u32 as *mut u32, reg);
            }
        }
    }
}

pub mod sta {
    pub mod ccrcfail {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018034u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod dcrcfail {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctimeout {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtimeout {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod txunderr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxoverr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod cmdrend {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod cmdsent {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod dataend {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod stbiterr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbckend {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod cmdact {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod txact {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxact {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfifohe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxfifohf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfifof {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxfifof {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod txfifoe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxfifoe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod txdavl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxdavl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod sdioit {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
    pub mod ceataend {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018034u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018034u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40018034u32 as *mut u32, reg);
            }
        }
    }
}

pub mod icr {
    pub mod ccrcfailc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018038u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod dcrcfailc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctimeoutc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtimeoutc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod txunderrc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxoverrc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod cmdrendc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod cmdsentc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod dataendc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod stbiterrc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbckendc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod sdioitc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
    pub mod ceataendc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40018038u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018038u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40018038u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mask {
    pub mod ccrcfailie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001803Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dcrcfailie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ctimeoutie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dtimeoutie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txunderrie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rxoverrie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cmdrendie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cmdsentie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dataendie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stbiterrie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dbackendie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod cmdactie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txactie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rxactie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txfifoheie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rxfifohfie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txfifofie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rxfifofie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txfifoeie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rxfifoeie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txdavlie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rxdavlie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sdioitie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ceatendie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001803Cu32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001803Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x4001803Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod fifocnt {
    pub mod fif0count {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018048u32 as *const u32) & 0xFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018048u32 as *const u32);
                reg &= 0xFF000000u32;
                reg |= val & 0xFFFFFF;
                core::ptr::write_volatile(0x40018048u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fifo {
    pub mod fifodata {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40018080u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40018080u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40018080u32 as *mut u32, reg);
            }
        }
    }
}

