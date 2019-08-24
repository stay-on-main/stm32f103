pub mod crl {
    pub mod mode0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40011000u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 6) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 6;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 14) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 14;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 16) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 16;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 22) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 22;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 24) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 24;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 26) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 26;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011000u32 as *const u32) >> 30) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011000u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 30;
                core::ptr::write_volatile(0x40011000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod crh {
    pub mod mode8 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40011004u32 as *const u32) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= val & 0x3;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 2) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 2;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 6) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 6;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 14) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 14;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 16) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 16;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 18) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 18;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 20) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 20;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 22) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 22;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 24) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 24;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 26) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 26;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod mode15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 28) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 28;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cnf15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011004u32 as *const u32) >> 30) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011004u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 30;
                core::ptr::write_volatile(0x40011004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod idr {
    pub mod idr0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40011008u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
    pub mod idr15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011008u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40011008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod odr {
    pub mod odr0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001100Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod odr15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001100Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001100Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x4001100Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod bsrr {
    pub mod bs0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40011010u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod bs15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br0 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 29) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 29;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 30) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 30;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
    pub mod br15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011010u32 as *const u32) >> 31) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011010u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 31;
                core::ptr::write_volatile(0x40011010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod brr {
    pub mod br0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40011014u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
    pub mod br15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011014u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011014u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40011014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod lckr {
    pub mod lck0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40011018u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck8 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck12 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lck15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
    pub mod lckk {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40011018u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40011018u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40011018u32 as *mut u32, reg);
            }
        }
    }
}

