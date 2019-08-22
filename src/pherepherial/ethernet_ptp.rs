pub struct Ptptscr {
   raw: u32,
}

impl Ptptscr {
    #[inline(always)]
    pub fn tse_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tse(mut self, val: u32) -> Ptptscr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tsfcu_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsfcu(mut self, val: u32) -> Ptptscr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn tssti_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tssti(mut self, val: u32) -> Ptptscr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn tsstu_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsstu(mut self, val: u32) -> Ptptscr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn tsite_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsite(mut self, val: u32) -> Ptptscr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn tsaru_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsaru(mut self, val: u32) -> Ptptscr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod ptptscr {
    #[inline(always)]
    pub fn read() -> super::Ptptscr {
        super::Ptptscr {
            raw: unsafe { *((0x40028700 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptptscr) {
       unsafe { *((0x40028700 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Ptpssir {
   raw: u32,
}

impl Ptpssir {
    #[inline(always)]
    pub fn stssi_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn stssi(mut self, val: u32) -> Ptpssir {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod ptpssir {
    #[inline(always)]
    pub fn read() -> super::Ptpssir {
        super::Ptpssir {
            raw: unsafe { *((0x40028700 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptpssir) {
       unsafe { *((0x40028700 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Ptptshr {
   raw: u32,
}

impl Ptptshr {
    #[inline(always)]
    pub fn sts_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn sts(mut self, val: u32) -> Ptptshr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod ptptshr {
    #[inline(always)]
    pub fn read() -> super::Ptptshr {
        super::Ptptshr {
            raw: unsafe { *((0x40028700 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptptshr) {
       unsafe { *((0x40028700 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Ptptslr {
   raw: u32,
}

impl Ptptslr {
    #[inline(always)]
    pub fn stss_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 31) - 1)
    }

    #[inline(always)]
    pub fn stss(mut self, val: u32) -> Ptptslr {
        self.raw = (self.raw & !(((1 << 31) - 1) << 0)) | ((val & ((1 << 31) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stpns_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stpns(mut self, val: u32) -> Ptptslr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod ptptslr {
    #[inline(always)]
    pub fn read() -> super::Ptptslr {
        super::Ptptslr {
            raw: unsafe { *((0x40028700 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptptslr) {
       unsafe { *((0x40028700 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Ptptshur {
   raw: u32,
}

impl Ptptshur {
    #[inline(always)]
    pub fn tsus_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn tsus(mut self, val: u32) -> Ptptshur {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod ptptshur {
    #[inline(always)]
    pub fn read() -> super::Ptptshur {
        super::Ptptshur {
            raw: unsafe { *((0x40028700 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptptshur) {
       unsafe { *((0x40028700 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Ptptslur {
   raw: u32,
}

impl Ptptslur {
    #[inline(always)]
    pub fn tsuss_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 31) - 1)
    }

    #[inline(always)]
    pub fn tsuss(mut self, val: u32) -> Ptptslur {
        self.raw = (self.raw & !(((1 << 31) - 1) << 0)) | ((val & ((1 << 31) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tsupns_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tsupns(mut self, val: u32) -> Ptptslur {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod ptptslur {
    #[inline(always)]
    pub fn read() -> super::Ptptslur {
        super::Ptptslur {
            raw: unsafe { *((0x40028700 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptptslur) {
       unsafe { *((0x40028700 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Ptptsar {
   raw: u32,
}

impl Ptptsar {
    #[inline(always)]
    pub fn tsa_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn tsa(mut self, val: u32) -> Ptptsar {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod ptptsar {
    #[inline(always)]
    pub fn read() -> super::Ptptsar {
        super::Ptptsar {
            raw: unsafe { *((0x40028700 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptptsar) {
       unsafe { *((0x40028700 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Ptptthr {
   raw: u32,
}

impl Ptptthr {
    #[inline(always)]
    pub fn ttsh_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ttsh(mut self, val: u32) -> Ptptthr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0x1C) as *mut u32) = self.raw; }
    }
}

pub mod ptptthr {
    #[inline(always)]
    pub fn read() -> super::Ptptthr {
        super::Ptptthr {
            raw: unsafe { *((0x40028700 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptptthr) {
       unsafe { *((0x40028700 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Ptpttlr {
   raw: u32,
}

impl Ptpttlr {
    #[inline(always)]
    pub fn ttsl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn ttsl(mut self, val: u32) -> Ptpttlr {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40028700 + 0x20) as *mut u32) = self.raw; }
    }
}

pub mod ptpttlr {
    #[inline(always)]
    pub fn read() -> super::Ptpttlr {
        super::Ptpttlr {
            raw: unsafe { *((0x40028700 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ptpttlr) {
       unsafe { *((0x40028700 + 0x20) as *mut u32) = val.raw; }
    }
}

