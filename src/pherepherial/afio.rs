pub struct Evcr {
   raw: u32,
}

impl Evcr {
    #[inline(always)]
    pub fn pin_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn pin(mut self, val: u32) -> Evcr {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn port_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn port(mut self, val: u32) -> Evcr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn evoe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn evoe(mut self, val: u32) -> Evcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40010000u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod evcr {
    #[inline(always)]
    pub fn read() -> super::Evcr {
        super::Evcr {
            raw: unsafe { *((0x40010000u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Evcr) {
       unsafe { *((0x40010000u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Mapr {
   raw: u32,
}

impl Mapr {
    #[inline(always)]
    pub fn spi1_remap_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn spi1_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn i2c1_remap_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn i2c1_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn usart1_remap_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usart1_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn usart2_remap_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usart2_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn usart3_remap_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn usart3_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn tim1_remap_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn tim1_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 6)) | ((val & ((1 << 2) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn tim2_remap_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn tim2_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn tim3_remap_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn tim3_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn tim4_remap_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim4_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn can_remap_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn can_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 13)) | ((val & ((1 << 2) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn pd01_remap_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pd01_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn tim5ch4_iremap_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim5ch4_iremap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn adc1_etrginj_remap_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc1_etrginj_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn adc1_etrgreg_remap_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc1_etrgreg_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn adc2_etrginj_remap_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc2_etrginj_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn adc2_etrgreg_remap_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc2_etrgreg_remap(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn swj_cfg_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn swj_cfg(mut self, val: u32) -> Mapr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 24)) | ((val & ((1 << 3) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40010000u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod mapr {
    #[inline(always)]
    pub fn read() -> super::Mapr {
        super::Mapr {
            raw: unsafe { *((0x40010000u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mapr) {
       unsafe { *((0x40010000u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Exticr1 {
   raw: u32,
}

impl Exticr1 {
    #[inline(always)]
    pub fn exti0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti0(mut self, val: u32) -> Exticr1 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn exti1_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti1(mut self, val: u32) -> Exticr1 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn exti2_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti2(mut self, val: u32) -> Exticr1 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 8)) | ((val & ((1 << 4) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn exti3_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti3(mut self, val: u32) -> Exticr1 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 12)) | ((val & ((1 << 4) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40010000u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod exticr1 {
    #[inline(always)]
    pub fn read() -> super::Exticr1 {
        super::Exticr1 {
            raw: unsafe { *((0x40010000u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Exticr1) {
       unsafe { *((0x40010000u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Exticr2 {
   raw: u32,
}

impl Exticr2 {
    #[inline(always)]
    pub fn exti4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti4(mut self, val: u32) -> Exticr2 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn exti5_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti5(mut self, val: u32) -> Exticr2 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn exti6_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti6(mut self, val: u32) -> Exticr2 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 8)) | ((val & ((1 << 4) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn exti7_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti7(mut self, val: u32) -> Exticr2 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 12)) | ((val & ((1 << 4) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40010000u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod exticr2 {
    #[inline(always)]
    pub fn read() -> super::Exticr2 {
        super::Exticr2 {
            raw: unsafe { *((0x40010000u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Exticr2) {
       unsafe { *((0x40010000u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Exticr3 {
   raw: u32,
}

impl Exticr3 {
    #[inline(always)]
    pub fn exti8_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti8(mut self, val: u32) -> Exticr3 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn exti9_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti9(mut self, val: u32) -> Exticr3 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn exti10_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti10(mut self, val: u32) -> Exticr3 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 8)) | ((val & ((1 << 4) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn exti11_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti11(mut self, val: u32) -> Exticr3 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 12)) | ((val & ((1 << 4) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40010000u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod exticr3 {
    #[inline(always)]
    pub fn read() -> super::Exticr3 {
        super::Exticr3 {
            raw: unsafe { *((0x40010000u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Exticr3) {
       unsafe { *((0x40010000u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Exticr4 {
   raw: u32,
}

impl Exticr4 {
    #[inline(always)]
    pub fn exti12_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti12(mut self, val: u32) -> Exticr4 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn exti13_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti13(mut self, val: u32) -> Exticr4 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn exti14_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti14(mut self, val: u32) -> Exticr4 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 8)) | ((val & ((1 << 4) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn exti15_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn exti15(mut self, val: u32) -> Exticr4 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 12)) | ((val & ((1 << 4) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40010000u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod exticr4 {
    #[inline(always)]
    pub fn read() -> super::Exticr4 {
        super::Exticr4 {
            raw: unsafe { *((0x40010000u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Exticr4) {
       unsafe { *((0x40010000u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Mapr2 {
   raw: u32,
}

impl Mapr2 {
    #[inline(always)]
    pub fn tim9_remap_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim9_remap(mut self, val: u32) -> Mapr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn tim10_remap_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim10_remap(mut self, val: u32) -> Mapr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn tim11_remap_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim11_remap(mut self, val: u32) -> Mapr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn tim13_remap_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim13_remap(mut self, val: u32) -> Mapr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn tim14_remap_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim14_remap(mut self, val: u32) -> Mapr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fsmc_nadv_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsmc_nadv(mut self, val: u32) -> Mapr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40010000u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod mapr2 {
    #[inline(always)]
    pub fn read() -> super::Mapr2 {
        super::Mapr2 {
            raw: unsafe { *((0x40010000u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mapr2) {
       unsafe { *((0x40010000u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

