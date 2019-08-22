pub struct Crl {
   raw: u32,
}

impl Crl {
    #[inline(always)]
    pub fn mode0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode0(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn cnf0_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf0(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn mode1_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode1(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn cnf1_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf1(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 6)) | ((val & ((1 << 2) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn mode2_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode2(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn cnf2_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf2(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn mode3_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode3(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn cnf3_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf3(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 14)) | ((val & ((1 << 2) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn mode4_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode4(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 16)) | ((val & ((1 << 2) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn cnf4_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf4(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mode5_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode5(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn cnf5_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf5(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 22)) | ((val & ((1 << 2) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn mode6_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode6(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 24)) | ((val & ((1 << 2) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn cnf6_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf6(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 26)) | ((val & ((1 << 2) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn mode7_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode7(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn cnf7_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf7(mut self, val: u32) -> Crl {
        self.raw = (self.raw & !(((1 << 2) - 1) << 30)) | ((val & ((1 << 2) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012000u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod crl {
    #[inline(always)]
    pub fn read() -> super::Crl {
        super::Crl {
            raw: unsafe { *((0x40012000u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Crl) {
       unsafe { *((0x40012000u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Crh {
   raw: u32,
}

impl Crh {
    #[inline(always)]
    pub fn mode8_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode8(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn cnf8_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf8(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn mode9_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode9(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn cnf9_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf9(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 6)) | ((val & ((1 << 2) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn mode10_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode10(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn cnf10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf10(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 10)) | ((val & ((1 << 2) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn mode11_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode11(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn cnf11_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf11(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 14)) | ((val & ((1 << 2) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn mode12_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode12(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 16)) | ((val & ((1 << 2) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn cnf12_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf12(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 18)) | ((val & ((1 << 2) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn mode13_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode13(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 20)) | ((val & ((1 << 2) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn cnf13_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf13(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 22)) | ((val & ((1 << 2) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn mode14_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode14(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 24)) | ((val & ((1 << 2) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn cnf14_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf14(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 26)) | ((val & ((1 << 2) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn mode15_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mode15(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn cnf15_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cnf15(mut self, val: u32) -> Crh {
        self.raw = (self.raw & !(((1 << 2) - 1) << 30)) | ((val & ((1 << 2) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012000u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod crh {
    #[inline(always)]
    pub fn read() -> super::Crh {
        super::Crh {
            raw: unsafe { *((0x40012000u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Crh) {
       unsafe { *((0x40012000u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Idr {
   raw: u32,
}

impl Idr {
    #[inline(always)]
    pub fn idr0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr0(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn idr1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr1(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn idr2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr2(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn idr3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr3(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn idr4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr4(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn idr5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr5(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn idr6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr6(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn idr7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr7(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn idr8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr8(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn idr9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr9(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn idr10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr10(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn idr11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr11(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn idr12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr12(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn idr13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr13(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn idr14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr14(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn idr15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn idr15(mut self, val: u32) -> Idr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012000u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod idr {
    #[inline(always)]
    pub fn read() -> super::Idr {
        super::Idr {
            raw: unsafe { *((0x40012000u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Idr) {
       unsafe { *((0x40012000u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Odr {
   raw: u32,
}

impl Odr {
    #[inline(always)]
    pub fn odr0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr0(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn odr1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr1(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn odr2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr2(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn odr3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr3(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn odr4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr4(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn odr5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr5(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn odr6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr6(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn odr7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr7(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn odr8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr8(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn odr9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr9(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn odr10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr10(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn odr11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr11(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn odr12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr12(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn odr13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr13(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn odr14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr14(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn odr15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn odr15(mut self, val: u32) -> Odr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012000u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod odr {
    #[inline(always)]
    pub fn read() -> super::Odr {
        super::Odr {
            raw: unsafe { *((0x40012000u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Odr) {
       unsafe { *((0x40012000u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Bsrr {
   raw: u32,
}

impl Bsrr {
    #[inline(always)]
    pub fn bs0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs0(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn bs1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs1(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn bs2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs2(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn bs3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs3(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn bs4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs4(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn bs5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs5(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn bs6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs6(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn bs7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs7(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn bs8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs8(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn bs9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs9(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn bs10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs10(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn bs11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs11(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn bs12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs12(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn bs13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs13(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn bs14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs14(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn bs15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bs15(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn br0_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br0(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn br1_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br1(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn br2_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br2(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn br3_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br3(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn br4_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br4(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn br5_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br5(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn br6_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br6(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn br7_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br7(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn br8_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br8(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn br9_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br9(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn br10_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br10(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn br11_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br11(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn br12_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br12(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn br13_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br13(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn br14_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br14(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn br15_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br15(mut self, val: u32) -> Bsrr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012000u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod bsrr {
    #[inline(always)]
    pub fn read() -> super::Bsrr {
        super::Bsrr {
            raw: unsafe { *((0x40012000u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bsrr) {
       unsafe { *((0x40012000u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Brr {
   raw: u32,
}

impl Brr {
    #[inline(always)]
    pub fn br0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br0(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn br1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br1(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn br2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br2(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn br3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br3(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn br4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br4(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn br5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br5(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn br6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br6(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn br7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br7(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn br8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br8(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn br9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br9(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn br10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br10(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn br11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br11(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn br12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br12(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn br13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br13(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn br14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br14(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn br15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn br15(mut self, val: u32) -> Brr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012000u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod brr {
    #[inline(always)]
    pub fn read() -> super::Brr {
        super::Brr {
            raw: unsafe { *((0x40012000u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Brr) {
       unsafe { *((0x40012000u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Lckr {
   raw: u32,
}

impl Lckr {
    #[inline(always)]
    pub fn lck0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck0(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn lck1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck1(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn lck2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck2(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn lck3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck3(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn lck4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck4(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn lck5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck5(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn lck6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck6(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn lck7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck7(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn lck8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck8(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn lck9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck9(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn lck10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck10(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn lck11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck11(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn lck12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck12(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn lck13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck13(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn lck14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck14(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn lck15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck15(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lckk_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lckk(mut self, val: u32) -> Lckr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40012000u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod lckr {
    #[inline(always)]
    pub fn read() -> super::Lckr {
        super::Lckr {
            raw: unsafe { *((0x40012000u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Lckr) {
       unsafe { *((0x40012000u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

