pub mod actrl {
    pub mod disfold {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E008u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E008u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xE000E008u32 as *mut u32, reg);
            }
        }
    }
    pub mod fpexcodis {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E008u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E008u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0xE000E008u32 as *mut u32, reg);
            }
        }
    }
    pub mod disramode {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E008u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E008u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xE000E008u32 as *mut u32, reg);
            }
        }
    }
    pub mod disitmatbflush {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E008u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E008u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0xE000E008u32 as *mut u32, reg);
            }
        }
    }
}

