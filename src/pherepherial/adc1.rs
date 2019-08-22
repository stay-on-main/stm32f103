pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn strt_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn strt(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn jstrt_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jstrt(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn jeoc_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jeoc(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn eoc_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eoc(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn awd_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn awd(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40012400u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40012400u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Cr1 {
   raw: u32,
}

impl Cr1 {
    #[inline(always)]
    pub fn awden_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn awden(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn jawden_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jawden(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn dualmod_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn dualmod(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 16)) | ((val & ((1 << 4) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn discnum_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn discnum(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 13)) | ((val & ((1 << 3) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn jdiscen_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jdiscen(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn discen_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn discen(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn jauto_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jauto(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn awdsgl_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn awdsgl(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn scan_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn scan(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn jeocie_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jeocie(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn awdie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn awdie(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn eocie_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eocie(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn awdch_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn awdch(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod cr1 {
    #[inline(always)]
    pub fn read() -> super::Cr1 {
        super::Cr1 {
            raw: unsafe { *((0x40012400u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr1) {
       unsafe { *((0x40012400u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Cr2 {
   raw: u32,
}

impl Cr2 {
    #[inline(always)]
    pub fn tsvrefe_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsvrefe(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn swstart_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swstart(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn jswstart_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jswstart(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn exttrig_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn exttrig(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn extsel_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn extsel(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 17)) | ((val & ((1 << 3) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn jexttrig_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jexttrig(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn jextsel_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn jextsel(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn align_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn align(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn dma_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dma(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn rstcal_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rstcal(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn cal_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cal(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cont_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cont(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn adon_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adon(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod cr2 {
    #[inline(always)]
    pub fn read() -> super::Cr2 {
        super::Cr2 {
            raw: unsafe { *((0x40012400u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr2) {
       unsafe { *((0x40012400u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Smpr1 {
   raw: u32,
}

impl Smpr1 {
    #[inline(always)]
    pub fn smp10_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp10(mut self, val: u32) -> Smpr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn smp11_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp11(mut self, val: u32) -> Smpr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 3)) | ((val & ((1 << 3) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn smp12_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp12(mut self, val: u32) -> Smpr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 6)) | ((val & ((1 << 3) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn smp13_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp13(mut self, val: u32) -> Smpr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 9)) | ((val & ((1 << 3) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn smp14_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp14(mut self, val: u32) -> Smpr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn smp15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp15(mut self, val: u32) -> Smpr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 15)) | ((val & ((1 << 3) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn smp16_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp16(mut self, val: u32) -> Smpr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 18)) | ((val & ((1 << 3) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn smp17_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp17(mut self, val: u32) -> Smpr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 21)) | ((val & ((1 << 3) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod smpr1 {
    #[inline(always)]
    pub fn read() -> super::Smpr1 {
        super::Smpr1 {
            raw: unsafe { *((0x40012400u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Smpr1) {
       unsafe { *((0x40012400u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Smpr2 {
   raw: u32,
}

impl Smpr2 {
    #[inline(always)]
    pub fn smp0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp0(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn smp1_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp1(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 3)) | ((val & ((1 << 3) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn smp2_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp2(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 6)) | ((val & ((1 << 3) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn smp3_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp3(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 9)) | ((val & ((1 << 3) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn smp4_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp4(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn smp5_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp5(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 15)) | ((val & ((1 << 3) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn smp6_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp6(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 18)) | ((val & ((1 << 3) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn smp7_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp7(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 21)) | ((val & ((1 << 3) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn smp8_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp8(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 24)) | ((val & ((1 << 3) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn smp9_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp9(mut self, val: u32) -> Smpr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 27)) | ((val & ((1 << 3) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod smpr2 {
    #[inline(always)]
    pub fn read() -> super::Smpr2 {
        super::Smpr2 {
            raw: unsafe { *((0x40012400u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Smpr2) {
       unsafe { *((0x40012400u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Jofr1 {
   raw: u32,
}

impl Jofr1 {
    #[inline(always)]
    pub fn joffset1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn joffset1(mut self, val: u32) -> Jofr1 {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod jofr1 {
    #[inline(always)]
    pub fn read() -> super::Jofr1 {
        super::Jofr1 {
            raw: unsafe { *((0x40012400u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jofr1) {
       unsafe { *((0x40012400u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Jofr2 {
   raw: u32,
}

impl Jofr2 {
    #[inline(always)]
    pub fn joffset2_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn joffset2(mut self, val: u32) -> Jofr2 {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod jofr2 {
    #[inline(always)]
    pub fn read() -> super::Jofr2 {
        super::Jofr2 {
            raw: unsafe { *((0x40012400u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jofr2) {
       unsafe { *((0x40012400u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Jofr3 {
   raw: u32,
}

impl Jofr3 {
    #[inline(always)]
    pub fn joffset3_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn joffset3(mut self, val: u32) -> Jofr3 {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod jofr3 {
    #[inline(always)]
    pub fn read() -> super::Jofr3 {
        super::Jofr3 {
            raw: unsafe { *((0x40012400u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jofr3) {
       unsafe { *((0x40012400u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Jofr4 {
   raw: u32,
}

impl Jofr4 {
    #[inline(always)]
    pub fn joffset4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn joffset4(mut self, val: u32) -> Jofr4 {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x20u32) as *mut u32) = self.raw; }
    }
}

pub mod jofr4 {
    #[inline(always)]
    pub fn read() -> super::Jofr4 {
        super::Jofr4 {
            raw: unsafe { *((0x40012400u32 + 0x20u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jofr4) {
       unsafe { *((0x40012400u32 + 0x20u32) as *mut u32) = val.raw; }
    }
}

pub struct Htr {
   raw: u32,
}

impl Htr {
    #[inline(always)]
    pub fn ht_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn ht(mut self, val: u32) -> Htr {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x24u32) as *mut u32) = self.raw; }
    }
}

pub mod htr {
    #[inline(always)]
    pub fn read() -> super::Htr {
        super::Htr {
            raw: unsafe { *((0x40012400u32 + 0x24u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Htr) {
       unsafe { *((0x40012400u32 + 0x24u32) as *mut u32) = val.raw; }
    }
}

pub struct Ltr {
   raw: u32,
}

impl Ltr {
    #[inline(always)]
    pub fn lt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn lt(mut self, val: u32) -> Ltr {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x28u32) as *mut u32) = self.raw; }
    }
}

pub mod ltr {
    #[inline(always)]
    pub fn read() -> super::Ltr {
        super::Ltr {
            raw: unsafe { *((0x40012400u32 + 0x28u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ltr) {
       unsafe { *((0x40012400u32 + 0x28u32) as *mut u32) = val.raw; }
    }
}

pub struct Sqr1 {
   raw: u32,
}

impl Sqr1 {
    #[inline(always)]
    pub fn l_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn l(mut self, val: u32) -> Sqr1 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn sq16_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq16(mut self, val: u32) -> Sqr1 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 15)) | ((val & ((1 << 5) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn sq15_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq15(mut self, val: u32) -> Sqr1 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 10)) | ((val & ((1 << 5) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn sq14_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq14(mut self, val: u32) -> Sqr1 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 5)) | ((val & ((1 << 5) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn sq13_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq13(mut self, val: u32) -> Sqr1 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x2Cu32) as *mut u32) = self.raw; }
    }
}

pub mod sqr1 {
    #[inline(always)]
    pub fn read() -> super::Sqr1 {
        super::Sqr1 {
            raw: unsafe { *((0x40012400u32 + 0x2Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sqr1) {
       unsafe { *((0x40012400u32 + 0x2Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Sqr2 {
   raw: u32,
}

impl Sqr2 {
    #[inline(always)]
    pub fn sq12_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq12(mut self, val: u32) -> Sqr2 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 25)) | ((val & ((1 << 5) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn sq11_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq11(mut self, val: u32) -> Sqr2 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 20)) | ((val & ((1 << 5) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn sq10_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq10(mut self, val: u32) -> Sqr2 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 15)) | ((val & ((1 << 5) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn sq9_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq9(mut self, val: u32) -> Sqr2 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 10)) | ((val & ((1 << 5) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn sq8_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq8(mut self, val: u32) -> Sqr2 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 5)) | ((val & ((1 << 5) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn sq7_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq7(mut self, val: u32) -> Sqr2 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x30u32) as *mut u32) = self.raw; }
    }
}

pub mod sqr2 {
    #[inline(always)]
    pub fn read() -> super::Sqr2 {
        super::Sqr2 {
            raw: unsafe { *((0x40012400u32 + 0x30u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sqr2) {
       unsafe { *((0x40012400u32 + 0x30u32) as *mut u32) = val.raw; }
    }
}

pub struct Sqr3 {
   raw: u32,
}

impl Sqr3 {
    #[inline(always)]
    pub fn sq6_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq6(mut self, val: u32) -> Sqr3 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 25)) | ((val & ((1 << 5) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn sq5_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq5(mut self, val: u32) -> Sqr3 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 20)) | ((val & ((1 << 5) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn sq4_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq4(mut self, val: u32) -> Sqr3 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 15)) | ((val & ((1 << 5) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn sq3_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq3(mut self, val: u32) -> Sqr3 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 10)) | ((val & ((1 << 5) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn sq2_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq2(mut self, val: u32) -> Sqr3 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 5)) | ((val & ((1 << 5) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn sq1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq1(mut self, val: u32) -> Sqr3 {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x34u32) as *mut u32) = self.raw; }
    }
}

pub mod sqr3 {
    #[inline(always)]
    pub fn read() -> super::Sqr3 {
        super::Sqr3 {
            raw: unsafe { *((0x40012400u32 + 0x34u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sqr3) {
       unsafe { *((0x40012400u32 + 0x34u32) as *mut u32) = val.raw; }
    }
}

pub struct Jsqr {
   raw: u32,
}

impl Jsqr {
    #[inline(always)]
    pub fn jl_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn jl(mut self, val: u32) -> Jsqr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn jsq4_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn jsq4(mut self, val: u32) -> Jsqr {
        self.raw = (self.raw & !(((1 << 5) - 1) << 15)) | ((val & ((1 << 5) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn jsq3_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn jsq3(mut self, val: u32) -> Jsqr {
        self.raw = (self.raw & !(((1 << 5) - 1) << 10)) | ((val & ((1 << 5) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn jsq2_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn jsq2(mut self, val: u32) -> Jsqr {
        self.raw = (self.raw & !(((1 << 5) - 1) << 5)) | ((val & ((1 << 5) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn jsq1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn jsq1(mut self, val: u32) -> Jsqr {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x38u32) as *mut u32) = self.raw; }
    }
}

pub mod jsqr {
    #[inline(always)]
    pub fn read() -> super::Jsqr {
        super::Jsqr {
            raw: unsafe { *((0x40012400u32 + 0x38u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jsqr) {
       unsafe { *((0x40012400u32 + 0x38u32) as *mut u32) = val.raw; }
    }
}

pub struct Jdr1 {
   raw: u32,
}

impl Jdr1 {
    #[inline(always)]
    pub fn jdata_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn jdata(mut self, val: u32) -> Jdr1 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x3Cu32) as *mut u32) = self.raw; }
    }
}

pub mod jdr1 {
    #[inline(always)]
    pub fn read() -> super::Jdr1 {
        super::Jdr1 {
            raw: unsafe { *((0x40012400u32 + 0x3Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jdr1) {
       unsafe { *((0x40012400u32 + 0x3Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Jdr2 {
   raw: u32,
}

impl Jdr2 {
    #[inline(always)]
    pub fn jdata_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn jdata(mut self, val: u32) -> Jdr2 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x40u32) as *mut u32) = self.raw; }
    }
}

pub mod jdr2 {
    #[inline(always)]
    pub fn read() -> super::Jdr2 {
        super::Jdr2 {
            raw: unsafe { *((0x40012400u32 + 0x40u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jdr2) {
       unsafe { *((0x40012400u32 + 0x40u32) as *mut u32) = val.raw; }
    }
}

pub struct Jdr3 {
   raw: u32,
}

impl Jdr3 {
    #[inline(always)]
    pub fn jdata_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn jdata(mut self, val: u32) -> Jdr3 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x44u32) as *mut u32) = self.raw; }
    }
}

pub mod jdr3 {
    #[inline(always)]
    pub fn read() -> super::Jdr3 {
        super::Jdr3 {
            raw: unsafe { *((0x40012400u32 + 0x44u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jdr3) {
       unsafe { *((0x40012400u32 + 0x44u32) as *mut u32) = val.raw; }
    }
}

pub struct Jdr4 {
   raw: u32,
}

impl Jdr4 {
    #[inline(always)]
    pub fn jdata_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn jdata(mut self, val: u32) -> Jdr4 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x48u32) as *mut u32) = self.raw; }
    }
}

pub mod jdr4 {
    #[inline(always)]
    pub fn read() -> super::Jdr4 {
        super::Jdr4 {
            raw: unsafe { *((0x40012400u32 + 0x48u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jdr4) {
       unsafe { *((0x40012400u32 + 0x48u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr {
   raw: u32,
}

impl Dr {
    #[inline(always)]
    pub fn data_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn data(mut self, val: u32) -> Dr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn adc2data_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn adc2data(mut self, val: u32) -> Dr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012400u32 + 0x4Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr {
    #[inline(always)]
    pub fn read() -> super::Dr {
        super::Dr {
            raw: unsafe { *((0x40012400u32 + 0x4Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr) {
       unsafe { *((0x40012400u32 + 0x4Cu32) as *mut u32) = val.raw; }
    }
}

