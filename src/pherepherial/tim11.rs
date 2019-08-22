pub struct Cr1 {
   raw: u32,
}

impl Cr1 {
    #[inline(always)]
    pub fn ckd_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ckd(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn arpe_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn arpe(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn urs_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn urs(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn udis_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn udis(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn cen_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cen(mut self, val: u32) -> Cr1 {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod cr1 {
    #[inline(always)]
    pub fn read() -> super::Cr1 {
        super::Cr1 {
            raw: unsafe { *((0x40015400u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr1) {
       unsafe { *((0x40015400u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Cr2 {
   raw: u32,
}

impl Cr2 {
    #[inline(always)]
    pub fn mms_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn mms(mut self, val: u32) -> Cr2 {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod cr2 {
    #[inline(always)]
    pub fn read() -> super::Cr2 {
        super::Cr2 {
            raw: unsafe { *((0x40015400u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr2) {
       unsafe { *((0x40015400u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Dier {
   raw: u32,
}

impl Dier {
    #[inline(always)]
    pub fn cc1ie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1ie(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn uie_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uie(mut self, val: u32) -> Dier {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0xCu32) as *mut u32) = self.raw; }
    }
}

pub mod dier {
    #[inline(always)]
    pub fn read() -> super::Dier {
        super::Dier {
            raw: unsafe { *((0x40015400u32 + 0xCu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dier) {
       unsafe { *((0x40015400u32 + 0xCu32) as *mut u32) = val.raw; }
    }
}

pub struct Sr {
   raw: u32,
}

impl Sr {
    #[inline(always)]
    pub fn cc1of_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1of(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn cc1if_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1if(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn uif_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uif(mut self, val: u32) -> Sr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x10u32) as *mut u32) = self.raw; }
    }
}

pub mod sr {
    #[inline(always)]
    pub fn read() -> super::Sr {
        super::Sr {
            raw: unsafe { *((0x40015400u32 + 0x10u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr) {
       unsafe { *((0x40015400u32 + 0x10u32) as *mut u32) = val.raw; }
    }
}

pub struct Egr {
   raw: u32,
}

impl Egr {
    #[inline(always)]
    pub fn cc1g_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1g(mut self, val: u32) -> Egr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ug_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ug(mut self, val: u32) -> Egr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x14u32) as *mut u32) = self.raw; }
    }
}

pub mod egr {
    #[inline(always)]
    pub fn read() -> super::Egr {
        super::Egr {
            raw: unsafe { *((0x40015400u32 + 0x14u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Egr) {
       unsafe { *((0x40015400u32 + 0x14u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccmr1_output {
   raw: u32,
}

impl Ccmr1_output {
    #[inline(always)]
    pub fn oc1m_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn oc1m(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 3) - 1) << 4)) | ((val & ((1 << 3) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn oc1pe_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn oc1pe(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn cc1s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc1s(mut self, val: u32) -> Ccmr1_output {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod ccmr1_output {
    #[inline(always)]
    pub fn read() -> super::Ccmr1_output {
        super::Ccmr1_output {
            raw: unsafe { *((0x40015400u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr1_output) {
       unsafe { *((0x40015400u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccmr1_input {
   raw: u32,
}

impl Ccmr1_input {
    #[inline(always)]
    pub fn ic1f_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ic1f(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ic1psc_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ic1psc(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn cc1s_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn cc1s(mut self, val: u32) -> Ccmr1_input {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x18u32) as *mut u32) = self.raw; }
    }
}

pub mod ccmr1_input {
    #[inline(always)]
    pub fn read() -> super::Ccmr1_input {
        super::Ccmr1_input {
            raw: unsafe { *((0x40015400u32 + 0x18u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccmr1_input) {
       unsafe { *((0x40015400u32 + 0x18u32) as *mut u32) = val.raw; }
    }
}

pub struct Ccer {
   raw: u32,
}

impl Ccer {
    #[inline(always)]
    pub fn cc1np_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1np(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn cc1p_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1p(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn cc1e_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cc1e(mut self, val: u32) -> Ccer {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x20u32) as *mut u32) = self.raw; }
    }
}

pub mod ccer {
    #[inline(always)]
    pub fn read() -> super::Ccer {
        super::Ccer {
            raw: unsafe { *((0x40015400u32 + 0x20u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccer) {
       unsafe { *((0x40015400u32 + 0x20u32) as *mut u32) = val.raw; }
    }
}

pub struct Cnt {
   raw: u32,
}

impl Cnt {
    #[inline(always)]
    pub fn cnt_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn cnt(mut self, val: u32) -> Cnt {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x24u32) as *mut u32) = self.raw; }
    }
}

pub mod cnt {
    #[inline(always)]
    pub fn read() -> super::Cnt {
        super::Cnt {
            raw: unsafe { *((0x40015400u32 + 0x24u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cnt) {
       unsafe { *((0x40015400u32 + 0x24u32) as *mut u32) = val.raw; }
    }
}

pub struct Psc {
   raw: u32,
}

impl Psc {
    #[inline(always)]
    pub fn psc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn psc(mut self, val: u32) -> Psc {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x28u32) as *mut u32) = self.raw; }
    }
}

pub mod psc {
    #[inline(always)]
    pub fn read() -> super::Psc {
        super::Psc {
            raw: unsafe { *((0x40015400u32 + 0x28u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Psc) {
       unsafe { *((0x40015400u32 + 0x28u32) as *mut u32) = val.raw; }
    }
}

pub struct Arr {
   raw: u32,
}

impl Arr {
    #[inline(always)]
    pub fn arr_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn arr(mut self, val: u32) -> Arr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x2Cu32) as *mut u32) = self.raw; }
    }
}

pub mod arr {
    #[inline(always)]
    pub fn read() -> super::Arr {
        super::Arr {
            raw: unsafe { *((0x40015400u32 + 0x2Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Arr) {
       unsafe { *((0x40015400u32 + 0x2Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ccr1 {
   raw: u32,
}

impl Ccr1 {
    #[inline(always)]
    pub fn ccr1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn ccr1(mut self, val: u32) -> Ccr1 {
        self.raw = (self.raw & !(((1 << 16) - 1) << 0)) | ((val & ((1 << 16) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40015400u32 + 0x34u32) as *mut u32) = self.raw; }
    }
}

pub mod ccr1 {
    #[inline(always)]
    pub fn read() -> super::Ccr1 {
        super::Ccr1 {
            raw: unsafe { *((0x40015400u32 + 0x34u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr1) {
       unsafe { *((0x40015400u32 + 0x34u32) as *mut u32) = val.raw; }
    }
}

