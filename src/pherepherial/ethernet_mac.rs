pub struct Maccr {
   raw: u32,
}

impl Maccr {
    #[inline(always)]
    pub fn re_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn re(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn te_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn te(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dc_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dc(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn bl_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn bl(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 5)) | ((val & ((1 << 2) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn apcs_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn apcs(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn rd_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rd(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn ipco_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ipco(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn dm_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dm(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn lm_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lm(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn rod_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rod(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fes_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fes(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn csd_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn csd(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ifg_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ifg(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 17)) | ((val & ((1 << 3) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn jd_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn jd(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn wd_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wd(mut self, val: u32) -> Maccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod maccr {
    #[inline(always)]
    pub fn read() -> super::Maccr {
        super::Maccr {
            raw: unsafe { *((0x40028000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maccr) {
       unsafe { *((0x40028000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Macffr {
   raw: u32,
}

impl Macffr {
    #[inline(always)]
    pub fn pm_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pm(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn hu_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hu(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn hm_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hm(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn daif_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn daif(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn pam_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pam(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn bfd_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bfd(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn pcf_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pcf(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 6)) | ((val & ((1 << 2) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn saif_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn saif(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn saf_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn saf(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn hpf_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hpf(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn ra_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ra(mut self, val: u32) -> Macffr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod macffr {
    #[inline(always)]
    pub fn read() -> super::Macffr {
        super::Macffr {
            raw: unsafe { *((0x40028000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macffr) {
       unsafe { *((0x40028000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Machthr {
   raw: u32,
}

impl Machthr {
    #[inline(always)]
    pub fn hth_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn hth(mut self, val: u32) -> Machthr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod machthr {
    #[inline(always)]
    pub fn read() -> super::Machthr {
        super::Machthr {
            raw: unsafe { *((0x40028000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Machthr) {
       unsafe { *((0x40028000 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Machtlr {
   raw: u32,
}

impl Machtlr {
    #[inline(always)]
    pub fn htl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn htl(mut self, val: u32) -> Machtlr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod machtlr {
    #[inline(always)]
    pub fn read() -> super::Machtlr {
        super::Machtlr {
            raw: unsafe { *((0x40028000 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Machtlr) {
       unsafe { *((0x40028000 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Macmiiar {
   raw: u32,
}

impl Macmiiar {
    #[inline(always)]
    pub fn mb_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mb(mut self, val: u32) -> Macmiiar {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn mw_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mw(mut self, val: u32) -> Macmiiar {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn cr_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn cr(mut self, val: u32) -> Macmiiar {
        self.raw = (self.raw & !(((1 << 3) - 1) << 2)) | ((val & ((1 << 3) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn mr_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn mr(mut self, val: u32) -> Macmiiar {
        self.raw = (self.raw & !(((1 << 5) - 1) << 6)) | ((val & ((1 << 5) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn pa_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn pa(mut self, val: u32) -> Macmiiar {
        self.raw = (self.raw & !(((1 << 5) - 1) << 11)) | ((val & ((1 << 5) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod macmiiar {
    #[inline(always)]
    pub fn read() -> super::Macmiiar {
        super::Macmiiar {
            raw: unsafe { *((0x40028000 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macmiiar) {
       unsafe { *((0x40028000 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Macmiidr {
   raw: u32,
}

impl Macmiidr {
    #[inline(always)]
    pub fn md_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn md(mut self, val: u32) -> Macmiidr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod macmiidr {
    #[inline(always)]
    pub fn read() -> super::Macmiidr {
        super::Macmiidr {
            raw: unsafe { *((0x40028000 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macmiidr) {
       unsafe { *((0x40028000 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Macfcr {
   raw: u32,
}

impl Macfcr {
    #[inline(always)]
    pub fn fcb_bpa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fcb_bpa(mut self, val: u32) -> Macfcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tfce_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tfce(mut self, val: u32) -> Macfcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn rfce_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rfce(mut self, val: u32) -> Macfcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn upfd_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn upfd(mut self, val: u32) -> Macfcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn plt_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn plt(mut self, val: u32) -> Macfcr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn zqpd_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn zqpd(mut self, val: u32) -> Macfcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn pt_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn pt(mut self, val: u32) -> Macfcr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod macfcr {
    #[inline(always)]
    pub fn read() -> super::Macfcr {
        super::Macfcr {
            raw: unsafe { *((0x40028000 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macfcr) {
       unsafe { *((0x40028000 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Macvlantr {
   raw: u32,
}

impl Macvlantr {
    #[inline(always)]
    pub fn vlanti_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn vlanti(mut self, val: u32) -> Macvlantr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn vlantc_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn vlantc(mut self, val: u32) -> Macvlantr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x1C) as *mut u32) = self.raw; }
    }
}

pub mod macvlantr {
    #[inline(always)]
    pub fn read() -> super::Macvlantr {
        super::Macvlantr {
            raw: unsafe { *((0x40028000 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macvlantr) {
       unsafe { *((0x40028000 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Macrwuffr {
   raw: u32,
}

pub mod macrwuffr {
    #[inline(always)]
    pub fn read() -> super::Macrwuffr {
        super::Macrwuffr {
            raw: unsafe { *((0x40028000 + 0x28) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macrwuffr) {
       unsafe { *((0x40028000 + 0x28) as *mut u32) = val.raw; }
    }
}

pub struct Macpmtcsr {
   raw: u32,
}

impl Macpmtcsr {
    #[inline(always)]
    pub fn pd_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pd(mut self, val: u32) -> Macpmtcsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn mpe_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mpe(mut self, val: u32) -> Macpmtcsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn wfe_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wfe(mut self, val: u32) -> Macpmtcsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn mpr_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mpr(mut self, val: u32) -> Macpmtcsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn wfr_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wfr(mut self, val: u32) -> Macpmtcsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn gu_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gu(mut self, val: u32) -> Macpmtcsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn wffrpr_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wffrpr(mut self, val: u32) -> Macpmtcsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x2C) as *mut u32) = self.raw; }
    }
}

pub mod macpmtcsr {
    #[inline(always)]
    pub fn read() -> super::Macpmtcsr {
        super::Macpmtcsr {
            raw: unsafe { *((0x40028000 + 0x2C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macpmtcsr) {
       unsafe { *((0x40028000 + 0x2C) as *mut u32) = val.raw; }
    }
}

pub struct Macsr {
   raw: u32,
}

impl Macsr {
    #[inline(always)]
    pub fn pmts_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pmts(mut self, val: u32) -> Macsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn mmcs_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmcs(mut self, val: u32) -> Macsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn mmcrs_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmcrs(mut self, val: u32) -> Macsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn mmcts_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmcts(mut self, val: u32) -> Macsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn tsts_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsts(mut self, val: u32) -> Macsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x38) as *mut u32) = self.raw; }
    }
}

pub mod macsr {
    #[inline(always)]
    pub fn read() -> super::Macsr {
        super::Macsr {
            raw: unsafe { *((0x40028000 + 0x38) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macsr) {
       unsafe { *((0x40028000 + 0x38) as *mut u32) = val.raw; }
    }
}

pub struct Macimr {
   raw: u32,
}

impl Macimr {
    #[inline(always)]
    pub fn pmtim_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pmtim(mut self, val: u32) -> Macimr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn tstim_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tstim(mut self, val: u32) -> Macimr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x3C) as *mut u32) = self.raw; }
    }
}

pub mod macimr {
    #[inline(always)]
    pub fn read() -> super::Macimr {
        super::Macimr {
            raw: unsafe { *((0x40028000 + 0x3C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Macimr) {
       unsafe { *((0x40028000 + 0x3C) as *mut u32) = val.raw; }
    }
}

pub struct Maca0hr {
   raw: u32,
}

impl Maca0hr {
    #[inline(always)]
    pub fn maca0h_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn maca0h(mut self, val: u32) -> Maca0hr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn mo_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mo(mut self, val: u32) -> Maca0hr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x40) as *mut u32) = self.raw; }
    }
}

pub mod maca0hr {
    #[inline(always)]
    pub fn read() -> super::Maca0hr {
        super::Maca0hr {
            raw: unsafe { *((0x40028000 + 0x40) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maca0hr) {
       unsafe { *((0x40028000 + 0x40) as *mut u32) = val.raw; }
    }
}

pub struct Maca0lr {
   raw: u32,
}

impl Maca0lr {
    #[inline(always)]
    pub fn maca0l_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn maca0l(mut self, val: u32) -> Maca0lr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x44) as *mut u32) = self.raw; }
    }
}

pub mod maca0lr {
    #[inline(always)]
    pub fn read() -> super::Maca0lr {
        super::Maca0lr {
            raw: unsafe { *((0x40028000 + 0x44) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maca0lr) {
       unsafe { *((0x40028000 + 0x44) as *mut u32) = val.raw; }
    }
}

pub struct Maca1hr {
   raw: u32,
}

impl Maca1hr {
    #[inline(always)]
    pub fn maca1h_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn maca1h(mut self, val: u32) -> Maca1hr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn mbc_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn mbc(mut self, val: u32) -> Maca1hr {
        self.raw = (self.raw & !(((1 << 6) - 1) << 24)) | ((val & ((1 << 6) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn sa_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sa(mut self, val: u32) -> Maca1hr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn ae_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ae(mut self, val: u32) -> Maca1hr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x48) as *mut u32) = self.raw; }
    }
}

pub mod maca1hr {
    #[inline(always)]
    pub fn read() -> super::Maca1hr {
        super::Maca1hr {
            raw: unsafe { *((0x40028000 + 0x48) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maca1hr) {
       unsafe { *((0x40028000 + 0x48) as *mut u32) = val.raw; }
    }
}

pub struct Maca1lr {
   raw: u32,
}

impl Maca1lr {
    #[inline(always)]
    pub fn maca1l_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn maca1l(mut self, val: u32) -> Maca1lr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x4C) as *mut u32) = self.raw; }
    }
}

pub mod maca1lr {
    #[inline(always)]
    pub fn read() -> super::Maca1lr {
        super::Maca1lr {
            raw: unsafe { *((0x40028000 + 0x4C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maca1lr) {
       unsafe { *((0x40028000 + 0x4C) as *mut u32) = val.raw; }
    }
}

pub struct Maca2hr {
   raw: u32,
}

impl Maca2hr {
    #[inline(always)]
    pub fn eth_maca2hr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn eth_maca2hr(mut self, val: u32) -> Maca2hr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn mbc_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn mbc(mut self, val: u32) -> Maca2hr {
        self.raw = (self.raw & !(((1 << 6) - 1) << 24)) | ((val & ((1 << 6) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn sa_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sa(mut self, val: u32) -> Maca2hr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn ae_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ae(mut self, val: u32) -> Maca2hr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x50) as *mut u32) = self.raw; }
    }
}

pub mod maca2hr {
    #[inline(always)]
    pub fn read() -> super::Maca2hr {
        super::Maca2hr {
            raw: unsafe { *((0x40028000 + 0x50) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maca2hr) {
       unsafe { *((0x40028000 + 0x50) as *mut u32) = val.raw; }
    }
}

pub struct Maca2lr {
   raw: u32,
}

impl Maca2lr {
    #[inline(always)]
    pub fn maca2l_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 31) - 1)
    }

    #[inline(always)]
    pub fn maca2l(mut self, val: u32) -> Maca2lr {
        self.raw = (self.raw & !(((1 << 31) - 1) << 0)) | ((val & ((1 << 31) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x54) as *mut u32) = self.raw; }
    }
}

pub mod maca2lr {
    #[inline(always)]
    pub fn read() -> super::Maca2lr {
        super::Maca2lr {
            raw: unsafe { *((0x40028000 + 0x54) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maca2lr) {
       unsafe { *((0x40028000 + 0x54) as *mut u32) = val.raw; }
    }
}

pub struct Maca3hr {
   raw: u32,
}

impl Maca3hr {
    #[inline(always)]
    pub fn maca3h_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn maca3h(mut self, val: u32) -> Maca3hr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn mbc_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn mbc(mut self, val: u32) -> Maca3hr {
        self.raw = (self.raw & !(((1 << 6) - 1) << 24)) | ((val & ((1 << 6) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn sa_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sa(mut self, val: u32) -> Maca3hr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn ae_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ae(mut self, val: u32) -> Maca3hr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x58) as *mut u32) = self.raw; }
    }
}

pub mod maca3hr {
    #[inline(always)]
    pub fn read() -> super::Maca3hr {
        super::Maca3hr {
            raw: unsafe { *((0x40028000 + 0x58) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maca3hr) {
       unsafe { *((0x40028000 + 0x58) as *mut u32) = val.raw; }
    }
}

pub struct Maca3lr {
   raw: u32,
}

impl Maca3lr {
    #[inline(always)]
    pub fn mbca3l_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn mbca3l(mut self, val: u32) -> Maca3lr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028000 + 0x5C) as *mut u32) = self.raw; }
    }
}

pub mod maca3lr {
    #[inline(always)]
    pub fn read() -> super::Maca3lr {
        super::Maca3lr {
            raw: unsafe { *((0x40028000 + 0x5C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Maca3lr) {
       unsafe { *((0x40028000 + 0x5C) as *mut u32) = val.raw; }
    }
}

