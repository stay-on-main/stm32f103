pub struct Fs_gotgctl {
   raw: u32,
}

impl Fs_gotgctl {
    #[inline(always)]
    pub fn srqscs_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn srqscs(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn srq_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn srq(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn hngscs_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hngscs(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn hnprq_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hnprq(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn hshnpen_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hshnpen(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn dhnpen_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dhnpen(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn cidsts_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cidsts(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn dbct_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbct(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn asvld_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn asvld(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn bsvld_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bsvld(mut self, val: u32) -> Fs_gotgctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gotgctl {
    #[inline(always)]
    pub fn read() -> super::Fs_gotgctl {
        super::Fs_gotgctl {
            raw: unsafe { *((0x50000000u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gotgctl) {
       unsafe { *((0x50000000u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gotgint {
   raw: u32,
}

impl Fs_gotgint {
    #[inline(always)]
    pub fn sedet_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sedet(mut self, val: u32) -> Fs_gotgint {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn srsschg_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn srsschg(mut self, val: u32) -> Fs_gotgint {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn hnsschg_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hnsschg(mut self, val: u32) -> Fs_gotgint {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn hngdet_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hngdet(mut self, val: u32) -> Fs_gotgint {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn adtochg_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adtochg(mut self, val: u32) -> Fs_gotgint {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn dbcdne_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbcdne(mut self, val: u32) -> Fs_gotgint {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gotgint {
    #[inline(always)]
    pub fn read() -> super::Fs_gotgint {
        super::Fs_gotgint {
            raw: unsafe { *((0x50000000u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gotgint) {
       unsafe { *((0x50000000u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gahbcfg {
   raw: u32,
}

impl Fs_gahbcfg {
    #[inline(always)]
    pub fn gint_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gint(mut self, val: u32) -> Fs_gahbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn txfelvl_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfelvl(mut self, val: u32) -> Fs_gahbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ptxfelvl_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ptxfelvl(mut self, val: u32) -> Fs_gahbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gahbcfg {
    #[inline(always)]
    pub fn read() -> super::Fs_gahbcfg {
        super::Fs_gahbcfg {
            raw: unsafe { *((0x50000000u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gahbcfg) {
       unsafe { *((0x50000000u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gusbcfg {
   raw: u32,
}

impl Fs_gusbcfg {
    #[inline(always)]
    pub fn tocal_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn tocal(mut self, val: u32) -> Fs_gusbcfg {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn physel_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn physel(mut self, val: u32) -> Fs_gusbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn srpcap_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn srpcap(mut self, val: u32) -> Fs_gusbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn hnpcap_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hnpcap(mut self, val: u32) -> Fs_gusbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn trdt_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn trdt(mut self, val: u32) -> Fs_gusbcfg {
        self.raw = (self.raw & !(((1 << 4) - 1) << 10)) | ((val & ((1 << 4) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fhmod_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fhmod(mut self, val: u32) -> Fs_gusbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fdmod_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fdmod(mut self, val: u32) -> Fs_gusbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn ctxpkt_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctxpkt(mut self, val: u32) -> Fs_gusbcfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gusbcfg {
    #[inline(always)]
    pub fn read() -> super::Fs_gusbcfg {
        super::Fs_gusbcfg {
            raw: unsafe { *((0x50000000u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gusbcfg) {
       unsafe { *((0x50000000u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_grstctl {
   raw: u32,
}

impl Fs_grstctl {
    #[inline(always)]
    pub fn csrst_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn csrst(mut self, val: u32) -> Fs_grstctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn hsrst_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hsrst(mut self, val: u32) -> Fs_grstctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fcrst_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fcrst(mut self, val: u32) -> Fs_grstctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rxfflsh_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxfflsh(mut self, val: u32) -> Fs_grstctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn txfflsh_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfflsh(mut self, val: u32) -> Fs_grstctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn txfnum_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn txfnum(mut self, val: u32) -> Fs_grstctl {
        self.raw = (self.raw & !(((1 << 5) - 1) << 6)) | ((val & ((1 << 5) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ahbidl_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ahbidl(mut self, val: u32) -> Fs_grstctl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_grstctl {
    #[inline(always)]
    pub fn read() -> super::Fs_grstctl {
        super::Fs_grstctl {
            raw: unsafe { *((0x50000000u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_grstctl) {
       unsafe { *((0x50000000u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gintsts {
   raw: u32,
}

impl Fs_gintsts {
    #[inline(always)]
    pub fn cmod_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmod(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn mmis_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmis(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn otgint_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn otgint(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn sof_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sof(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn rxflvl_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxflvl(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn nptxfe_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nptxfe(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn ginakeff_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ginakeff(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn goutnakeff_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn goutnakeff(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn esusp_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn esusp(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn usbsusp_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbsusp(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn usbrst_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbrst(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn enumdne_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn enumdne(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn isoodrp_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn isoodrp(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn eopf_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eopf(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn iepint_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iepint(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn oepint_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oepint(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn iisoixfr_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iisoixfr(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn ipxfr_incompisoout_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ipxfr_incompisoout(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn hprtint_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hprtint(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn hcint_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hcint(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn ptxfe_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ptxfe(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn cidschg_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cidschg(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn discint_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn discint(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn srqint_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn srqint(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn wkupint_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wkupint(mut self, val: u32) -> Fs_gintsts {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gintsts {
    #[inline(always)]
    pub fn read() -> super::Fs_gintsts {
        super::Fs_gintsts {
            raw: unsafe { *((0x50000000u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gintsts) {
       unsafe { *((0x50000000u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gintmsk {
   raw: u32,
}

impl Fs_gintmsk {
    #[inline(always)]
    pub fn mmism_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmism(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn otgint_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn otgint(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn sofm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sofm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn rxflvlm_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxflvlm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn nptxfem_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nptxfem(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn ginakeffm_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ginakeffm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn gonakeffm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gonakeffm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn esuspm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn esuspm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn usbsuspm_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbsuspm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn usbrst_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbrst(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn enumdnem_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn enumdnem(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn isoodrpm_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn isoodrpm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn eopfm_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eopfm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn epmism_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epmism(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn iepint_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iepint(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn oepint_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oepint(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn iisoixfrm_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iisoixfrm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn ipxfrm_iisooxfrm_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ipxfrm_iisooxfrm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn prtim_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn prtim(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn hcim_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hcim(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn ptxfem_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ptxfem(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn cidschgm_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cidschgm(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn discint_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn discint(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn srqim_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn srqim(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn wuim_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wuim(mut self, val: u32) -> Fs_gintmsk {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gintmsk {
    #[inline(always)]
    pub fn read() -> super::Fs_gintmsk {
        super::Fs_gintmsk {
            raw: unsafe { *((0x50000000u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gintmsk) {
       unsafe { *((0x50000000u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_grxstsr_device {
   raw: u32,
}

impl Fs_grxstsr_device {
    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_grxstsr_device {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn bcnt_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn bcnt(mut self, val: u32) -> Fs_grxstsr_device {
        self.raw = (self.raw & !(((1 << 11) - 1) << 4)) | ((val & ((1 << 11) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_grxstsr_device {
        self.raw = (self.raw & !(((1 << 2) - 1) << 15)) | ((val & ((1 << 2) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn pktsts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn pktsts(mut self, val: u32) -> Fs_grxstsr_device {
        self.raw = (self.raw & !(((1 << 4) - 1) << 17)) | ((val & ((1 << 4) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn frmnum_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn frmnum(mut self, val: u32) -> Fs_grxstsr_device {
        self.raw = (self.raw & !(((1 << 4) - 1) << 21)) | ((val & ((1 << 4) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod fs_grxstsr_device {
    #[inline(always)]
    pub fn read() -> super::Fs_grxstsr_device {
        super::Fs_grxstsr_device {
            raw: unsafe { *((0x50000000u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_grxstsr_device) {
       unsafe { *((0x50000000u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_grxstsr_host {
   raw: u32,
}

impl Fs_grxstsr_host {
    #[inline(always)]
    pub fn epnum_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn epnum(mut self, val: u32) -> Fs_grxstsr_host {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn bcnt_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn bcnt(mut self, val: u32) -> Fs_grxstsr_host {
        self.raw = (self.raw & !(((1 << 11) - 1) << 4)) | ((val & ((1 << 11) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dpid_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn dpid(mut self, val: u32) -> Fs_grxstsr_host {
        self.raw = (self.raw & !(((1 << 2) - 1) << 15)) | ((val & ((1 << 2) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn pktsts_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn pktsts(mut self, val: u32) -> Fs_grxstsr_host {
        self.raw = (self.raw & !(((1 << 4) - 1) << 17)) | ((val & ((1 << 4) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn frmnum_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn frmnum(mut self, val: u32) -> Fs_grxstsr_host {
        self.raw = (self.raw & !(((1 << 4) - 1) << 21)) | ((val & ((1 << 4) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod fs_grxstsr_host {
    #[inline(always)]
    pub fn read() -> super::Fs_grxstsr_host {
        super::Fs_grxstsr_host {
            raw: unsafe { *((0x50000000u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_grxstsr_host) {
       unsafe { *((0x50000000u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_grxfsiz {
   raw: u32,
}

impl Fs_grxfsiz {
    #[inline(always)]
    pub fn rxfd_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn rxfd(mut self, val: u32) -> Fs_grxfsiz {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x24u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_grxfsiz {
    #[inline(always)]
    pub fn read() -> super::Fs_grxfsiz {
        super::Fs_grxfsiz {
            raw: unsafe { *((0x50000000u32 + 0x24u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_grxfsiz) {
       unsafe { *((0x50000000u32 + 0x24u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gnptxfsiz_device {
   raw: u32,
}

impl Fs_gnptxfsiz_device {
    #[inline(always)]
    pub fn tx0fsa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn tx0fsa(mut self, val: u32) -> Fs_gnptxfsiz_device {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tx0fd_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn tx0fd(mut self, val: u32) -> Fs_gnptxfsiz_device {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x28u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gnptxfsiz_device {
    #[inline(always)]
    pub fn read() -> super::Fs_gnptxfsiz_device {
        super::Fs_gnptxfsiz_device {
            raw: unsafe { *((0x50000000u32 + 0x28u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gnptxfsiz_device) {
       unsafe { *((0x50000000u32 + 0x28u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gnptxfsiz_host {
   raw: u32,
}

impl Fs_gnptxfsiz_host {
    #[inline(always)]
    pub fn nptxfsa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn nptxfsa(mut self, val: u32) -> Fs_gnptxfsiz_host {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn nptxfd_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn nptxfd(mut self, val: u32) -> Fs_gnptxfsiz_host {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x28u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gnptxfsiz_host {
    #[inline(always)]
    pub fn read() -> super::Fs_gnptxfsiz_host {
        super::Fs_gnptxfsiz_host {
            raw: unsafe { *((0x50000000u32 + 0x28u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gnptxfsiz_host) {
       unsafe { *((0x50000000u32 + 0x28u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gnptxsts {
   raw: u32,
}

impl Fs_gnptxsts {
    #[inline(always)]
    pub fn nptxfsav_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn nptxfsav(mut self, val: u32) -> Fs_gnptxsts {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn nptqxsav_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn nptqxsav(mut self, val: u32) -> Fs_gnptxsts {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn nptxqtop_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn nptxqtop(mut self, val: u32) -> Fs_gnptxsts {
        self.raw = (self.raw & !(((1 << 7) - 1) << 24)) | ((val & ((1 << 7) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x2Cu32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gnptxsts {
    #[inline(always)]
    pub fn read() -> super::Fs_gnptxsts {
        super::Fs_gnptxsts {
            raw: unsafe { *((0x50000000u32 + 0x2Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gnptxsts) {
       unsafe { *((0x50000000u32 + 0x2Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_gccfg {
   raw: u32,
}

impl Fs_gccfg {
    #[inline(always)]
    pub fn pwrdwn_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwrdwn(mut self, val: u32) -> Fs_gccfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn vbusasen_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn vbusasen(mut self, val: u32) -> Fs_gccfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn vbusbsen_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn vbusbsen(mut self, val: u32) -> Fs_gccfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn sofouten_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sofouten(mut self, val: u32) -> Fs_gccfg {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x38u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_gccfg {
    #[inline(always)]
    pub fn read() -> super::Fs_gccfg {
        super::Fs_gccfg {
            raw: unsafe { *((0x50000000u32 + 0x38u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_gccfg) {
       unsafe { *((0x50000000u32 + 0x38u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_cid {
   raw: u32,
}

impl Fs_cid {
    #[inline(always)]
    pub fn product_id_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn product_id(mut self, val: u32) -> Fs_cid {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x3Cu32) as *mut u32) = self.raw; }
    }
}

pub mod fs_cid {
    #[inline(always)]
    pub fn read() -> super::Fs_cid {
        super::Fs_cid {
            raw: unsafe { *((0x50000000u32 + 0x3Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_cid) {
       unsafe { *((0x50000000u32 + 0x3Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_hptxfsiz {
   raw: u32,
}

impl Fs_hptxfsiz {
    #[inline(always)]
    pub fn ptxsa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ptxsa(mut self, val: u32) -> Fs_hptxfsiz {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ptxfsiz_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ptxfsiz(mut self, val: u32) -> Fs_hptxfsiz {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x100u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_hptxfsiz {
    #[inline(always)]
    pub fn read() -> super::Fs_hptxfsiz {
        super::Fs_hptxfsiz {
            raw: unsafe { *((0x50000000u32 + 0x100u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_hptxfsiz) {
       unsafe { *((0x50000000u32 + 0x100u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_dieptxf1 {
   raw: u32,
}

impl Fs_dieptxf1 {
    #[inline(always)]
    pub fn ineptxsa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptxsa(mut self, val: u32) -> Fs_dieptxf1 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ineptxfd_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptxfd(mut self, val: u32) -> Fs_dieptxf1 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x104u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_dieptxf1 {
    #[inline(always)]
    pub fn read() -> super::Fs_dieptxf1 {
        super::Fs_dieptxf1 {
            raw: unsafe { *((0x50000000u32 + 0x104u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_dieptxf1) {
       unsafe { *((0x50000000u32 + 0x104u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_dieptxf2 {
   raw: u32,
}

impl Fs_dieptxf2 {
    #[inline(always)]
    pub fn ineptxsa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptxsa(mut self, val: u32) -> Fs_dieptxf2 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ineptxfd_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptxfd(mut self, val: u32) -> Fs_dieptxf2 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x108u32) as *mut u32) = self.raw; }
    }
}

pub mod fs_dieptxf2 {
    #[inline(always)]
    pub fn read() -> super::Fs_dieptxf2 {
        super::Fs_dieptxf2 {
            raw: unsafe { *((0x50000000u32 + 0x108u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_dieptxf2) {
       unsafe { *((0x50000000u32 + 0x108u32) as *mut u32) = val.raw; }
    }
}

pub struct Fs_dieptxf3 {
   raw: u32,
}

impl Fs_dieptxf3 {
    #[inline(always)]
    pub fn ineptxsa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptxsa(mut self, val: u32) -> Fs_dieptxf3 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ineptxfd_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ineptxfd(mut self, val: u32) -> Fs_dieptxf3 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x50000000u32 + 0x10Cu32) as *mut u32) = self.raw; }
    }
}

pub mod fs_dieptxf3 {
    #[inline(always)]
    pub fn read() -> super::Fs_dieptxf3 {
        super::Fs_dieptxf3 {
            raw: unsafe { *((0x50000000u32 + 0x10Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_dieptxf3) {
       unsafe { *((0x50000000u32 + 0x10Cu32) as *mut u32) = val.raw; }
    }
}

