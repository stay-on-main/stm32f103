pub mod cr {
    pub mod lpds {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007000u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40007000u32 as *mut u32, reg);
            }
        }
    }
    pub mod pdds {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007000u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40007000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cwuf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007000u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40007000u32 as *mut u32, reg);
            }
        }
    }
    pub mod csbf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007000u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40007000u32 as *mut u32, reg);
            }
        }
    }
    pub mod pvde {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007000u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40007000u32 as *mut u32, reg);
            }
        }
    }
    pub mod pls {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007000u32 as *const u32) >> 5) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007000u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 5;
                core::ptr::write_volatile(0x40007000u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007000u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40007000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod csr {
    pub mod wuf {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007004u32 as *const u32) & 0x1
            }
        }

    }
    pub mod sbf {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007004u32 as *const u32) >> 1) & 0x1
            }
        }

    }
    pub mod pvdo {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007004u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod ewup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007004u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40007004u32 as *mut u32, reg);
            }
        }
    }
}

