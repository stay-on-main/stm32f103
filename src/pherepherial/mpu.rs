pub mod mpu_typer {
    pub mod separate {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED90u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED90u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE000ED90u32 as *mut u32, reg);
            }
        }
    }
    pub mod dregion {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED90u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED90u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000ED90u32 as *mut u32, reg);
            }
        }
    }
    pub mod iregion {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED90u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED90u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000ED90u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mpu_ctrl {
    pub mod enable {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED94u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE000ED94u32 as *mut u32, reg);
            }
        }
    }
    pub mod hfnmiena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED94u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE000ED94u32 as *mut u32, reg);
            }
        }
    }
    pub mod privdefena {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED94u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED94u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xE000ED94u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mpu_rnr {
    pub mod region {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED98u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED98u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000ED98u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mpu_rbar {
    pub mod region {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000ED9Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED9Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0xE000ED9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod valid {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED9Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED9Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0xE000ED9Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod addr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000ED9Cu32 as *const u32) >> 5) & 0x7FFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000ED9Cu32 as *const u32);
                reg &= 0xF8000000u32;
                reg |= (val & 0x7FFFFFF) << 5;
                core::ptr::write_volatile(0xE000ED9Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod mpu_rasr {
    pub mod enable {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000EDA0u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod size {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000EDA0u32 as *const u32) >> 1) & 0x1F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFFE0u32;
                reg |= (val & 0x1F) << 1;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod srd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000EDA0u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod b {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000EDA0u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod c {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000EDA0u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod s {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000EDA0u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod tex {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000EDA0u32 as *const u32) >> 19) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 19;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod ap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000EDA0u32 as *const u32) >> 24) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 24;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
    pub mod xn {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000EDA0u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EDA0u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0xE000EDA0u32 as *mut u32, reg);
            }
        }
    }
}

