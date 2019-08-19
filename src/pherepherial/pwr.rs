pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn lpds_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lpds_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn pdds_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pdds_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn cwuf_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cwuf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn csbf_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn csbf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn pvde_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pvde_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn pls_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn pls_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 5)) | ((val & ((1 << 3) - 1)) << 5)
    }

    #[inline(always)]
    pub fn dbp_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0x40007000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0x40007000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Csr {
   raw: u32,
}

impl Csr {
    #[inline(always)]
    pub fn wuf_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wuf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn sbf_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sbf_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn pvdo_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pvdo_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn ewup_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ewup_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

}

pub mod csr {
    #[inline(always)]
    pub fn read() -> super::Csr {
        super::Csr {
            raw: unsafe { *((0x40007000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Csr) {
       unsafe { *((0x40007000 + 0x4) as *mut u32) = val.raw; }
    }
}

