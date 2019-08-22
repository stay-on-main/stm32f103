pub struct Ctrl {
   raw: u32,
}

impl Ctrl {
    #[inline(always)]
    pub fn enable_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn enable(mut self, val: u32) -> Ctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tickint_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tickint(mut self, val: u32) -> Ctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn clksource_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn clksource(mut self, val: u32) -> Ctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn countflag_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn countflag(mut self, val: u32) -> Ctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E010 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod ctrl {
    #[inline(always)]
    pub fn read() -> super::Ctrl {
        super::Ctrl {
            raw: unsafe { *((0xE000E010 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ctrl) {
       unsafe { *((0xE000E010 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Load_ {
   raw: u32,
}

impl Load_ {
    #[inline(always)]
    pub fn reload_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 24) - 1)
    }

    #[inline(always)]
    pub fn reload(mut self, val: u32) -> Load_ {
        self.raw = (self.raw & !(((1 << 24) - 1) << 0)) | ((val & ((1 << 24) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E010 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod load_ {
    #[inline(always)]
    pub fn read() -> super::Load_ {
        super::Load_ {
            raw: unsafe { *((0xE000E010 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Load_) {
       unsafe { *((0xE000E010 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Val {
   raw: u32,
}

impl Val {
    #[inline(always)]
    pub fn current_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 24) - 1)
    }

    #[inline(always)]
    pub fn current(mut self, val: u32) -> Val {
        self.raw = (self.raw & !(((1 << 24) - 1) << 0)) | ((val & ((1 << 24) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E010 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod val {
    #[inline(always)]
    pub fn read() -> super::Val {
        super::Val {
            raw: unsafe { *((0xE000E010 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Val) {
       unsafe { *((0xE000E010 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Calib {
   raw: u32,
}

impl Calib {
    #[inline(always)]
    pub fn tenms_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 24) - 1)
    }

    #[inline(always)]
    pub fn tenms(mut self, val: u32) -> Calib {
        self.raw = (self.raw & !(((1 << 24) - 1) << 0)) | ((val & ((1 << 24) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E010 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod calib {
    #[inline(always)]
    pub fn read() -> super::Calib {
        super::Calib {
            raw: unsafe { *((0xE000E010 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Calib) {
       unsafe { *((0xE000E010 + 0xC) as *mut u32) = val.raw; }
    }
}

