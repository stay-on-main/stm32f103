pub struct Cr1 {
   raw: u32,
}

impl Cr1 {
    #[inline(always)]
    pub fn ckd_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ckd(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn arpe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn arpe(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn cms_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cms(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 5)) | ((val & ((1 << 2) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn opm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn opm(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn urs_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn urs(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn udis_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn udis(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn cen_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cen(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod cr1 {
    #[inline(always)]
    pub fn read() -> super::Cr1 {
        super::Cr1 {
            raw: unsafe { *((0x40000C00u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr1) {
       unsafe { *((0x40000C00u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Cr2 {
   raw: u32,
}

impl Cr2 {
    #[inline(always)]
    pub fn ti1s_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ti1s(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn mms_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn mms(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ccds_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ccds(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod cr2 {
    #[inline(always)]
    pub fn read() -> super::Cr2 {
        super::Cr2 {
            raw: unsafe { *((0x40000C00u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr2) {
       unsafe { *((0x40000C00u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Smcr {
   raw: u32,
}

impl Smcr {
    #[inline(always)]
    pub fn etp_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn etp(mut self, val: u32) -> Smcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn ece_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ece(mut self, val: u32) -> Smcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn etps_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn etps(mut self, val: u32) -> Smcr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn etf_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn etf(mut self, val: u32) -> Smcr {
        self.raw = (self.raw & !(((1 << 4) - 1) << 8)) | ((val & ((1 << 4) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn msm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn msm(mut self, val: u32) -> Smcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ts_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ts(mut self, val: u32) -> Smcr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn sms_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn sms(mut self, val: u32) -> Smcr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod smcr {
    #[inline(always)]
    pub fn read() -> super::Smcr {
        super::Smcr {
            raw: unsafe { *((0x40000C00u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Smcr) {
       unsafe { *((0x40000C00u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Dier {
   raw: u32,
}

impl Dier {
    #[inline(always)]
    pub fn tde_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tde(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn cc4de_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4de(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn cc3de_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3de(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn cc2de_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2de(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cc1de_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1de(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn ude_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ude(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn tie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tie(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn cc4ie_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4ie(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn cc3ie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3ie(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn cc2ie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2ie(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cc1ie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1ie(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn uie_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uie(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod dier {
    #[inline(always)]
    pub fn read() -> super::Dier {
        super::Dier {
            raw: unsafe { *((0x40000C00u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dier) {
       unsafe { *((0x40000C00u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn cc4of_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4of(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn cc3of_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3of(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn cc2of_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2of(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cc1of_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1of(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn tif_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tif(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn cc4if_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4if(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn cc3if_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3if(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn cc2if_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2if(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cc1if_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1if(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn uif_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uif(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40000C00u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40000C00u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Egr {
   raw: u32,
}

impl Egr {
    #[inline(always)]
    pub fn tg_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tg(mut self, val: u32) -> Egr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn cc4g_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4g(mut self, val: u32) -> Egr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn cc3g_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3g(mut self, val: u32) -> Egr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn cc2g_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2g(mut self, val: u32) -> Egr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cc1g_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1g(mut self, val: u32) -> Egr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ug_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ug(mut self, val: u32) -> Egr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod egr {
    #[inline(always)]
    pub fn read() -> super::Egr {
        super::Egr {
            raw: unsafe { *((0x40000C00u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Egr) {
       unsafe { *((0x40000C00u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccmr1_output {
   raw: u32,
}

impl Ccmr1_output {
    #[inline(always)]
    pub fn oc2ce_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc2ce(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn oc2m_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc2m(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn oc2pe_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc2pe(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn oc2fe_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc2fe(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cc2s_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc2s(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn oc1ce_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc1ce(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn oc1m_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc1m(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn oc1pe_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc1pe(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn oc1fe_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc1fe(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cc1s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc1s(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod ccmr1_output {
    #[inline(always)]
    pub fn read() -> super::Ccmr1_output {
        super::Ccmr1_output {
            raw: unsafe { *((0x40000C00u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr1_output) {
       unsafe { *((0x40000C00u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccmr1_input {
   raw: u32,
}

impl Ccmr1_input {
    #[inline(always)]
    pub fn ic2f_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ic2f(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 4) - 1) << 12)) | ((val & ((1 << 4) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn ic2psc_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ic2psc(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cc2s_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc2s(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ic1f_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ic1f(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ic1psc_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ic1psc(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cc1s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc1s(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod ccmr1_input {
    #[inline(always)]
    pub fn read() -> super::Ccmr1_input {
        super::Ccmr1_input {
            raw: unsafe { *((0x40000C00u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr1_input) {
       unsafe { *((0x40000C00u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccmr2_output {
   raw: u32,
}

impl Ccmr2_output {
    #[inline(always)]
    pub fn o24ce_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn o24ce(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn oc4m_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc4m(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn oc4pe_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc4pe(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn oc4fe_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc4fe(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cc4s_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc4s(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn oc3ce_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc3ce(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn oc3m_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc3m(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn oc3pe_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc3pe(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn oc3fe_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc3fe(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cc3s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc3s(mut self, val: u32) -> Ccmr2_output {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod ccmr2_output {
    #[inline(always)]
    pub fn read() -> super::Ccmr2_output {
        super::Ccmr2_output {
            raw: unsafe { *((0x40000C00u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr2_output) {
       unsafe { *((0x40000C00u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ccmr2_input {
   raw: u32,
}

impl Ccmr2_input {
    #[inline(always)]
    pub fn ic4f_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ic4f(mut self, val: u32) -> Ccmr2_input {
        self.raw = (self.raw & !(((1 << 4) - 1) << 12)) | ((val & ((1 << 4) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn ic4psc_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ic4psc(mut self, val: u32) -> Ccmr2_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cc4s_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc4s(mut self, val: u32) -> Ccmr2_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ic3f_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ic3f(mut self, val: u32) -> Ccmr2_input {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ic3psc_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ic3psc(mut self, val: u32) -> Ccmr2_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cc3s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc3s(mut self, val: u32) -> Ccmr2_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod ccmr2_input {
    #[inline(always)]
    pub fn read() -> super::Ccmr2_input {
        super::Ccmr2_input {
            raw: unsafe { *((0x40000C00u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr2_input) {
       unsafe { *((0x40000C00u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ccer {
   raw: u32,
}

impl Ccer {
    #[inline(always)]
    pub fn cc4p_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4p(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn cc4e_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4e(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn cc3p_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3p(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn cc3e_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3e(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn cc2p_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2p(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn cc2e_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2e(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn cc1p_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1p(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn cc1e_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1e(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x20u32) as *mut u32) = self.raw; }
    }
}

pub mod ccer {
    #[inline(always)]
    pub fn read() -> super::Ccer {
        super::Ccer {
            raw: unsafe { *((0x40000C00u32 + 0x20u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccer) {
       unsafe { *((0x40000C00u32 + 0x20u32) as *mut u32) = val.raw; }
    }
}

pub struct Cnt {
   raw: u32,
}

impl Cnt {
    #[inline(always)]
    pub fn cnt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn cnt(mut self, val: u32) -> Cnt {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x24u32) as *mut u32) = self.raw; }
    }
}

pub mod cnt {
    #[inline(always)]
    pub fn read() -> super::Cnt {
        super::Cnt {
            raw: unsafe { *((0x40000C00u32 + 0x24u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cnt) {
       unsafe { *((0x40000C00u32 + 0x24u32) as *mut u32) = val.raw; }
    }
}

pub struct Psc {
   raw: u32,
}

impl Psc {
    #[inline(always)]
    pub fn psc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn psc(mut self, val: u32) -> Psc {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x28u32) as *mut u32) = self.raw; }
    }
}

pub mod psc {
    #[inline(always)]
    pub fn read() -> super::Psc {
        super::Psc {
            raw: unsafe { *((0x40000C00u32 + 0x28u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Psc) {
       unsafe { *((0x40000C00u32 + 0x28u32) as *mut u32) = val.raw; }
    }
}

pub struct Arr {
   raw: u32,
}

impl Arr {
    #[inline(always)]
    pub fn arr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn arr(mut self, val: u32) -> Arr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x2Cu32) as *mut u32) = self.raw; }
    }
}

pub mod arr {
    #[inline(always)]
    pub fn read() -> super::Arr {
        super::Arr {
            raw: unsafe { *((0x40000C00u32 + 0x2Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Arr) {
       unsafe { *((0x40000C00u32 + 0x2Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr1 {
   raw: u32,
}

impl Ccr1 {
    #[inline(always)]
    pub fn ccr1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ccr1(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x34u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr1 {
    #[inline(always)]
    pub fn read() -> super::Ccr1 {
        super::Ccr1 {
            raw: unsafe { *((0x40000C00u32 + 0x34u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr1) {
       unsafe { *((0x40000C00u32 + 0x34u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr2 {
   raw: u32,
}

impl Ccr2 {
    #[inline(always)]
    pub fn ccr2_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ccr2(mut self, val: u32) -> Ccr2 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x38u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr2 {
    #[inline(always)]
    pub fn read() -> super::Ccr2 {
        super::Ccr2 {
            raw: unsafe { *((0x40000C00u32 + 0x38u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr2) {
       unsafe { *((0x40000C00u32 + 0x38u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr3 {
   raw: u32,
}

impl Ccr3 {
    #[inline(always)]
    pub fn ccr3_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ccr3(mut self, val: u32) -> Ccr3 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x3Cu32) as *mut u32) = self.raw; }
    }
}

pub mod ccr3 {
    #[inline(always)]
    pub fn read() -> super::Ccr3 {
        super::Ccr3 {
            raw: unsafe { *((0x40000C00u32 + 0x3Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr3) {
       unsafe { *((0x40000C00u32 + 0x3Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr4 {
   raw: u32,
}

impl Ccr4 {
    #[inline(always)]
    pub fn ccr4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ccr4(mut self, val: u32) -> Ccr4 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x40u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr4 {
    #[inline(always)]
    pub fn read() -> super::Ccr4 {
        super::Ccr4 {
            raw: unsafe { *((0x40000C00u32 + 0x40u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr4) {
       unsafe { *((0x40000C00u32 + 0x40u32) as *mut u32) = val.raw; }
    }
}

pub struct Dcr {
   raw: u32,
}

impl Dcr {
    #[inline(always)]
    pub fn dbl_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn dbl(mut self, val: u32) -> Dcr {
        self.raw = (self.raw & !(((1 << 5) - 1) << 8)) | ((val & ((1 << 5) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dba_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn dba(mut self, val: u32) -> Dcr {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x48u32) as *mut u32) = self.raw; }
    }
}

pub mod dcr {
    #[inline(always)]
    pub fn read() -> super::Dcr {
        super::Dcr {
            raw: unsafe { *((0x40000C00u32 + 0x48u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dcr) {
       unsafe { *((0x40000C00u32 + 0x48u32) as *mut u32) = val.raw; }
    }
}

pub struct Dmar {
   raw: u32,
}

impl Dmar {
    #[inline(always)]
    pub fn dmab_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn dmab(mut self, val: u32) -> Dmar {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40000C00u32 + 0x4Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dmar {
    #[inline(always)]
    pub fn read() -> super::Dmar {
        super::Dmar {
            raw: unsafe { *((0x40000C00u32 + 0x4Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmar) {
       unsafe { *((0x40000C00u32 + 0x4Cu32) as *mut u32) = val.raw; }
    }
}

