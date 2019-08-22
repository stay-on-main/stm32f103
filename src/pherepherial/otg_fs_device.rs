pub struct Fs_dcfg {
   raw: u32,
}

impl Fs_dcfg {
    #[inline(always)]
    pub fn dspd_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dspd(mut self, val: u32) -> Fs_dcfg {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn nzlsohsk_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nzlsohsk(mut self, val: u32) -> Fs_dcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_dcfg {
        self.raw = (self.raw & !(((1 << 7) - 1) << 4)) | ((val & ((1 << 7) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn pfivl_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pfivl(mut self, val: u32) -> Fs_dcfg {
        self.raw = (self.raw & !(((1 << 2) - 1) << 11)) | ((val & ((1 << 2) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_dcfg {
    #[inline(always)]
    pub fn read() -> super::Fs_dcfg {
        super::Fs_dcfg {
            raw: unsafe { *((0x50000800u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_dcfg) {
       unsafe { *((0x50000800u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_dctl {
   raw: u32,
}

impl Fs_dctl {
    #[inline(always)]
    pub fn rwusig_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rwusig(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn sdis_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sdis(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ginsts_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ginsts(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn gonsts_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gonsts(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn tctl_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn tctl(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn sginak_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sginak(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn cginak_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cginak(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn sgonak_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sgonak(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn cgonak_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cgonak(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn poprgdne_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn poprgdne(mut self, val: u32) -> Fs_dctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_dctl {
    #[inline(always)]
    pub fn read() -> super::Fs_dctl {
        super::Fs_dctl {
            raw: unsafe { *((0x50000800u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_dctl) {
       unsafe { *((0x50000800u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_dsts {
   raw: u32,
}

impl Fs_dsts {
    #[inline(always)]
    pub fn suspsts_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn suspsts(mut self, val: u32) -> Fs_dsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn enumspd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn enumspd(mut self, val: u32) -> Fs_dsts {
        self.raw = (self.raw & !(((1 << 2) - 1) << 1)) | ((val & ((1 << 2) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn eerr_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eerr(mut self, val: u32) -> Fs_dsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fnsof_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 14) - 1)
    }

    #[inline(always)]
    pub fn fnsof(mut self, val: u32) -> Fs_dsts {
        self.raw = (self.raw & !(((1 << 14) - 1) << 8)) | ((val & ((1 << 14) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_dsts {
    #[inline(always)]
    pub fn read() -> super::Fs_dsts {
        super::Fs_dsts {
            raw: unsafe { *((0x50000800u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_dsts) {
       unsafe { *((0x50000800u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_diepmsk {
   raw: u32,
}

impl Fs_diepmsk {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_diepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epdm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdm(mut self, val: u32) -> Fs_diepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn tom_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tom(mut self, val: u32) -> Fs_diepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ittxfemsk_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ittxfemsk(mut self, val: u32) -> Fs_diepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn inepnmm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn inepnmm(mut self, val: u32) -> Fs_diepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn inepnem_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn inepnem(mut self, val: u32) -> Fs_diepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_diepmsk {
    #[inline(always)]
    pub fn read() -> super::Fs_diepmsk {
        super::Fs_diepmsk {
            raw: unsafe { *((0x50000800u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_diepmsk) {
       unsafe { *((0x50000800u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_doepmsk {
   raw: u32,
}

impl Fs_doepmsk {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_doepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epdm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdm(mut self, val: u32) -> Fs_doepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stupm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stupm(mut self, val: u32) -> Fs_doepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn otepdm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn otepdm(mut self, val: u32) -> Fs_doepmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_doepmsk {
    #[inline(always)]
    pub fn read() -> super::Fs_doepmsk {
        super::Fs_doepmsk {
            raw: unsafe { *((0x50000800u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_doepmsk) {
       unsafe { *((0x50000800u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_daint {
   raw: u32,
}

impl Fs_daint {
    #[inline(always)]
    pub fn iepint_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn iepint(mut self, val: u32) -> Fs_daint {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn oepint_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn oepint(mut self, val: u32) -> Fs_daint {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_daint {
    #[inline(always)]
    pub fn read() -> super::Fs_daint {
        super::Fs_daint {
            raw: unsafe { *((0x50000800u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_daint) {
       unsafe { *((0x50000800u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_daintmsk {
   raw: u32,
}

impl Fs_daintmsk {
    #[inline(always)]
    pub fn iepm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn iepm(mut self, val: u32) -> Fs_daintmsk {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn oepint_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn oepint(mut self, val: u32) -> Fs_daintmsk {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod fs_daintmsk {
    #[inline(always)]
    pub fn read() -> super::Fs_daintmsk {
        super::Fs_daintmsk {
            raw: unsafe { *((0x50000800u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_daintmsk) {
       unsafe { *((0x50000800u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dvbusdis {
   raw: u32,
}

impl Dvbusdis {
    #[inline(always)]
    pub fn vbusdt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn vbusdt(mut self, val: u32) -> Dvbusdis {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x28u32) as *mut u32) = self.raw; }
    }
}

pub mod dvbusdis {
    #[inline(always)]
    pub fn read() -> super::Dvbusdis {
        super::Dvbusdis {
            raw: unsafe { *((0x50000800u32 + 0x28u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dvbusdis) {
       unsafe { *((0x50000800u32 + 0x28u32) as *mut u32) = val.raw; }
    }
}

pub struct Dvbuspulse {
   raw: u32,
}

impl Dvbuspulse {
    #[inline(always)]
    pub fn dvbusp_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dvbusp(mut self, val: u32) -> Dvbuspulse {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x2Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dvbuspulse {
    #[inline(always)]
    pub fn read() -> super::Dvbuspulse {
        super::Dvbuspulse {
            raw: unsafe { *((0x50000800u32 + 0x2Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dvbuspulse) {
       unsafe { *((0x50000800u32 + 0x2Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Diepempmsk {
   raw: u32,
}

impl Diepempmsk {
    #[inline(always)]
    pub fn ineptxfem_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptxfem(mut self, val: u32) -> Diepempmsk {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x34u32) as *mut u32) = self.raw; }
    }
}

pub mod diepempmsk {
    #[inline(always)]
    pub fn read() -> super::Diepempmsk {
        super::Diepempmsk {
            raw: unsafe { *((0x50000800u32 + 0x34u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Diepempmsk) {
       unsafe { *((0x50000800u32 + 0x34u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_diepctl0 {
   raw: u32,
}

impl Fs_diepctl0 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn usbaep_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbaep(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn naksts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn naksts(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn txfnum_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn txfnum(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 22)) | ((val & ((1 << 4) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn cnak_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnak(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn snak_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snak(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn epdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdis(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn epena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epena(mut self, val: u32) -> Fs_diepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x100u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_diepctl0 {
    #[inline(always)]
    pub fn read() -> super::Fs_diepctl0 {
        super::Fs_diepctl0 {
            raw: unsafe { *((0x50000800u32 + 0x100u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_diepctl0) {
       unsafe { *((0x50000800u32 + 0x100u32) as *mut u32) = val.raw; }
    }
}

pub struct Diepctl1 {
   raw: u32,
}

impl Diepctl1 {
    #[inline(always)]
    pub fn epena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epena(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn epdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdis(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn soddfrm_sd1pid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn soddfrm_sd1pid(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn snak_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snak(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn cnak_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnak(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn txfnum_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn txfnum(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 22)) | ((val & ((1 << 4) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn naksts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn naksts(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eonum_dpid_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eonum_dpid(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn usbaep_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbaep(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Diepctl1 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x120u32) as *mut u32) = self.raw; }
    }
}

pub mod diepctl1 {
    #[inline(always)]
    pub fn read() -> super::Diepctl1 {
        super::Diepctl1 {
            raw: unsafe { *((0x50000800u32 + 0x120u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Diepctl1) {
       unsafe { *((0x50000800u32 + 0x120u32) as *mut u32) = val.raw; }
    }
}

pub struct Diepctl2 {
   raw: u32,
}

impl Diepctl2 {
    #[inline(always)]
    pub fn epena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epena(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn epdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdis(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn soddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn soddfrm(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn snak_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snak(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn cnak_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnak(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn txfnum_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn txfnum(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 22)) | ((val & ((1 << 4) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn naksts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn naksts(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eonum_dpid_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eonum_dpid(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn usbaep_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbaep(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Diepctl2 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x140u32) as *mut u32) = self.raw; }
    }
}

pub mod diepctl2 {
    #[inline(always)]
    pub fn read() -> super::Diepctl2 {
        super::Diepctl2 {
            raw: unsafe { *((0x50000800u32 + 0x140u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Diepctl2) {
       unsafe { *((0x50000800u32 + 0x140u32) as *mut u32) = val.raw; }
    }
}

pub struct Diepctl3 {
   raw: u32,
}

impl Diepctl3 {
    #[inline(always)]
    pub fn epena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epena(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn epdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdis(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn soddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn soddfrm(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn snak_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snak(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn cnak_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnak(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn txfnum_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn txfnum(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 22)) | ((val & ((1 << 4) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn naksts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn naksts(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eonum_dpid_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eonum_dpid(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn usbaep_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbaep(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Diepctl3 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x160u32) as *mut u32) = self.raw; }
    }
}

pub mod diepctl3 {
    #[inline(always)]
    pub fn read() -> super::Diepctl3 {
        super::Diepctl3 {
            raw: unsafe { *((0x50000800u32 + 0x160u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Diepctl3) {
       unsafe { *((0x50000800u32 + 0x160u32) as *mut u32) = val.raw; }
    }
}

pub struct Doepctl0 {
   raw: u32,
}

impl Doepctl0 {
    #[inline(always)]
    pub fn epena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epena(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn epdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdis(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn snak_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snak(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn cnak_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnak(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn snpm_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snpm(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn naksts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn naksts(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn usbaep_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbaep(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Doepctl0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x300u32) as *mut u32) = self.raw; }
    }
}

pub mod doepctl0 {
    #[inline(always)]
    pub fn read() -> super::Doepctl0 {
        super::Doepctl0 {
            raw: unsafe { *((0x50000800u32 + 0x300u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doepctl0) {
       unsafe { *((0x50000800u32 + 0x300u32) as *mut u32) = val.raw; }
    }
}

pub struct Doepctl1 {
   raw: u32,
}

impl Doepctl1 {
    #[inline(always)]
    pub fn epena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epena(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn epdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdis(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn soddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn soddfrm(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn snak_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snak(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn cnak_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnak(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn snpm_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snpm(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn naksts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn naksts(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eonum_dpid_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eonum_dpid(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn usbaep_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbaep(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Doepctl1 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x320u32) as *mut u32) = self.raw; }
    }
}

pub mod doepctl1 {
    #[inline(always)]
    pub fn read() -> super::Doepctl1 {
        super::Doepctl1 {
            raw: unsafe { *((0x50000800u32 + 0x320u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doepctl1) {
       unsafe { *((0x50000800u32 + 0x320u32) as *mut u32) = val.raw; }
    }
}

pub struct Doepctl2 {
   raw: u32,
}

impl Doepctl2 {
    #[inline(always)]
    pub fn epena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epena(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn epdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdis(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn soddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn soddfrm(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn snak_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snak(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn cnak_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnak(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn snpm_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snpm(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn naksts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn naksts(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eonum_dpid_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eonum_dpid(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn usbaep_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbaep(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Doepctl2 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x340u32) as *mut u32) = self.raw; }
    }
}

pub mod doepctl2 {
    #[inline(always)]
    pub fn read() -> super::Doepctl2 {
        super::Doepctl2 {
            raw: unsafe { *((0x50000800u32 + 0x340u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doepctl2) {
       unsafe { *((0x50000800u32 + 0x340u32) as *mut u32) = val.raw; }
    }
}

pub struct Doepctl3 {
   raw: u32,
}

impl Doepctl3 {
    #[inline(always)]
    pub fn epena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epena(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn epdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdis(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn soddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn soddfrm(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sd0pid_sevnfrm(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn snak_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snak(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn cnak_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnak(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn snpm_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn snpm(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn naksts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn naksts(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eonum_dpid_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eonum_dpid(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn usbaep_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbaep(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Doepctl3 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x360u32) as *mut u32) = self.raw; }
    }
}

pub mod doepctl3 {
    #[inline(always)]
    pub fn read() -> super::Doepctl3 {
        super::Doepctl3 {
            raw: unsafe { *((0x50000800u32 + 0x360u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doepctl3) {
       unsafe { *((0x50000800u32 + 0x360u32) as *mut u32) = val.raw; }
    }
}

pub struct Diepint0 {
   raw: u32,
}

impl Diepint0 {
    #[inline(always)]
    pub fn txfe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfe(mut self, val: u32) -> Diepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn inepne_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn inepne(mut self, val: u32) -> Diepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ittxfe_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ittxfe(mut self, val: u32) -> Diepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn toc_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn toc(mut self, val: u32) -> Diepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn epdisd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdisd(mut self, val: u32) -> Diepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Diepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x108u32) as *mut u32) = self.raw; }
    }
}

pub mod diepint0 {
    #[inline(always)]
    pub fn read() -> super::Diepint0 {
        super::Diepint0 {
            raw: unsafe { *((0x50000800u32 + 0x108u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Diepint0) {
       unsafe { *((0x50000800u32 + 0x108u32) as *mut u32) = val.raw; }
    }
}

pub struct Diepint1 {
   raw: u32,
}

impl Diepint1 {
    #[inline(always)]
    pub fn txfe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfe(mut self, val: u32) -> Diepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn inepne_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn inepne(mut self, val: u32) -> Diepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ittxfe_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ittxfe(mut self, val: u32) -> Diepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn toc_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn toc(mut self, val: u32) -> Diepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn epdisd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdisd(mut self, val: u32) -> Diepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Diepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x128u32) as *mut u32) = self.raw; }
    }
}

pub mod diepint1 {
    #[inline(always)]
    pub fn read() -> super::Diepint1 {
        super::Diepint1 {
            raw: unsafe { *((0x50000800u32 + 0x128u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Diepint1) {
       unsafe { *((0x50000800u32 + 0x128u32) as *mut u32) = val.raw; }
    }
}

pub struct Diepint2 {
   raw: u32,
}

impl Diepint2 {
    #[inline(always)]
    pub fn txfe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfe(mut self, val: u32) -> Diepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn inepne_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn inepne(mut self, val: u32) -> Diepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ittxfe_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ittxfe(mut self, val: u32) -> Diepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn toc_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn toc(mut self, val: u32) -> Diepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn epdisd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdisd(mut self, val: u32) -> Diepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Diepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x148u32) as *mut u32) = self.raw; }
    }
}

pub mod diepint2 {
    #[inline(always)]
    pub fn read() -> super::Diepint2 {
        super::Diepint2 {
            raw: unsafe { *((0x50000800u32 + 0x148u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Diepint2) {
       unsafe { *((0x50000800u32 + 0x148u32) as *mut u32) = val.raw; }
    }
}

pub struct Diepint3 {
   raw: u32,
}

impl Diepint3 {
    #[inline(always)]
    pub fn txfe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfe(mut self, val: u32) -> Diepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn inepne_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn inepne(mut self, val: u32) -> Diepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ittxfe_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ittxfe(mut self, val: u32) -> Diepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn toc_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn toc(mut self, val: u32) -> Diepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn epdisd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdisd(mut self, val: u32) -> Diepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Diepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x168u32) as *mut u32) = self.raw; }
    }
}

pub mod diepint3 {
    #[inline(always)]
    pub fn read() -> super::Diepint3 {
        super::Diepint3 {
            raw: unsafe { *((0x50000800u32 + 0x168u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Diepint3) {
       unsafe { *((0x50000800u32 + 0x168u32) as *mut u32) = val.raw; }
    }
}

pub struct Doepint0 {
   raw: u32,
}

impl Doepint0 {
    #[inline(always)]
    pub fn b2bstup_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn b2bstup(mut self, val: u32) -> Doepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn otepdis_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn otepdis(mut self, val: u32) -> Doepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn stup_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stup(mut self, val: u32) -> Doepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn epdisd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdisd(mut self, val: u32) -> Doepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Doepint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x308u32) as *mut u32) = self.raw; }
    }
}

pub mod doepint0 {
    #[inline(always)]
    pub fn read() -> super::Doepint0 {
        super::Doepint0 {
            raw: unsafe { *((0x50000800u32 + 0x308u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doepint0) {
       unsafe { *((0x50000800u32 + 0x308u32) as *mut u32) = val.raw; }
    }
}

pub struct Doepint1 {
   raw: u32,
}

impl Doepint1 {
    #[inline(always)]
    pub fn b2bstup_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn b2bstup(mut self, val: u32) -> Doepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn otepdis_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn otepdis(mut self, val: u32) -> Doepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn stup_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stup(mut self, val: u32) -> Doepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn epdisd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdisd(mut self, val: u32) -> Doepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Doepint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x328u32) as *mut u32) = self.raw; }
    }
}

pub mod doepint1 {
    #[inline(always)]
    pub fn read() -> super::Doepint1 {
        super::Doepint1 {
            raw: unsafe { *((0x50000800u32 + 0x328u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doepint1) {
       unsafe { *((0x50000800u32 + 0x328u32) as *mut u32) = val.raw; }
    }
}

pub struct Doepint2 {
   raw: u32,
}

impl Doepint2 {
    #[inline(always)]
    pub fn b2bstup_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn b2bstup(mut self, val: u32) -> Doepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn otepdis_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn otepdis(mut self, val: u32) -> Doepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn stup_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stup(mut self, val: u32) -> Doepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn epdisd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdisd(mut self, val: u32) -> Doepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Doepint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x348u32) as *mut u32) = self.raw; }
    }
}

pub mod doepint2 {
    #[inline(always)]
    pub fn read() -> super::Doepint2 {
        super::Doepint2 {
            raw: unsafe { *((0x50000800u32 + 0x348u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doepint2) {
       unsafe { *((0x50000800u32 + 0x348u32) as *mut u32) = val.raw; }
    }
}

pub struct Doepint3 {
   raw: u32,
}

impl Doepint3 {
    #[inline(always)]
    pub fn b2bstup_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn b2bstup(mut self, val: u32) -> Doepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn otepdis_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn otepdis(mut self, val: u32) -> Doepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn stup_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stup(mut self, val: u32) -> Doepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn epdisd_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdisd(mut self, val: u32) -> Doepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Doepint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x368u32) as *mut u32) = self.raw; }
    }
}

pub mod doepint3 {
    #[inline(always)]
    pub fn read() -> super::Doepint3 {
        super::Doepint3 {
            raw: unsafe { *((0x50000800u32 + 0x368u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doepint3) {
       unsafe { *((0x50000800u32 + 0x368u32) as *mut u32) = val.raw; }
    }
}

pub struct Dieptsiz0 {
   raw: u32,
}

impl Dieptsiz0 {
    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Dieptsiz0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 19)) | ((val & ((1 << 2) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Dieptsiz0 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 0)) | ((val & ((1 << 7) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x110u32) as *mut u32) = self.raw; }
    }
}

pub mod dieptsiz0 {
    #[inline(always)]
    pub fn read() -> super::Dieptsiz0 {
        super::Dieptsiz0 {
            raw: unsafe { *((0x50000800u32 + 0x110u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dieptsiz0) {
       unsafe { *((0x50000800u32 + 0x110u32) as *mut u32) = val.raw; }
    }
}

pub struct Doeptsiz0 {
   raw: u32,
}

impl Doeptsiz0 {
    #[inline(always)]
    pub fn stupcnt_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stupcnt(mut self, val: u32) -> Doeptsiz0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Doeptsiz0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Doeptsiz0 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 0)) | ((val & ((1 << 7) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x310u32) as *mut u32) = self.raw; }
    }
}

pub mod doeptsiz0 {
    #[inline(always)]
    pub fn read() -> super::Doeptsiz0 {
        super::Doeptsiz0 {
            raw: unsafe { *((0x50000800u32 + 0x310u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doeptsiz0) {
       unsafe { *((0x50000800u32 + 0x310u32) as *mut u32) = val.raw; }
    }
}

pub struct Dieptsiz1 {
   raw: u32,
}

impl Dieptsiz1 {
    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Dieptsiz1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Dieptsiz1 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Dieptsiz1 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x130u32) as *mut u32) = self.raw; }
    }
}

pub mod dieptsiz1 {
    #[inline(always)]
    pub fn read() -> super::Dieptsiz1 {
        super::Dieptsiz1 {
            raw: unsafe { *((0x50000800u32 + 0x130u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dieptsiz1) {
       unsafe { *((0x50000800u32 + 0x130u32) as *mut u32) = val.raw; }
    }
}

pub struct Dieptsiz2 {
   raw: u32,
}

impl Dieptsiz2 {
    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Dieptsiz2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Dieptsiz2 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Dieptsiz2 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x150u32) as *mut u32) = self.raw; }
    }
}

pub mod dieptsiz2 {
    #[inline(always)]
    pub fn read() -> super::Dieptsiz2 {
        super::Dieptsiz2 {
            raw: unsafe { *((0x50000800u32 + 0x150u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dieptsiz2) {
       unsafe { *((0x50000800u32 + 0x150u32) as *mut u32) = val.raw; }
    }
}

pub struct Dieptsiz3 {
   raw: u32,
}

impl Dieptsiz3 {
    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Dieptsiz3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Dieptsiz3 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Dieptsiz3 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x170u32) as *mut u32) = self.raw; }
    }
}

pub mod dieptsiz3 {
    #[inline(always)]
    pub fn read() -> super::Dieptsiz3 {
        super::Dieptsiz3 {
            raw: unsafe { *((0x50000800u32 + 0x170u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dieptsiz3) {
       unsafe { *((0x50000800u32 + 0x170u32) as *mut u32) = val.raw; }
    }
}

pub struct Dtxfsts0 {
   raw: u32,
}

impl Dtxfsts0 {
    #[inline(always)]
    pub fn ineptfsav_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptfsav(mut self, val: u32) -> Dtxfsts0 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x118u32) as *mut u32) = self.raw; }
    }
}

pub mod dtxfsts0 {
    #[inline(always)]
    pub fn read() -> super::Dtxfsts0 {
        super::Dtxfsts0 {
            raw: unsafe { *((0x50000800u32 + 0x118u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dtxfsts0) {
       unsafe { *((0x50000800u32 + 0x118u32) as *mut u32) = val.raw; }
    }
}

pub struct Dtxfsts1 {
   raw: u32,
}

impl Dtxfsts1 {
    #[inline(always)]
    pub fn ineptfsav_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptfsav(mut self, val: u32) -> Dtxfsts1 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x138u32) as *mut u32) = self.raw; }
    }
}

pub mod dtxfsts1 {
    #[inline(always)]
    pub fn read() -> super::Dtxfsts1 {
        super::Dtxfsts1 {
            raw: unsafe { *((0x50000800u32 + 0x138u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dtxfsts1) {
       unsafe { *((0x50000800u32 + 0x138u32) as *mut u32) = val.raw; }
    }
}

pub struct Dtxfsts2 {
   raw: u32,
}

impl Dtxfsts2 {
    #[inline(always)]
    pub fn ineptfsav_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptfsav(mut self, val: u32) -> Dtxfsts2 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x158u32) as *mut u32) = self.raw; }
    }
}

pub mod dtxfsts2 {
    #[inline(always)]
    pub fn read() -> super::Dtxfsts2 {
        super::Dtxfsts2 {
            raw: unsafe { *((0x50000800u32 + 0x158u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dtxfsts2) {
       unsafe { *((0x50000800u32 + 0x158u32) as *mut u32) = val.raw; }
    }
}

pub struct Dtxfsts3 {
   raw: u32,
}

impl Dtxfsts3 {
    #[inline(always)]
    pub fn ineptfsav_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptfsav(mut self, val: u32) -> Dtxfsts3 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x178u32) as *mut u32) = self.raw; }
    }
}

pub mod dtxfsts3 {
    #[inline(always)]
    pub fn read() -> super::Dtxfsts3 {
        super::Dtxfsts3 {
            raw: unsafe { *((0x50000800u32 + 0x178u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dtxfsts3) {
       unsafe { *((0x50000800u32 + 0x178u32) as *mut u32) = val.raw; }
    }
}

pub struct Doeptsiz1 {
   raw: u32,
}

impl Doeptsiz1 {
    #[inline(always)]
    pub fn rxdpid_stupcnt_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn rxdpid_stupcnt(mut self, val: u32) -> Doeptsiz1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Doeptsiz1 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Doeptsiz1 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x330u32) as *mut u32) = self.raw; }
    }
}

pub mod doeptsiz1 {
    #[inline(always)]
    pub fn read() -> super::Doeptsiz1 {
        super::Doeptsiz1 {
            raw: unsafe { *((0x50000800u32 + 0x330u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doeptsiz1) {
       unsafe { *((0x50000800u32 + 0x330u32) as *mut u32) = val.raw; }
    }
}

pub struct Doeptsiz2 {
   raw: u32,
}

impl Doeptsiz2 {
    #[inline(always)]
    pub fn rxdpid_stupcnt_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn rxdpid_stupcnt(mut self, val: u32) -> Doeptsiz2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Doeptsiz2 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Doeptsiz2 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x350u32) as *mut u32) = self.raw; }
    }
}

pub mod doeptsiz2 {
    #[inline(always)]
    pub fn read() -> super::Doeptsiz2 {
        super::Doeptsiz2 {
            raw: unsafe { *((0x50000800u32 + 0x350u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doeptsiz2) {
       unsafe { *((0x50000800u32 + 0x350u32) as *mut u32) = val.raw; }
    }
}

pub struct Doeptsiz3 {
   raw: u32,
}

impl Doeptsiz3 {
    #[inline(always)]
    pub fn rxdpid_stupcnt_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn rxdpid_stupcnt(mut self, val: u32) -> Doeptsiz3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Doeptsiz3 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Doeptsiz3 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000800u32 + 0x370u32) as *mut u32) = self.raw; }
    }
}

pub mod doeptsiz3 {
    #[inline(always)]
    pub fn read() -> super::Doeptsiz3 {
        super::Doeptsiz3 {
            raw: unsafe { *((0x50000800u32 + 0x370u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Doeptsiz3) {
       unsafe { *((0x50000800u32 + 0x370u32) as *mut u32) = val.raw; }
    }
}

