pub struct Dr1 {
   raw: u32,
}

impl Dr1 {
    #[inline(always)]
    pub fn d1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d1(mut self, val: u32) -> Dr1 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod dr1 {
    #[inline(always)]
    pub fn read() -> super::Dr1 {
        super::Dr1 {
            raw: unsafe { *((0x40006C00u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr1) {
       unsafe { *((0x40006C00u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr2 {
   raw: u32,
}

impl Dr2 {
    #[inline(always)]
    pub fn d2_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d2(mut self, val: u32) -> Dr2 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod dr2 {
    #[inline(always)]
    pub fn read() -> super::Dr2 {
        super::Dr2 {
            raw: unsafe { *((0x40006C00u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr2) {
       unsafe { *((0x40006C00u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr3 {
   raw: u32,
}

impl Dr3 {
    #[inline(always)]
    pub fn d3_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d3(mut self, val: u32) -> Dr3 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod dr3 {
    #[inline(always)]
    pub fn read() -> super::Dr3 {
        super::Dr3 {
            raw: unsafe { *((0x40006C00u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr3) {
       unsafe { *((0x40006C00u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr4 {
   raw: u32,
}

impl Dr4 {
    #[inline(always)]
    pub fn d4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d4(mut self, val: u32) -> Dr4 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod dr4 {
    #[inline(always)]
    pub fn read() -> super::Dr4 {
        super::Dr4 {
            raw: unsafe { *((0x40006C00u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr4) {
       unsafe { *((0x40006C00u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr5 {
   raw: u32,
}

impl Dr5 {
    #[inline(always)]
    pub fn d5_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d5(mut self, val: u32) -> Dr5 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod dr5 {
    #[inline(always)]
    pub fn read() -> super::Dr5 {
        super::Dr5 {
            raw: unsafe { *((0x40006C00u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr5) {
       unsafe { *((0x40006C00u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr6 {
   raw: u32,
}

impl Dr6 {
    #[inline(always)]
    pub fn d6_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d6(mut self, val: u32) -> Dr6 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod dr6 {
    #[inline(always)]
    pub fn read() -> super::Dr6 {
        super::Dr6 {
            raw: unsafe { *((0x40006C00u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr6) {
       unsafe { *((0x40006C00u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr7 {
   raw: u32,
}

impl Dr7 {
    #[inline(always)]
    pub fn d7_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d7(mut self, val: u32) -> Dr7 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod dr7 {
    #[inline(always)]
    pub fn read() -> super::Dr7 {
        super::Dr7 {
            raw: unsafe { *((0x40006C00u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr7) {
       unsafe { *((0x40006C00u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr8 {
   raw: u32,
}

impl Dr8 {
    #[inline(always)]
    pub fn d8_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d8(mut self, val: u32) -> Dr8 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr8 {
    #[inline(always)]
    pub fn read() -> super::Dr8 {
        super::Dr8 {
            raw: unsafe { *((0x40006C00u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr8) {
       unsafe { *((0x40006C00u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr9 {
   raw: u32,
}

impl Dr9 {
    #[inline(always)]
    pub fn d9_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d9(mut self, val: u32) -> Dr9 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x20u32) as *mut u32) = self.raw; }
    }
}

pub mod dr9 {
    #[inline(always)]
    pub fn read() -> super::Dr9 {
        super::Dr9 {
            raw: unsafe { *((0x40006C00u32 + 0x20u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr9) {
       unsafe { *((0x40006C00u32 + 0x20u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr10 {
   raw: u32,
}

impl Dr10 {
    #[inline(always)]
    pub fn d10_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d10(mut self, val: u32) -> Dr10 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x24u32) as *mut u32) = self.raw; }
    }
}

pub mod dr10 {
    #[inline(always)]
    pub fn read() -> super::Dr10 {
        super::Dr10 {
            raw: unsafe { *((0x40006C00u32 + 0x24u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr10) {
       unsafe { *((0x40006C00u32 + 0x24u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr11 {
   raw: u32,
}

impl Dr11 {
    #[inline(always)]
    pub fn dr11_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn dr11(mut self, val: u32) -> Dr11 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x3Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr11 {
    #[inline(always)]
    pub fn read() -> super::Dr11 {
        super::Dr11 {
            raw: unsafe { *((0x40006C00u32 + 0x3Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr11) {
       unsafe { *((0x40006C00u32 + 0x3Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr12 {
   raw: u32,
}

impl Dr12 {
    #[inline(always)]
    pub fn dr12_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn dr12(mut self, val: u32) -> Dr12 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x40u32) as *mut u32) = self.raw; }
    }
}

pub mod dr12 {
    #[inline(always)]
    pub fn read() -> super::Dr12 {
        super::Dr12 {
            raw: unsafe { *((0x40006C00u32 + 0x40u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr12) {
       unsafe { *((0x40006C00u32 + 0x40u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr13 {
   raw: u32,
}

impl Dr13 {
    #[inline(always)]
    pub fn dr13_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn dr13(mut self, val: u32) -> Dr13 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x44u32) as *mut u32) = self.raw; }
    }
}

pub mod dr13 {
    #[inline(always)]
    pub fn read() -> super::Dr13 {
        super::Dr13 {
            raw: unsafe { *((0x40006C00u32 + 0x44u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr13) {
       unsafe { *((0x40006C00u32 + 0x44u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr14 {
   raw: u32,
}

impl Dr14 {
    #[inline(always)]
    pub fn d14_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d14(mut self, val: u32) -> Dr14 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x48u32) as *mut u32) = self.raw; }
    }
}

pub mod dr14 {
    #[inline(always)]
    pub fn read() -> super::Dr14 {
        super::Dr14 {
            raw: unsafe { *((0x40006C00u32 + 0x48u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr14) {
       unsafe { *((0x40006C00u32 + 0x48u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr15 {
   raw: u32,
}

impl Dr15 {
    #[inline(always)]
    pub fn d15_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d15(mut self, val: u32) -> Dr15 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x4Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr15 {
    #[inline(always)]
    pub fn read() -> super::Dr15 {
        super::Dr15 {
            raw: unsafe { *((0x40006C00u32 + 0x4Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr15) {
       unsafe { *((0x40006C00u32 + 0x4Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr16 {
   raw: u32,
}

impl Dr16 {
    #[inline(always)]
    pub fn d16_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d16(mut self, val: u32) -> Dr16 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x50u32) as *mut u32) = self.raw; }
    }
}

pub mod dr16 {
    #[inline(always)]
    pub fn read() -> super::Dr16 {
        super::Dr16 {
            raw: unsafe { *((0x40006C00u32 + 0x50u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr16) {
       unsafe { *((0x40006C00u32 + 0x50u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr17 {
   raw: u32,
}

impl Dr17 {
    #[inline(always)]
    pub fn d17_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d17(mut self, val: u32) -> Dr17 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x54u32) as *mut u32) = self.raw; }
    }
}

pub mod dr17 {
    #[inline(always)]
    pub fn read() -> super::Dr17 {
        super::Dr17 {
            raw: unsafe { *((0x40006C00u32 + 0x54u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr17) {
       unsafe { *((0x40006C00u32 + 0x54u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr18 {
   raw: u32,
}

impl Dr18 {
    #[inline(always)]
    pub fn d18_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d18(mut self, val: u32) -> Dr18 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x58u32) as *mut u32) = self.raw; }
    }
}

pub mod dr18 {
    #[inline(always)]
    pub fn read() -> super::Dr18 {
        super::Dr18 {
            raw: unsafe { *((0x40006C00u32 + 0x58u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr18) {
       unsafe { *((0x40006C00u32 + 0x58u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr19 {
   raw: u32,
}

impl Dr19 {
    #[inline(always)]
    pub fn d19_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d19(mut self, val: u32) -> Dr19 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x5Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr19 {
    #[inline(always)]
    pub fn read() -> super::Dr19 {
        super::Dr19 {
            raw: unsafe { *((0x40006C00u32 + 0x5Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr19) {
       unsafe { *((0x40006C00u32 + 0x5Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr20 {
   raw: u32,
}

impl Dr20 {
    #[inline(always)]
    pub fn d20_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d20(mut self, val: u32) -> Dr20 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x60u32) as *mut u32) = self.raw; }
    }
}

pub mod dr20 {
    #[inline(always)]
    pub fn read() -> super::Dr20 {
        super::Dr20 {
            raw: unsafe { *((0x40006C00u32 + 0x60u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr20) {
       unsafe { *((0x40006C00u32 + 0x60u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr21 {
   raw: u32,
}

impl Dr21 {
    #[inline(always)]
    pub fn d21_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d21(mut self, val: u32) -> Dr21 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x64u32) as *mut u32) = self.raw; }
    }
}

pub mod dr21 {
    #[inline(always)]
    pub fn read() -> super::Dr21 {
        super::Dr21 {
            raw: unsafe { *((0x40006C00u32 + 0x64u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr21) {
       unsafe { *((0x40006C00u32 + 0x64u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr22 {
   raw: u32,
}

impl Dr22 {
    #[inline(always)]
    pub fn d22_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d22(mut self, val: u32) -> Dr22 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x68u32) as *mut u32) = self.raw; }
    }
}

pub mod dr22 {
    #[inline(always)]
    pub fn read() -> super::Dr22 {
        super::Dr22 {
            raw: unsafe { *((0x40006C00u32 + 0x68u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr22) {
       unsafe { *((0x40006C00u32 + 0x68u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr23 {
   raw: u32,
}

impl Dr23 {
    #[inline(always)]
    pub fn d23_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d23(mut self, val: u32) -> Dr23 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x6Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr23 {
    #[inline(always)]
    pub fn read() -> super::Dr23 {
        super::Dr23 {
            raw: unsafe { *((0x40006C00u32 + 0x6Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr23) {
       unsafe { *((0x40006C00u32 + 0x6Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr24 {
   raw: u32,
}

impl Dr24 {
    #[inline(always)]
    pub fn d24_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d24(mut self, val: u32) -> Dr24 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x70u32) as *mut u32) = self.raw; }
    }
}

pub mod dr24 {
    #[inline(always)]
    pub fn read() -> super::Dr24 {
        super::Dr24 {
            raw: unsafe { *((0x40006C00u32 + 0x70u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr24) {
       unsafe { *((0x40006C00u32 + 0x70u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr25 {
   raw: u32,
}

impl Dr25 {
    #[inline(always)]
    pub fn d25_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d25(mut self, val: u32) -> Dr25 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x74u32) as *mut u32) = self.raw; }
    }
}

pub mod dr25 {
    #[inline(always)]
    pub fn read() -> super::Dr25 {
        super::Dr25 {
            raw: unsafe { *((0x40006C00u32 + 0x74u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr25) {
       unsafe { *((0x40006C00u32 + 0x74u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr26 {
   raw: u32,
}

impl Dr26 {
    #[inline(always)]
    pub fn d26_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d26(mut self, val: u32) -> Dr26 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x78u32) as *mut u32) = self.raw; }
    }
}

pub mod dr26 {
    #[inline(always)]
    pub fn read() -> super::Dr26 {
        super::Dr26 {
            raw: unsafe { *((0x40006C00u32 + 0x78u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr26) {
       unsafe { *((0x40006C00u32 + 0x78u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr27 {
   raw: u32,
}

impl Dr27 {
    #[inline(always)]
    pub fn d27_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d27(mut self, val: u32) -> Dr27 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x7Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr27 {
    #[inline(always)]
    pub fn read() -> super::Dr27 {
        super::Dr27 {
            raw: unsafe { *((0x40006C00u32 + 0x7Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr27) {
       unsafe { *((0x40006C00u32 + 0x7Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr28 {
   raw: u32,
}

impl Dr28 {
    #[inline(always)]
    pub fn d28_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d28(mut self, val: u32) -> Dr28 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x80u32) as *mut u32) = self.raw; }
    }
}

pub mod dr28 {
    #[inline(always)]
    pub fn read() -> super::Dr28 {
        super::Dr28 {
            raw: unsafe { *((0x40006C00u32 + 0x80u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr28) {
       unsafe { *((0x40006C00u32 + 0x80u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr29 {
   raw: u32,
}

impl Dr29 {
    #[inline(always)]
    pub fn d29_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d29(mut self, val: u32) -> Dr29 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x84u32) as *mut u32) = self.raw; }
    }
}

pub mod dr29 {
    #[inline(always)]
    pub fn read() -> super::Dr29 {
        super::Dr29 {
            raw: unsafe { *((0x40006C00u32 + 0x84u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr29) {
       unsafe { *((0x40006C00u32 + 0x84u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr30 {
   raw: u32,
}

impl Dr30 {
    #[inline(always)]
    pub fn d30_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d30(mut self, val: u32) -> Dr30 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x88u32) as *mut u32) = self.raw; }
    }
}

pub mod dr30 {
    #[inline(always)]
    pub fn read() -> super::Dr30 {
        super::Dr30 {
            raw: unsafe { *((0x40006C00u32 + 0x88u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr30) {
       unsafe { *((0x40006C00u32 + 0x88u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr31 {
   raw: u32,
}

impl Dr31 {
    #[inline(always)]
    pub fn d31_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d31(mut self, val: u32) -> Dr31 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x8Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr31 {
    #[inline(always)]
    pub fn read() -> super::Dr31 {
        super::Dr31 {
            raw: unsafe { *((0x40006C00u32 + 0x8Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr31) {
       unsafe { *((0x40006C00u32 + 0x8Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr32 {
   raw: u32,
}

impl Dr32 {
    #[inline(always)]
    pub fn d32_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d32(mut self, val: u32) -> Dr32 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x90u32) as *mut u32) = self.raw; }
    }
}

pub mod dr32 {
    #[inline(always)]
    pub fn read() -> super::Dr32 {
        super::Dr32 {
            raw: unsafe { *((0x40006C00u32 + 0x90u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr32) {
       unsafe { *((0x40006C00u32 + 0x90u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr33 {
   raw: u32,
}

impl Dr33 {
    #[inline(always)]
    pub fn d33_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d33(mut self, val: u32) -> Dr33 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x94u32) as *mut u32) = self.raw; }
    }
}

pub mod dr33 {
    #[inline(always)]
    pub fn read() -> super::Dr33 {
        super::Dr33 {
            raw: unsafe { *((0x40006C00u32 + 0x94u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr33) {
       unsafe { *((0x40006C00u32 + 0x94u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr34 {
   raw: u32,
}

impl Dr34 {
    #[inline(always)]
    pub fn d34_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d34(mut self, val: u32) -> Dr34 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x98u32) as *mut u32) = self.raw; }
    }
}

pub mod dr34 {
    #[inline(always)]
    pub fn read() -> super::Dr34 {
        super::Dr34 {
            raw: unsafe { *((0x40006C00u32 + 0x98u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr34) {
       unsafe { *((0x40006C00u32 + 0x98u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr35 {
   raw: u32,
}

impl Dr35 {
    #[inline(always)]
    pub fn d35_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d35(mut self, val: u32) -> Dr35 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x9Cu32) as *mut u32) = self.raw; }
    }
}

pub mod dr35 {
    #[inline(always)]
    pub fn read() -> super::Dr35 {
        super::Dr35 {
            raw: unsafe { *((0x40006C00u32 + 0x9Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr35) {
       unsafe { *((0x40006C00u32 + 0x9Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr36 {
   raw: u32,
}

impl Dr36 {
    #[inline(always)]
    pub fn d36_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d36(mut self, val: u32) -> Dr36 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0xA0u32) as *mut u32) = self.raw; }
    }
}

pub mod dr36 {
    #[inline(always)]
    pub fn read() -> super::Dr36 {
        super::Dr36 {
            raw: unsafe { *((0x40006C00u32 + 0xA0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr36) {
       unsafe { *((0x40006C00u32 + 0xA0u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr37 {
   raw: u32,
}

impl Dr37 {
    #[inline(always)]
    pub fn d37_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d37(mut self, val: u32) -> Dr37 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0xA4u32) as *mut u32) = self.raw; }
    }
}

pub mod dr37 {
    #[inline(always)]
    pub fn read() -> super::Dr37 {
        super::Dr37 {
            raw: unsafe { *((0x40006C00u32 + 0xA4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr37) {
       unsafe { *((0x40006C00u32 + 0xA4u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr38 {
   raw: u32,
}

impl Dr38 {
    #[inline(always)]
    pub fn d38_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d38(mut self, val: u32) -> Dr38 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0xA8u32) as *mut u32) = self.raw; }
    }
}

pub mod dr38 {
    #[inline(always)]
    pub fn read() -> super::Dr38 {
        super::Dr38 {
            raw: unsafe { *((0x40006C00u32 + 0xA8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr38) {
       unsafe { *((0x40006C00u32 + 0xA8u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr39 {
   raw: u32,
}

impl Dr39 {
    #[inline(always)]
    pub fn d39_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d39(mut self, val: u32) -> Dr39 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0xACu32) as *mut u32) = self.raw; }
    }
}

pub mod dr39 {
    #[inline(always)]
    pub fn read() -> super::Dr39 {
        super::Dr39 {
            raw: unsafe { *((0x40006C00u32 + 0xACu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr39) {
       unsafe { *((0x40006C00u32 + 0xACu32) as *mut u32) = val.raw; }
    }
}

pub struct Dr40 {
   raw: u32,
}

impl Dr40 {
    #[inline(always)]
    pub fn d40_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d40(mut self, val: u32) -> Dr40 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0xB0u32) as *mut u32) = self.raw; }
    }
}

pub mod dr40 {
    #[inline(always)]
    pub fn read() -> super::Dr40 {
        super::Dr40 {
            raw: unsafe { *((0x40006C00u32 + 0xB0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr40) {
       unsafe { *((0x40006C00u32 + 0xB0u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr41 {
   raw: u32,
}

impl Dr41 {
    #[inline(always)]
    pub fn d41_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d41(mut self, val: u32) -> Dr41 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0xB4u32) as *mut u32) = self.raw; }
    }
}

pub mod dr41 {
    #[inline(always)]
    pub fn read() -> super::Dr41 {
        super::Dr41 {
            raw: unsafe { *((0x40006C00u32 + 0xB4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr41) {
       unsafe { *((0x40006C00u32 + 0xB4u32) as *mut u32) = val.raw; }
    }
}

pub struct Dr42 {
   raw: u32,
}

impl Dr42 {
    #[inline(always)]
    pub fn d42_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn d42(mut self, val: u32) -> Dr42 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0xB8u32) as *mut u32) = self.raw; }
    }
}

pub mod dr42 {
    #[inline(always)]
    pub fn read() -> super::Dr42 {
        super::Dr42 {
            raw: unsafe { *((0x40006C00u32 + 0xB8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dr42) {
       unsafe { *((0x40006C00u32 + 0xB8u32) as *mut u32) = val.raw; }
    }
}

pub struct Rtccr {
   raw: u32,
}

impl Rtccr {
    #[inline(always)]
    pub fn cal_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn cal(mut self, val: u32) -> Rtccr {
        self.raw = (self.raw & !(((1 << 7) - 1) << 0)) | ((val & ((1 << 7) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn cco_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cco(mut self, val: u32) -> Rtccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn asoe_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn asoe(mut self, val: u32) -> Rtccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn asos_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn asos(mut self, val: u32) -> Rtccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x28u32) as *mut u32) = self.raw; }
    }
}

pub mod rtccr {
    #[inline(always)]
    pub fn read() -> super::Rtccr {
        super::Rtccr {
            raw: unsafe { *((0x40006C00u32 + 0x28u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Rtccr) {
       unsafe { *((0x40006C00u32 + 0x28u32) as *mut u32) = val.raw; }
    }
}

pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn tpe_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tpe(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tpal_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tpal(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x2Cu32) as *mut u32) = self.raw; }
    }
}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0x40006C00u32 + 0x2Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0x40006C00u32 + 0x2Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Csr {
   raw: u32,
}

impl Csr {
    #[inline(always)]
    pub fn cte_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cte(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn cti_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cti(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn tpie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tpie(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn tef_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tef(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn tif_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tif(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006C00u32 + 0x30u32) as *mut u32) = self.raw; }
    }
}

pub mod csr {
    #[inline(always)]
    pub fn read() -> super::Csr {
        super::Csr {
            raw: unsafe { *((0x40006C00u32 + 0x30u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Csr) {
       unsafe { *((0x40006C00u32 + 0x30u32) as *mut u32) = val.raw; }
    }
}

