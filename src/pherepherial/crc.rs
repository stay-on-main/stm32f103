pub mod dr {
    pub mod dr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40023000u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40023000u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40023000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod idr {
    pub mod idr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40023004u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40023004u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40023004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr {
    pub mod reset {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40023008u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40023008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40023008u32 as *mut u32, reg);
            }
        }
    }
}

