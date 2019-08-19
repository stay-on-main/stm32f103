pub struct Mmccr {
   raw: u32,
}

impl Mmccr {
    #[inline(always)]
    pub fn cr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn csr_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn csr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn ror_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ror_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn mcf_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mcf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31)
    }

}

pub mod mmccr {
    #[inline(always)]
    pub fn read() -> super::Mmccr {
        super::Mmccr {
            raw: unsafe { *((0x40028100 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmccr) {
       unsafe { *((0x40028100 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Mmcrir {
   raw: u32,
}

impl Mmcrir {
    #[inline(always)]
    pub fn rfces_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rfces_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn rfaes_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rfaes_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn rgufs_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rgufs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

}

pub mod mmcrir {
    #[inline(always)]
    pub fn read() -> super::Mmcrir {
        super::Mmcrir {
            raw: unsafe { *((0x40028100 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmcrir) {
       unsafe { *((0x40028100 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Mmctir {
   raw: u32,
}

impl Mmctir {
    #[inline(always)]
    pub fn tgfscs_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgfscs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn tgfmscs_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgfmscs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn tgfs_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgfs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21)
    }

}

pub mod mmctir {
    #[inline(always)]
    pub fn read() -> super::Mmctir {
        super::Mmctir {
            raw: unsafe { *((0x40028100 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmctir) {
       unsafe { *((0x40028100 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Mmcrimr {
   raw: u32,
}

impl Mmcrimr {
    #[inline(always)]
    pub fn rfcem_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rfcem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn rfaem_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rfaem_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn rgufm_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rgufm_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

}

pub mod mmcrimr {
    #[inline(always)]
    pub fn read() -> super::Mmcrimr {
        super::Mmcrimr {
            raw: unsafe { *((0x40028100 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmcrimr) {
       unsafe { *((0x40028100 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Mmctimr {
   raw: u32,
}

impl Mmctimr {
    #[inline(always)]
    pub fn tgfscm_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgfscm_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn tgfmscm_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgfmscm_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn tgfm_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgfm_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21)
    }

}

pub mod mmctimr {
    #[inline(always)]
    pub fn read() -> super::Mmctimr {
        super::Mmctimr {
            raw: unsafe { *((0x40028100 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmctimr) {
       unsafe { *((0x40028100 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Mmctgfsccr {
   raw: u32,
}

impl Mmctgfsccr {
    #[inline(always)]
    pub fn tgfscc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn tgfscc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod mmctgfsccr {
    #[inline(always)]
    pub fn read() -> super::Mmctgfsccr {
        super::Mmctgfsccr {
            raw: unsafe { *((0x40028100 + 0x4C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmctgfsccr) {
       unsafe { *((0x40028100 + 0x4C) as *mut u32) = val.raw; }
    }
}

pub struct Mmctgfmsccr {
   raw: u32,
}

impl Mmctgfmsccr {
    #[inline(always)]
    pub fn tgfmscc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn tgfmscc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod mmctgfmsccr {
    #[inline(always)]
    pub fn read() -> super::Mmctgfmsccr {
        super::Mmctgfmsccr {
            raw: unsafe { *((0x40028100 + 0x50) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmctgfmsccr) {
       unsafe { *((0x40028100 + 0x50) as *mut u32) = val.raw; }
    }
}

pub struct Mmctgfcr {
   raw: u32,
}

impl Mmctgfcr {
    #[inline(always)]
    pub fn tgfc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn tgfc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod mmctgfcr {
    #[inline(always)]
    pub fn read() -> super::Mmctgfcr {
        super::Mmctgfcr {
            raw: unsafe { *((0x40028100 + 0x68) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmctgfcr) {
       unsafe { *((0x40028100 + 0x68) as *mut u32) = val.raw; }
    }
}

pub struct Mmcrfcecr {
   raw: u32,
}

impl Mmcrfcecr {
    #[inline(always)]
    pub fn rfcfc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn rfcfc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod mmcrfcecr {
    #[inline(always)]
    pub fn read() -> super::Mmcrfcecr {
        super::Mmcrfcecr {
            raw: unsafe { *((0x40028100 + 0x94) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmcrfcecr) {
       unsafe { *((0x40028100 + 0x94) as *mut u32) = val.raw; }
    }
}

pub struct Mmcrfaecr {
   raw: u32,
}

impl Mmcrfaecr {
    #[inline(always)]
    pub fn rfaec_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn rfaec_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod mmcrfaecr {
    #[inline(always)]
    pub fn read() -> super::Mmcrfaecr {
        super::Mmcrfaecr {
            raw: unsafe { *((0x40028100 + 0x98) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmcrfaecr) {
       unsafe { *((0x40028100 + 0x98) as *mut u32) = val.raw; }
    }
}

pub struct Mmcrgufcr {
   raw: u32,
}

impl Mmcrgufcr {
    #[inline(always)]
    pub fn rgufc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn rgufc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod mmcrgufcr {
    #[inline(always)]
    pub fn read() -> super::Mmcrgufcr {
        super::Mmcrgufcr {
            raw: unsafe { *((0x40028100 + 0xC4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmcrgufcr) {
       unsafe { *((0x40028100 + 0xC4) as *mut u32) = val.raw; }
    }
}

