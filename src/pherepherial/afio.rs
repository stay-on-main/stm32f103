pub mod evcr {
    pub mod pin {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010000u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010000u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40010000u32 as *mut u32, reg);
            }
        }
    }
    pub mod port {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010000u32 as *const u32) >> 4) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010000u32 as *const u32);
                reg &= 0xFFFFFF8Fu32;
                reg |= (val & 0x7) << 4;
                core::ptr::write_volatile(0x40010000u32 as *mut u32, reg);
            }
        }
    }
    pub mod evoe {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010000u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010000u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40010000u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mapr {
    pub mod spi1_remap {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010004u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod i2c1_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod usart1_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod usart2_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod usart3_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim1_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 6) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFFF3Fu32;
                reg |= (val & 0x3) << 6;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim2_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 8) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFFCFFu32;
                reg |= (val & 0x3) << 8;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim3_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 10) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFF3FFu32;
                reg |= (val & 0x3) << 10;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim4_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod can_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 13) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFF9FFFu32;
                reg |= (val & 0x3) << 13;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod pd01_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod tim5ch4_iremap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 16) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFEFFFFu32;
                reg |= (val & 0x1) << 16;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod adc1_etrginj_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 17) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFDFFFFu32;
                reg |= (val & 0x1) << 17;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod adc1_etrgreg_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 18) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFFBFFFFu32;
                reg |= (val & 0x1) << 18;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod adc2_etrginj_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 19) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFF7FFFFu32;
                reg |= (val & 0x1) << 19;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod adc2_etrgreg_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 20) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xFFEFFFFFu32;
                reg |= (val & 0x1) << 20;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
    pub mod swj_cfg {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010004u32 as *const u32) >> 24) & 0x7
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010004u32 as *const u32);
                reg &= 0xF8FFFFFFu32;
                reg |= (val & 0x7) << 24;
                core::ptr::write_volatile(0x40010004u32 as *mut u32, reg);
            }
        }
    }
}

pub mod exticr1 {
    pub mod exti0 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010008u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010008u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40010008u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti1 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010008u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010008u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x40010008u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti2 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010008u32 as *const u32) >> 8) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010008u32 as *const u32);
                reg &= 0xFFFFF0FFu32;
                reg |= (val & 0xF) << 8;
                core::ptr::write_volatile(0x40010008u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti3 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010008u32 as *const u32) >> 12) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010008u32 as *const u32);
                reg &= 0xFFFF0FFFu32;
                reg |= (val & 0xF) << 12;
                core::ptr::write_volatile(0x40010008u32 as *mut u32, reg);
            }
        }
    }
}

pub mod exticr2 {
    pub mod exti4 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x4001000Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001000Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x4001000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod exti5 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001000Cu32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001000Cu32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x4001000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod exti6 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001000Cu32 as *const u32) >> 8) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001000Cu32 as *const u32);
                reg &= 0xFFFFF0FFu32;
                reg |= (val & 0xF) << 8;
                core::ptr::write_volatile(0x4001000Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod exti7 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001000Cu32 as *const u32) >> 12) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001000Cu32 as *const u32);
                reg &= 0xFFFF0FFFu32;
                reg |= (val & 0xF) << 12;
                core::ptr::write_volatile(0x4001000Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod exticr3 {
    pub mod exti8 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010010u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010010u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40010010u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti9 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010010u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010010u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x40010010u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti10 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010010u32 as *const u32) >> 8) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010010u32 as *const u32);
                reg &= 0xFFFFF0FFu32;
                reg |= (val & 0xF) << 8;
                core::ptr::write_volatile(0x40010010u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti11 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010010u32 as *const u32) >> 12) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010010u32 as *const u32);
                reg &= 0xFFFF0FFFu32;
                reg |= (val & 0xF) << 12;
                core::ptr::write_volatile(0x40010010u32 as *mut u32, reg);
            }
        }
    }
}

pub mod exticr4 {
    pub mod exti12 {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40010014u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010014u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40010014u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti13 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010014u32 as *const u32) >> 4) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010014u32 as *const u32);
                reg &= 0xFFFFFF0Fu32;
                reg |= (val & 0xF) << 4;
                core::ptr::write_volatile(0x40010014u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti14 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010014u32 as *const u32) >> 8) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010014u32 as *const u32);
                reg &= 0xFFFFF0FFu32;
                reg |= (val & 0xF) << 8;
                core::ptr::write_volatile(0x40010014u32 as *mut u32, reg);
            }
        }
    }
    pub mod exti15 {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40010014u32 as *const u32) >> 12) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40010014u32 as *const u32);
                reg &= 0xFFFF0FFFu32;
                reg |= (val & 0xF) << 12;
                core::ptr::write_volatile(0x40010014u32 as *mut u32, reg);
            }
        }
    }
}

pub mod mapr2 {
    pub mod tim9_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001001Cu32 as *const u32) >> 5) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001001Cu32 as *const u32);
                reg &= 0xFFFFFFDFu32;
                reg |= (val & 0x1) << 5;
                core::ptr::write_volatile(0x4001001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim10_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001001Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001001Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x4001001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim11_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001001Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001001Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x4001001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim13_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001001Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001001Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x4001001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod tim14_remap {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001001Cu32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001001Cu32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x4001001Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod fsmc_nadv {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x4001001Cu32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x4001001Cu32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x4001001Cu32 as *mut u32, reg);
            }
        }
    }
}

