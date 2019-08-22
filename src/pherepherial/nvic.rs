pub struct Iser0 {
   raw: u32,
}

impl Iser0 {
    #[inline(always)]
    pub fn setena_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn setena(mut self, val: u32) -> Iser0 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x0u32) as *mut u32) = self.raw; }
    }
}

pub mod iser0 {
    #[inline(always)]
    pub fn read() -> super::Iser0 {
        super::Iser0 {
            raw: unsafe { *((0xE000E100u32 + 0x0u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Iser0) {
       unsafe { *((0xE000E100u32 + 0x0u32) as *mut u32) = val.raw; }
    }
}

pub struct Iser1 {
   raw: u32,
}

impl Iser1 {
    #[inline(always)]
    pub fn setena_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn setena(mut self, val: u32) -> Iser1 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x4u32) as *mut u32) = self.raw; }
    }
}

pub mod iser1 {
    #[inline(always)]
    pub fn read() -> super::Iser1 {
        super::Iser1 {
            raw: unsafe { *((0xE000E100u32 + 0x4u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Iser1) {
       unsafe { *((0xE000E100u32 + 0x4u32) as *mut u32) = val.raw; }
    }
}

pub struct Icer0 {
   raw: u32,
}

impl Icer0 {
    #[inline(always)]
    pub fn clrena_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn clrena(mut self, val: u32) -> Icer0 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x80u32) as *mut u32) = self.raw; }
    }
}

pub mod icer0 {
    #[inline(always)]
    pub fn read() -> super::Icer0 {
        super::Icer0 {
            raw: unsafe { *((0xE000E100u32 + 0x80u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Icer0) {
       unsafe { *((0xE000E100u32 + 0x80u32) as *mut u32) = val.raw; }
    }
}

pub struct Icer1 {
   raw: u32,
}

impl Icer1 {
    #[inline(always)]
    pub fn clrena_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn clrena(mut self, val: u32) -> Icer1 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x84u32) as *mut u32) = self.raw; }
    }
}

pub mod icer1 {
    #[inline(always)]
    pub fn read() -> super::Icer1 {
        super::Icer1 {
            raw: unsafe { *((0xE000E100u32 + 0x84u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Icer1) {
       unsafe { *((0xE000E100u32 + 0x84u32) as *mut u32) = val.raw; }
    }
}

pub struct Ispr0 {
   raw: u32,
}

impl Ispr0 {
    #[inline(always)]
    pub fn setpend_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn setpend(mut self, val: u32) -> Ispr0 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x100u32) as *mut u32) = self.raw; }
    }
}

pub mod ispr0 {
    #[inline(always)]
    pub fn read() -> super::Ispr0 {
        super::Ispr0 {
            raw: unsafe { *((0xE000E100u32 + 0x100u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ispr0) {
       unsafe { *((0xE000E100u32 + 0x100u32) as *mut u32) = val.raw; }
    }
}

pub struct Ispr1 {
   raw: u32,
}

impl Ispr1 {
    #[inline(always)]
    pub fn setpend_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn setpend(mut self, val: u32) -> Ispr1 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x104u32) as *mut u32) = self.raw; }
    }
}

pub mod ispr1 {
    #[inline(always)]
    pub fn read() -> super::Ispr1 {
        super::Ispr1 {
            raw: unsafe { *((0xE000E100u32 + 0x104u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ispr1) {
       unsafe { *((0xE000E100u32 + 0x104u32) as *mut u32) = val.raw; }
    }
}

pub struct Icpr0 {
   raw: u32,
}

impl Icpr0 {
    #[inline(always)]
    pub fn clrpend_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn clrpend(mut self, val: u32) -> Icpr0 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x180u32) as *mut u32) = self.raw; }
    }
}

pub mod icpr0 {
    #[inline(always)]
    pub fn read() -> super::Icpr0 {
        super::Icpr0 {
            raw: unsafe { *((0xE000E100u32 + 0x180u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Icpr0) {
       unsafe { *((0xE000E100u32 + 0x180u32) as *mut u32) = val.raw; }
    }
}

pub struct Icpr1 {
   raw: u32,
}

impl Icpr1 {
    #[inline(always)]
    pub fn clrpend_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn clrpend(mut self, val: u32) -> Icpr1 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x184u32) as *mut u32) = self.raw; }
    }
}

pub mod icpr1 {
    #[inline(always)]
    pub fn read() -> super::Icpr1 {
        super::Icpr1 {
            raw: unsafe { *((0xE000E100u32 + 0x184u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Icpr1) {
       unsafe { *((0xE000E100u32 + 0x184u32) as *mut u32) = val.raw; }
    }
}

pub struct Iabr0 {
   raw: u32,
}

impl Iabr0 {
    #[inline(always)]
    pub fn active_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn active(mut self, val: u32) -> Iabr0 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x200u32) as *mut u32) = self.raw; }
    }
}

pub mod iabr0 {
    #[inline(always)]
    pub fn read() -> super::Iabr0 {
        super::Iabr0 {
            raw: unsafe { *((0xE000E100u32 + 0x200u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Iabr0) {
       unsafe { *((0xE000E100u32 + 0x200u32) as *mut u32) = val.raw; }
    }
}

pub struct Iabr1 {
   raw: u32,
}

impl Iabr1 {
    #[inline(always)]
    pub fn active_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn active(mut self, val: u32) -> Iabr1 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x204u32) as *mut u32) = self.raw; }
    }
}

pub mod iabr1 {
    #[inline(always)]
    pub fn read() -> super::Iabr1 {
        super::Iabr1 {
            raw: unsafe { *((0xE000E100u32 + 0x204u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Iabr1) {
       unsafe { *((0xE000E100u32 + 0x204u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr0 {
   raw: u32,
}

impl Ipr0 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr0 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr0 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr0 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr0 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x300u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr0 {
    #[inline(always)]
    pub fn read() -> super::Ipr0 {
        super::Ipr0 {
            raw: unsafe { *((0xE000E100u32 + 0x300u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr0) {
       unsafe { *((0xE000E100u32 + 0x300u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr1 {
   raw: u32,
}

impl Ipr1 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr1 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr1 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr1 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr1 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x304u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr1 {
    #[inline(always)]
    pub fn read() -> super::Ipr1 {
        super::Ipr1 {
            raw: unsafe { *((0xE000E100u32 + 0x304u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr1) {
       unsafe { *((0xE000E100u32 + 0x304u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr2 {
   raw: u32,
}

impl Ipr2 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr2 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr2 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr2 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr2 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x308u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr2 {
    #[inline(always)]
    pub fn read() -> super::Ipr2 {
        super::Ipr2 {
            raw: unsafe { *((0xE000E100u32 + 0x308u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr2) {
       unsafe { *((0xE000E100u32 + 0x308u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr3 {
   raw: u32,
}

impl Ipr3 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr3 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr3 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr3 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr3 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x30Cu32) as *mut u32) = self.raw; }
    }
}

pub mod ipr3 {
    #[inline(always)]
    pub fn read() -> super::Ipr3 {
        super::Ipr3 {
            raw: unsafe { *((0xE000E100u32 + 0x30Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr3) {
       unsafe { *((0xE000E100u32 + 0x30Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr4 {
   raw: u32,
}

impl Ipr4 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr4 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr4 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr4 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr4 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x310u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr4 {
    #[inline(always)]
    pub fn read() -> super::Ipr4 {
        super::Ipr4 {
            raw: unsafe { *((0xE000E100u32 + 0x310u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr4) {
       unsafe { *((0xE000E100u32 + 0x310u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr5 {
   raw: u32,
}

impl Ipr5 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr5 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr5 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr5 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr5 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x314u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr5 {
    #[inline(always)]
    pub fn read() -> super::Ipr5 {
        super::Ipr5 {
            raw: unsafe { *((0xE000E100u32 + 0x314u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr5) {
       unsafe { *((0xE000E100u32 + 0x314u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr6 {
   raw: u32,
}

impl Ipr6 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr6 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr6 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr6 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr6 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x318u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr6 {
    #[inline(always)]
    pub fn read() -> super::Ipr6 {
        super::Ipr6 {
            raw: unsafe { *((0xE000E100u32 + 0x318u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr6) {
       unsafe { *((0xE000E100u32 + 0x318u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr7 {
   raw: u32,
}

impl Ipr7 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr7 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr7 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr7 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr7 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x31Cu32) as *mut u32) = self.raw; }
    }
}

pub mod ipr7 {
    #[inline(always)]
    pub fn read() -> super::Ipr7 {
        super::Ipr7 {
            raw: unsafe { *((0xE000E100u32 + 0x31Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr7) {
       unsafe { *((0xE000E100u32 + 0x31Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr8 {
   raw: u32,
}

impl Ipr8 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr8 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr8 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr8 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr8 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x320u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr8 {
    #[inline(always)]
    pub fn read() -> super::Ipr8 {
        super::Ipr8 {
            raw: unsafe { *((0xE000E100u32 + 0x320u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr8) {
       unsafe { *((0xE000E100u32 + 0x320u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr9 {
   raw: u32,
}

impl Ipr9 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr9 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr9 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr9 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr9 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x324u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr9 {
    #[inline(always)]
    pub fn read() -> super::Ipr9 {
        super::Ipr9 {
            raw: unsafe { *((0xE000E100u32 + 0x324u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr9) {
       unsafe { *((0xE000E100u32 + 0x324u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr10 {
   raw: u32,
}

impl Ipr10 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr10 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr10 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr10 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr10 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x328u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr10 {
    #[inline(always)]
    pub fn read() -> super::Ipr10 {
        super::Ipr10 {
            raw: unsafe { *((0xE000E100u32 + 0x328u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr10) {
       unsafe { *((0xE000E100u32 + 0x328u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr11 {
   raw: u32,
}

impl Ipr11 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr11 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr11 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr11 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr11 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x32Cu32) as *mut u32) = self.raw; }
    }
}

pub mod ipr11 {
    #[inline(always)]
    pub fn read() -> super::Ipr11 {
        super::Ipr11 {
            raw: unsafe { *((0xE000E100u32 + 0x32Cu32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr11) {
       unsafe { *((0xE000E100u32 + 0x32Cu32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr12 {
   raw: u32,
}

impl Ipr12 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr12 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr12 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr12 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr12 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x330u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr12 {
    #[inline(always)]
    pub fn read() -> super::Ipr12 {
        super::Ipr12 {
            raw: unsafe { *((0xE000E100u32 + 0x330u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr12) {
       unsafe { *((0xE000E100u32 + 0x330u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr13 {
   raw: u32,
}

impl Ipr13 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr13 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr13 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr13 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr13 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x334u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr13 {
    #[inline(always)]
    pub fn read() -> super::Ipr13 {
        super::Ipr13 {
            raw: unsafe { *((0xE000E100u32 + 0x334u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr13) {
       unsafe { *((0xE000E100u32 + 0x334u32) as *mut u32) = val.raw; }
    }
}

pub struct Ipr14 {
   raw: u32,
}

impl Ipr14 {
    #[inline(always)]
    pub fn ipr_n0_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n0(mut self, val: u32) -> Ipr14 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ipr_n1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n1(mut self, val: u32) -> Ipr14 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ipr_n2_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n2(mut self, val: u32) -> Ipr14 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn ipr_n3_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ipr_n3(mut self, val: u32) -> Ipr14 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000E100u32 + 0x338u32) as *mut u32) = self.raw; }
    }
}

pub mod ipr14 {
    #[inline(always)]
    pub fn read() -> super::Ipr14 {
        super::Ipr14 {
            raw: unsafe { *((0xE000E100u32 + 0x338u32) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ipr14) {
       unsafe { *((0xE000E100u32 + 0x338u32) as *mut u32) = val.raw; }
    }
}

