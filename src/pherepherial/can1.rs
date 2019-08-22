pub struct Can_mcr {
   raw: u32,
}

impl Can_mcr {
    #[inline(always)]
    pub fn dbf_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbf(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn reset_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn reset(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn ttcm_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ttcm(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn abom_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn abom(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn awum_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn awum(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn nart_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nart(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn rflm_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rflm(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn txfp_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfp(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn sleep_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sleep(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn inrq_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn inrq(mut self, val: u32) -> Can_mcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod can_mcr {
    #[inline(always)]
    pub fn read() -> super::Can_mcr {
        super::Can_mcr {
            raw: unsafe { *((0x40006400u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_mcr) {
       unsafe { *((0x40006400u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_msr {
   raw: u32,
}

impl Can_msr {
    #[inline(always)]
    pub fn rx_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rx(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn samp_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn samp(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn rxm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxm(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn txm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txm(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn slaki_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn slaki(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn wkui_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wkui(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn erri_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn erri(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn slak_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn slak(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn inak_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn inak(mut self, val: u32) -> Can_msr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod can_msr {
    #[inline(always)]
    pub fn read() -> super::Can_msr {
        super::Can_msr {
            raw: unsafe { *((0x40006400u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_msr) {
       unsafe { *((0x40006400u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tsr {
   raw: u32,
}

impl Can_tsr {
    #[inline(always)]
    pub fn low2_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn low2(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn low1_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn low1(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn low0_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn low0(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn tme2_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tme2(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn tme1_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tme1(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn tme0_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tme0(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn code_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn code(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 24)) | ((val & ((1 << 2) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn abrq2_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn abrq2(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn terr2_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn terr2(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn alst2_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn alst2(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn txok2_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txok2(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn rqcp2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rqcp2(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn abrq1_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn abrq1(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn terr1_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn terr1(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn alst1_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn alst1(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn txok1_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txok1(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn rqcp1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rqcp1(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn abrq0_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn abrq0(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn terr0_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn terr0(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn alst0_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn alst0(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn txok0_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txok0(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn rqcp0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rqcp0(mut self, val: u32) -> Can_tsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x8u32) as *mut u32) = self.raw; }
    }
}

pub mod can_tsr {
    #[inline(always)]
    pub fn read() -> super::Can_tsr {
        super::Can_tsr {
            raw: unsafe { *((0x40006400u32 + 0x8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tsr) {
       unsafe { *((0x40006400u32 + 0x8u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_rf0r {
   raw: u32,
}

impl Can_rf0r {
    #[inline(always)]
    pub fn rfom0_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rfom0(mut self, val: u32) -> Can_rf0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fovr0_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fovr0(mut self, val: u32) -> Can_rf0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn full0_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn full0(mut self, val: u32) -> Can_rf0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fmp0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn fmp0(mut self, val: u32) -> Can_rf0r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod can_rf0r {
    #[inline(always)]
    pub fn read() -> super::Can_rf0r {
        super::Can_rf0r {
            raw: unsafe { *((0x40006400u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_rf0r) {
       unsafe { *((0x40006400u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Can_rf1r {
   raw: u32,
}

impl Can_rf1r {
    #[inline(always)]
    pub fn rfom1_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rfom1(mut self, val: u32) -> Can_rf1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fovr1_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fovr1(mut self, val: u32) -> Can_rf1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn full1_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn full1(mut self, val: u32) -> Can_rf1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fmp1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn fmp1(mut self, val: u32) -> Can_rf1r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod can_rf1r {
    #[inline(always)]
    pub fn read() -> super::Can_rf1r {
        super::Can_rf1r {
            raw: unsafe { *((0x40006400u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_rf1r) {
       unsafe { *((0x40006400u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_ier {
   raw: u32,
}

impl Can_ier {
    #[inline(always)]
    pub fn slkie_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn slkie(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn wkuie_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wkuie(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn errie_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn errie(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn lecie_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lecie(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn bofie_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bofie(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn epvie_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epvie(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn ewgie_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ewgie(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fovie1_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fovie1(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ffie1_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffie1(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fmpie1_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fmpie1(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fovie0_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fovie0(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ffie0_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffie0(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fmpie0_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fmpie0(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn tmeie_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tmeie(mut self, val: u32) -> Can_ier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod can_ier {
    #[inline(always)]
    pub fn read() -> super::Can_ier {
        super::Can_ier {
            raw: unsafe { *((0x40006400u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_ier) {
       unsafe { *((0x40006400u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_esr {
   raw: u32,
}

impl Can_esr {
    #[inline(always)]
    pub fn rec_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn rec(mut self, val: u32) -> Can_esr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn tec_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn tec(mut self, val: u32) -> Can_esr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn lec_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn lec(mut self, val: u32) -> Can_esr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn boff_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn boff(mut self, val: u32) -> Can_esr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn epvf_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn epvf(mut self, val: u32) -> Can_esr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ewgf_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ewgf(mut self, val: u32) -> Can_esr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod can_esr {
    #[inline(always)]
    pub fn read() -> super::Can_esr {
        super::Can_esr {
            raw: unsafe { *((0x40006400u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_esr) {
       unsafe { *((0x40006400u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_btr {
   raw: u32,
}

impl Can_btr {
    #[inline(always)]
    pub fn silm_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn silm(mut self, val: u32) -> Can_btr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn lbkm_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lbkm(mut self, val: u32) -> Can_btr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn sjw_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn sjw(mut self, val: u32) -> Can_btr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 24)) | ((val & ((1 << 2) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn ts2_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ts2(mut self, val: u32) -> Can_btr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 20)) | ((val & ((1 << 3) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn ts1_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ts1(mut self, val: u32) -> Can_btr {
        self.raw = (self.raw & !(((1 << 4) - 1) << 16)) | ((val & ((1 << 4) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn brp_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 10) - 1)
    }

    #[inline(always)]
    pub fn brp(mut self, val: u32) -> Can_btr {
        self.raw = (self.raw & !(((1 << 10) - 1) << 0)) | ((val & ((1 << 10) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1Cu32) as *mut u32) = self.raw; }
    }
}

pub mod can_btr {
    #[inline(always)]
    pub fn read() -> super::Can_btr {
        super::Can_btr {
            raw: unsafe { *((0x40006400u32 + 0x1Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_btr) {
       unsafe { *((0x40006400u32 + 0x1Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Can_ti0r {
   raw: u32,
}

impl Can_ti0r {
    #[inline(always)]
    pub fn stid_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn stid(mut self, val: u32) -> Can_ti0r {
        self.raw = (self.raw & !(((1 << 11) - 1) << 21)) | ((val & ((1 << 11) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn exid_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 18) - 1)
    }

    #[inline(always)]
    pub fn exid(mut self, val: u32) -> Can_ti0r {
        self.raw = (self.raw & !(((1 << 18) - 1) << 3)) | ((val & ((1 << 18) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ide_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ide(mut self, val: u32) -> Can_ti0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rtr_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rtr(mut self, val: u32) -> Can_ti0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn txrq_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txrq(mut self, val: u32) -> Can_ti0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x180u32) as *mut u32) = self.raw; }
    }
}

pub mod can_ti0r {
    #[inline(always)]
    pub fn read() -> super::Can_ti0r {
        super::Can_ti0r {
            raw: unsafe { *((0x40006400u32 + 0x180u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_ti0r) {
       unsafe { *((0x40006400u32 + 0x180u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdt0r {
   raw: u32,
}

impl Can_tdt0r {
    #[inline(always)]
    pub fn time_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn time(mut self, val: u32) -> Can_tdt0r {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn tgt_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgt(mut self, val: u32) -> Can_tdt0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dlc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn dlc(mut self, val: u32) -> Can_tdt0r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x184u32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdt0r {
    #[inline(always)]
    pub fn read() -> super::Can_tdt0r {
        super::Can_tdt0r {
            raw: unsafe { *((0x40006400u32 + 0x184u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdt0r) {
       unsafe { *((0x40006400u32 + 0x184u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdl0r {
   raw: u32,
}

impl Can_tdl0r {
    #[inline(always)]
    pub fn data3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data3(mut self, val: u32) -> Can_tdl0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data2(mut self, val: u32) -> Can_tdl0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data1(mut self, val: u32) -> Can_tdl0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data0(mut self, val: u32) -> Can_tdl0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x188u32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdl0r {
    #[inline(always)]
    pub fn read() -> super::Can_tdl0r {
        super::Can_tdl0r {
            raw: unsafe { *((0x40006400u32 + 0x188u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdl0r) {
       unsafe { *((0x40006400u32 + 0x188u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdh0r {
   raw: u32,
}

impl Can_tdh0r {
    #[inline(always)]
    pub fn data7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data7(mut self, val: u32) -> Can_tdh0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data6_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data6(mut self, val: u32) -> Can_tdh0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data5_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data5(mut self, val: u32) -> Can_tdh0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data4(mut self, val: u32) -> Can_tdh0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x18Cu32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdh0r {
    #[inline(always)]
    pub fn read() -> super::Can_tdh0r {
        super::Can_tdh0r {
            raw: unsafe { *((0x40006400u32 + 0x18Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdh0r) {
       unsafe { *((0x40006400u32 + 0x18Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Can_ti1r {
   raw: u32,
}

impl Can_ti1r {
    #[inline(always)]
    pub fn stid_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn stid(mut self, val: u32) -> Can_ti1r {
        self.raw = (self.raw & !(((1 << 11) - 1) << 21)) | ((val & ((1 << 11) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn exid_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 18) - 1)
    }

    #[inline(always)]
    pub fn exid(mut self, val: u32) -> Can_ti1r {
        self.raw = (self.raw & !(((1 << 18) - 1) << 3)) | ((val & ((1 << 18) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ide_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ide(mut self, val: u32) -> Can_ti1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rtr_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rtr(mut self, val: u32) -> Can_ti1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn txrq_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txrq(mut self, val: u32) -> Can_ti1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x190u32) as *mut u32) = self.raw; }
    }
}

pub mod can_ti1r {
    #[inline(always)]
    pub fn read() -> super::Can_ti1r {
        super::Can_ti1r {
            raw: unsafe { *((0x40006400u32 + 0x190u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_ti1r) {
       unsafe { *((0x40006400u32 + 0x190u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdt1r {
   raw: u32,
}

impl Can_tdt1r {
    #[inline(always)]
    pub fn time_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn time(mut self, val: u32) -> Can_tdt1r {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn tgt_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgt(mut self, val: u32) -> Can_tdt1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dlc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn dlc(mut self, val: u32) -> Can_tdt1r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x194u32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdt1r {
    #[inline(always)]
    pub fn read() -> super::Can_tdt1r {
        super::Can_tdt1r {
            raw: unsafe { *((0x40006400u32 + 0x194u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdt1r) {
       unsafe { *((0x40006400u32 + 0x194u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdl1r {
   raw: u32,
}

impl Can_tdl1r {
    #[inline(always)]
    pub fn data3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data3(mut self, val: u32) -> Can_tdl1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data2(mut self, val: u32) -> Can_tdl1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data1(mut self, val: u32) -> Can_tdl1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data0(mut self, val: u32) -> Can_tdl1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x198u32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdl1r {
    #[inline(always)]
    pub fn read() -> super::Can_tdl1r {
        super::Can_tdl1r {
            raw: unsafe { *((0x40006400u32 + 0x198u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdl1r) {
       unsafe { *((0x40006400u32 + 0x198u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdh1r {
   raw: u32,
}

impl Can_tdh1r {
    #[inline(always)]
    pub fn data7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data7(mut self, val: u32) -> Can_tdh1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data6_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data6(mut self, val: u32) -> Can_tdh1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data5_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data5(mut self, val: u32) -> Can_tdh1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data4(mut self, val: u32) -> Can_tdh1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x19Cu32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdh1r {
    #[inline(always)]
    pub fn read() -> super::Can_tdh1r {
        super::Can_tdh1r {
            raw: unsafe { *((0x40006400u32 + 0x19Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdh1r) {
       unsafe { *((0x40006400u32 + 0x19Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Can_ti2r {
   raw: u32,
}

impl Can_ti2r {
    #[inline(always)]
    pub fn stid_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn stid(mut self, val: u32) -> Can_ti2r {
        self.raw = (self.raw & !(((1 << 11) - 1) << 21)) | ((val & ((1 << 11) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn exid_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 18) - 1)
    }

    #[inline(always)]
    pub fn exid(mut self, val: u32) -> Can_ti2r {
        self.raw = (self.raw & !(((1 << 18) - 1) << 3)) | ((val & ((1 << 18) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ide_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ide(mut self, val: u32) -> Can_ti2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rtr_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rtr(mut self, val: u32) -> Can_ti2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn txrq_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txrq(mut self, val: u32) -> Can_ti2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1A0u32) as *mut u32) = self.raw; }
    }
}

pub mod can_ti2r {
    #[inline(always)]
    pub fn read() -> super::Can_ti2r {
        super::Can_ti2r {
            raw: unsafe { *((0x40006400u32 + 0x1A0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_ti2r) {
       unsafe { *((0x40006400u32 + 0x1A0u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdt2r {
   raw: u32,
}

impl Can_tdt2r {
    #[inline(always)]
    pub fn time_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn time(mut self, val: u32) -> Can_tdt2r {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn tgt_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tgt(mut self, val: u32) -> Can_tdt2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dlc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn dlc(mut self, val: u32) -> Can_tdt2r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1A4u32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdt2r {
    #[inline(always)]
    pub fn read() -> super::Can_tdt2r {
        super::Can_tdt2r {
            raw: unsafe { *((0x40006400u32 + 0x1A4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdt2r) {
       unsafe { *((0x40006400u32 + 0x1A4u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdl2r {
   raw: u32,
}

impl Can_tdl2r {
    #[inline(always)]
    pub fn data3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data3(mut self, val: u32) -> Can_tdl2r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data2(mut self, val: u32) -> Can_tdl2r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data1(mut self, val: u32) -> Can_tdl2r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data0(mut self, val: u32) -> Can_tdl2r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1A8u32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdl2r {
    #[inline(always)]
    pub fn read() -> super::Can_tdl2r {
        super::Can_tdl2r {
            raw: unsafe { *((0x40006400u32 + 0x1A8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdl2r) {
       unsafe { *((0x40006400u32 + 0x1A8u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_tdh2r {
   raw: u32,
}

impl Can_tdh2r {
    #[inline(always)]
    pub fn data7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data7(mut self, val: u32) -> Can_tdh2r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data6_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data6(mut self, val: u32) -> Can_tdh2r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data5_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data5(mut self, val: u32) -> Can_tdh2r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data4(mut self, val: u32) -> Can_tdh2r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1ACu32) as *mut u32) = self.raw; }
    }
}

pub mod can_tdh2r {
    #[inline(always)]
    pub fn read() -> super::Can_tdh2r {
        super::Can_tdh2r {
            raw: unsafe { *((0x40006400u32 + 0x1ACu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_tdh2r) {
       unsafe { *((0x40006400u32 + 0x1ACu32) as *mut u32) = val.raw; }
    }
}

pub struct Can_ri0r {
   raw: u32,
}

impl Can_ri0r {
    #[inline(always)]
    pub fn stid_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn stid(mut self, val: u32) -> Can_ri0r {
        self.raw = (self.raw & !(((1 << 11) - 1) << 21)) | ((val & ((1 << 11) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn exid_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 18) - 1)
    }

    #[inline(always)]
    pub fn exid(mut self, val: u32) -> Can_ri0r {
        self.raw = (self.raw & !(((1 << 18) - 1) << 3)) | ((val & ((1 << 18) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ide_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ide(mut self, val: u32) -> Can_ri0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rtr_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rtr(mut self, val: u32) -> Can_ri0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1B0u32) as *mut u32) = self.raw; }
    }
}

pub mod can_ri0r {
    #[inline(always)]
    pub fn read() -> super::Can_ri0r {
        super::Can_ri0r {
            raw: unsafe { *((0x40006400u32 + 0x1B0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_ri0r) {
       unsafe { *((0x40006400u32 + 0x1B0u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_rdt0r {
   raw: u32,
}

impl Can_rdt0r {
    #[inline(always)]
    pub fn time_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn time(mut self, val: u32) -> Can_rdt0r {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fmi_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn fmi(mut self, val: u32) -> Can_rdt0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dlc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn dlc(mut self, val: u32) -> Can_rdt0r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1B4u32) as *mut u32) = self.raw; }
    }
}

pub mod can_rdt0r {
    #[inline(always)]
    pub fn read() -> super::Can_rdt0r {
        super::Can_rdt0r {
            raw: unsafe { *((0x40006400u32 + 0x1B4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_rdt0r) {
       unsafe { *((0x40006400u32 + 0x1B4u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_rdl0r {
   raw: u32,
}

impl Can_rdl0r {
    #[inline(always)]
    pub fn data3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data3(mut self, val: u32) -> Can_rdl0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data2(mut self, val: u32) -> Can_rdl0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data1(mut self, val: u32) -> Can_rdl0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data0(mut self, val: u32) -> Can_rdl0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1B8u32) as *mut u32) = self.raw; }
    }
}

pub mod can_rdl0r {
    #[inline(always)]
    pub fn read() -> super::Can_rdl0r {
        super::Can_rdl0r {
            raw: unsafe { *((0x40006400u32 + 0x1B8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_rdl0r) {
       unsafe { *((0x40006400u32 + 0x1B8u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_rdh0r {
   raw: u32,
}

impl Can_rdh0r {
    #[inline(always)]
    pub fn data7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data7(mut self, val: u32) -> Can_rdh0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data6_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data6(mut self, val: u32) -> Can_rdh0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data5_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data5(mut self, val: u32) -> Can_rdh0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data4(mut self, val: u32) -> Can_rdh0r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1BCu32) as *mut u32) = self.raw; }
    }
}

pub mod can_rdh0r {
    #[inline(always)]
    pub fn read() -> super::Can_rdh0r {
        super::Can_rdh0r {
            raw: unsafe { *((0x40006400u32 + 0x1BCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_rdh0r) {
       unsafe { *((0x40006400u32 + 0x1BCu32) as *mut u32) = val.raw; }
    }
}

pub struct Can_ri1r {
   raw: u32,
}

impl Can_ri1r {
    #[inline(always)]
    pub fn stid_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn stid(mut self, val: u32) -> Can_ri1r {
        self.raw = (self.raw & !(((1 << 11) - 1) << 21)) | ((val & ((1 << 11) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn exid_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 18) - 1)
    }

    #[inline(always)]
    pub fn exid(mut self, val: u32) -> Can_ri1r {
        self.raw = (self.raw & !(((1 << 18) - 1) << 3)) | ((val & ((1 << 18) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ide_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ide(mut self, val: u32) -> Can_ri1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rtr_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rtr(mut self, val: u32) -> Can_ri1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1C0u32) as *mut u32) = self.raw; }
    }
}

pub mod can_ri1r {
    #[inline(always)]
    pub fn read() -> super::Can_ri1r {
        super::Can_ri1r {
            raw: unsafe { *((0x40006400u32 + 0x1C0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_ri1r) {
       unsafe { *((0x40006400u32 + 0x1C0u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_rdt1r {
   raw: u32,
}

impl Can_rdt1r {
    #[inline(always)]
    pub fn time_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn time(mut self, val: u32) -> Can_rdt1r {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fmi_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn fmi(mut self, val: u32) -> Can_rdt1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dlc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn dlc(mut self, val: u32) -> Can_rdt1r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1C4u32) as *mut u32) = self.raw; }
    }
}

pub mod can_rdt1r {
    #[inline(always)]
    pub fn read() -> super::Can_rdt1r {
        super::Can_rdt1r {
            raw: unsafe { *((0x40006400u32 + 0x1C4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_rdt1r) {
       unsafe { *((0x40006400u32 + 0x1C4u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_rdl1r {
   raw: u32,
}

impl Can_rdl1r {
    #[inline(always)]
    pub fn data3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data3(mut self, val: u32) -> Can_rdl1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data2(mut self, val: u32) -> Can_rdl1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data1(mut self, val: u32) -> Can_rdl1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data0(mut self, val: u32) -> Can_rdl1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1C8u32) as *mut u32) = self.raw; }
    }
}

pub mod can_rdl1r {
    #[inline(always)]
    pub fn read() -> super::Can_rdl1r {
        super::Can_rdl1r {
            raw: unsafe { *((0x40006400u32 + 0x1C8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_rdl1r) {
       unsafe { *((0x40006400u32 + 0x1C8u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_rdh1r {
   raw: u32,
}

impl Can_rdh1r {
    #[inline(always)]
    pub fn data7_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data7(mut self, val: u32) -> Can_rdh1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn data6_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data6(mut self, val: u32) -> Can_rdh1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn data5_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data5(mut self, val: u32) -> Can_rdh1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn data4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn data4(mut self, val: u32) -> Can_rdh1r {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x1CCu32) as *mut u32) = self.raw; }
    }
}

pub mod can_rdh1r {
    #[inline(always)]
    pub fn read() -> super::Can_rdh1r {
        super::Can_rdh1r {
            raw: unsafe { *((0x40006400u32 + 0x1CCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_rdh1r) {
       unsafe { *((0x40006400u32 + 0x1CCu32) as *mut u32) = val.raw; }
    }
}

pub struct Can_fmr {
   raw: u32,
}

impl Can_fmr {
    #[inline(always)]
    pub fn finit_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn finit(mut self, val: u32) -> Can_fmr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x200u32) as *mut u32) = self.raw; }
    }
}

pub mod can_fmr {
    #[inline(always)]
    pub fn read() -> super::Can_fmr {
        super::Can_fmr {
            raw: unsafe { *((0x40006400u32 + 0x200u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_fmr) {
       unsafe { *((0x40006400u32 + 0x200u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_fm1r {
   raw: u32,
}

impl Can_fm1r {
    #[inline(always)]
    pub fn fbm0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm0(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fbm1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm1(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fbm2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm2(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fbm3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm3(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fbm4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm4(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fbm5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm5(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fbm6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm6(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fbm7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm7(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fbm8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm8(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fbm9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm9(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fbm10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm10(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fbm11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm11(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fbm12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm12(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fbm13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fbm13(mut self, val: u32) -> Can_fm1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x204u32) as *mut u32) = self.raw; }
    }
}

pub mod can_fm1r {
    #[inline(always)]
    pub fn read() -> super::Can_fm1r {
        super::Can_fm1r {
            raw: unsafe { *((0x40006400u32 + 0x204u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_fm1r) {
       unsafe { *((0x40006400u32 + 0x204u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_fs1r {
   raw: u32,
}

impl Can_fs1r {
    #[inline(always)]
    pub fn fsc0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc0(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fsc1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc1(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fsc2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc2(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fsc3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc3(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fsc4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc4(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fsc5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc5(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fsc6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc6(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fsc7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc7(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fsc8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc8(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fsc9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc9(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fsc10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc10(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fsc11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc11(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fsc12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc12(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fsc13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsc13(mut self, val: u32) -> Can_fs1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x20Cu32) as *mut u32) = self.raw; }
    }
}

pub mod can_fs1r {
    #[inline(always)]
    pub fn read() -> super::Can_fs1r {
        super::Can_fs1r {
            raw: unsafe { *((0x40006400u32 + 0x20Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_fs1r) {
       unsafe { *((0x40006400u32 + 0x20Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Can_ffa1r {
   raw: u32,
}

impl Can_ffa1r {
    #[inline(always)]
    pub fn ffa0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa0(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ffa1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa1(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ffa2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa2(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn ffa3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa3(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn ffa4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa4(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ffa5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa5(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn ffa6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa6(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ffa7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa7(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ffa8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa8(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ffa9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa9(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn ffa10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa10(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn ffa11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa11(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn ffa12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa12(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn ffa13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ffa13(mut self, val: u32) -> Can_ffa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x214u32) as *mut u32) = self.raw; }
    }
}

pub mod can_ffa1r {
    #[inline(always)]
    pub fn read() -> super::Can_ffa1r {
        super::Can_ffa1r {
            raw: unsafe { *((0x40006400u32 + 0x214u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_ffa1r) {
       unsafe { *((0x40006400u32 + 0x214u32) as *mut u32) = val.raw; }
    }
}

pub struct Can_fa1r {
   raw: u32,
}

impl Can_fa1r {
    #[inline(always)]
    pub fn fact0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact0(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fact1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact1(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fact2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact2(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fact3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact3(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fact4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact4(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fact5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact5(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fact6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact6(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fact7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact7(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fact8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact8(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fact9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact9(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fact10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact10(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fact11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact11(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fact12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact12(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fact13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fact13(mut self, val: u32) -> Can_fa1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x21Cu32) as *mut u32) = self.raw; }
    }
}

pub mod can_fa1r {
    #[inline(always)]
    pub fn read() -> super::Can_fa1r {
        super::Can_fa1r {
            raw: unsafe { *((0x40006400u32 + 0x21Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Can_fa1r) {
       unsafe { *((0x40006400u32 + 0x21Cu32) as *mut u32) = val.raw; }
    }
}

pub struct F0r1 {
   raw: u32,
}

impl F0r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F0r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x240u32) as *mut u32) = self.raw; }
    }
}

pub mod f0r1 {
    #[inline(always)]
    pub fn read() -> super::F0r1 {
        super::F0r1 {
            raw: unsafe { *((0x40006400u32 + 0x240u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F0r1) {
       unsafe { *((0x40006400u32 + 0x240u32) as *mut u32) = val.raw; }
    }
}

pub struct F0r2 {
   raw: u32,
}

impl F0r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F0r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x244u32) as *mut u32) = self.raw; }
    }
}

pub mod f0r2 {
    #[inline(always)]
    pub fn read() -> super::F0r2 {
        super::F0r2 {
            raw: unsafe { *((0x40006400u32 + 0x244u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F0r2) {
       unsafe { *((0x40006400u32 + 0x244u32) as *mut u32) = val.raw; }
    }
}

pub struct F1r1 {
   raw: u32,
}

impl F1r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F1r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x248u32) as *mut u32) = self.raw; }
    }
}

pub mod f1r1 {
    #[inline(always)]
    pub fn read() -> super::F1r1 {
        super::F1r1 {
            raw: unsafe { *((0x40006400u32 + 0x248u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F1r1) {
       unsafe { *((0x40006400u32 + 0x248u32) as *mut u32) = val.raw; }
    }
}

pub struct F1r2 {
   raw: u32,
}

impl F1r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F1r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x24Cu32) as *mut u32) = self.raw; }
    }
}

pub mod f1r2 {
    #[inline(always)]
    pub fn read() -> super::F1r2 {
        super::F1r2 {
            raw: unsafe { *((0x40006400u32 + 0x24Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F1r2) {
       unsafe { *((0x40006400u32 + 0x24Cu32) as *mut u32) = val.raw; }
    }
}

pub struct F2r1 {
   raw: u32,
}

impl F2r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F2r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x250u32) as *mut u32) = self.raw; }
    }
}

pub mod f2r1 {
    #[inline(always)]
    pub fn read() -> super::F2r1 {
        super::F2r1 {
            raw: unsafe { *((0x40006400u32 + 0x250u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F2r1) {
       unsafe { *((0x40006400u32 + 0x250u32) as *mut u32) = val.raw; }
    }
}

pub struct F2r2 {
   raw: u32,
}

impl F2r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F2r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x254u32) as *mut u32) = self.raw; }
    }
}

pub mod f2r2 {
    #[inline(always)]
    pub fn read() -> super::F2r2 {
        super::F2r2 {
            raw: unsafe { *((0x40006400u32 + 0x254u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F2r2) {
       unsafe { *((0x40006400u32 + 0x254u32) as *mut u32) = val.raw; }
    }
}

pub struct F3r1 {
   raw: u32,
}

impl F3r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F3r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x258u32) as *mut u32) = self.raw; }
    }
}

pub mod f3r1 {
    #[inline(always)]
    pub fn read() -> super::F3r1 {
        super::F3r1 {
            raw: unsafe { *((0x40006400u32 + 0x258u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F3r1) {
       unsafe { *((0x40006400u32 + 0x258u32) as *mut u32) = val.raw; }
    }
}

pub struct F3r2 {
   raw: u32,
}

impl F3r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F3r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x25Cu32) as *mut u32) = self.raw; }
    }
}

pub mod f3r2 {
    #[inline(always)]
    pub fn read() -> super::F3r2 {
        super::F3r2 {
            raw: unsafe { *((0x40006400u32 + 0x25Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F3r2) {
       unsafe { *((0x40006400u32 + 0x25Cu32) as *mut u32) = val.raw; }
    }
}

pub struct F4r1 {
   raw: u32,
}

impl F4r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F4r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x260u32) as *mut u32) = self.raw; }
    }
}

pub mod f4r1 {
    #[inline(always)]
    pub fn read() -> super::F4r1 {
        super::F4r1 {
            raw: unsafe { *((0x40006400u32 + 0x260u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F4r1) {
       unsafe { *((0x40006400u32 + 0x260u32) as *mut u32) = val.raw; }
    }
}

pub struct F4r2 {
   raw: u32,
}

impl F4r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F4r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x264u32) as *mut u32) = self.raw; }
    }
}

pub mod f4r2 {
    #[inline(always)]
    pub fn read() -> super::F4r2 {
        super::F4r2 {
            raw: unsafe { *((0x40006400u32 + 0x264u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F4r2) {
       unsafe { *((0x40006400u32 + 0x264u32) as *mut u32) = val.raw; }
    }
}

pub struct F5r1 {
   raw: u32,
}

impl F5r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F5r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x268u32) as *mut u32) = self.raw; }
    }
}

pub mod f5r1 {
    #[inline(always)]
    pub fn read() -> super::F5r1 {
        super::F5r1 {
            raw: unsafe { *((0x40006400u32 + 0x268u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F5r1) {
       unsafe { *((0x40006400u32 + 0x268u32) as *mut u32) = val.raw; }
    }
}

pub struct F5r2 {
   raw: u32,
}

impl F5r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F5r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x26Cu32) as *mut u32) = self.raw; }
    }
}

pub mod f5r2 {
    #[inline(always)]
    pub fn read() -> super::F5r2 {
        super::F5r2 {
            raw: unsafe { *((0x40006400u32 + 0x26Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F5r2) {
       unsafe { *((0x40006400u32 + 0x26Cu32) as *mut u32) = val.raw; }
    }
}

pub struct F6r1 {
   raw: u32,
}

impl F6r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F6r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x270u32) as *mut u32) = self.raw; }
    }
}

pub mod f6r1 {
    #[inline(always)]
    pub fn read() -> super::F6r1 {
        super::F6r1 {
            raw: unsafe { *((0x40006400u32 + 0x270u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F6r1) {
       unsafe { *((0x40006400u32 + 0x270u32) as *mut u32) = val.raw; }
    }
}

pub struct F6r2 {
   raw: u32,
}

impl F6r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F6r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x274u32) as *mut u32) = self.raw; }
    }
}

pub mod f6r2 {
    #[inline(always)]
    pub fn read() -> super::F6r2 {
        super::F6r2 {
            raw: unsafe { *((0x40006400u32 + 0x274u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F6r2) {
       unsafe { *((0x40006400u32 + 0x274u32) as *mut u32) = val.raw; }
    }
}

pub struct F7r1 {
   raw: u32,
}

impl F7r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F7r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x278u32) as *mut u32) = self.raw; }
    }
}

pub mod f7r1 {
    #[inline(always)]
    pub fn read() -> super::F7r1 {
        super::F7r1 {
            raw: unsafe { *((0x40006400u32 + 0x278u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F7r1) {
       unsafe { *((0x40006400u32 + 0x278u32) as *mut u32) = val.raw; }
    }
}

pub struct F7r2 {
   raw: u32,
}

impl F7r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F7r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x27Cu32) as *mut u32) = self.raw; }
    }
}

pub mod f7r2 {
    #[inline(always)]
    pub fn read() -> super::F7r2 {
        super::F7r2 {
            raw: unsafe { *((0x40006400u32 + 0x27Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F7r2) {
       unsafe { *((0x40006400u32 + 0x27Cu32) as *mut u32) = val.raw; }
    }
}

pub struct F8r1 {
   raw: u32,
}

impl F8r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F8r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x280u32) as *mut u32) = self.raw; }
    }
}

pub mod f8r1 {
    #[inline(always)]
    pub fn read() -> super::F8r1 {
        super::F8r1 {
            raw: unsafe { *((0x40006400u32 + 0x280u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F8r1) {
       unsafe { *((0x40006400u32 + 0x280u32) as *mut u32) = val.raw; }
    }
}

pub struct F8r2 {
   raw: u32,
}

impl F8r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F8r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x284u32) as *mut u32) = self.raw; }
    }
}

pub mod f8r2 {
    #[inline(always)]
    pub fn read() -> super::F8r2 {
        super::F8r2 {
            raw: unsafe { *((0x40006400u32 + 0x284u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F8r2) {
       unsafe { *((0x40006400u32 + 0x284u32) as *mut u32) = val.raw; }
    }
}

pub struct F9r1 {
   raw: u32,
}

impl F9r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F9r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x288u32) as *mut u32) = self.raw; }
    }
}

pub mod f9r1 {
    #[inline(always)]
    pub fn read() -> super::F9r1 {
        super::F9r1 {
            raw: unsafe { *((0x40006400u32 + 0x288u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F9r1) {
       unsafe { *((0x40006400u32 + 0x288u32) as *mut u32) = val.raw; }
    }
}

pub struct F9r2 {
   raw: u32,
}

impl F9r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F9r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x28Cu32) as *mut u32) = self.raw; }
    }
}

pub mod f9r2 {
    #[inline(always)]
    pub fn read() -> super::F9r2 {
        super::F9r2 {
            raw: unsafe { *((0x40006400u32 + 0x28Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F9r2) {
       unsafe { *((0x40006400u32 + 0x28Cu32) as *mut u32) = val.raw; }
    }
}

pub struct F10r1 {
   raw: u32,
}

impl F10r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F10r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x290u32) as *mut u32) = self.raw; }
    }
}

pub mod f10r1 {
    #[inline(always)]
    pub fn read() -> super::F10r1 {
        super::F10r1 {
            raw: unsafe { *((0x40006400u32 + 0x290u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F10r1) {
       unsafe { *((0x40006400u32 + 0x290u32) as *mut u32) = val.raw; }
    }
}

pub struct F10r2 {
   raw: u32,
}

impl F10r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F10r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x294u32) as *mut u32) = self.raw; }
    }
}

pub mod f10r2 {
    #[inline(always)]
    pub fn read() -> super::F10r2 {
        super::F10r2 {
            raw: unsafe { *((0x40006400u32 + 0x294u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F10r2) {
       unsafe { *((0x40006400u32 + 0x294u32) as *mut u32) = val.raw; }
    }
}

pub struct F11r1 {
   raw: u32,
}

impl F11r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F11r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x298u32) as *mut u32) = self.raw; }
    }
}

pub mod f11r1 {
    #[inline(always)]
    pub fn read() -> super::F11r1 {
        super::F11r1 {
            raw: unsafe { *((0x40006400u32 + 0x298u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F11r1) {
       unsafe { *((0x40006400u32 + 0x298u32) as *mut u32) = val.raw; }
    }
}

pub struct F11r2 {
   raw: u32,
}

impl F11r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F11r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x29Cu32) as *mut u32) = self.raw; }
    }
}

pub mod f11r2 {
    #[inline(always)]
    pub fn read() -> super::F11r2 {
        super::F11r2 {
            raw: unsafe { *((0x40006400u32 + 0x29Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F11r2) {
       unsafe { *((0x40006400u32 + 0x29Cu32) as *mut u32) = val.raw; }
    }
}

pub struct F12r1 {
   raw: u32,
}

impl F12r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F12r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x2A0u32) as *mut u32) = self.raw; }
    }
}

pub mod f12r1 {
    #[inline(always)]
    pub fn read() -> super::F12r1 {
        super::F12r1 {
            raw: unsafe { *((0x40006400u32 + 0x2A0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F12r1) {
       unsafe { *((0x40006400u32 + 0x2A0u32) as *mut u32) = val.raw; }
    }
}

pub struct F12r2 {
   raw: u32,
}

impl F12r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F12r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x2A4u32) as *mut u32) = self.raw; }
    }
}

pub mod f12r2 {
    #[inline(always)]
    pub fn read() -> super::F12r2 {
        super::F12r2 {
            raw: unsafe { *((0x40006400u32 + 0x2A4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F12r2) {
       unsafe { *((0x40006400u32 + 0x2A4u32) as *mut u32) = val.raw; }
    }
}

pub struct F13r1 {
   raw: u32,
}

impl F13r1 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F13r1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x2A8u32) as *mut u32) = self.raw; }
    }
}

pub mod f13r1 {
    #[inline(always)]
    pub fn read() -> super::F13r1 {
        super::F13r1 {
            raw: unsafe { *((0x40006400u32 + 0x2A8u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F13r1) {
       unsafe { *((0x40006400u32 + 0x2A8u32) as *mut u32) = val.raw; }
    }
}

pub struct F13r2 {
   raw: u32,
}

impl F13r2 {
    #[inline(always)]
    pub fn fb0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb0(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn fb1_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb1(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn fb2_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb2(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fb3_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb3(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn fb4_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb4(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn fb5_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb5(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn fb6_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb6(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fb7_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb7(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn fb8_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb8(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn fb9_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb9(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn fb10_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb10(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn fb11_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb11(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn fb12_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb12(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn fb13_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb13(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn fb14_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb14(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn fb15_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb15(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn fb16_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb16(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn fb17_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb17(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn fb18_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb18(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn fb19_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb19(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn fb20_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb20(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn fb21_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb21(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn fb22_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb22(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn fb23_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb23(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn fb24_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb24(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn fb25_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb25(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn fb26_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb26(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn fb27_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb27(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn fb28_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb28(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn fb29_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb29(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn fb30_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb30(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn fb31_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fb31(mut self, val: u32) -> F13r2 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40006400u32 + 0x2ACu32) as *mut u32) = self.raw; }
    }
}

pub mod f13r2 {
    #[inline(always)]
    pub fn read() -> super::F13r2 {
        super::F13r2 {
            raw: unsafe { *((0x40006400u32 + 0x2ACu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::F13r2) {
       unsafe { *((0x40006400u32 + 0x2ACu32) as *mut u32) = val.raw; }
    }
}

