pub mod iser0 {
    pub mod setena {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E100u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E100u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E100u32 as *mut u32, reg);
            }
        }
    }
}

pub mod iser1 {
    pub mod setena {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E104u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E104u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E104u32 as *mut u32, reg);
            }
        }
    }
}

pub mod icer0 {
    pub mod clrena {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E180u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E180u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E180u32 as *mut u32, reg);
            }
        }
    }
}

pub mod icer1 {
    pub mod clrena {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E184u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E184u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E184u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ispr0 {
    pub mod setpend {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E200u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E200u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E200u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ispr1 {
    pub mod setpend {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E204u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E204u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E204u32 as *mut u32, reg);
            }
        }
    }
}

pub mod icpr0 {
    pub mod clrpend {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E280u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E280u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E280u32 as *mut u32, reg);
            }
        }
    }
}

pub mod icpr1 {
    pub mod clrpend {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E284u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E284u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E284u32 as *mut u32, reg);
            }
        }
    }
}

pub mod iabr0 {
    pub mod active {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E300u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E300u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E300u32 as *mut u32, reg);
            }
        }
    }
}

pub mod iabr1 {
    pub mod active {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E304u32 as *const u32) & 0xFFFFFFFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E304u32 as *const u32);
                reg &= 0x0u32;
                reg |= val & 0xFFFFFFFF;
                core::ptr::write_volatile(0xE000E304u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr0 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E400u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E400u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E400u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E400u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E400u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E400u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E400u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E400u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E400u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E400u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E400u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E400u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr1 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E404u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E404u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E404u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E404u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E404u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E404u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E404u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E404u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E404u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E404u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr2 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E408u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E408u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E408u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E408u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E408u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E408u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E408u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E408u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E408u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E408u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E408u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E408u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr3 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E40Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E40Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E40Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E40Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E40Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E40Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E40Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E40Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E40Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E40Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E40Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E40Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr4 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E410u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E410u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E410u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E410u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E410u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E410u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E410u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E410u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E410u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E410u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E410u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E410u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr5 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E414u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E414u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E414u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E414u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E414u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E414u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E414u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E414u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E414u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E414u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E414u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E414u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr6 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E418u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E418u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E418u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E418u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E418u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E418u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E418u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E418u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E418u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E418u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E418u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E418u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr7 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E41Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E41Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E41Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E41Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E41Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E41Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E41Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E41Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E41Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E41Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E41Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E41Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr8 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E420u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E420u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E420u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E420u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E420u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E420u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E420u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E420u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E420u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E420u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E420u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E420u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr9 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E424u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E424u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E424u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E424u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E424u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E424u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E424u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E424u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E424u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E424u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E424u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E424u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr10 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E428u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E428u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E428u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E428u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E428u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E428u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E428u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E428u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E428u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E428u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E428u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E428u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr11 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E42Cu32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E42Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E42Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E42Cu32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E42Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E42Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E42Cu32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E42Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E42Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E42Cu32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E42Cu32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E42Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr12 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E430u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E430u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E430u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E430u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E430u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E430u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E430u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E430u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E430u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E430u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E430u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E430u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr13 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E434u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E434u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E434u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E434u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E434u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E434u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E434u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E434u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E434u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E434u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E434u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E434u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ipr14 {
    pub mod ipr_n0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000E438u32 as *const u32) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E438u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= val & 0xFF;
                core::ptr::write_volatile(0xE000E438u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E438u32 as *const u32) >> 8) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E438u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 8;
                core::ptr::write_volatile(0xE000E438u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E438u32 as *const u32) >> 16) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E438u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 16;
                core::ptr::write_volatile(0xE000E438u32 as *mut u32, reg);
            }
        }
    }
    pub mod ipr_n3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0xE000E438u32 as *const u32) >> 24) & 0xFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000E438u32 as *const u32);
                reg &= 0xFFFFFF00u32;
                reg |= (val & 0xFF) << 24;
                core::ptr::write_volatile(0xE000E438u32 as *mut u32, reg);
            }
        }
    }
}

