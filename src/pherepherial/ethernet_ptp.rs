pub mod ptptscr {
    pub mod tse {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028700u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028700u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40028700u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsfcu {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028700u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028700u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40028700u32 as *mut u32, reg);
            }
        }
    }
    pub mod tssti {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028700u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028700u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40028700u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsstu {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028700u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028700u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40028700u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsite {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028700u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028700u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40028700u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsaru {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028700u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028700u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40028700u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ptpssir {
    pub mod stssi {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028704u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028704u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40028704u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ptptshr {
    pub mod sts {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028708u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028708u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028708u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ptptslr {
    pub mod stss {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002870Cu32 as *const u32) & 0x7FFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002870Cu32 as *const u32);
                reg &= 0x80000000u32;
                reg |= val & 0x7FFFFFFF;
                core::ptr::write_volatile(0x4002870Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stpns {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002870Cu32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002870Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x4002870Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ptptshur {
    pub mod tsus {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028710u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028710u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028710u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ptptslur {
    pub mod tsuss {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028714u32 as *const u32) & 0x7FFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028714u32 as *const u32);
                reg &= 0x80000000u32;
                reg |= val & 0x7FFFFFFF;
                core::ptr::write_volatile(0x40028714u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsupns {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40028714u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028714u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40028714u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ptptsar {
    pub mod tsa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028718u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028718u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028718u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ptptthr {
    pub mod ttsh {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002871Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002871Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002871Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ptpttlr {
    pub mod ttsl {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40028720u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40028720u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40028720u32 as *mut u32, reg);
            }
        }
    }
}

