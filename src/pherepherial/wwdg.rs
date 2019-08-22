pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn t_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn t(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 7) - 1) << 0)) | ((val & ((1 << 7) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn wdga_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wdga(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40002C00u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0x40002C00u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0x40002C00u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Cfr {
   raw: u32,
}

impl Cfr {
    #[inline(always)]
    pub fn w_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn w(mut self, val: u32) -> Cfr {
        self.raw = (self.raw & !(((1 << 7) - 1) << 0)) | ((val & ((1 << 7) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn wdgtb_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn wdgtb(mut self, val: u32) -> Cfr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 7)) | ((val & ((1 << 2) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ewi_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ewi(mut self, val: u32) -> Cfr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40002C00u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod cfr {
    #[inline(always)]
    pub fn read() -> super::Cfr {
        super::Cfr {
            raw: unsafe { *((0x40002C00u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cfr) {
       unsafe { *((0x40002C00u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn ewi_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ewi(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40002C00u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40002C00u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40002C00u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

