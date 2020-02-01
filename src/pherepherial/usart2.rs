pub mod sr {
    pub mod cts {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod lbd {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod txe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod tc {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod rxne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod idle {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod ore {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod ne {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod fe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004400u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
    pub mod pe {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40004400u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40004400u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr {
    pub mod dr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40004404u32 as *const u32) & 0x1FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004404u32 as *const u32);
                reg &= 0xFFFFFE00u32;
                reg |= val & 0x1FF;
                core::ptr::write_volatile(0x40004404u32 as *mut u32, reg);
            }
        }
    }
}

pub mod brr {
    pub mod div_mantissa {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004408u32 as *const u32) >> 4) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004408u32 as *const u32);
                reg &= 0xFFFF000Fu32;
                reg |= (val & 0xFFF) << 4;
                core::ptr::write_volatile(0x40004408u32 as *mut u32, reg);
            }
        }
    }
    pub mod div_fraction {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40004408u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004408u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40004408u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr1 {
    pub mod ue {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod m {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod wake {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod pce {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ps {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod peie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod txeie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tcie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rxneie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod idleie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod te {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod re {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod rwu {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000440Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod sbk {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000440Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000440Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x4000440Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr2 {
    pub mod linen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004410u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
    pub mod stop {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004410u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
    pub mod clken {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004410u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
    pub mod cpol {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004410u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
    pub mod cpha {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004410u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
    pub mod lbcl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004410u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
    pub mod lbdie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004410u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
    pub mod lbdl {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004410u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
    pub mod add {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40004410u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004410u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40004410u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr3 {
    pub mod ctsie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctse {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod rtse {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod dmat {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod dmar {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod scen {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod nack {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod hdsel {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod irlp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod iren {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004414u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
    pub mod eie {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40004414u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004414u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40004414u32 as *mut u32, reg);
            }
        }
    }
}

pub mod gtpr {
    pub mod gt {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40004418u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004418u32 as *const u32);
                reg &= 0xFFFF00FFu32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x40004418u32 as *mut u32, reg);
            }
        }
    }
    pub mod psc {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40004418u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40004418u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40004418u32 as *mut u32, reg);
            }
        }
    }
}

