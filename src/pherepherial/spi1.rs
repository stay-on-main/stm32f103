pub struct Cr1 {
   raw: u32,
}

impl Cr1 {
    #[inline(always)]
    pub fn bidimode_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bidimode(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn bidioe_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bidioe(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn crcen_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn crcen(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn crcnext_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn crcnext(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dff_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dff(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn rxonly_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxonly(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn ssm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ssm(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn ssi_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ssi(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn lsbfirst_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsbfirst(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn spe_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn spe(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn br_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn br(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 3)) | ((val & ((1 << 3) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn mstr_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mstr(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cpol_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cpol(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn cpha_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cpha(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod cr1 {
    #[inline(always)]
    pub fn read() -> super::Cr1 {
        super::Cr1 {
            raw: unsafe { *((0x40013000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr1) {
       unsafe { *((0x40013000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Cr2 {
   raw: u32,
}

impl Cr2 {
    #[inline(always)]
    pub fn txeie_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txeie(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn rxneie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxneie(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn errie_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn errie(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn ssoe_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ssoe(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn txdmaen_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txdmaen(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn rxdmaen_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxdmaen(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod cr2 {
    #[inline(always)]
    pub fn read() -> super::Cr2 {
        super::Cr2 {
            raw: unsafe { *((0x40013000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr2) {
       unsafe { *((0x40013000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn bsy_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bsy(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ovr_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ovr(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn modf_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn modf(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn crcerr_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn crcerr(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn udr_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn udr(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn chside_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chside(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn txe_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txe(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn rxne_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxne(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40013000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40013000 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Dr {
   raw: u32,
}

impl Dr {
    #[inline(always)]
    pub fn dr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn dr(mut self, val: u32) -> Dr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod dr {
    #[inline(always)]
    pub fn read() -> super::Dr {
        super::Dr {
            raw: unsafe { *((0x40013000 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr) {
       unsafe { *((0x40013000 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Crcpr {
   raw: u32,
}

impl Crcpr {
    #[inline(always)]
    pub fn crcpoly_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn crcpoly(mut self, val: u32) -> Crcpr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod crcpr {
    #[inline(always)]
    pub fn read() -> super::Crcpr {
        super::Crcpr {
            raw: unsafe { *((0x40013000 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Crcpr) {
       unsafe { *((0x40013000 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Rxcrcr {
   raw: u32,
}

impl Rxcrcr {
    #[inline(always)]
    pub fn rxcrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn rxcrc(mut self, val: u32) -> Rxcrcr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod rxcrcr {
    #[inline(always)]
    pub fn read() -> super::Rxcrcr {
        super::Rxcrcr {
            raw: unsafe { *((0x40013000 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Rxcrcr) {
       unsafe { *((0x40013000 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Txcrcr {
   raw: u32,
}

impl Txcrcr {
    #[inline(always)]
    pub fn txcrc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn txcrc(mut self, val: u32) -> Txcrcr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod txcrcr {
    #[inline(always)]
    pub fn read() -> super::Txcrcr {
        super::Txcrcr {
            raw: unsafe { *((0x40013000 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Txcrcr) {
       unsafe { *((0x40013000 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct I2scfgr {
   raw: u32,
}

impl I2scfgr {
    #[inline(always)]
    pub fn i2smod_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn i2smod(mut self, val: u32) -> I2scfgr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn i2se_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn i2se(mut self, val: u32) -> I2scfgr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn i2scfg_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn i2scfg(mut self, val: u32) -> I2scfgr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn pcmsync_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pcmsync(mut self, val: u32) -> I2scfgr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn i2sstd_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn i2sstd(mut self, val: u32) -> I2scfgr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ckpol_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ckpol(mut self, val: u32) -> I2scfgr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn datlen_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn datlen(mut self, val: u32) -> I2scfgr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 1)) | ((val & ((1 << 2) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn chlen_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn chlen(mut self, val: u32) -> I2scfgr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0x1C) as *mut u32) = self.raw; }
    }
}

pub mod i2scfgr {
    #[inline(always)]
    pub fn read() -> super::I2scfgr {
        super::I2scfgr {
            raw: unsafe { *((0x40013000 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::I2scfgr) {
       unsafe { *((0x40013000 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct I2spr {
   raw: u32,
}

impl I2spr {
    #[inline(always)]
    pub fn mckoe_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mckoe(mut self, val: u32) -> I2spr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn odd_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odd(mut self, val: u32) -> I2spr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn i2sdiv_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn i2sdiv(mut self, val: u32) -> I2spr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40013000 + 0x20) as *mut u32) = self.raw; }
    }
}

pub mod i2spr {
    #[inline(always)]
    pub fn read() -> super::I2spr {
        super::I2spr {
            raw: unsafe { *((0x40013000 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::I2spr) {
       unsafe { *((0x40013000 + 0x20) as *mut u32) = val.raw; }
    }
}

