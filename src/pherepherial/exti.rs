pub struct Imr {
   raw: u32,
}

impl Imr {
    #[inline(always)]
    pub fn mr0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr0_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn mr1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn mr2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn mr3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn mr4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn mr5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn mr6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn mr7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn mr8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr8_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn mr9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr9_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn mr10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr10_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn mr11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn mr12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr12_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mr13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr13_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn mr14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn mr15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn mr16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr16_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn mr17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr17_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn mr18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr18_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

}

pub mod imr {
    #[inline(always)]
    pub fn read() -> super::Imr {
        super::Imr {
            raw: unsafe { *((0x40010400 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Imr) {
       unsafe { *((0x40010400 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Emr {
   raw: u32,
}

impl Emr {
    #[inline(always)]
    pub fn mr0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr0_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn mr1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn mr2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn mr3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn mr4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn mr5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn mr6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn mr7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn mr8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr8_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn mr9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr9_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn mr10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr10_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn mr11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn mr12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr12_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn mr13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr13_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn mr14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn mr15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn mr16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr16_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn mr17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr17_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn mr18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mr18_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

}

pub mod emr {
    #[inline(always)]
    pub fn read() -> super::Emr {
        super::Emr {
            raw: unsafe { *((0x40010400 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Emr) {
       unsafe { *((0x40010400 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Rtsr {
   raw: u32,
}

impl Rtsr {
    #[inline(always)]
    pub fn tr0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr0_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tr1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn tr2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn tr3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn tr4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn tr5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn tr6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn tr7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn tr8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr8_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn tr9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr9_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn tr10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr10_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn tr11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn tr12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr12_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn tr13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr13_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn tr14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn tr15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn tr16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr16_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn tr17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr17_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn tr18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr18_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

}

pub mod rtsr {
    #[inline(always)]
    pub fn read() -> super::Rtsr {
        super::Rtsr {
            raw: unsafe { *((0x40010400 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Rtsr) {
       unsafe { *((0x40010400 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Ftsr {
   raw: u32,
}

impl Ftsr {
    #[inline(always)]
    pub fn tr0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr0_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn tr1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn tr2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn tr3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn tr4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn tr5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn tr6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn tr7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn tr8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr8_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn tr9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr9_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn tr10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr10_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn tr11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn tr12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr12_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn tr13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr13_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn tr14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn tr15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn tr16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr16_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn tr17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr17_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn tr18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tr18_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

}

pub mod ftsr {
    #[inline(always)]
    pub fn read() -> super::Ftsr {
        super::Ftsr {
            raw: unsafe { *((0x40010400 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ftsr) {
       unsafe { *((0x40010400 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Swier {
   raw: u32,
}

impl Swier {
    #[inline(always)]
    pub fn swier0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier0_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn swier1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn swier2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn swier3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn swier4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn swier5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn swier6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn swier7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn swier8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier8_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn swier9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier9_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn swier10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier10_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn swier11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn swier12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier12_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn swier13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier13_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn swier14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn swier15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn swier16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier16_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn swier17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier17_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn swier18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn swier18_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

}

pub mod swier {
    #[inline(always)]
    pub fn read() -> super::Swier {
        super::Swier {
            raw: unsafe { *((0x40010400 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Swier) {
       unsafe { *((0x40010400 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Pr {
   raw: u32,
}

impl Pr {
    #[inline(always)]
    pub fn pr0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr0_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn pr1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr1_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn pr2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr2_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn pr3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr3_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn pr4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn pr5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn pr6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn pr7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr7_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn pr8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr8_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn pr9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr9_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn pr10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr10_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn pr11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn pr12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr12_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn pr13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr13_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn pr14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn pr15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn pr16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr16_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn pr17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr17_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn pr18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pr18_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

}

pub mod pr {
    #[inline(always)]
    pub fn read() -> super::Pr {
        super::Pr {
            raw: unsafe { *((0x40010400 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pr) {
       unsafe { *((0x40010400 + 0x14) as *mut u32) = val.raw; }
    }
}

