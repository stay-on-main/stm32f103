pub struct Acr {
   raw: u32,
}

impl Acr {
    #[inline(always)]
    pub fn latency_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn latency(mut self, val: u32) -> Acr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn hlfcya_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hlfcya(mut self, val: u32) -> Acr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn prftbe_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn prftbe(mut self, val: u32) -> Acr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn prftbs_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn prftbs(mut self, val: u32) -> Acr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40022000 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod acr {
    #[inline(always)]
    pub fn read() -> super::Acr {
        super::Acr {
            raw: unsafe { *((0x40022000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Acr) {
       unsafe { *((0x40022000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Keyr {
   raw: u32,
}

impl Keyr {
    #[inline(always)]
    pub fn key_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn key(mut self, val: u32) -> Keyr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40022000 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod keyr {
    #[inline(always)]
    pub fn read() -> super::Keyr {
        super::Keyr {
            raw: unsafe { *((0x40022000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Keyr) {
       unsafe { *((0x40022000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Optkeyr {
   raw: u32,
}

impl Optkeyr {
    #[inline(always)]
    pub fn optkey_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn optkey(mut self, val: u32) -> Optkeyr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40022000 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod optkeyr {
    #[inline(always)]
    pub fn read() -> super::Optkeyr {
        super::Optkeyr {
            raw: unsafe { *((0x40022000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Optkeyr) {
       unsafe { *((0x40022000 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn eop_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eop(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn wrprterr_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wrprterr(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn pgerr_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pgerr(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn bsy_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bsy(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40022000 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40022000 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40022000 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn pg_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pg(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn per_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn per(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn mer_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mer(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn optpg_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn optpg(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn opter_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn opter(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn strt_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn strt(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn lock_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lock(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn optwre_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn optwre(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn errie_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn errie(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn eopie_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eopie(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40022000 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0x40022000 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0x40022000 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Ar {
   raw: u32,
}

impl Ar {
    #[inline(always)]
    pub fn far_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn far(mut self, val: u32) -> Ar {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40022000 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod ar {
    #[inline(always)]
    pub fn read() -> super::Ar {
        super::Ar {
            raw: unsafe { *((0x40022000 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ar) {
       unsafe { *((0x40022000 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Obr {
   raw: u32,
}

impl Obr {
    #[inline(always)]
    pub fn opterr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn opterr(mut self, val: u32) -> Obr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn rdprt_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rdprt(mut self, val: u32) -> Obr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn wdg_sw_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wdg_sw(mut self, val: u32) -> Obr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn nrst_stop_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nrst_stop(mut self, val: u32) -> Obr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn nrst_stdby_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nrst_stdby(mut self, val: u32) -> Obr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn data0_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data0(mut self, val: u32) -> Obr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 10)) | ((val & ((1 << 8) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn data1_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data1(mut self, val: u32) -> Obr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 18)) | ((val & ((1 << 8) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40022000 + 0x1C) as *mut u32) = self.raw; }
    }
}

pub mod obr {
    #[inline(always)]
    pub fn read() -> super::Obr {
        super::Obr {
            raw: unsafe { *((0x40022000 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Obr) {
       unsafe { *((0x40022000 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Wrpr {
   raw: u32,
}

impl Wrpr {
    #[inline(always)]
    pub fn wrp_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn wrp(mut self, val: u32) -> Wrpr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40022000 + 0x20) as *mut u32) = self.raw; }
    }
}

pub mod wrpr {
    #[inline(always)]
    pub fn read() -> super::Wrpr {
        super::Wrpr {
            raw: unsafe { *((0x40022000 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Wrpr) {
       unsafe { *((0x40022000 + 0x20) as *mut u32) = val.raw; }
    }
}

