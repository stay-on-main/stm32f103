pub mod isr {
    pub mod gif1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020000u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod htif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod teif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod gif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod htif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod teif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod gif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod htif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod teif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod gif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod htif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod teif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod gif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod htif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod teif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod gif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod htif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod teif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod gif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod htif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
    pub mod teif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020000u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020000u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40020000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ifcr {
    pub mod cgif1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020004u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 24) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 24;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 21) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 21;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 25) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 25;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 22) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 22;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 26) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 26;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 23) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 23;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020004u32 as *const u32) >> 27) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 27;
                core::ptr::write_volatile(0x40020004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr1 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020008u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020008u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020008u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr1 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002000Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002000Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4002000Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar1 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020010u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020010u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar1 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020014u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020014u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr2 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002001Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002001Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002001Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4002001Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr2 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020020u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020020u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020020u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar2 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020024u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020024u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020024u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar2 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020028u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020028u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020028u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr3 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020030u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020030u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020030u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020030u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr3 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020034u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020034u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020034u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar3 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020038u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020038u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020038u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar3 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002003Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002003Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002003Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr4 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020044u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020044u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020044u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020044u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr4 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020048u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020048u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020048u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar4 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002004Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002004Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002004Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar4 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020050u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020050u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020050u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr5 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020058u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020058u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020058u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020058u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr5 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002005Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002005Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4002005Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar5 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020060u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020060u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020060u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar5 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020064u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020064u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020064u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr6 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002006Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002006Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002006Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4002006Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr6 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020070u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020070u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020070u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar6 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020074u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020074u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020074u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar6 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020078u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020078u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020078u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr7 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020080u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020080u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020080u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020080u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr7 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020084u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020084u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020084u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar7 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020088u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020088u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020088u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar7 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002008Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002008Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002008Cu32 as *mut u32, reg);
            }
        }
    }
}

