pub struct Stir {
   raw: u32,
}

impl Stir {
    #[inline(always)]
    pub fn intid_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 9) - 1)
    }

    #[inline(always)]
    pub fn intid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 9) - 1) << 0)) | ((val & ((1 << 9) - 1)) << 0)
    }

}

pub mod stir {
    #[inline(always)]
    pub fn read() -> super::Stir {
        super::Stir {
            raw: unsafe { *((0xE000EF00 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Stir) {
       unsafe { *((0xE000EF00 + 0x0) as *mut u32) = val.raw; }
    }
}

