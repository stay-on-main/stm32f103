pub struct Actrl {
   raw: u32,
}

impl Actrl {
    #[inline(always)]
    pub fn disfold_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn disfold(mut self, val: u32) -> Actrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fpexcodis_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fpexcodis(mut self, val: u32) -> Actrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn disramode_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn disramode(mut self, val: u32) -> Actrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn disitmatbflush_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn disitmatbflush(mut self, val: u32) -> Actrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E008u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod actrl {
    #[inline(always)]
    pub fn read() -> super::Actrl {
        super::Actrl {
            raw: unsafe { *((0xE000E008u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Actrl) {
       unsafe { *((0xE000E008u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

