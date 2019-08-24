pub mod idcode {
    pub mod dev_id {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE0042000u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042000u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0xE0042000u32 as *mut u32, reg);
            }
        }
    }
    pub mod rev_id {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042000u32 as *const u32) >> 16) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042000u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= (val & 0xFFFF) << 16;
                core::ptr::write_volatile(0xE0042000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr {
    pub mod dbg_sleep {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE0042004u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_standby {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod trace_ioen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod trace_mode {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 6) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 6;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_iwdg_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_wwdg_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_tim1_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_tim2_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_tim3_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_tim4_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_can1_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_i2c1_smbus_timeout {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_i2c2_smbus_timeout {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_tim8_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_tim5_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_tim6_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_tim7_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
    pub mod dbg_can2_stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE0042004u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE0042004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0xE0042004u32 as *mut u32, reg);
            }
        }
    }
}

