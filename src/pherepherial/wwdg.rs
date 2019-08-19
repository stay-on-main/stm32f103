pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn t_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn t_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 7) - 1) << 0)) | ((val & ((1 << 7) - 1)) << 0)
    }

    #[inline(always)]
    pub fn wdga_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wdga_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0x40002C00 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0x40002C00 + 0x0) as *mut u32) = val.raw; }
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
    pub fn w_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 7) - 1) << 0)) | ((val & ((1 << 7) - 1)) << 0)
    }

    #[inline(always)]
    pub fn wdgtb_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn wdgtb_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 7)) | ((val & ((1 << 2) - 1)) << 7)
    }

    #[inline(always)]
    pub fn ewi_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ewi_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

}

pub mod cfr {
    #[inline(always)]
    pub fn read() -> super::Cfr {
        super::Cfr {
            raw: unsafe { *((0x40002C00 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cfr) {
       unsafe { *((0x40002C00 + 0x4) as *mut u32) = val.raw; }
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
    pub fn ewi_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40002C00 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40002C00 + 0x8) as *mut u32) = val.raw; }
    }
}

