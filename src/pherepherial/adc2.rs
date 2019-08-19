pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn strt_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn strt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn jstrt_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jstrt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn jeoc_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jeoc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn eoc_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eoc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn awd_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn awd_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40012800 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40012800 + 0x0) as *mut u32) = val.raw; }
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
    pub fn awden_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23)
    }

    #[inline(always)]
    pub fn jawden_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jawden_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22)
    }

    #[inline(always)]
    pub fn discnum_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn discnum_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 13)) | ((val & ((1 << 3) - 1)) << 13)
    }

    #[inline(always)]
    pub fn jdiscen_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jdiscen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn discen_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn discen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn jauto_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jauto_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn awdsgl_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn awdsgl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn scan_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn scan_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn jeocie_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jeocie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn awdie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn awdie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn eocie_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eocie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn awdch_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn awdch_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0)
    }

}

pub mod cr1 {
    #[inline(always)]
    pub fn read() -> super::Cr1 {
        super::Cr1 {
            raw: unsafe { *((0x40012800 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr1) {
       unsafe { *((0x40012800 + 0x4) as *mut u32) = val.raw; }
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
    pub fn tsvrefe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23)
    }

    #[inline(always)]
    pub fn swstart_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swstart_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22)
    }

    #[inline(always)]
    pub fn jswstart_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jswstart_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21)
    }

    #[inline(always)]
    pub fn exttrig_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn exttrig_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20)
    }

    #[inline(always)]
    pub fn extsel_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn extsel_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 17)) | ((val & ((1 << 3) - 1)) << 17)
    }

    #[inline(always)]
    pub fn jexttrig_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jexttrig_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn jextsel_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn jextsel_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12)
    }

    #[inline(always)]
    pub fn align_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn align_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn dma_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dma_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn rstcal_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rstcal_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn cal_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cal_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cont_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cont_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn adon_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adon_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod cr2 {
    #[inline(always)]
    pub fn read() -> super::Cr2 {
        super::Cr2 {
            raw: unsafe { *((0x40012800 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr2) {
       unsafe { *((0x40012800 + 0x8) as *mut u32) = val.raw; }
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
    pub fn smp10_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0)
    }

    #[inline(always)]
    pub fn smp11_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 3)) | ((val & ((1 << 3) - 1)) << 3)
    }

    #[inline(always)]
    pub fn smp12_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp12_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 6)) | ((val & ((1 << 3) - 1)) << 6)
    }

    #[inline(always)]
    pub fn smp13_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp13_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 9)) | ((val & ((1 << 3) - 1)) << 9)
    }

    #[inline(always)]
    pub fn smp14_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12)
    }

    #[inline(always)]
    pub fn smp15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 15)) | ((val & ((1 << 3) - 1)) << 15)
    }

    #[inline(always)]
    pub fn smp16_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp16_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 18)) | ((val & ((1 << 3) - 1)) << 18)
    }

    #[inline(always)]
    pub fn smp17_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp17_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 21)) | ((val & ((1 << 3) - 1)) << 21)
    }

}

pub mod smpr1 {
    #[inline(always)]
    pub fn read() -> super::Smpr1 {
        super::Smpr1 {
            raw: unsafe { *((0x40012800 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Smpr1) {
       unsafe { *((0x40012800 + 0xC) as *mut u32) = val.raw; }
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
    pub fn smp0_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0)
    }

    #[inline(always)]
    pub fn smp1_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 3)) | ((val & ((1 << 3) - 1)) << 3)
    }

    #[inline(always)]
    pub fn smp2_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 6)) | ((val & ((1 << 3) - 1)) << 6)
    }

    #[inline(always)]
    pub fn smp3_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 9)) | ((val & ((1 << 3) - 1)) << 9)
    }

    #[inline(always)]
    pub fn smp4_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12)
    }

    #[inline(always)]
    pub fn smp5_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 15)) | ((val & ((1 << 3) - 1)) << 15)
    }

    #[inline(always)]
    pub fn smp6_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 18)) | ((val & ((1 << 3) - 1)) << 18)
    }

    #[inline(always)]
    pub fn smp7_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 21)) | ((val & ((1 << 3) - 1)) << 21)
    }

    #[inline(always)]
    pub fn smp8_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp8_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 24)) | ((val & ((1 << 3) - 1)) << 24)
    }

    #[inline(always)]
    pub fn smp9_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn smp9_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 27)) | ((val & ((1 << 3) - 1)) << 27)
    }

}

pub mod smpr2 {
    #[inline(always)]
    pub fn read() -> super::Smpr2 {
        super::Smpr2 {
            raw: unsafe { *((0x40012800 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Smpr2) {
       unsafe { *((0x40012800 + 0x10) as *mut u32) = val.raw; }
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
    pub fn joffset1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod jofr1 {
    #[inline(always)]
    pub fn read() -> super::Jofr1 {
        super::Jofr1 {
            raw: unsafe { *((0x40012800 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jofr1) {
       unsafe { *((0x40012800 + 0x14) as *mut u32) = val.raw; }
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
    pub fn joffset2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod jofr2 {
    #[inline(always)]
    pub fn read() -> super::Jofr2 {
        super::Jofr2 {
            raw: unsafe { *((0x40012800 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jofr2) {
       unsafe { *((0x40012800 + 0x18) as *mut u32) = val.raw; }
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
    pub fn joffset3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod jofr3 {
    #[inline(always)]
    pub fn read() -> super::Jofr3 {
        super::Jofr3 {
            raw: unsafe { *((0x40012800 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jofr3) {
       unsafe { *((0x40012800 + 0x1C) as *mut u32) = val.raw; }
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
    pub fn joffset4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod jofr4 {
    #[inline(always)]
    pub fn read() -> super::Jofr4 {
        super::Jofr4 {
            raw: unsafe { *((0x40012800 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jofr4) {
       unsafe { *((0x40012800 + 0x20) as *mut u32) = val.raw; }
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
    pub fn ht_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod htr {
    #[inline(always)]
    pub fn read() -> super::Htr {
        super::Htr {
            raw: unsafe { *((0x40012800 + 0x24) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Htr) {
       unsafe { *((0x40012800 + 0x24) as *mut u32) = val.raw; }
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
    pub fn lt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod ltr {
    #[inline(always)]
    pub fn read() -> super::Ltr {
        super::Ltr {
            raw: unsafe { *((0x40012800 + 0x28) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ltr) {
       unsafe { *((0x40012800 + 0x28) as *mut u32) = val.raw; }
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
    pub fn l_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn sq16_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq16_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 15)) | ((val & ((1 << 5) - 1)) << 15)
    }

    #[inline(always)]
    pub fn sq15_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 10)) | ((val & ((1 << 5) - 1)) << 10)
    }

    #[inline(always)]
    pub fn sq14_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 5)) | ((val & ((1 << 5) - 1)) << 5)
    }

    #[inline(always)]
    pub fn sq13_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq13_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0)
    }

}

pub mod sqr1 {
    #[inline(always)]
    pub fn read() -> super::Sqr1 {
        super::Sqr1 {
            raw: unsafe { *((0x40012800 + 0x2C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sqr1) {
       unsafe { *((0x40012800 + 0x2C) as *mut u32) = val.raw; }
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
    pub fn sq12_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 25)) | ((val & ((1 << 5) - 1)) << 25)
    }

    #[inline(always)]
    pub fn sq11_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 20)) | ((val & ((1 << 5) - 1)) << 20)
    }

    #[inline(always)]
    pub fn sq10_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq10_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 15)) | ((val & ((1 << 5) - 1)) << 15)
    }

    #[inline(always)]
    pub fn sq9_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq9_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 10)) | ((val & ((1 << 5) - 1)) << 10)
    }

    #[inline(always)]
    pub fn sq8_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq8_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 5)) | ((val & ((1 << 5) - 1)) << 5)
    }

    #[inline(always)]
    pub fn sq7_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0)
    }

}

pub mod sqr2 {
    #[inline(always)]
    pub fn read() -> super::Sqr2 {
        super::Sqr2 {
            raw: unsafe { *((0x40012800 + 0x30) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sqr2) {
       unsafe { *((0x40012800 + 0x30) as *mut u32) = val.raw; }
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
    pub fn sq6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 25)) | ((val & ((1 << 5) - 1)) << 25)
    }

    #[inline(always)]
    pub fn sq5_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 20)) | ((val & ((1 << 5) - 1)) << 20)
    }

    #[inline(always)]
    pub fn sq4_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 15)) | ((val & ((1 << 5) - 1)) << 15)
    }

    #[inline(always)]
    pub fn sq3_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 10)) | ((val & ((1 << 5) - 1)) << 10)
    }

    #[inline(always)]
    pub fn sq2_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 5)) | ((val & ((1 << 5) - 1)) << 5)
    }

    #[inline(always)]
    pub fn sq1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn sq1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0)
    }

}

pub mod sqr3 {
    #[inline(always)]
    pub fn read() -> super::Sqr3 {
        super::Sqr3 {
            raw: unsafe { *((0x40012800 + 0x34) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sqr3) {
       unsafe { *((0x40012800 + 0x34) as *mut u32) = val.raw; }
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
    pub fn jl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20)
    }

    #[inline(always)]
    pub fn jsq4_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn jsq4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 15)) | ((val & ((1 << 5) - 1)) << 15)
    }

    #[inline(always)]
    pub fn jsq3_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn jsq3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 10)) | ((val & ((1 << 5) - 1)) << 10)
    }

    #[inline(always)]
    pub fn jsq2_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn jsq2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 5)) | ((val & ((1 << 5) - 1)) << 5)
    }

    #[inline(always)]
    pub fn jsq1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn jsq1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0)
    }

}

pub mod jsqr {
    #[inline(always)]
    pub fn read() -> super::Jsqr {
        super::Jsqr {
            raw: unsafe { *((0x40012800 + 0x38) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jsqr) {
       unsafe { *((0x40012800 + 0x38) as *mut u32) = val.raw; }
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
    pub fn jdata_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod jdr1 {
    #[inline(always)]
    pub fn read() -> super::Jdr1 {
        super::Jdr1 {
            raw: unsafe { *((0x40012800 + 0x3C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jdr1) {
       unsafe { *((0x40012800 + 0x3C) as *mut u32) = val.raw; }
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
    pub fn jdata_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod jdr2 {
    #[inline(always)]
    pub fn read() -> super::Jdr2 {
        super::Jdr2 {
            raw: unsafe { *((0x40012800 + 0x40) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jdr2) {
       unsafe { *((0x40012800 + 0x40) as *mut u32) = val.raw; }
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
    pub fn jdata_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod jdr3 {
    #[inline(always)]
    pub fn read() -> super::Jdr3 {
        super::Jdr3 {
            raw: unsafe { *((0x40012800 + 0x44) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jdr3) {
       unsafe { *((0x40012800 + 0x44) as *mut u32) = val.raw; }
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
    pub fn jdata_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod jdr4 {
    #[inline(always)]
    pub fn read() -> super::Jdr4 {
        super::Jdr4 {
            raw: unsafe { *((0x40012800 + 0x48) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Jdr4) {
       unsafe { *((0x40012800 + 0x48) as *mut u32) = val.raw; }
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
    pub fn data_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod dr {
    #[inline(always)]
    pub fn read() -> super::Dr {
        super::Dr {
            raw: unsafe { *((0x40012800 + 0x4C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr) {
       unsafe { *((0x40012800 + 0x4C) as *mut u32) = val.raw; }
    }
}

