pub struct Stir {
   raw: u32,
}

impl Stir {
    #[inline(always)]
    pub fn intid_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 9) - 1)
    }

    #[inline(always)]
    pub fn intid(mut self, val: u32) -> Stir {
        self.raw = (self.raw & !(((1 << 9) - 1) << 0)) | ((val & ((1 << 9) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000EF00u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod stir {
    #[inline(always)]
    pub fn read() -> super::Stir {
        super::Stir {
            raw: unsafe { *((0xE000EF00u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Stir) {
       unsafe { *((0xE000EF00u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

