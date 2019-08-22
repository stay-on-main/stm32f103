pub struct Dmabmr {
   raw: u32,
}

impl Dmabmr {
    #[inline(always)]
    pub fn sr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sr(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn da_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn da(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn dsl_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn dsl(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 5) - 1) << 2)) | ((val & ((1 << 5) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn pbl_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn pbl(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 6) - 1) << 8)) | ((val & ((1 << 6) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn rtpr_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn rtpr(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 14)) | ((val & ((1 << 2) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn rdp_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn rdp(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 6) - 1) << 17)) | ((val & ((1 << 6) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn usp_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usp(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fpm_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fpm(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn aab_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn aab(mut self, val: u32) -> Dmabmr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod dmabmr {
    #[inline(always)]
    pub fn read() -> super::Dmabmr {
        super::Dmabmr {
            raw: unsafe { *((0x40029000u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmabmr) {
       unsafe { *((0x40029000u32 + 0x0u32) as *mut u32) = val.raw; }
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
    pub fn tpd(mut self, val: u32) -> Dmatpdr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod dmatpdr {
    #[inline(always)]
    pub fn read() -> super::Dmatpdr {
        super::Dmatpdr {
            raw: unsafe { *((0x40029000u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmatpdr) {
       unsafe { *((0x40029000u32 + 0x4u32) as *mut u32) = val.raw; }
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
    pub fn rpd(mut self, val: u32) -> Dmarpdr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod dmarpdr {
    #[inline(always)]
    pub fn read() -> super::Dmarpdr {
        super::Dmarpdr {
            raw: unsafe { *((0x40029000u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmarpdr) {
       unsafe { *((0x40029000u32 + 0x8u32) as *mut u32) = val.raw; }
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
    pub fn srl(mut self, val: u32) -> Dmardlar {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod dmardlar {
    #[inline(always)]
    pub fn read() -> super::Dmardlar {
        super::Dmardlar {
            raw: unsafe { *((0x40029000u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmardlar) {
       unsafe { *((0x40029000u32 + 0xCu32) as *mut u32) = val.raw; }
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
    pub fn stl(mut self, val: u32) -> Dmatdlar {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod dmatdlar {
    #[inline(always)]
    pub fn read() -> super::Dmatdlar {
        super::Dmatdlar {
            raw: unsafe { *((0x40029000u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmatdlar) {
       unsafe { *((0x40029000u32 + 0x10u32) as *mut u32) = val.raw; }
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
    pub fn ts(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tpss_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tpss(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn tbus_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tbus(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn tjts_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tjts(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ros_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ros(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn tus_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tus(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn rs_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rs(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn rbus_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rbus(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn rpss_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rpss(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn pwts_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwts(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn ets_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ets(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fbes_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbes(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn ers_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ers(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ais_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ais(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn nis_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nis(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn rps_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn rps(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 17)) | ((val & ((1 << 3) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn tps_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn tps(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 20)) | ((val & ((1 << 3) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn ebs_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ebs(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 23)) | ((val & ((1 << 3) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn mmcs_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmcs(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn pmts_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pmts(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn tsts_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsts(mut self, val: u32) -> Dmasr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod dmasr {
    #[inline(always)]
    pub fn read() -> super::Dmasr {
        super::Dmasr {
            raw: unsafe { *((0x40029000u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmasr) {
       unsafe { *((0x40029000u32 + 0x14u32) as *mut u32) = val.raw; }
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
    pub fn sr(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn osf_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn osf(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rtc_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn rtc(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 3)) | ((val & ((1 << 2) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fugf_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fugf(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fef_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fef(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn st_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn st(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn ttc_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ttc(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 14)) | ((val & ((1 << 3) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ftf_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ftf(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn tsf_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsf(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn dfrf_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dfrf(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn rsf_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rsf(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn dtcefd_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtcefd(mut self, val: u32) -> Dmaomr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod dmaomr {
    #[inline(always)]
    pub fn read() -> super::Dmaomr {
        super::Dmaomr {
            raw: unsafe { *((0x40029000u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmaomr) {
       unsafe { *((0x40029000u32 + 0x18u32) as *mut u32) = val.raw; }
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
    pub fn tie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tpsie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tpsie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn tbuie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tbuie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn tjtie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tjtie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn roie_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn roie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn tuie_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tuie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn rie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn rbuie_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rbuie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn rpsie_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rpsie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn rwtie_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rwtie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn etie_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn etie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fbeie_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbeie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn erie_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn erie(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn aise_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn aise(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn nise_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nise(mut self, val: u32) -> Dmaier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dmaier {
    #[inline(always)]
    pub fn read() -> super::Dmaier {
        super::Dmaier {
            raw: unsafe { *((0x40029000u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmaier) {
       unsafe { *((0x40029000u32 + 0x1Cu32) as *mut u32) = val.raw; }
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
    pub fn mfc(mut self, val: u32) -> Dmamfbocr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn omfc_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn omfc(mut self, val: u32) -> Dmamfbocr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn mfa_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mfa(mut self, val: u32) -> Dmamfbocr {
        self.raw = (self.raw & !(((1 << 11) - 1) << 17)) | ((val & ((1 << 11) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn ofoc_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ofoc(mut self, val: u32) -> Dmamfbocr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x20u32) as *mut u32) = self.raw; }
    }
}

pub mod dmamfbocr {
    #[inline(always)]
    pub fn read() -> super::Dmamfbocr {
        super::Dmamfbocr {
            raw: unsafe { *((0x40029000u32 + 0x20u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmamfbocr) {
       unsafe { *((0x40029000u32 + 0x20u32) as *mut u32) = val.raw; }
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
    pub fn htdap(mut self, val: u32) -> Dmachtdr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x48u32) as *mut u32) = self.raw; }
    }
}

pub mod dmachtdr {
    #[inline(always)]
    pub fn read() -> super::Dmachtdr {
        super::Dmachtdr {
            raw: unsafe { *((0x40029000u32 + 0x48u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmachtdr) {
       unsafe { *((0x40029000u32 + 0x48u32) as *mut u32) = val.raw; }
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
    pub fn hrdap(mut self, val: u32) -> Dmachrdr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x4Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dmachrdr {
    #[inline(always)]
    pub fn read() -> super::Dmachrdr {
        super::Dmachrdr {
            raw: unsafe { *((0x40029000u32 + 0x4Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmachrdr) {
       unsafe { *((0x40029000u32 + 0x4Cu32) as *mut u32) = val.raw; }
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
    pub fn htbap(mut self, val: u32) -> Dmachtbar {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x50u32) as *mut u32) = self.raw; }
    }
}

pub mod dmachtbar {
    #[inline(always)]
    pub fn read() -> super::Dmachtbar {
        super::Dmachtbar {
            raw: unsafe { *((0x40029000u32 + 0x50u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmachtbar) {
       unsafe { *((0x40029000u32 + 0x50u32) as *mut u32) = val.raw; }
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
    pub fn hrbap(mut self, val: u32) -> Dmachrbar {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40029000u32 + 0x54u32) as *mut u32) = self.raw; }
    }
}

pub mod dmachrbar {
    #[inline(always)]
    pub fn read() -> super::Dmachrbar {
        super::Dmachrbar {
            raw: unsafe { *((0x40029000u32 + 0x54u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dmachrbar) {
       unsafe { *((0x40029000u32 + 0x54u32) as *mut u32) = val.raw; }
    }
}

