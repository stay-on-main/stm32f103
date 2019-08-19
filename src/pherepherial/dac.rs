pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn en1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn boff1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn boff1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn ten1_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ten1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn tsel1_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn tsel1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 3)) | ((val & ((1 << 3) - 1)) << 3)
    }

    #[inline(always)]
    pub fn wave1_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn wave1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 6)) | ((val & ((1 << 2) - 1)) << 6)
    }

    #[inline(always)]
    pub fn mamp1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn mamp1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 8)) | ((val & ((1 << 4) - 1)) << 8)
    }

    #[inline(always)]
    pub fn dmaen1_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dmaen1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn en2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn en2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn boff2_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn boff2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn ten2_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ten2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

    #[inline(always)]
    pub fn tsel2_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn tsel2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 19)) | ((val & ((1 << 3) - 1)) << 19)
    }

    #[inline(always)]
    pub fn wave2_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn wave2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 22)) | ((val & ((1 << 2) - 1)) << 22)
    }

    #[inline(always)]
    pub fn mamp2_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn mamp2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn dmaen2_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dmaen2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28)
    }

}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0x40007400 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0x40007400 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Swtrigr {
   raw: u32,
}

impl Swtrigr {
    #[inline(always)]
    pub fn swtrig1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swtrig1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn swtrig2_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swtrig2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

}

pub mod swtrigr {
    #[inline(always)]
    pub fn read() -> super::Swtrigr {
        super::Swtrigr {
            raw: unsafe { *((0x40007400 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Swtrigr) {
       unsafe { *((0x40007400 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Dhr12r1 {
   raw: u32,
}

impl Dhr12r1 {
    #[inline(always)]
    pub fn dacc1dhr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc1dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod dhr12r1 {
    #[inline(always)]
    pub fn read() -> super::Dhr12r1 {
        super::Dhr12r1 {
            raw: unsafe { *((0x40007400 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr12r1) {
       unsafe { *((0x40007400 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Dhr12l1 {
   raw: u32,
}

impl Dhr12l1 {
    #[inline(always)]
    pub fn dacc1dhr_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc1dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 4)) | ((val & ((1 << 12) - 1)) << 4)
    }

}

pub mod dhr12l1 {
    #[inline(always)]
    pub fn read() -> super::Dhr12l1 {
        super::Dhr12l1 {
            raw: unsafe { *((0x40007400 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr12l1) {
       unsafe { *((0x40007400 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Dhr8r1 {
   raw: u32,
}

impl Dhr8r1 {
    #[inline(always)]
    pub fn dacc1dhr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn dacc1dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod dhr8r1 {
    #[inline(always)]
    pub fn read() -> super::Dhr8r1 {
        super::Dhr8r1 {
            raw: unsafe { *((0x40007400 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr8r1) {
       unsafe { *((0x40007400 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Dhr12r2 {
   raw: u32,
}

impl Dhr12r2 {
    #[inline(always)]
    pub fn dacc2dhr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc2dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod dhr12r2 {
    #[inline(always)]
    pub fn read() -> super::Dhr12r2 {
        super::Dhr12r2 {
            raw: unsafe { *((0x40007400 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr12r2) {
       unsafe { *((0x40007400 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Dhr12l2 {
   raw: u32,
}

impl Dhr12l2 {
    #[inline(always)]
    pub fn dacc2dhr_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc2dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 4)) | ((val & ((1 << 12) - 1)) << 4)
    }

}

pub mod dhr12l2 {
    #[inline(always)]
    pub fn read() -> super::Dhr12l2 {
        super::Dhr12l2 {
            raw: unsafe { *((0x40007400 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr12l2) {
       unsafe { *((0x40007400 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Dhr8r2 {
   raw: u32,
}

impl Dhr8r2 {
    #[inline(always)]
    pub fn dacc2dhr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn dacc2dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod dhr8r2 {
    #[inline(always)]
    pub fn read() -> super::Dhr8r2 {
        super::Dhr8r2 {
            raw: unsafe { *((0x40007400 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr8r2) {
       unsafe { *((0x40007400 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Dhr12rd {
   raw: u32,
}

impl Dhr12rd {
    #[inline(always)]
    pub fn dacc1dhr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc1dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

    #[inline(always)]
    pub fn dacc2dhr_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc2dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 16)) | ((val & ((1 << 12) - 1)) << 16)
    }

}

pub mod dhr12rd {
    #[inline(always)]
    pub fn read() -> super::Dhr12rd {
        super::Dhr12rd {
            raw: unsafe { *((0x40007400 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr12rd) {
       unsafe { *((0x40007400 + 0x20) as *mut u32) = val.raw; }
    }
}

pub struct Dhr12ld {
   raw: u32,
}

impl Dhr12ld {
    #[inline(always)]
    pub fn dacc1dhr_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc1dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 4)) | ((val & ((1 << 12) - 1)) << 4)
    }

    #[inline(always)]
    pub fn dacc2dhr_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc2dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 20)) | ((val & ((1 << 12) - 1)) << 20)
    }

}

pub mod dhr12ld {
    #[inline(always)]
    pub fn read() -> super::Dhr12ld {
        super::Dhr12ld {
            raw: unsafe { *((0x40007400 + 0x24) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr12ld) {
       unsafe { *((0x40007400 + 0x24) as *mut u32) = val.raw; }
    }
}

pub struct Dhr8rd {
   raw: u32,
}

impl Dhr8rd {
    #[inline(always)]
    pub fn dacc1dhr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn dacc1dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

    #[inline(always)]
    pub fn dacc2dhr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn dacc2dhr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

}

pub mod dhr8rd {
    #[inline(always)]
    pub fn read() -> super::Dhr8rd {
        super::Dhr8rd {
            raw: unsafe { *((0x40007400 + 0x28) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dhr8rd) {
       unsafe { *((0x40007400 + 0x28) as *mut u32) = val.raw; }
    }
}

pub struct Dor1 {
   raw: u32,
}

impl Dor1 {
    #[inline(always)]
    pub fn dacc1dor_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc1dor_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod dor1 {
    #[inline(always)]
    pub fn read() -> super::Dor1 {
        super::Dor1 {
            raw: unsafe { *((0x40007400 + 0x2C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dor1) {
       unsafe { *((0x40007400 + 0x2C) as *mut u32) = val.raw; }
    }
}

pub struct Dor2 {
   raw: u32,
}

impl Dor2 {
    #[inline(always)]
    pub fn dacc2dor_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dacc2dor_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0)
    }

}

pub mod dor2 {
    #[inline(always)]
    pub fn read() -> super::Dor2 {
        super::Dor2 {
            raw: unsafe { *((0x40007400 + 0x30) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dor2) {
       unsafe { *((0x40007400 + 0x30) as *mut u32) = val.raw; }
    }
}

