pub mod cr {
    pub mod en1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007400u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod boff1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod ten1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsel1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 3) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 3;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod wave1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 6) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 6;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mamp1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 8) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 8;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod dmaen1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod en2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod boff2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod ten2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod tsel2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 19) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFF8u32;
                reg |= (val & 0x7) << 19;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod wave2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 22) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFCu32;
                reg |= (val & 0x3) << 22;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod mamp2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 24) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= (val & 0xF) << 24;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
    pub mod dmaen2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007400u32 as *const u32) >> 28) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007400u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 28;
                core::ptr::write_volatile(0x40007400u32 as *mut u32, reg);
            }
        }
    }
}

pub mod swtrigr {
    pub mod swtrig1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007404u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007404u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40007404u32 as *mut u32, reg);
            }
        }
    }
    pub mod swtrig2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007404u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007404u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40007404u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr12r1 {
    pub mod dacc1dhr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007408u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007408u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40007408u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr12l1 {
    pub mod dacc1dhr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4000740Cu32 as *const u32) >> 4) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000740Cu32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= (val & 0xFFF) << 4;
                core::ptr::write_volatile(0x4000740Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr8r1 {
    pub mod dacc1dhr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007410u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007410u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40007410u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr12r2 {
    pub mod dacc2dhr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007414u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007414u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40007414u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr12l2 {
    pub mod dacc2dhr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007418u32 as *const u32) >> 4) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007418u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= (val & 0xFFF) << 4;
                core::ptr::write_volatile(0x40007418u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr8r2 {
    pub mod dacc2dhr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000741Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000741Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x4000741Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr12rd {
    pub mod dacc1dhr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007420u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007420u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40007420u32 as *mut u32, reg);
            }
        }
    }
    pub mod dacc2dhr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007420u32 as *const u32) >> 16) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007420u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= (val & 0xFFF) << 16;
                core::ptr::write_volatile(0x40007420u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr12ld {
    pub mod dacc1dhr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007424u32 as *const u32) >> 4) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007424u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= (val & 0xFFF) << 4;
                core::ptr::write_volatile(0x40007424u32 as *mut u32, reg);
            }
        }
    }
    pub mod dacc2dhr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007424u32 as *const u32) >> 20) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007424u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= (val & 0xFFF) << 20;
                core::ptr::write_volatile(0x40007424u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dhr8rd {
    pub mod dacc1dhr {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007428u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007428u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0x40007428u32 as *mut u32, reg);
            }
        }
    }
    pub mod dacc2dhr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40007428u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007428u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0x40007428u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dor1 {
    pub mod dacc1dor {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4000742Cu32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4000742Cu32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x4000742Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dor2 {
    pub mod dacc2dor {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40007430u32 as *const u32) & 0xFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40007430u32 as *const u32);
                reg &= 0xFFFFF000u32;
                reg |= val & 0xFFF;
                core::ptr::write_volatile(0x40007430u32 as *mut u32, reg);
            }
        }
    }
}

