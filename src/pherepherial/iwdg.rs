pub struct Kr {
   raw: u32,
}

impl Kr {
    #[inline(always)]
    pub fn key_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn key_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0)
    }

}

pub mod kr {
    #[inline(always)]
    pub fn read() -> super::Kr {
        super::Kr {
            raw: unsafe { *((0x40003000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Kr) {
       unsafe { *((0x40003000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Pr {
   raw: u32,
}

impl Pr {
    #[inline(always)]
    pub fn pr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn pr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 0)) | ((val & ((1 << 3) - 1)) << 0)
    }

}

pub mod pr {
    #[inline(always)]
    pub fn read() -> super::Pr {
        super::Pr {
            raw: unsafe { *((0x40003000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pr) {
       unsafe { *((0x40003000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Rlr {
   raw: u32,
}

impl Rlr {
    #[inline(always)]
    pub fn rl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn rl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod rlr {
    #[inline(always)]
    pub fn read() -> super::Rlr {
        super::Rlr {
            raw: unsafe { *((0x40003000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Rlr) {
       unsafe { *((0x40003000 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn pvu_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pvu_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn rvu_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rvu_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40003000 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40003000 + 0xC) as *mut u32) = val.raw; }
    }
}

