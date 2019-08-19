pub struct Dmabmr {
   raw: u32,
}

impl Dmabmr {
    #[inline(always)]
    pub fn sr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn da_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn da_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn dsl_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn dsl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 2)) | ((val & ((1 << 5) - 1)) << 2)
    }

    #[inline(always)]
    pub fn pbl_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn pbl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 6) - 1) << 8)) | ((val & ((1 << 6) - 1)) << 8)
    }

    #[inline(always)]
    pub fn rtpr_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn rtpr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 14)) | ((val & ((1 << 2) - 1)) << 14)
    }

    #[inline(always)]
    pub fn fb_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn rdp_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn rdp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 6) - 1) << 17)) | ((val & ((1 << 6) - 1)) << 17)
    }

    #[inline(always)]
    pub fn usp_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23)
    }

    #[inline(always)]
    pub fn fpm_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fpm_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24)
    }

    #[inline(always)]
    pub fn aab_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn aab_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25)
    }

}

pub mod dmabmr {
    #[inline(always)]
    pub fn read() -> super::Dmabmr {
        super::Dmabmr {
            raw: unsafe { *((0x40029000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmabmr) {
       unsafe { *((0x40029000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Dmatpdr {
   raw: u32,
}

impl Dmatpdr {
    #[inline(always)]
    pub fn tpd_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn tpd_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dmatpdr {
    #[inline(always)]
    pub fn read() -> super::Dmatpdr {
        super::Dmatpdr {
            raw: unsafe { *((0x40029000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmatpdr) {
       unsafe { *((0x40029000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Dmarpdr {
   raw: u32,
}

impl Dmarpdr {
    #[inline(always)]
    pub fn rpd_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn rpd_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dmarpdr {
    #[inline(always)]
    pub fn read() -> super::Dmarpdr {
        super::Dmarpdr {
            raw: unsafe { *((0x40029000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmarpdr) {
       unsafe { *((0x40029000 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Dmardlar {
   raw: u32,
}

impl Dmardlar {
    #[inline(always)]
    pub fn srl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn srl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dmardlar {
    #[inline(always)]
    pub fn read() -> super::Dmardlar {
        super::Dmardlar {
            raw: unsafe { *((0x40029000 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmardlar) {
       unsafe { *((0x40029000 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Dmatdlar {
   raw: u32,
}

impl Dmatdlar {
    #[inline(always)]
    pub fn stl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn stl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dmatdlar {
    #[inline(always)]
    pub fn read() -> super::Dmatdlar {
        super::Dmatdlar {
            raw: unsafe { *((0x40029000 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmatdlar) {
       unsafe { *((0x40029000 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Dmasr {
   raw: u32,
}

impl Dmasr {
    #[inline(always)]
    pub fn ts_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ts_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tpss_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tpss_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn tbus_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tbus_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn tjts_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tjts_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn ros_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ros_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn tus_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tus_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn rs_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn rbus_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rbus_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn rpss_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rpss_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn pwts_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwts_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn ets_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ets_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn fbes_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbes_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn ers_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ers_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn ais_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ais_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn nis_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nis_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn rps_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn rps_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 17)) | ((val & ((1 << 3) - 1)) << 17)
    }

    #[inline(always)]
    pub fn tps_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn tps_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 20)) | ((val & ((1 << 3) - 1)) << 20)
    }

    #[inline(always)]
    pub fn ebs_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ebs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 23)) | ((val & ((1 << 3) - 1)) << 23)
    }

    #[inline(always)]
    pub fn mmcs_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmcs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27)
    }

    #[inline(always)]
    pub fn pmts_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pmts_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28)
    }

    #[inline(always)]
    pub fn tsts_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsts_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29)
    }

}

pub mod dmasr {
    #[inline(always)]
    pub fn read() -> super::Dmasr {
        super::Dmasr {
            raw: unsafe { *((0x40029000 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmasr) {
       unsafe { *((0x40029000 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Dmaomr {
   raw: u32,
}

impl Dmaomr {
    #[inline(always)]
    pub fn sr_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn osf_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn osf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn rtc_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn rtc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 3)) | ((val & ((1 << 2) - 1)) << 3)
    }

    #[inline(always)]
    pub fn fugf_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fugf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn fef_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fef_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn st_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn st_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn ttc_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ttc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 14)) | ((val & ((1 << 3) - 1)) << 14)
    }

    #[inline(always)]
    pub fn ftf_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ftf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20)
    }

    #[inline(always)]
    pub fn tsf_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21)
    }

    #[inline(always)]
    pub fn dfrf_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dfrf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24)
    }

    #[inline(always)]
    pub fn rsf_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rsf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25)
    }

    #[inline(always)]
    pub fn dtcefd_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtcefd_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26)
    }

}

pub mod dmaomr {
    #[inline(always)]
    pub fn read() -> super::Dmaomr {
        super::Dmaomr {
            raw: unsafe { *((0x40029000 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmaomr) {
       unsafe { *((0x40029000 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Dmaier {
   raw: u32,
}

impl Dmaier {
    #[inline(always)]
    pub fn tie_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tpsie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tpsie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn tbuie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tbuie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn tjtie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tjtie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn roie_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn roie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn tuie_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tuie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn rie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn rbuie_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rbuie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn rpsie_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rpsie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn rwtie_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rwtie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn etie_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn etie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn fbeie_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbeie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn erie_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn erie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn aise_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn aise_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn nise_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nise_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

}

pub mod dmaier {
    #[inline(always)]
    pub fn read() -> super::Dmaier {
        super::Dmaier {
            raw: unsafe { *((0x40029000 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmaier) {
       unsafe { *((0x40029000 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Dmamfbocr {
   raw: u32,
}

impl Dmamfbocr {
    #[inline(always)]
    pub fn mfc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn mfc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

    #[inline(always)]
    pub fn omfc_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn omfc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn mfa_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mfa_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 11) - 1) << 17)) | ((val & ((1 << 11) - 1)) << 17)
    }

    #[inline(always)]
    pub fn ofoc_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ofoc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28)
    }

}

pub mod dmamfbocr {
    #[inline(always)]
    pub fn read() -> super::Dmamfbocr {
        super::Dmamfbocr {
            raw: unsafe { *((0x40029000 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmamfbocr) {
       unsafe { *((0x40029000 + 0x20) as *mut u32) = val.raw; }
    }
}

pub struct Dmachtdr {
   raw: u32,
}

impl Dmachtdr {
    #[inline(always)]
    pub fn htdap_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn htdap_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dmachtdr {
    #[inline(always)]
    pub fn read() -> super::Dmachtdr {
        super::Dmachtdr {
            raw: unsafe { *((0x40029000 + 0x48) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmachtdr) {
       unsafe { *((0x40029000 + 0x48) as *mut u32) = val.raw; }
    }
}

pub struct Dmachrdr {
   raw: u32,
}

impl Dmachrdr {
    #[inline(always)]
    pub fn hrdap_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn hrdap_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dmachrdr {
    #[inline(always)]
    pub fn read() -> super::Dmachrdr {
        super::Dmachrdr {
            raw: unsafe { *((0x40029000 + 0x4C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmachrdr) {
       unsafe { *((0x40029000 + 0x4C) as *mut u32) = val.raw; }
    }
}

pub struct Dmachtbar {
   raw: u32,
}

impl Dmachtbar {
    #[inline(always)]
    pub fn htbap_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn htbap_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dmachtbar {
    #[inline(always)]
    pub fn read() -> super::Dmachtbar {
        super::Dmachtbar {
            raw: unsafe { *((0x40029000 + 0x50) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmachtbar) {
       unsafe { *((0x40029000 + 0x50) as *mut u32) = val.raw; }
    }
}

pub struct Dmachrbar {
   raw: u32,
}

impl Dmachrbar {
    #[inline(always)]
    pub fn hrbap_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn hrbap_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dmachrbar {
    #[inline(always)]
    pub fn read() -> super::Dmachrbar {
        super::Dmachrbar {
            raw: unsafe { *((0x40029000 + 0x54) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmachrbar) {
       unsafe { *((0x40029000 + 0x54) as *mut u32) = val.raw; }
    }
}

