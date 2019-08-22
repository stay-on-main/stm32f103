pub struct Isr {
   raw: u32,
}

impl Isr {
    #[inline(always)]
    pub fn gif1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif1(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tcif1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif1(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn htif1_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif1(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn teif1_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif1(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn gif2_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif2(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn tcif2_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif2(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn htif2_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif2(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn teif2_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif2(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn gif3_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif3(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn tcif3_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif3(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn htif3_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif3(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn teif3_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif3(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn gif4_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif4(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn tcif4_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif4(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn htif4_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif4(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn teif4_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif4(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn gif5_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif5(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn tcif5_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif5(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn htif5_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif5(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn teif5_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif5(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn gif6_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif6(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn tcif6_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif6(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn htif6_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif6(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn teif6_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif6(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn gif7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif7(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn tcif7_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif7(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn htif7_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif7(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn teif7_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif7(mut self, val: u32) -> Isr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod isr {
    #[inline(always)]
    pub fn read() -> super::Isr {
        super::Isr {
            raw: unsafe { *((0x40020400u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Isr) {
       unsafe { *((0x40020400u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Ifcr {
   raw: u32,
}

impl Ifcr {
    #[inline(always)]
    pub fn cgif1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif1(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn cgif2_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif2(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn cgif3_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif3(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn cgif4_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif4(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn cgif5_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif5(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn cgif6_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif6(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn cgif7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif7(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn ctcif1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif1(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ctcif2_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif2(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn ctcif3_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif3(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn ctcif4_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif4(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn ctcif5_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif5(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn ctcif6_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif6(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn ctcif7_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif7(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn chtif1_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif1(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn chtif2_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif2(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn chtif3_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif3(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn chtif4_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif4(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn chtif5_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif5(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn chtif6_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif6(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn chtif7_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif7(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn cteif1_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif1(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn cteif2_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif2(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn cteif3_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif3(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn cteif4_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif4(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn cteif5_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif5(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn cteif6_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif6(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn cteif7_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif7(mut self, val: u32) -> Ifcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod ifcr {
    #[inline(always)]
    pub fn read() -> super::Ifcr {
        super::Ifcr {
            raw: unsafe { *((0x40020400u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ifcr) {
       unsafe { *((0x40020400u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr1 {
   raw: u32,
}

impl Ccr1 {
    #[inline(always)]
    pub fn en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr1 {
    #[inline(always)]
    pub fn read() -> super::Ccr1 {
        super::Ccr1 {
            raw: unsafe { *((0x40020400u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr1) {
       unsafe { *((0x40020400u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Cndtr1 {
   raw: u32,
}

impl Cndtr1 {
    #[inline(always)]
    pub fn ndt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ndt(mut self, val: u32) -> Cndtr1 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod cndtr1 {
    #[inline(always)]
    pub fn read() -> super::Cndtr1 {
        super::Cndtr1 {
            raw: unsafe { *((0x40020400u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr1) {
       unsafe { *((0x40020400u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Cpar1 {
   raw: u32,
}

impl Cpar1 {
    #[inline(always)]
    pub fn pa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn pa(mut self, val: u32) -> Cpar1 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod cpar1 {
    #[inline(always)]
    pub fn read() -> super::Cpar1 {
        super::Cpar1 {
            raw: unsafe { *((0x40020400u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar1) {
       unsafe { *((0x40020400u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Cmar1 {
   raw: u32,
}

impl Cmar1 {
    #[inline(always)]
    pub fn ma_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ma(mut self, val: u32) -> Cmar1 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod cmar1 {
    #[inline(always)]
    pub fn read() -> super::Cmar1 {
        super::Cmar1 {
            raw: unsafe { *((0x40020400u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar1) {
       unsafe { *((0x40020400u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr2 {
   raw: u32,
}

impl Ccr2 {
    #[inline(always)]
    pub fn en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod ccr2 {
    #[inline(always)]
    pub fn read() -> super::Ccr2 {
        super::Ccr2 {
            raw: unsafe { *((0x40020400u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr2) {
       unsafe { *((0x40020400u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Cndtr2 {
   raw: u32,
}

impl Cndtr2 {
    #[inline(always)]
    pub fn ndt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ndt(mut self, val: u32) -> Cndtr2 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x20u32) as *mut u32) = self.raw; }
    }
}

pub mod cndtr2 {
    #[inline(always)]
    pub fn read() -> super::Cndtr2 {
        super::Cndtr2 {
            raw: unsafe { *((0x40020400u32 + 0x20u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr2) {
       unsafe { *((0x40020400u32 + 0x20u32) as *mut u32) = val.raw; }
    }
}

pub struct Cpar2 {
   raw: u32,
}

impl Cpar2 {
    #[inline(always)]
    pub fn pa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn pa(mut self, val: u32) -> Cpar2 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x24u32) as *mut u32) = self.raw; }
    }
}

pub mod cpar2 {
    #[inline(always)]
    pub fn read() -> super::Cpar2 {
        super::Cpar2 {
            raw: unsafe { *((0x40020400u32 + 0x24u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar2) {
       unsafe { *((0x40020400u32 + 0x24u32) as *mut u32) = val.raw; }
    }
}

pub struct Cmar2 {
   raw: u32,
}

impl Cmar2 {
    #[inline(always)]
    pub fn ma_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ma(mut self, val: u32) -> Cmar2 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x28u32) as *mut u32) = self.raw; }
    }
}

pub mod cmar2 {
    #[inline(always)]
    pub fn read() -> super::Cmar2 {
        super::Cmar2 {
            raw: unsafe { *((0x40020400u32 + 0x28u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar2) {
       unsafe { *((0x40020400u32 + 0x28u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr3 {
   raw: u32,
}

impl Ccr3 {
    #[inline(always)]
    pub fn en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x30u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr3 {
    #[inline(always)]
    pub fn read() -> super::Ccr3 {
        super::Ccr3 {
            raw: unsafe { *((0x40020400u32 + 0x30u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr3) {
       unsafe { *((0x40020400u32 + 0x30u32) as *mut u32) = val.raw; }
    }
}

pub struct Cndtr3 {
   raw: u32,
}

impl Cndtr3 {
    #[inline(always)]
    pub fn ndt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ndt(mut self, val: u32) -> Cndtr3 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x34u32) as *mut u32) = self.raw; }
    }
}

pub mod cndtr3 {
    #[inline(always)]
    pub fn read() -> super::Cndtr3 {
        super::Cndtr3 {
            raw: unsafe { *((0x40020400u32 + 0x34u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr3) {
       unsafe { *((0x40020400u32 + 0x34u32) as *mut u32) = val.raw; }
    }
}

pub struct Cpar3 {
   raw: u32,
}

impl Cpar3 {
    #[inline(always)]
    pub fn pa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn pa(mut self, val: u32) -> Cpar3 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x38u32) as *mut u32) = self.raw; }
    }
}

pub mod cpar3 {
    #[inline(always)]
    pub fn read() -> super::Cpar3 {
        super::Cpar3 {
            raw: unsafe { *((0x40020400u32 + 0x38u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar3) {
       unsafe { *((0x40020400u32 + 0x38u32) as *mut u32) = val.raw; }
    }
}

pub struct Cmar3 {
   raw: u32,
}

impl Cmar3 {
    #[inline(always)]
    pub fn ma_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ma(mut self, val: u32) -> Cmar3 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x3Cu32) as *mut u32) = self.raw; }
    }
}

pub mod cmar3 {
    #[inline(always)]
    pub fn read() -> super::Cmar3 {
        super::Cmar3 {
            raw: unsafe { *((0x40020400u32 + 0x3Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar3) {
       unsafe { *((0x40020400u32 + 0x3Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr4 {
   raw: u32,
}

impl Ccr4 {
    #[inline(always)]
    pub fn en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x44u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr4 {
    #[inline(always)]
    pub fn read() -> super::Ccr4 {
        super::Ccr4 {
            raw: unsafe { *((0x40020400u32 + 0x44u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr4) {
       unsafe { *((0x40020400u32 + 0x44u32) as *mut u32) = val.raw; }
    }
}

pub struct Cndtr4 {
   raw: u32,
}

impl Cndtr4 {
    #[inline(always)]
    pub fn ndt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ndt(mut self, val: u32) -> Cndtr4 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x48u32) as *mut u32) = self.raw; }
    }
}

pub mod cndtr4 {
    #[inline(always)]
    pub fn read() -> super::Cndtr4 {
        super::Cndtr4 {
            raw: unsafe { *((0x40020400u32 + 0x48u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr4) {
       unsafe { *((0x40020400u32 + 0x48u32) as *mut u32) = val.raw; }
    }
}

pub struct Cpar4 {
   raw: u32,
}

impl Cpar4 {
    #[inline(always)]
    pub fn pa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn pa(mut self, val: u32) -> Cpar4 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x4Cu32) as *mut u32) = self.raw; }
    }
}

pub mod cpar4 {
    #[inline(always)]
    pub fn read() -> super::Cpar4 {
        super::Cpar4 {
            raw: unsafe { *((0x40020400u32 + 0x4Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar4) {
       unsafe { *((0x40020400u32 + 0x4Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Cmar4 {
   raw: u32,
}

impl Cmar4 {
    #[inline(always)]
    pub fn ma_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ma(mut self, val: u32) -> Cmar4 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x50u32) as *mut u32) = self.raw; }
    }
}

pub mod cmar4 {
    #[inline(always)]
    pub fn read() -> super::Cmar4 {
        super::Cmar4 {
            raw: unsafe { *((0x40020400u32 + 0x50u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar4) {
       unsafe { *((0x40020400u32 + 0x50u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr5 {
   raw: u32,
}

impl Ccr5 {
    #[inline(always)]
    pub fn en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem(mut self, val: u32) -> Ccr5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x58u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr5 {
    #[inline(always)]
    pub fn read() -> super::Ccr5 {
        super::Ccr5 {
            raw: unsafe { *((0x40020400u32 + 0x58u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr5) {
       unsafe { *((0x40020400u32 + 0x58u32) as *mut u32) = val.raw; }
    }
}

pub struct Cndtr5 {
   raw: u32,
}

impl Cndtr5 {
    #[inline(always)]
    pub fn ndt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ndt(mut self, val: u32) -> Cndtr5 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x5Cu32) as *mut u32) = self.raw; }
    }
}

pub mod cndtr5 {
    #[inline(always)]
    pub fn read() -> super::Cndtr5 {
        super::Cndtr5 {
            raw: unsafe { *((0x40020400u32 + 0x5Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr5) {
       unsafe { *((0x40020400u32 + 0x5Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Cpar5 {
   raw: u32,
}

impl Cpar5 {
    #[inline(always)]
    pub fn pa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn pa(mut self, val: u32) -> Cpar5 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x60u32) as *mut u32) = self.raw; }
    }
}

pub mod cpar5 {
    #[inline(always)]
    pub fn read() -> super::Cpar5 {
        super::Cpar5 {
            raw: unsafe { *((0x40020400u32 + 0x60u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar5) {
       unsafe { *((0x40020400u32 + 0x60u32) as *mut u32) = val.raw; }
    }
}

pub struct Cmar5 {
   raw: u32,
}

impl Cmar5 {
    #[inline(always)]
    pub fn ma_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ma(mut self, val: u32) -> Cmar5 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x64u32) as *mut u32) = self.raw; }
    }
}

pub mod cmar5 {
    #[inline(always)]
    pub fn read() -> super::Cmar5 {
        super::Cmar5 {
            raw: unsafe { *((0x40020400u32 + 0x64u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar5) {
       unsafe { *((0x40020400u32 + 0x64u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr6 {
   raw: u32,
}

impl Ccr6 {
    #[inline(always)]
    pub fn en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem(mut self, val: u32) -> Ccr6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x6Cu32) as *mut u32) = self.raw; }
    }
}

pub mod ccr6 {
    #[inline(always)]
    pub fn read() -> super::Ccr6 {
        super::Ccr6 {
            raw: unsafe { *((0x40020400u32 + 0x6Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr6) {
       unsafe { *((0x40020400u32 + 0x6Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Cndtr6 {
   raw: u32,
}

impl Cndtr6 {
    #[inline(always)]
    pub fn ndt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ndt(mut self, val: u32) -> Cndtr6 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x70u32) as *mut u32) = self.raw; }
    }
}

pub mod cndtr6 {
    #[inline(always)]
    pub fn read() -> super::Cndtr6 {
        super::Cndtr6 {
            raw: unsafe { *((0x40020400u32 + 0x70u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr6) {
       unsafe { *((0x40020400u32 + 0x70u32) as *mut u32) = val.raw; }
    }
}

pub struct Cpar6 {
   raw: u32,
}

impl Cpar6 {
    #[inline(always)]
    pub fn pa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn pa(mut self, val: u32) -> Cpar6 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x74u32) as *mut u32) = self.raw; }
    }
}

pub mod cpar6 {
    #[inline(always)]
    pub fn read() -> super::Cpar6 {
        super::Cpar6 {
            raw: unsafe { *((0x40020400u32 + 0x74u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar6) {
       unsafe { *((0x40020400u32 + 0x74u32) as *mut u32) = val.raw; }
    }
}

pub struct Cmar6 {
   raw: u32,
}

impl Cmar6 {
    #[inline(always)]
    pub fn ma_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ma(mut self, val: u32) -> Cmar6 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x78u32) as *mut u32) = self.raw; }
    }
}

pub mod cmar6 {
    #[inline(always)]
    pub fn read() -> super::Cmar6 {
        super::Cmar6 {
            raw: unsafe { *((0x40020400u32 + 0x78u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar6) {
       unsafe { *((0x40020400u32 + 0x78u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr7 {
   raw: u32,
}

impl Ccr7 {
    #[inline(always)]
    pub fn en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem(mut self, val: u32) -> Ccr7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x80u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr7 {
    #[inline(always)]
    pub fn read() -> super::Ccr7 {
        super::Ccr7 {
            raw: unsafe { *((0x40020400u32 + 0x80u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr7) {
       unsafe { *((0x40020400u32 + 0x80u32) as *mut u32) = val.raw; }
    }
}

pub struct Cndtr7 {
   raw: u32,
}

impl Cndtr7 {
    #[inline(always)]
    pub fn ndt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ndt(mut self, val: u32) -> Cndtr7 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x84u32) as *mut u32) = self.raw; }
    }
}

pub mod cndtr7 {
    #[inline(always)]
    pub fn read() -> super::Cndtr7 {
        super::Cndtr7 {
            raw: unsafe { *((0x40020400u32 + 0x84u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr7) {
       unsafe { *((0x40020400u32 + 0x84u32) as *mut u32) = val.raw; }
    }
}

pub struct Cpar7 {
   raw: u32,
}

impl Cpar7 {
    #[inline(always)]
    pub fn pa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn pa(mut self, val: u32) -> Cpar7 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x88u32) as *mut u32) = self.raw; }
    }
}

pub mod cpar7 {
    #[inline(always)]
    pub fn read() -> super::Cpar7 {
        super::Cpar7 {
            raw: unsafe { *((0x40020400u32 + 0x88u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar7) {
       unsafe { *((0x40020400u32 + 0x88u32) as *mut u32) = val.raw; }
    }
}

pub struct Cmar7 {
   raw: u32,
}

impl Cmar7 {
    #[inline(always)]
    pub fn ma_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ma(mut self, val: u32) -> Cmar7 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40020400u32 + 0x8Cu32) as *mut u32) = self.raw; }
    }
}

pub mod cmar7 {
    #[inline(always)]
    pub fn read() -> super::Cmar7 {
        super::Cmar7 {
            raw: unsafe { *((0x40020400u32 + 0x8Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar7) {
       unsafe { *((0x40020400u32 + 0x8Cu32) as *mut u32) = val.raw; }
    }
}

