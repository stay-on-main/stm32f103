pub struct Fs_hcfg {
   raw: u32,
}

impl Fs_hcfg {
    #[inline(always)]
    pub fn fslspcs_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn fslspcs(mut self, val: u32) -> Fs_hcfg {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fslss_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fslss(mut self, val: u32) -> Fs_hcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcfg {
    #[inline(always)]
    pub fn read() -> super::Fs_hcfg {
        super::Fs_hcfg {
            raw: unsafe { *((0x50000400 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcfg) {
       unsafe { *((0x50000400 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Hfir {
   raw: u32,
}

impl Hfir {
    #[inline(always)]
    pub fn frivl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn frivl(mut self, val: u32) -> Hfir {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod hfir {
    #[inline(always)]
    pub fn read() -> super::Hfir {
        super::Hfir {
            raw: unsafe { *((0x50000400 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Hfir) {
       unsafe { *((0x50000400 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hfnum {
   raw: u32,
}

impl Fs_hfnum {
    #[inline(always)]
    pub fn frnum_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn frnum(mut self, val: u32) -> Fs_hfnum {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ftrem_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ftrem(mut self, val: u32) -> Fs_hfnum {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod fs_hfnum {
    #[inline(always)]
    pub fn read() -> super::Fs_hfnum {
        super::Fs_hfnum {
            raw: unsafe { *((0x50000400 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hfnum) {
       unsafe { *((0x50000400 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hptxsts {
   raw: u32,
}

impl Fs_hptxsts {
    #[inline(always)]
    pub fn ptxfsavl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ptxfsavl(mut self, val: u32) -> Fs_hptxsts {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ptxqsav_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ptxqsav(mut self, val: u32) -> Fs_hptxsts {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ptxqtop_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ptxqtop(mut self, val: u32) -> Fs_hptxsts {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod fs_hptxsts {
    #[inline(always)]
    pub fn read() -> super::Fs_hptxsts {
        super::Fs_hptxsts {
            raw: unsafe { *((0x50000400 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hptxsts) {
       unsafe { *((0x50000400 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Haint {
   raw: u32,
}

impl Haint {
    #[inline(always)]
    pub fn haint_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn haint(mut self, val: u32) -> Haint {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod haint {
    #[inline(always)]
    pub fn read() -> super::Haint {
        super::Haint {
            raw: unsafe { *((0x50000400 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Haint) {
       unsafe { *((0x50000400 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Haintmsk {
   raw: u32,
}

impl Haintmsk {
    #[inline(always)]
    pub fn haintm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn haintm(mut self, val: u32) -> Haintmsk {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod haintmsk {
    #[inline(always)]
    pub fn read() -> super::Haintmsk {
        super::Haintmsk {
            raw: unsafe { *((0x50000400 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Haintmsk) {
       unsafe { *((0x50000400 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hprt {
   raw: u32,
}

impl Fs_hprt {
    #[inline(always)]
    pub fn pcsts_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pcsts(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pcdet_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pcdet(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn pena_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pena(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn penchng_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn penchng(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn poca_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn poca(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn pocchng_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pocchng(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pres_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pres(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn psusp_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn psusp(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn prst_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn prst(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn plsts_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn plsts(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn ppwr_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ppwr(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn ptctl_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ptctl(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 4) - 1) << 13)) | ((val & ((1 << 4) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn pspd_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pspd(mut self, val: u32) -> Fs_hprt {
        self.raw = (self.raw & !(((1 << 2) - 1) << 17)) | ((val & ((1 << 2) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x40) as *mut u32) = self.raw; }
    }
}

pub mod fs_hprt {
    #[inline(always)]
    pub fn read() -> super::Fs_hprt {
        super::Fs_hprt {
            raw: unsafe { *((0x50000400 + 0x40) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hprt) {
       unsafe { *((0x50000400 + 0x40) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcchar0 {
   raw: u32,
}

impl Fs_hcchar0 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 11)) | ((val & ((1 << 4) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn epdir_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdir(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lsdev_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsdev(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 22)) | ((val & ((1 << 7) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn oddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oddfrm(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn chdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chdis(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn chena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chena(mut self, val: u32) -> Fs_hcchar0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x100) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcchar0 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcchar0 {
        super::Fs_hcchar0 {
            raw: unsafe { *((0x50000400 + 0x100) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcchar0) {
       unsafe { *((0x50000400 + 0x100) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcchar1 {
   raw: u32,
}

impl Fs_hcchar1 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 11)) | ((val & ((1 << 4) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn epdir_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdir(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lsdev_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsdev(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 22)) | ((val & ((1 << 7) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn oddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oddfrm(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn chdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chdis(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn chena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chena(mut self, val: u32) -> Fs_hcchar1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x120) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcchar1 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcchar1 {
        super::Fs_hcchar1 {
            raw: unsafe { *((0x50000400 + 0x120) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcchar1) {
       unsafe { *((0x50000400 + 0x120) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcchar2 {
   raw: u32,
}

impl Fs_hcchar2 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 11)) | ((val & ((1 << 4) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn epdir_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdir(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lsdev_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsdev(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 22)) | ((val & ((1 << 7) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn oddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oddfrm(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn chdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chdis(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn chena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chena(mut self, val: u32) -> Fs_hcchar2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x140) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcchar2 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcchar2 {
        super::Fs_hcchar2 {
            raw: unsafe { *((0x50000400 + 0x140) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcchar2) {
       unsafe { *((0x50000400 + 0x140) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcchar3 {
   raw: u32,
}

impl Fs_hcchar3 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 11)) | ((val & ((1 << 4) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn epdir_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdir(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lsdev_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsdev(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 22)) | ((val & ((1 << 7) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn oddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oddfrm(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn chdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chdis(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn chena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chena(mut self, val: u32) -> Fs_hcchar3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x160) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcchar3 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcchar3 {
        super::Fs_hcchar3 {
            raw: unsafe { *((0x50000400 + 0x160) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcchar3) {
       unsafe { *((0x50000400 + 0x160) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcchar4 {
   raw: u32,
}

impl Fs_hcchar4 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 11)) | ((val & ((1 << 4) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn epdir_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdir(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lsdev_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsdev(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 22)) | ((val & ((1 << 7) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn oddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oddfrm(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn chdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chdis(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn chena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chena(mut self, val: u32) -> Fs_hcchar4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x180) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcchar4 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcchar4 {
        super::Fs_hcchar4 {
            raw: unsafe { *((0x50000400 + 0x180) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcchar4) {
       unsafe { *((0x50000400 + 0x180) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcchar5 {
   raw: u32,
}

impl Fs_hcchar5 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 11)) | ((val & ((1 << 4) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn epdir_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdir(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lsdev_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsdev(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 22)) | ((val & ((1 << 7) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn oddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oddfrm(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn chdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chdis(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn chena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chena(mut self, val: u32) -> Fs_hcchar5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1A0) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcchar5 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcchar5 {
        super::Fs_hcchar5 {
            raw: unsafe { *((0x50000400 + 0x1A0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcchar5) {
       unsafe { *((0x50000400 + 0x1A0) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcchar6 {
   raw: u32,
}

impl Fs_hcchar6 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 11)) | ((val & ((1 << 4) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn epdir_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdir(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lsdev_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsdev(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 22)) | ((val & ((1 << 7) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn oddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oddfrm(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn chdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chdis(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn chena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chena(mut self, val: u32) -> Fs_hcchar6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1C0) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcchar6 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcchar6 {
        super::Fs_hcchar6 {
            raw: unsafe { *((0x50000400 + 0x1C0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcchar6) {
       unsafe { *((0x50000400 + 0x1C0) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcchar7 {
   raw: u32,
}

impl Fs_hcchar7 {
    #[inline(always)]
    pub fn mpsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn mpsiz(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 11)) | ((val & ((1 << 4) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn epdir_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epdir(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lsdev_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsdev(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn eptyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn eptyp(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mcnt_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mcnt(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dad_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn dad(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 22)) | ((val & ((1 << 7) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn oddfrm_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oddfrm(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn chdis_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chdis(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn chena_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chena(mut self, val: u32) -> Fs_hcchar7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1E0) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcchar7 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcchar7 {
        super::Fs_hcchar7 {
            raw: unsafe { *((0x50000400 + 0x1E0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcchar7) {
       unsafe { *((0x50000400 + 0x1E0) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcint0 {
   raw: u32,
}

impl Fs_hcint0 {
    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chh_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chh(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nak_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nak(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txerr_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerr(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberr(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmor_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmor(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterr(mut self, val: u32) -> Fs_hcint0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x108) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcint0 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcint0 {
        super::Fs_hcint0 {
            raw: unsafe { *((0x50000400 + 0x108) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcint0) {
       unsafe { *((0x50000400 + 0x108) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcint1 {
   raw: u32,
}

impl Fs_hcint1 {
    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chh_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chh(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nak_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nak(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txerr_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerr(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberr(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmor_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmor(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterr(mut self, val: u32) -> Fs_hcint1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x128) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcint1 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcint1 {
        super::Fs_hcint1 {
            raw: unsafe { *((0x50000400 + 0x128) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcint1) {
       unsafe { *((0x50000400 + 0x128) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcint2 {
   raw: u32,
}

impl Fs_hcint2 {
    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chh_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chh(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nak_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nak(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txerr_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerr(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberr(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmor_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmor(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterr(mut self, val: u32) -> Fs_hcint2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x148) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcint2 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcint2 {
        super::Fs_hcint2 {
            raw: unsafe { *((0x50000400 + 0x148) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcint2) {
       unsafe { *((0x50000400 + 0x148) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcint3 {
   raw: u32,
}

impl Fs_hcint3 {
    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chh_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chh(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nak_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nak(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txerr_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerr(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberr(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmor_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmor(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterr(mut self, val: u32) -> Fs_hcint3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x168) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcint3 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcint3 {
        super::Fs_hcint3 {
            raw: unsafe { *((0x50000400 + 0x168) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcint3) {
       unsafe { *((0x50000400 + 0x168) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcint4 {
   raw: u32,
}

impl Fs_hcint4 {
    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chh_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chh(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nak_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nak(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txerr_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerr(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberr(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmor_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmor(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterr(mut self, val: u32) -> Fs_hcint4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x188) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcint4 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcint4 {
        super::Fs_hcint4 {
            raw: unsafe { *((0x50000400 + 0x188) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcint4) {
       unsafe { *((0x50000400 + 0x188) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcint5 {
   raw: u32,
}

impl Fs_hcint5 {
    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chh_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chh(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nak_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nak(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txerr_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerr(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberr(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmor_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmor(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterr(mut self, val: u32) -> Fs_hcint5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1A8) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcint5 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcint5 {
        super::Fs_hcint5 {
            raw: unsafe { *((0x50000400 + 0x1A8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcint5) {
       unsafe { *((0x50000400 + 0x1A8) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcint6 {
   raw: u32,
}

impl Fs_hcint6 {
    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chh_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chh(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nak_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nak(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txerr_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerr(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberr(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmor_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmor(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterr(mut self, val: u32) -> Fs_hcint6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1C8) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcint6 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcint6 {
        super::Fs_hcint6 {
            raw: unsafe { *((0x50000400 + 0x1C8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcint6) {
       unsafe { *((0x50000400 + 0x1C8) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcint7 {
   raw: u32,
}

impl Fs_hcint7 {
    #[inline(always)]
    pub fn xfrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrc(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chh_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chh(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stall_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stall(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nak_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nak(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txerr_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerr(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberr(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmor_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmor(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterr(mut self, val: u32) -> Fs_hcint7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1E8) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcint7 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcint7 {
        super::Fs_hcint7 {
            raw: unsafe { *((0x50000400 + 0x1E8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcint7) {
       unsafe { *((0x50000400 + 0x1E8) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcintmsk0 {
   raw: u32,
}

impl Fs_hcintmsk0 {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chhm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chhm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stallm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stallm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nakm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nakm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ackm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ackm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nyet_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nyet(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn txerrm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerrm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberrm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberrm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmorm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmorm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterrm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterrm(mut self, val: u32) -> Fs_hcintmsk0 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x10C) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcintmsk0 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcintmsk0 {
        super::Fs_hcintmsk0 {
            raw: unsafe { *((0x50000400 + 0x10C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcintmsk0) {
       unsafe { *((0x50000400 + 0x10C) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcintmsk1 {
   raw: u32,
}

impl Fs_hcintmsk1 {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chhm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chhm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stallm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stallm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nakm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nakm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ackm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ackm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nyet_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nyet(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn txerrm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerrm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberrm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberrm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmorm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmorm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterrm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterrm(mut self, val: u32) -> Fs_hcintmsk1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x12C) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcintmsk1 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcintmsk1 {
        super::Fs_hcintmsk1 {
            raw: unsafe { *((0x50000400 + 0x12C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcintmsk1) {
       unsafe { *((0x50000400 + 0x12C) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcintmsk2 {
   raw: u32,
}

impl Fs_hcintmsk2 {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chhm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chhm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stallm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stallm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nakm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nakm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ackm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ackm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nyet_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nyet(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn txerrm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerrm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberrm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberrm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmorm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmorm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterrm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterrm(mut self, val: u32) -> Fs_hcintmsk2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x14C) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcintmsk2 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcintmsk2 {
        super::Fs_hcintmsk2 {
            raw: unsafe { *((0x50000400 + 0x14C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcintmsk2) {
       unsafe { *((0x50000400 + 0x14C) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcintmsk3 {
   raw: u32,
}

impl Fs_hcintmsk3 {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chhm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chhm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stallm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stallm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nakm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nakm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ackm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ackm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nyet_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nyet(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn txerrm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerrm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberrm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberrm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmorm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmorm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterrm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterrm(mut self, val: u32) -> Fs_hcintmsk3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x16C) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcintmsk3 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcintmsk3 {
        super::Fs_hcintmsk3 {
            raw: unsafe { *((0x50000400 + 0x16C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcintmsk3) {
       unsafe { *((0x50000400 + 0x16C) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcintmsk4 {
   raw: u32,
}

impl Fs_hcintmsk4 {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chhm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chhm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stallm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stallm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nakm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nakm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ackm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ackm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nyet_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nyet(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn txerrm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerrm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberrm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberrm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmorm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmorm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterrm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterrm(mut self, val: u32) -> Fs_hcintmsk4 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x18C) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcintmsk4 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcintmsk4 {
        super::Fs_hcintmsk4 {
            raw: unsafe { *((0x50000400 + 0x18C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcintmsk4) {
       unsafe { *((0x50000400 + 0x18C) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcintmsk5 {
   raw: u32,
}

impl Fs_hcintmsk5 {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chhm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chhm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stallm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stallm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nakm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nakm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ackm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ackm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nyet_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nyet(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn txerrm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerrm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberrm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberrm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmorm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmorm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterrm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterrm(mut self, val: u32) -> Fs_hcintmsk5 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1AC) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcintmsk5 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcintmsk5 {
        super::Fs_hcintmsk5 {
            raw: unsafe { *((0x50000400 + 0x1AC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcintmsk5) {
       unsafe { *((0x50000400 + 0x1AC) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcintmsk6 {
   raw: u32,
}

impl Fs_hcintmsk6 {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chhm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chhm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stallm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stallm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nakm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nakm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ackm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ackm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nyet_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nyet(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn txerrm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerrm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberrm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberrm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmorm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmorm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterrm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterrm(mut self, val: u32) -> Fs_hcintmsk6 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1CC) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcintmsk6 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcintmsk6 {
        super::Fs_hcintmsk6 {
            raw: unsafe { *((0x50000400 + 0x1CC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcintmsk6) {
       unsafe { *((0x50000400 + 0x1CC) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hcintmsk7 {
   raw: u32,
}

impl Fs_hcintmsk7 {
    #[inline(always)]
    pub fn xfrcm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xfrcm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn chhm_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chhm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn stallm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stallm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nakm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nakm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ackm_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ackm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nyet_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nyet(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn txerrm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txerrm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bberrm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bberrm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn frmorm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn frmorm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dterrm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dterrm(mut self, val: u32) -> Fs_hcintmsk7 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1EC) as *mut u32) = self.raw; }
    }
}

pub mod fs_hcintmsk7 {
    #[inline(always)]
    pub fn read() -> super::Fs_hcintmsk7 {
        super::Fs_hcintmsk7 {
            raw: unsafe { *((0x50000400 + 0x1EC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hcintmsk7) {
       unsafe { *((0x50000400 + 0x1EC) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hctsiz0 {
   raw: u32,
}

impl Fs_hctsiz0 {
    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Fs_hctsiz0 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Fs_hctsiz0 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_hctsiz0 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x110) as *mut u32) = self.raw; }
    }
}

pub mod fs_hctsiz0 {
    #[inline(always)]
    pub fn read() -> super::Fs_hctsiz0 {
        super::Fs_hctsiz0 {
            raw: unsafe { *((0x50000400 + 0x110) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hctsiz0) {
       unsafe { *((0x50000400 + 0x110) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hctsiz1 {
   raw: u32,
}

impl Fs_hctsiz1 {
    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Fs_hctsiz1 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Fs_hctsiz1 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_hctsiz1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x130) as *mut u32) = self.raw; }
    }
}

pub mod fs_hctsiz1 {
    #[inline(always)]
    pub fn read() -> super::Fs_hctsiz1 {
        super::Fs_hctsiz1 {
            raw: unsafe { *((0x50000400 + 0x130) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hctsiz1) {
       unsafe { *((0x50000400 + 0x130) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hctsiz2 {
   raw: u32,
}

impl Fs_hctsiz2 {
    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Fs_hctsiz2 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Fs_hctsiz2 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_hctsiz2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x150) as *mut u32) = self.raw; }
    }
}

pub mod fs_hctsiz2 {
    #[inline(always)]
    pub fn read() -> super::Fs_hctsiz2 {
        super::Fs_hctsiz2 {
            raw: unsafe { *((0x50000400 + 0x150) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hctsiz2) {
       unsafe { *((0x50000400 + 0x150) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hctsiz3 {
   raw: u32,
}

impl Fs_hctsiz3 {
    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Fs_hctsiz3 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Fs_hctsiz3 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_hctsiz3 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x170) as *mut u32) = self.raw; }
    }
}

pub mod fs_hctsiz3 {
    #[inline(always)]
    pub fn read() -> super::Fs_hctsiz3 {
        super::Fs_hctsiz3 {
            raw: unsafe { *((0x50000400 + 0x170) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hctsiz3) {
       unsafe { *((0x50000400 + 0x170) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hctsiz4 {
   raw: u32,
}

impl Fs_hctsiz4 {
    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Fs_hctsiz4 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Fs_hctsiz4 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_hctsiz4 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x190) as *mut u32) = self.raw; }
    }
}

pub mod fs_hctsiz4 {
    #[inline(always)]
    pub fn read() -> super::Fs_hctsiz4 {
        super::Fs_hctsiz4 {
            raw: unsafe { *((0x50000400 + 0x190) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hctsiz4) {
       unsafe { *((0x50000400 + 0x190) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hctsiz5 {
   raw: u32,
}

impl Fs_hctsiz5 {
    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Fs_hctsiz5 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Fs_hctsiz5 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_hctsiz5 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1B0) as *mut u32) = self.raw; }
    }
}

pub mod fs_hctsiz5 {
    #[inline(always)]
    pub fn read() -> super::Fs_hctsiz5 {
        super::Fs_hctsiz5 {
            raw: unsafe { *((0x50000400 + 0x1B0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hctsiz5) {
       unsafe { *((0x50000400 + 0x1B0) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hctsiz6 {
   raw: u32,
}

impl Fs_hctsiz6 {
    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Fs_hctsiz6 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Fs_hctsiz6 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_hctsiz6 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1D0) as *mut u32) = self.raw; }
    }
}

pub mod fs_hctsiz6 {
    #[inline(always)]
    pub fn read() -> super::Fs_hctsiz6 {
        super::Fs_hctsiz6 {
            raw: unsafe { *((0x50000400 + 0x1D0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hctsiz6) {
       unsafe { *((0x50000400 + 0x1D0) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hctsiz7 {
   raw: u32,
}

impl Fs_hctsiz7 {
    #[inline(always)]
    pub fn xfrsiz_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 19) - 1)
    }

    #[inline(always)]
    pub fn xfrsiz(mut self, val: u32) -> Fs_hctsiz7 {
        self.raw = (self.raw & !(((1 << 19) - 1) << 0)) | ((val & ((1 << 19) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pktcnt_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn pktcnt(mut self, val: u32) -> Fs_hctsiz7 {
        self.raw = (self.raw & !(((1 << 10) - 1) << 19)) | ((val & ((1 << 10) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_hctsiz7 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 29)) | ((val & ((1 << 2) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000400 + 0x1F0) as *mut u32) = self.raw; }
    }
}

pub mod fs_hctsiz7 {
    #[inline(always)]
    pub fn read() -> super::Fs_hctsiz7 {
        super::Fs_hctsiz7 {
            raw: unsafe { *((0x50000400 + 0x1F0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hctsiz7) {
       unsafe { *((0x50000400 + 0x1F0) as *mut u32) = val.raw; }
    }
}

