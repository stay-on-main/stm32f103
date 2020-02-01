pub mod isr {
    pub mod gif1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020400u32 as *const u32) & 0x1
            }
        }

    }
    pub mod tcif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 1) & 0x1
            }
        }

    }
    pub mod htif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 2) & 0x1
            }
        }

    }
    pub mod teif1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 3) & 0x1
            }
        }

    }
    pub mod gif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 4) & 0x1
            }
        }

    }
    pub mod tcif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 5) & 0x1
            }
        }

    }
    pub mod htif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 6) & 0x1
            }
        }

    }
    pub mod teif2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 7) & 0x1
            }
        }

    }
    pub mod gif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 8) & 0x1
            }
        }

    }
    pub mod tcif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 9) & 0x1
            }
        }

    }
    pub mod htif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 10) & 0x1
            }
        }

    }
    pub mod teif3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 11) & 0x1
            }
        }

    }
    pub mod gif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 12) & 0x1
            }
        }

    }
    pub mod tcif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 13) & 0x1
            }
        }

    }
    pub mod htif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 14) & 0x1
            }
        }

    }
    pub mod teif4 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 15) & 0x1
            }
        }

    }
    pub mod gif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 16) & 0x1
            }
        }

    }
    pub mod tcif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 17) & 0x1
            }
        }

    }
    pub mod htif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 18) & 0x1
            }
        }

    }
    pub mod teif5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 19) & 0x1
            }
        }

    }
    pub mod gif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 20) & 0x1
            }
        }

    }
    pub mod tcif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 21) & 0x1
            }
        }

    }
    pub mod htif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 22) & 0x1
            }
        }

    }
    pub mod teif6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 23) & 0x1
            }
        }

    }
    pub mod gif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 24) & 0x1
            }
        }

    }
    pub mod tcif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 25) & 0x1
            }
        }

    }
    pub mod htif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 26) & 0x1
            }
        }

    }
    pub mod teif7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020400u32 as *const u32) >> 27) & 0x1
            }
        }

    }
}

pub mod ifcr {
    pub mod cgif1 {
        pub fn set(val: u32) {
            unsafe {
                let reg = val & 0x1;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif2 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif3 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 8;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif4 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 12;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif5 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 16;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif6 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 20;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cgif7 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 24;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif1 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif2 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif3 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 9;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif4 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 13;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif5 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 17;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif6 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 21;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctcif7 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 25;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif1 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif2 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif3 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 10;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif4 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif5 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 18;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif6 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 22;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod chtif7 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 26;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif1 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif2 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif3 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 11;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif4 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 15;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif5 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 19;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif6 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 23;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
    pub mod cteif7 {
        pub fn set(val: u32) {
            unsafe {
                let reg = (val & 0x1) << 27;
                core::ptr::write_volatile(0x40020404u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr1 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020408u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020408u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020408u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020408u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr1 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002040Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002040Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4002040Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar1 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020410u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020410u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020410u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar1 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020414u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020414u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020414u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr2 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002041Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002041Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002041Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4002041Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr2 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020420u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020420u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020420u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar2 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020424u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020424u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020424u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar2 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020428u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020428u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020428u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr3 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020430u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020430u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020430u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020430u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr3 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020434u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020434u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020434u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar3 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020438u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020438u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020438u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar3 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002043Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002043Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002043Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr4 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020444u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020444u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020444u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020444u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr4 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020448u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020448u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020448u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar4 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002044Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002044Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002044Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar4 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020450u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020450u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020450u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr5 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020458u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020458u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020458u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020458u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr5 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002045Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002045Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x4002045Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar5 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020460u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020460u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020460u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar5 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020464u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020464u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020464u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr6 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002046Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4002046Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002046Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x4002046Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr6 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020470u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020470u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020470u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar6 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020474u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020474u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020474u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar6 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020478u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020478u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020478u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ccr7 {
    pub mod en {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020480u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod htie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod teie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod circ {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod pinc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod minc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod psize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod msize {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod pl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
    pub mod mem2mem {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40020480u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020480u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40020480u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cndtr7 {
    pub mod ndt {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020484u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020484u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40020484u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cpar7 {
    pub mod pa {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40020488u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40020488u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x40020488u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cmar7 {
    pub mod ma {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4002048Cu32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4002048Cu32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0x4002048Cu32 as *mut u32, reg);
            }
        }
    }
}

