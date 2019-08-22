pub struct Cr1 {
   raw: u32,
}

impl Cr1 {
    #[inline(always)]
    pub fn swrst_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swrst(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn alert_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn alert(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn pec_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pec(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn pos_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pos(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn ack_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ack(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn stop_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stop(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn start_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn start(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn nostretch_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nostretch(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn engc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn engc(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn enpec_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn enpec(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn enarp_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn enarp(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn smbtype_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn smbtype(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn smbus_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn smbus(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn pe_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pe(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod cr1 {
    #[inline(always)]
    pub fn read() -> super::Cr1 {
        super::Cr1 {
            raw: unsafe { *((0x40005400 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr1) {
       unsafe { *((0x40005400 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Cr2 {
   raw: u32,
}

impl Cr2 {
    #[inline(always)]
    pub fn last_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn last(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dmaen_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dmaen(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn itbufen_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn itbufen(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn itevten_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn itevten(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn iterren_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iterren(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn freq_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn freq(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 6) - 1) << 0)) | ((val & ((1 << 6) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod cr2 {
    #[inline(always)]
    pub fn read() -> super::Cr2 {
        super::Cr2 {
            raw: unsafe { *((0x40005400 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr2) {
       unsafe { *((0x40005400 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Oar1 {
   raw: u32,
}

impl Oar1 {
    #[inline(always)]
    pub fn addmode_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn addmode(mut self, val: u32) -> Oar1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn add10_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn add10(mut self, val: u32) -> Oar1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn add7_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn add7(mut self, val: u32) -> Oar1 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 1)) | ((val & ((1 << 7) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn add0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn add0(mut self, val: u32) -> Oar1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod oar1 {
    #[inline(always)]
    pub fn read() -> super::Oar1 {
        super::Oar1 {
            raw: unsafe { *((0x40005400 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Oar1) {
       unsafe { *((0x40005400 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Oar2 {
   raw: u32,
}

impl Oar2 {
    #[inline(always)]
    pub fn add2_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn add2(mut self, val: u32) -> Oar2 {
        self.raw = (self.raw & !(((1 << 7) - 1) << 1)) | ((val & ((1 << 7) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn endual_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn endual(mut self, val: u32) -> Oar2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod oar2 {
    #[inline(always)]
    pub fn read() -> super::Oar2 {
        super::Oar2 {
            raw: unsafe { *((0x40005400 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Oar2) {
       unsafe { *((0x40005400 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Dr {
   raw: u32,
}

impl Dr {
    #[inline(always)]
    pub fn dr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn dr(mut self, val: u32) -> Dr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod dr {
    #[inline(always)]
    pub fn read() -> super::Dr {
        super::Dr {
            raw: unsafe { *((0x40005400 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr) {
       unsafe { *((0x40005400 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Sr1 {
   raw: u32,
}

impl Sr1 {
    #[inline(always)]
    pub fn smbalert_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn smbalert(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn timeout_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn timeout(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn pecerr_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pecerr(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn ovr_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ovr(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn af_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn af(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn arlo_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn arlo(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn berr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn berr(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn txe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txe(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn rxne_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxne(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn stopf_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stopf(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn add10_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn add10(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn btf_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn btf(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn addr_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn addr(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn sb_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sb(mut self, val: u32) -> Sr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod sr1 {
    #[inline(always)]
    pub fn read() -> super::Sr1 {
        super::Sr1 {
            raw: unsafe { *((0x40005400 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr1) {
       unsafe { *((0x40005400 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Sr2 {
   raw: u32,
}

impl Sr2 {
    #[inline(always)]
    pub fn pec_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pec(mut self, val: u32) -> Sr2 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dualf_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dualf(mut self, val: u32) -> Sr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn smbhost_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn smbhost(mut self, val: u32) -> Sr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn smbdefault_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn smbdefault(mut self, val: u32) -> Sr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn gencall_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gencall(mut self, val: u32) -> Sr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn tra_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tra(mut self, val: u32) -> Sr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn busy_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn busy(mut self, val: u32) -> Sr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn msl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn msl(mut self, val: u32) -> Sr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod sr2 {
    #[inline(always)]
    pub fn read() -> super::Sr2 {
        super::Sr2 {
            raw: unsafe { *((0x40005400 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr2) {
       unsafe { *((0x40005400 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Ccr {
   raw: u32,
}

impl Ccr {
    #[inline(always)]
    pub fn f_s_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn f_s(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn duty_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn duty(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ccr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn ccr(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0x1C) as *mut u32) = self.raw; }
    }
}

pub mod ccr {
    #[inline(always)]
    pub fn read() -> super::Ccr {
        super::Ccr {
            raw: unsafe { *((0x40005400 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr) {
       unsafe { *((0x40005400 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Trise {
   raw: u32,
}

impl Trise {
    #[inline(always)]
    pub fn trise_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn trise(mut self, val: u32) -> Trise {
        self.raw = (self.raw & !(((1 << 6) - 1) << 0)) | ((val & ((1 << 6) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005400 + 0x20) as *mut u32) = self.raw; }
    }
}

pub mod trise {
    #[inline(always)]
    pub fn read() -> super::Trise {
        super::Trise {
            raw: unsafe { *((0x40005400 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Trise) {
       unsafe { *((0x40005400 + 0x20) as *mut u32) = val.raw; }
    }
}

