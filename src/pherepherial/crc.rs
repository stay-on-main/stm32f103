pub struct Dr {
   raw: u32,
}

impl Dr {
    #[inline(always)]
    pub fn dr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn dr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod dr {
    #[inline(always)]
    pub fn read() -> super::Dr {
        super::Dr {
            raw: unsafe { *((0x40023000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr) {
       unsafe { *((0x40023000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Idr {
   raw: u32,
}

impl Idr {
    #[inline(always)]
    pub fn idr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn idr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod idr {
    #[inline(always)]
    pub fn read() -> super::Idr {
        super::Idr {
            raw: unsafe { *((0x40023000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Idr) {
       unsafe { *((0x40023000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn reset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn reset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0x40023000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0x40023000 + 0x8) as *mut u32) = val.raw; }
    }
}

