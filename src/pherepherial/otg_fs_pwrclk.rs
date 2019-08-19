pub struct Fs_pcgcctl {
   raw: u32,
}

impl Fs_pcgcctl {
    #[inline(always)]
    pub fn stppclk_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stppclk_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn gatehclk_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn gatehclk_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn physusp_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn physusp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

}

pub mod fs_pcgcctl {
    #[inline(always)]
    pub fn read() -> super::Fs_pcgcctl {
        super::Fs_pcgcctl {
            raw: unsafe { *((0x50000E00 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fs_pcgcctl) {
       unsafe { *((0x50000E00 + 0x0) as *mut u32) = val.raw; }
    }
}

