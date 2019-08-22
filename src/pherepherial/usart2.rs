pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn cts_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cts(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn lbd_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lbd(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn txe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txe(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn tc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tc(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn rxne_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxne(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn idle_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idle(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ore_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ore(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ne_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ne(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fe_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fe(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn pe_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pe(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40004400 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40004400 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40004400 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Dr {
   raw: u32,
}

impl Dr {
    #[inline(always)]
    pub fn dr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 9) - 1)
    }

    #[inline(always)]
    pub fn dr(mut self, val: u32) -> Dr {
        self.raw = (self.raw & !(((1 << 9) - 1) << 0)) | ((val & ((1 << 9) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40004400 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod dr {
    #[inline(always)]
    pub fn read() -> super::Dr {
        super::Dr {
            raw: unsafe { *((0x40004400 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr) {
       unsafe { *((0x40004400 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Brr {
   raw: u32,
}

impl Brr {
    #[inline(always)]
    pub fn div_mantissa_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn div_mantissa(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 12) - 1) << 4)) | ((val & ((1 << 12) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn div_fraction_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn div_fraction(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40004400 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod brr {
    #[inline(always)]
    pub fn read() -> super::Brr {
        super::Brr {
            raw: unsafe { *((0x40004400 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Brr) {
       unsafe { *((0x40004400 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Cr1 {
   raw: u32,
}

impl Cr1 {
    #[inline(always)]
    pub fn ue_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ue(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn m_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn m(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn wake_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wake(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn pce_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pce(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn ps_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ps(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn peie_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn peie(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn txeie_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txeie(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn tcie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tcie(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn rxneie_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxneie(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn idleie_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idleie(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn te_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn te(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn re_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn re(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rwu_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rwu(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn sbk_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sbk(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40004400 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod cr1 {
    #[inline(always)]
    pub fn read() -> super::Cr1 {
        super::Cr1 {
            raw: unsafe { *((0x40004400 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr1) {
       unsafe { *((0x40004400 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Cr2 {
   raw: u32,
}

impl Cr2 {
    #[inline(always)]
    pub fn linen_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn linen(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn stop_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stop(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn clken_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn clken(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn cpol_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cpol(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cpha_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cpha(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn lbcl_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lbcl(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn lbdie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lbdie(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn lbdl_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lbdl(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn add_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn add(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40004400 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod cr2 {
    #[inline(always)]
    pub fn read() -> super::Cr2 {
        super::Cr2 {
            raw: unsafe { *((0x40004400 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr2) {
       unsafe { *((0x40004400 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Cr3 {
   raw: u32,
}

impl Cr3 {
    #[inline(always)]
    pub fn ctsie_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctsie(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn ctse_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctse(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn rtse_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rtse(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dmat_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dmat(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn dmar_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dmar(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn scen_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn scen(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nack_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nack(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn hdsel_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hdsel(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn irlp_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn irlp(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn iren_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iren(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn eie_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eie(mut self, val: u32) -> Cr3 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40004400 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod cr3 {
    #[inline(always)]
    pub fn read() -> super::Cr3 {
        super::Cr3 {
            raw: unsafe { *((0x40004400 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr3) {
       unsafe { *((0x40004400 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Gtpr {
   raw: u32,
}

impl Gtpr {
    #[inline(always)]
    pub fn gt_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn gt(mut self, val: u32) -> Gtpr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn psc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn psc(mut self, val: u32) -> Gtpr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40004400 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod gtpr {
    #[inline(always)]
    pub fn read() -> super::Gtpr {
        super::Gtpr {
            raw: unsafe { *((0x40004400 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Gtpr) {
       unsafe { *((0x40004400 + 0x18) as *mut u32) = val.raw; }
    }
}

