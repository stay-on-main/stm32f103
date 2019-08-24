pub mod dmabmr {
    pub mod sr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029000u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod da {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod dsl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 2) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= (val & 0x1F) << 2;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod pbl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 8) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= (val & 0x3F) << 8;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtpr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 14) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 14;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod fb {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod rdp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 17) & 0x3F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFC0u32;
                reg |= (val & 0x3F) << 17;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod usp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod fpm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
    pub mod aab {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029000u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40029000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmatpdr {
    pub mod tpd {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029004u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029004u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40029004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmarpdr {
    pub mod rpd {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029008u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029008u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40029008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmardlar {
    pub mod srl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002900Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002900Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002900Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmatdlar {
    pub mod stl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029010u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029010u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40029010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmasr {
    pub mod ts {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029014u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod tpss {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod tbus {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod tjts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod ros {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod tus {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod rs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod rbus {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod rpss {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod pwts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod ets {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod fbes {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod ers {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod ais {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod nis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40029014u32 as *mut u32, reg);
            }
        }
    }
    pub mod rps {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 17) & 0x7
            }
        }

    }
    pub mod tps {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 20) & 0x7
            }
        }

    }
    pub mod ebs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 23) & 0x7
            }
        }

    }
    pub mod mmcs {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 27) & 0x1
            }
        }

    }
    pub mod pmts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 28) & 0x1
            }
        }

    }
    pub mod tsts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029014u32 as *const u32) >> 29) & 0x1
            }
        }

    }
}

pub mod dmaomr {
    pub mod sr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod osf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 3) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 3;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod fugf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod fef {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod st {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod ttc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 14) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 14;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod ftf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod dfrf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod rsf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtcefd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029018u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40029018u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmaier {
    pub mod tie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002901Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tpsie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tbuie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tjtie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod roie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tuie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rbuie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rpsie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rwtie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod etie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fbeie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod erie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod aise {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod nise {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002901Cu32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002901Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x4002901Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmamfbocr {
    pub mod mfc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029020u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029020u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40029020u32 as *mut u32, reg);
            }
        }
    }
    pub mod omfc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029020u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029020u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40029020u32 as *mut u32, reg);
            }
        }
    }
    pub mod mfa {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029020u32 as *const u32) >> 17) & 0x7FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029020u32 as *const u32);
                reg &= 0xFFFFF800u32;
                reg |= (val & 0x7FF) << 17;
                core::ptr::write_volatile(0x40029020u32 as *mut u32, reg);
            }
        }
    }
    pub mod ofoc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40029020u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029020u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40029020u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmachtdr {
    pub mod htdap {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029048u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029048u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40029048u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmachrdr {
    pub mod hrdap {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002904Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002904Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002904Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmachtbar {
    pub mod htbap {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029050u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029050u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40029050u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dmachrbar {
    pub mod hrbap {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40029054u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40029054u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40029054u32 as *mut u32, reg);
            }
        }
    }
}

