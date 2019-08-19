pub struct Mpu_typer {
   raw: u32,
}

impl Mpu_typer {
    #[inline(always)]
    pub fn separate_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn separate_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn dregion_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn dregion_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn iregion_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn iregion_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

}

pub mod mpu_typer {
    #[inline(always)]
    pub fn read() -> super::Mpu_typer {
        super::Mpu_typer {
            raw: unsafe { *((0xE000ED90 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mpu_typer) {
       unsafe { *((0xE000ED90 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Mpu_ctrl {
   raw: u32,
}

impl Mpu_ctrl {
    #[inline(always)]
    pub fn enable_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn enable_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn hfnmiena_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hfnmiena_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn privdefena_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn privdefena_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

}

pub mod mpu_ctrl {
    #[inline(always)]
    pub fn read() -> super::Mpu_ctrl {
        super::Mpu_ctrl {
            raw: unsafe { *((0xE000ED90 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mpu_ctrl) {
       unsafe { *((0xE000ED90 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Mpu_rnr {
   raw: u32,
}

impl Mpu_rnr {
    #[inline(always)]
    pub fn region_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn region_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod mpu_rnr {
    #[inline(always)]
    pub fn read() -> super::Mpu_rnr {
        super::Mpu_rnr {
            raw: unsafe { *((0xE000ED90 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mpu_rnr) {
       unsafe { *((0xE000ED90 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Mpu_rbar {
   raw: u32,
}

impl Mpu_rbar {
    #[inline(always)]
    pub fn region_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn region_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

    #[inline(always)]
    pub fn valid_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn valid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addr_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 27) - 1)
    }

    #[inline(always)]
    pub fn addr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 27) - 1) << 5)) | ((val & ((1 << 27) - 1)) << 5)
    }

}

pub mod mpu_rbar {
    #[inline(always)]
    pub fn read() -> super::Mpu_rbar {
        super::Mpu_rbar {
            raw: unsafe { *((0xE000ED90 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mpu_rbar) {
       unsafe { *((0xE000ED90 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Mpu_rasr {
   raw: u32,
}

impl Mpu_rasr {
    #[inline(always)]
    pub fn enable_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn enable_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn size_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn size_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 5) - 1) << 1)) | ((val & ((1 << 5) - 1)) << 1)
    }

    #[inline(always)]
    pub fn srd_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn srd_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn b_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn b_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn c_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn c_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn s_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn s_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

    #[inline(always)]
    pub fn tex_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn tex_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 19)) | ((val & ((1 << 3) - 1)) << 19)
    }

    #[inline(always)]
    pub fn ap_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ap_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 24)) | ((val & ((1 << 3) - 1)) << 24)
    }

    #[inline(always)]
    pub fn xn_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn xn_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28)
    }

}

pub mod mpu_rasr {
    #[inline(always)]
    pub fn read() -> super::Mpu_rasr {
        super::Mpu_rasr {
            raw: unsafe { *((0xE000ED90 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mpu_rasr) {
       unsafe { *((0xE000ED90 + 0x10) as *mut u32) = val.raw; }
    }
}

