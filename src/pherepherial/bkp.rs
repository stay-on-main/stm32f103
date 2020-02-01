pub mod dr1 {
    pub mod d1 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C00u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C00u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C00u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr2 {
    pub mod d2 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C04u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C04u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C04u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr3 {
    pub mod d3 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C08u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C08u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C08u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr4 {
    pub mod d4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C0Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C0Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C0Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr5 {
    pub mod d5 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C10u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C10u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C10u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr6 {
    pub mod d6 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C14u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C14u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C14u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr7 {
    pub mod d7 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C18u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C18u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C18u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr8 {
    pub mod d8 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C1Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C1Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C1Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr9 {
    pub mod d9 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C20u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C20u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C20u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr10 {
    pub mod d10 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C24u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C24u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C24u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr11 {
    pub mod dr11 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C3Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C3Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C3Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr12 {
    pub mod dr12 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C40u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C40u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C40u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr13 {
    pub mod dr13 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C44u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C44u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C44u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr14 {
    pub mod d14 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C48u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C48u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C48u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr15 {
    pub mod d15 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C4Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C4Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C4Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr16 {
    pub mod d16 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C50u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C50u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C50u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr17 {
    pub mod d17 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C54u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C54u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C54u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr18 {
    pub mod d18 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C58u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C58u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C58u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr19 {
    pub mod d19 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C5Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C5Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C5Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr20 {
    pub mod d20 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C60u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C60u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C60u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr21 {
    pub mod d21 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C64u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C64u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C64u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr22 {
    pub mod d22 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C68u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C68u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C68u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr23 {
    pub mod d23 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C6Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C6Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C6Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr24 {
    pub mod d24 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C70u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C70u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C70u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr25 {
    pub mod d25 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C74u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C74u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C74u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr26 {
    pub mod d26 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C78u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C78u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C78u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr27 {
    pub mod d27 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C7Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C7Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C7Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr28 {
    pub mod d28 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C80u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C80u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C80u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr29 {
    pub mod d29 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C84u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C84u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C84u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr30 {
    pub mod d30 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C88u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C88u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C88u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr31 {
    pub mod d31 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C8Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C8Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C8Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr32 {
    pub mod d32 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C90u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C90u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C90u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr33 {
    pub mod d33 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C94u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C94u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C94u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr34 {
    pub mod d34 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C98u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C98u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C98u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr35 {
    pub mod d35 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C9Cu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C9Cu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006C9Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr36 {
    pub mod d36 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006CA0u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006CA0u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006CA0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr37 {
    pub mod d37 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006CA4u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006CA4u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006CA4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr38 {
    pub mod d38 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006CA8u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006CA8u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006CA8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr39 {
    pub mod d39 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006CACu32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006CACu32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006CACu32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr40 {
    pub mod d40 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006CB0u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006CB0u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006CB0u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr41 {
    pub mod d41 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006CB4u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006CB4u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006CB4u32 as *mut u32, reg);
            }
        }
    }
}

pub mod dr42 {
    pub mod d42 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006CB8u32 as *const u32) & 0xFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006CB8u32 as *const u32);
                reg &= 0xFFFF0000u32;
                reg |= val & 0xFFFF;
                core::ptr::write_volatile(0x40006CB8u32 as *mut u32, reg);
            }
        }
    }
}

pub mod rtccr {
    pub mod cal {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C28u32 as *const u32) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C28u32 as *const u32);
                reg &= 0xFFFFFF80u32;
                reg |= val & 0x7F;
                core::ptr::write_volatile(0x40006C28u32 as *mut u32, reg);
            }
        }
    }
    pub mod cco {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006C28u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C28u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40006C28u32 as *mut u32, reg);
            }
        }
    }
    pub mod asoe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006C28u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C28u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006C28u32 as *mut u32, reg);
            }
        }
    }
    pub mod asos {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006C28u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C28u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006C28u32 as *mut u32, reg);
            }
        }
    }
}

pub mod cr {
    pub mod tpe {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C2Cu32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C2Cu32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006C2Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tpal {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006C2Cu32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C2Cu32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006C2Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod csr {
    pub mod cte {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40006C30u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C30u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40006C30u32 as *mut u32, reg);
            }
        }
    }
    pub mod cti {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006C30u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C30u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40006C30u32 as *mut u32, reg);
            }
        }
    }
    pub mod tpie {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006C30u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C30u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40006C30u32 as *mut u32, reg);
            }
        }
    }
    pub mod tef {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006C30u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C30u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40006C30u32 as *mut u32, reg);
            }
        }
    }
    pub mod tif {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40006C30u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40006C30u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40006C30u32 as *mut u32, reg);
            }
        }
    }
}

