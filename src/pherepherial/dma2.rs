pub struct Isr {
   raw: u32,
}

impl Isr {
    #[inline(always)]
    pub fn gif1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tcif1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn htif1_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn teif1_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn gif2_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn tcif2_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn htif2_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn teif2_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn gif3_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn tcif3_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn htif3_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn teif3_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn gif4_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn tcif4_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn htif4_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn teif4_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn gif5_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn tcif5_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn htif5_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

    #[inline(always)]
    pub fn teif5_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19)
    }

    #[inline(always)]
    pub fn gif6_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20)
    }

    #[inline(always)]
    pub fn tcif6_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21)
    }

    #[inline(always)]
    pub fn htif6_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22)
    }

    #[inline(always)]
    pub fn teif6_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23)
    }

    #[inline(always)]
    pub fn gif7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gif7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24)
    }

    #[inline(always)]
    pub fn tcif7_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcif7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25)
    }

    #[inline(always)]
    pub fn htif7_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htif7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26)
    }

    #[inline(always)]
    pub fn teif7_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teif7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27)
    }

}

pub mod isr {
    #[inline(always)]
    pub fn read() -> super::Isr {
        super::Isr {
            raw: unsafe { *((0x40020400 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Isr) {
       unsafe { *((0x40020400 + 0x0) as *mut u32) = val.raw; }
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
    pub fn cgif1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn cgif2_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn cgif3_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn cgif4_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn cgif5_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn cgif6_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20)
    }

    #[inline(always)]
    pub fn cgif7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgif7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24)
    }

    #[inline(always)]
    pub fn ctcif1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn ctcif2_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn ctcif3_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn ctcif4_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn ctcif5_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn ctcif6_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21)
    }

    #[inline(always)]
    pub fn ctcif7_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctcif7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25)
    }

    #[inline(always)]
    pub fn chtif1_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn chtif2_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn chtif3_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn chtif4_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn chtif5_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

    #[inline(always)]
    pub fn chtif6_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22)
    }

    #[inline(always)]
    pub fn chtif7_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chtif7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26)
    }

    #[inline(always)]
    pub fn cteif1_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn cteif2_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn cteif3_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn cteif4_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn cteif5_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19)
    }

    #[inline(always)]
    pub fn cteif6_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23)
    }

    #[inline(always)]
    pub fn cteif7_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cteif7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27)
    }

}

pub mod ifcr {
    #[inline(always)]
    pub fn read() -> super::Ifcr {
        super::Ifcr {
            raw: unsafe { *((0x40020400 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ifcr) {
       unsafe { *((0x40020400 + 0x4) as *mut u32) = val.raw; }
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
    pub fn en_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

}

pub mod ccr1 {
    #[inline(always)]
    pub fn read() -> super::Ccr1 {
        super::Ccr1 {
            raw: unsafe { *((0x40020400 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr1) {
       unsafe { *((0x40020400 + 0x8) as *mut u32) = val.raw; }
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
    pub fn ndt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cndtr1 {
    #[inline(always)]
    pub fn read() -> super::Cndtr1 {
        super::Cndtr1 {
            raw: unsafe { *((0x40020400 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr1) {
       unsafe { *((0x40020400 + 0xC) as *mut u32) = val.raw; }
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
    pub fn pa_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cpar1 {
    #[inline(always)]
    pub fn read() -> super::Cpar1 {
        super::Cpar1 {
            raw: unsafe { *((0x40020400 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar1) {
       unsafe { *((0x40020400 + 0x10) as *mut u32) = val.raw; }
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
    pub fn ma_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cmar1 {
    #[inline(always)]
    pub fn read() -> super::Cmar1 {
        super::Cmar1 {
            raw: unsafe { *((0x40020400 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar1) {
       unsafe { *((0x40020400 + 0x14) as *mut u32) = val.raw; }
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
    pub fn en_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

}

pub mod ccr2 {
    #[inline(always)]
    pub fn read() -> super::Ccr2 {
        super::Ccr2 {
            raw: unsafe { *((0x40020400 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr2) {
       unsafe { *((0x40020400 + 0x1C) as *mut u32) = val.raw; }
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
    pub fn ndt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cndtr2 {
    #[inline(always)]
    pub fn read() -> super::Cndtr2 {
        super::Cndtr2 {
            raw: unsafe { *((0x40020400 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr2) {
       unsafe { *((0x40020400 + 0x20) as *mut u32) = val.raw; }
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
    pub fn pa_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cpar2 {
    #[inline(always)]
    pub fn read() -> super::Cpar2 {
        super::Cpar2 {
            raw: unsafe { *((0x40020400 + 0x24) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar2) {
       unsafe { *((0x40020400 + 0x24) as *mut u32) = val.raw; }
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
    pub fn ma_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cmar2 {
    #[inline(always)]
    pub fn read() -> super::Cmar2 {
        super::Cmar2 {
            raw: unsafe { *((0x40020400 + 0x28) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar2) {
       unsafe { *((0x40020400 + 0x28) as *mut u32) = val.raw; }
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
    pub fn en_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

}

pub mod ccr3 {
    #[inline(always)]
    pub fn read() -> super::Ccr3 {
        super::Ccr3 {
            raw: unsafe { *((0x40020400 + 0x30) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr3) {
       unsafe { *((0x40020400 + 0x30) as *mut u32) = val.raw; }
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
    pub fn ndt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cndtr3 {
    #[inline(always)]
    pub fn read() -> super::Cndtr3 {
        super::Cndtr3 {
            raw: unsafe { *((0x40020400 + 0x34) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr3) {
       unsafe { *((0x40020400 + 0x34) as *mut u32) = val.raw; }
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
    pub fn pa_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cpar3 {
    #[inline(always)]
    pub fn read() -> super::Cpar3 {
        super::Cpar3 {
            raw: unsafe { *((0x40020400 + 0x38) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar3) {
       unsafe { *((0x40020400 + 0x38) as *mut u32) = val.raw; }
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
    pub fn ma_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cmar3 {
    #[inline(always)]
    pub fn read() -> super::Cmar3 {
        super::Cmar3 {
            raw: unsafe { *((0x40020400 + 0x3C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar3) {
       unsafe { *((0x40020400 + 0x3C) as *mut u32) = val.raw; }
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
    pub fn en_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

}

pub mod ccr4 {
    #[inline(always)]
    pub fn read() -> super::Ccr4 {
        super::Ccr4 {
            raw: unsafe { *((0x40020400 + 0x44) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr4) {
       unsafe { *((0x40020400 + 0x44) as *mut u32) = val.raw; }
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
    pub fn ndt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cndtr4 {
    #[inline(always)]
    pub fn read() -> super::Cndtr4 {
        super::Cndtr4 {
            raw: unsafe { *((0x40020400 + 0x48) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr4) {
       unsafe { *((0x40020400 + 0x48) as *mut u32) = val.raw; }
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
    pub fn pa_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cpar4 {
    #[inline(always)]
    pub fn read() -> super::Cpar4 {
        super::Cpar4 {
            raw: unsafe { *((0x40020400 + 0x4C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar4) {
       unsafe { *((0x40020400 + 0x4C) as *mut u32) = val.raw; }
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
    pub fn ma_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cmar4 {
    #[inline(always)]
    pub fn read() -> super::Cmar4 {
        super::Cmar4 {
            raw: unsafe { *((0x40020400 + 0x50) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar4) {
       unsafe { *((0x40020400 + 0x50) as *mut u32) = val.raw; }
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
    pub fn en_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

}

pub mod ccr5 {
    #[inline(always)]
    pub fn read() -> super::Ccr5 {
        super::Ccr5 {
            raw: unsafe { *((0x40020400 + 0x58) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr5) {
       unsafe { *((0x40020400 + 0x58) as *mut u32) = val.raw; }
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
    pub fn ndt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cndtr5 {
    #[inline(always)]
    pub fn read() -> super::Cndtr5 {
        super::Cndtr5 {
            raw: unsafe { *((0x40020400 + 0x5C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr5) {
       unsafe { *((0x40020400 + 0x5C) as *mut u32) = val.raw; }
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
    pub fn pa_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cpar5 {
    #[inline(always)]
    pub fn read() -> super::Cpar5 {
        super::Cpar5 {
            raw: unsafe { *((0x40020400 + 0x60) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar5) {
       unsafe { *((0x40020400 + 0x60) as *mut u32) = val.raw; }
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
    pub fn ma_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cmar5 {
    #[inline(always)]
    pub fn read() -> super::Cmar5 {
        super::Cmar5 {
            raw: unsafe { *((0x40020400 + 0x64) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar5) {
       unsafe { *((0x40020400 + 0x64) as *mut u32) = val.raw; }
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
    pub fn en_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

}

pub mod ccr6 {
    #[inline(always)]
    pub fn read() -> super::Ccr6 {
        super::Ccr6 {
            raw: unsafe { *((0x40020400 + 0x6C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr6) {
       unsafe { *((0x40020400 + 0x6C) as *mut u32) = val.raw; }
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
    pub fn ndt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cndtr6 {
    #[inline(always)]
    pub fn read() -> super::Cndtr6 {
        super::Cndtr6 {
            raw: unsafe { *((0x40020400 + 0x70) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr6) {
       unsafe { *((0x40020400 + 0x70) as *mut u32) = val.raw; }
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
    pub fn pa_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cpar6 {
    #[inline(always)]
    pub fn read() -> super::Cpar6 {
        super::Cpar6 {
            raw: unsafe { *((0x40020400 + 0x74) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar6) {
       unsafe { *((0x40020400 + 0x74) as *mut u32) = val.raw; }
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
    pub fn ma_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cmar6 {
    #[inline(always)]
    pub fn read() -> super::Cmar6 {
        super::Cmar6 {
            raw: unsafe { *((0x40020400 + 0x78) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar6) {
       unsafe { *((0x40020400 + 0x78) as *mut u32) = val.raw; }
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
    pub fn en_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn htie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn htie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn teie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn teie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn circ_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn circ_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn pinc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn minc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn minc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn psize_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn psize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn msize_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn msize_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn pl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mem2mem_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mem2mem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

}

pub mod ccr7 {
    #[inline(always)]
    pub fn read() -> super::Ccr7 {
        super::Ccr7 {
            raw: unsafe { *((0x40020400 + 0x80) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr7) {
       unsafe { *((0x40020400 + 0x80) as *mut u32) = val.raw; }
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
    pub fn ndt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cndtr7 {
    #[inline(always)]
    pub fn read() -> super::Cndtr7 {
        super::Cndtr7 {
            raw: unsafe { *((0x40020400 + 0x84) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cndtr7) {
       unsafe { *((0x40020400 + 0x84) as *mut u32) = val.raw; }
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
    pub fn pa_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cpar7 {
    #[inline(always)]
    pub fn read() -> super::Cpar7 {
        super::Cpar7 {
            raw: unsafe { *((0x40020400 + 0x88) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpar7) {
       unsafe { *((0x40020400 + 0x88) as *mut u32) = val.raw; }
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
    pub fn ma_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod cmar7 {
    #[inline(always)]
    pub fn read() -> super::Cmar7 {
        super::Cmar7 {
            raw: unsafe { *((0x40020400 + 0x8C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmar7) {
       unsafe { *((0x40020400 + 0x8C) as *mut u32) = val.raw; }
    }
}

