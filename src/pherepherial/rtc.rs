pub struct Crh {
   raw: u32,
}

impl Crh {
    #[inline(always)]
    pub fn secie_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn secie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn alrie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn alrie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn owie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn owie_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

}

pub mod crh {
    #[inline(always)]
    pub fn read() -> super::Crh {
        super::Crh {
            raw: unsafe { *((0x40002800 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Crh) {
       unsafe { *((0x40002800 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Crl {
   raw: u32,
}

impl Crl {
    #[inline(always)]
    pub fn secf_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn secf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn alrf_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn alrf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn owf_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn owf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn rsf_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rsf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn cnf_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cnf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn rtoff_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rtoff_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

}

pub mod crl {
    #[inline(always)]
    pub fn read() -> super::Crl {
        super::Crl {
            raw: unsafe { *((0x40002800 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Crl) {
       unsafe { *((0x40002800 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Prlh {
   raw: u32,
}

impl Prlh {
    #[inline(always)]
    pub fn prlh_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn prlh_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod prlh {
    #[inline(always)]
    pub fn read() -> super::Prlh {
        super::Prlh {
            raw: unsafe { *((0x40002800 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Prlh) {
       unsafe { *((0x40002800 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Prll {
   raw: u32,
}

impl Prll {
    #[inline(always)]
    pub fn prll_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn prll_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod prll {
    #[inline(always)]
    pub fn read() -> super::Prll {
        super::Prll {
            raw: unsafe { *((0x40002800 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Prll) {
       unsafe { *((0x40002800 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Divh {
   raw: u32,
}

impl Divh {
    #[inline(always)]
    pub fn divh_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn divh_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod divh {
    #[inline(always)]
    pub fn read() -> super::Divh {
        super::Divh {
            raw: unsafe { *((0x40002800 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Divh) {
       unsafe { *((0x40002800 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Divl {
   raw: u32,
}

impl Divl {
    #[inline(always)]
    pub fn divl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn divl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod divl {
    #[inline(always)]
    pub fn read() -> super::Divl {
        super::Divl {
            raw: unsafe { *((0x40002800 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Divl) {
       unsafe { *((0x40002800 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Cnth {
   raw: u32,
}

impl Cnth {
    #[inline(always)]
    pub fn cnth_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn cnth_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cnth {
    #[inline(always)]
    pub fn read() -> super::Cnth {
        super::Cnth {
            raw: unsafe { *((0x40002800 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cnth) {
       unsafe { *((0x40002800 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Cntl {
   raw: u32,
}

impl Cntl {
    #[inline(always)]
    pub fn cntl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn cntl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod cntl {
    #[inline(always)]
    pub fn read() -> super::Cntl {
        super::Cntl {
            raw: unsafe { *((0x40002800 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cntl) {
       unsafe { *((0x40002800 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Alrh {
   raw: u32,
}

impl Alrh {
    #[inline(always)]
    pub fn alrh_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn alrh_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod alrh {
    #[inline(always)]
    pub fn read() -> super::Alrh {
        super::Alrh {
            raw: unsafe { *((0x40002800 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Alrh) {
       unsafe { *((0x40002800 + 0x20) as *mut u32) = val.raw; }
    }
}

pub struct Alrl {
   raw: u32,
}

impl Alrl {
    #[inline(always)]
    pub fn alrl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn alrl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod alrl {
    #[inline(always)]
    pub fn read() -> super::Alrl {
        super::Alrl {
            raw: unsafe { *((0x40002800 + 0x24) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Alrl) {
       unsafe { *((0x40002800 + 0x24) as *mut u32) = val.raw; }
    }
}

