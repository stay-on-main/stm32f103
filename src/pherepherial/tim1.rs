pub struct Cr1 {
   raw: u32,
}

impl Cr1 {
    #[inline(always)]
    pub fn ckd_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ckd_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn arpe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn arpe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn cms_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cms_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 5)) | ((val & ((1 << 2) - 1)) << 5)
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
    pub fn opm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn opm_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn urs_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn urs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn udis_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn udis_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn cen_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod cr1 {
    #[inline(always)]
    pub fn read() -> super::Cr1 {
        super::Cr1 {
            raw: unsafe { *((0x40012C00 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr1) {
       unsafe { *((0x40012C00 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Cr2 {
   raw: u32,
}

impl Cr2 {
    #[inline(always)]
    pub fn ois4_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ois4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn ois3n_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ois3n_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn ois3_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ois3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn ois2n_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ois2n_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn ois2_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ois2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn ois1n_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ois1n_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn ois1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ois1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn ti1s_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ti1s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn mms_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn mms_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4)
    }

    #[inline(always)]
    pub fn ccds_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ccds_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn ccus_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ccus_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn ccpc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ccpc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod cr2 {
    #[inline(always)]
    pub fn read() -> super::Cr2 {
        super::Cr2 {
            raw: unsafe { *((0x40012C00 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr2) {
       unsafe { *((0x40012C00 + 0x4) as *mut u32) = val.raw; }
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
    pub fn etp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn ece_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ece_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn etps_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn etps_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12)
    }

    #[inline(always)]
    pub fn etf_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn etf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 8)) | ((val & ((1 << 4) - 1)) << 8)
    }

    #[inline(always)]
    pub fn msm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn msm_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn ts_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ts_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4)
    }

    #[inline(always)]
    pub fn sms_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn sms_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0)
    }

}

pub mod smcr {
    #[inline(always)]
    pub fn read() -> super::Smcr {
        super::Smcr {
            raw: unsafe { *((0x40012C00 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Smcr) {
       unsafe { *((0x40012C00 + 0x8) as *mut u32) = val.raw; }
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
    pub fn tde_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn comde_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn comde_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn cc4de_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4de_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn cc3de_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3de_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn cc2de_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2de_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn cc1de_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1de_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn ude_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ude_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn tie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn cc4ie_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4ie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn cc3ie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3ie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn cc2ie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2ie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cc1ie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1ie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn uie_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn bie_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn comie_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn comie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

}

pub mod dier {
    #[inline(always)]
    pub fn read() -> super::Dier {
        super::Dier {
            raw: unsafe { *((0x40012C00 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dier) {
       unsafe { *((0x40012C00 + 0xC) as *mut u32) = val.raw; }
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
    pub fn cc4of_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn cc3of_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3of_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn cc2of_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2of_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn cc1of_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1of_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn bif_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bif_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn tif_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tif_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn comif_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn comif_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn cc4if_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4if_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn cc3if_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3if_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn cc2if_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2if_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cc1if_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1if_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn uif_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uif_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40012C00 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40012C00 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Egr {
   raw: u32,
}

impl Egr {
    #[inline(always)]
    pub fn bg_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bg_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn tg_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tg_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn comg_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn comg_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn cc4g_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4g_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn cc3g_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3g_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn cc2g_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2g_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cc1g_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1g_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn ug_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ug_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod egr {
    #[inline(always)]
    pub fn read() -> super::Egr {
        super::Egr {
            raw: unsafe { *((0x40012C00 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Egr) {
       unsafe { *((0x40012C00 + 0x14) as *mut u32) = val.raw; }
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
    pub fn oc2ce_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn oc2m_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc2m_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12)
    }

    #[inline(always)]
    pub fn oc2pe_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc2pe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn oc2fe_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc2fe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn cc2s_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc2s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn oc1ce_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc1ce_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn oc1m_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc1m_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4)
    }

    #[inline(always)]
    pub fn oc1pe_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc1pe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn oc1fe_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc1fe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cc1s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc1s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0)
    }

}

pub mod ccmr1_output {
    #[inline(always)]
    pub fn read() -> super::Ccmr1_output {
        super::Ccmr1_output {
            raw: unsafe { *((0x40012C00 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr1_output) {
       unsafe { *((0x40012C00 + 0x18) as *mut u32) = val.raw; }
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
    pub fn ic2f_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 12)) | ((val & ((1 << 4) - 1)) << 12)
    }

    #[inline(always)]
    pub fn ic2pcs_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ic2pcs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn cc2s_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc2s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn ic1f_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ic1f_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn icpcs_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn icpcs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cc1s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc1s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0)
    }

}

pub mod ccmr1_input {
    #[inline(always)]
    pub fn read() -> super::Ccmr1_input {
        super::Ccmr1_input {
            raw: unsafe { *((0x40012C00 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr1_input) {
       unsafe { *((0x40012C00 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Ccmr2_output {
   raw: u32,
}

impl Ccmr2_output {
    #[inline(always)]
    pub fn oc4ce_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc4ce_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn oc4m_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc4m_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 12)) | ((val & ((1 << 3) - 1)) << 12)
    }

    #[inline(always)]
    pub fn oc4pe_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc4pe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn oc4fe_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc4fe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn cc4s_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc4s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn oc3ce_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc3ce_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn oc3m_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc3m_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4)
    }

    #[inline(always)]
    pub fn oc3pe_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc3pe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn oc3fe_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc3fe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cc3s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc3s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0)
    }

}

pub mod ccmr2_output {
    #[inline(always)]
    pub fn read() -> super::Ccmr2_output {
        super::Ccmr2_output {
            raw: unsafe { *((0x40012C00 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr2_output) {
       unsafe { *((0x40012C00 + 0x1C) as *mut u32) = val.raw; }
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
    pub fn ic4f_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 12)) | ((val & ((1 << 4) - 1)) << 12)
    }

    #[inline(always)]
    pub fn ic4psc_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ic4psc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10)
    }

    #[inline(always)]
    pub fn cc4s_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc4s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn ic3f_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ic3f_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn ic3psc_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ic3psc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cc3s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc3s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0)
    }

}

pub mod ccmr2_input {
    #[inline(always)]
    pub fn read() -> super::Ccmr2_input {
        super::Ccmr2_input {
            raw: unsafe { *((0x40012C00 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr2_input) {
       unsafe { *((0x40012C00 + 0x1C) as *mut u32) = val.raw; }
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
    pub fn cc4p_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn cc4e_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc4e_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn cc3np_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3np_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn cc3ne_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3ne_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn cc3p_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3p_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn cc3e_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc3e_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn cc2np_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2np_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn cc2ne_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2ne_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn cc2p_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2p_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn cc2e_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc2e_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn cc1np_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1np_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn cc1ne_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1ne_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn cc1p_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1p_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn cc1e_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1e_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod ccer {
    #[inline(always)]
    pub fn read() -> super::Ccer {
        super::Ccer {
            raw: unsafe { *((0x40012C00 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccer) {
       unsafe { *((0x40012C00 + 0x20) as *mut u32) = val.raw; }
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
    pub fn cnt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cnt {
    #[inline(always)]
    pub fn read() -> super::Cnt {
        super::Cnt {
            raw: unsafe { *((0x40012C00 + 0x24) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cnt) {
       unsafe { *((0x40012C00 + 0x24) as *mut u32) = val.raw; }
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
    pub fn psc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod psc {
    #[inline(always)]
    pub fn read() -> super::Psc {
        super::Psc {
            raw: unsafe { *((0x40012C00 + 0x28) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Psc) {
       unsafe { *((0x40012C00 + 0x28) as *mut u32) = val.raw; }
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
    pub fn arr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod arr {
    #[inline(always)]
    pub fn read() -> super::Arr {
        super::Arr {
            raw: unsafe { *((0x40012C00 + 0x2C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Arr) {
       unsafe { *((0x40012C00 + 0x2C) as *mut u32) = val.raw; }
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
    pub fn ccr1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod ccr1 {
    #[inline(always)]
    pub fn read() -> super::Ccr1 {
        super::Ccr1 {
            raw: unsafe { *((0x40012C00 + 0x34) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr1) {
       unsafe { *((0x40012C00 + 0x34) as *mut u32) = val.raw; }
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
    pub fn ccr2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod ccr2 {
    #[inline(always)]
    pub fn read() -> super::Ccr2 {
        super::Ccr2 {
            raw: unsafe { *((0x40012C00 + 0x38) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr2) {
       unsafe { *((0x40012C00 + 0x38) as *mut u32) = val.raw; }
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
    pub fn ccr3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod ccr3 {
    #[inline(always)]
    pub fn read() -> super::Ccr3 {
        super::Ccr3 {
            raw: unsafe { *((0x40012C00 + 0x3C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr3) {
       unsafe { *((0x40012C00 + 0x3C) as *mut u32) = val.raw; }
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
    pub fn ccr4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod ccr4 {
    #[inline(always)]
    pub fn read() -> super::Ccr4 {
        super::Ccr4 {
            raw: unsafe { *((0x40012C00 + 0x40) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr4) {
       unsafe { *((0x40012C00 + 0x40) as *mut u32) = val.raw; }
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
    pub fn dbl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 8)) | ((val & ((1 << 5) - 1)) << 8)
    }

    #[inline(always)]
    pub fn dba_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn dba_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 0)) | ((val & ((1 << 5) - 1)) << 0)
    }

}

pub mod dcr {
    #[inline(always)]
    pub fn read() -> super::Dcr {
        super::Dcr {
            raw: unsafe { *((0x40012C00 + 0x48) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dcr) {
       unsafe { *((0x40012C00 + 0x48) as *mut u32) = val.raw; }
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
    pub fn dmab_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod dmar {
    #[inline(always)]
    pub fn read() -> super::Dmar {
        super::Dmar {
            raw: unsafe { *((0x40012C00 + 0x4C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmar) {
       unsafe { *((0x40012C00 + 0x4C) as *mut u32) = val.raw; }
    }
}

pub struct Rcr {
   raw: u32,
}

impl Rcr {
    #[inline(always)]
    pub fn rep_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn rep_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod rcr {
    #[inline(always)]
    pub fn read() -> super::Rcr {
        super::Rcr {
            raw: unsafe { *((0x40012C00 + 0x30) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Rcr) {
       unsafe { *((0x40012C00 + 0x30) as *mut u32) = val.raw; }
    }
}

pub struct Bdtr {
   raw: u32,
}

impl Bdtr {
    #[inline(always)]
    pub fn moe_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn moe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn aoe_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn aoe_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn bkp_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bkp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn bke_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bke_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn ossr_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ossr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn ossi_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ossi_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn lock_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn lock_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8)
    }

    #[inline(always)]
    pub fn dtg_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn dtg_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod bdtr {
    #[inline(always)]
    pub fn read() -> super::Bdtr {
        super::Bdtr {
            raw: unsafe { *((0x40012C00 + 0x44) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bdtr) {
       unsafe { *((0x40012C00 + 0x44) as *mut u32) = val.raw; }
    }
}

