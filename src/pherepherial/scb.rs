pub struct Cpuid {
   raw: u32,
}

impl Cpuid {
    #[inline(always)]
    pub fn revision_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn revision_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

    #[inline(always)]
    pub fn partno_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn partno_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 12) - 1) << 4)) | ((val & ((1 << 12) - 1)) << 4)
    }

    #[inline(always)]
    pub fn constant_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn constant_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 16)) | ((val & ((1 << 4) - 1)) << 16)
    }

    #[inline(always)]
    pub fn variant_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn variant_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn implementer_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn implementer_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

}

pub mod cpuid {
    #[inline(always)]
    pub fn read() -> super::Cpuid {
        super::Cpuid {
            raw: unsafe { *((0xE000ED00 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cpuid) {
       unsafe { *((0xE000ED00 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Icsr {
   raw: u32,
}

impl Icsr {
    #[inline(always)]
    pub fn vectactive_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 9) - 1)
    }

    #[inline(always)]
    pub fn vectactive_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 9) - 1) << 0)) | ((val & ((1 << 9) - 1)) << 0)
    }

    #[inline(always)]
    pub fn rettobase_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rettobase_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn vectpending_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn vectpending_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 7) - 1) << 12)) | ((val & ((1 << 7) - 1)) << 12)
    }

    #[inline(always)]
    pub fn isrpending_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn isrpending_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22)
    }

    #[inline(always)]
    pub fn pendstclr_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendstclr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25)
    }

    #[inline(always)]
    pub fn pendstset_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendstset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26)
    }

    #[inline(always)]
    pub fn pendsvclr_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendsvclr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27)
    }

    #[inline(always)]
    pub fn pendsvset_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendsvset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28)
    }

    #[inline(always)]
    pub fn nmipendset_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nmipendset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31)
    }

}

pub mod icsr {
    #[inline(always)]
    pub fn read() -> super::Icsr {
        super::Icsr {
            raw: unsafe { *((0xE000ED00 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Icsr) {
       unsafe { *((0xE000ED00 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Vtor {
   raw: u32,
}

impl Vtor {
    #[inline(always)]
    pub fn tbloff_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 21) - 1)
    }

    #[inline(always)]
    pub fn tbloff_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 21) - 1) << 9)) | ((val & ((1 << 21) - 1)) << 9)
    }

}

pub mod vtor {
    #[inline(always)]
    pub fn read() -> super::Vtor {
        super::Vtor {
            raw: unsafe { *((0xE000ED00 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Vtor) {
       unsafe { *((0xE000ED00 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Aircr {
   raw: u32,
}

impl Aircr {
    #[inline(always)]
    pub fn vectreset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn vectreset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn vectclractive_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn vectclractive_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn sysresetreq_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sysresetreq_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn prigroup_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn prigroup_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 8)) | ((val & ((1 << 3) - 1)) << 8)
    }

    #[inline(always)]
    pub fn endianess_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn endianess_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn vectkeystat_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn vectkeystat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16)
    }

}

pub mod aircr {
    #[inline(always)]
    pub fn read() -> super::Aircr {
        super::Aircr {
            raw: unsafe { *((0xE000ED00 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Aircr) {
       unsafe { *((0xE000ED00 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Scr {
   raw: u32,
}

impl Scr {
    #[inline(always)]
    pub fn sleeponexit_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sleeponexit_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn sleepdeep_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sleepdeep_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn seveonpend_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn seveonpend_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

}

pub mod scr {
    #[inline(always)]
    pub fn read() -> super::Scr {
        super::Scr {
            raw: unsafe { *((0xE000ED00 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Scr) {
       unsafe { *((0xE000ED00 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Ccr {
   raw: u32,
}

impl Ccr {
    #[inline(always)]
    pub fn nonbasethrdena_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nonbasethrdena_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn usersetmpend_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usersetmpend_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn unalign__trp_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn unalign__trp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn div_0_trp_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn div_0_trp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn bfhfnmign_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bfhfnmign_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn stkalign_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stkalign_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

}

pub mod ccr {
    #[inline(always)]
    pub fn read() -> super::Ccr {
        super::Ccr {
            raw: unsafe { *((0xE000ED00 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ccr) {
       unsafe { *((0xE000ED00 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Shpr1 {
   raw: u32,
}

impl Shpr1 {
    #[inline(always)]
    pub fn pri_4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_4_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

    #[inline(always)]
    pub fn pri_5_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_5_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn pri_6_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_6_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

}

pub mod shpr1 {
    #[inline(always)]
    pub fn read() -> super::Shpr1 {
        super::Shpr1 {
            raw: unsafe { *((0xE000ED00 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Shpr1) {
       unsafe { *((0xE000ED00 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Shpr2 {
   raw: u32,
}

impl Shpr2 {
    #[inline(always)]
    pub fn pri_11_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_11_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

}

pub mod shpr2 {
    #[inline(always)]
    pub fn read() -> super::Shpr2 {
        super::Shpr2 {
            raw: unsafe { *((0xE000ED00 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Shpr2) {
       unsafe { *((0xE000ED00 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Shpr3 {
   raw: u32,
}

impl Shpr3 {
    #[inline(always)]
    pub fn pri_14_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_14_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

    #[inline(always)]
    pub fn pri_15_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_15_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

}

pub mod shpr3 {
    #[inline(always)]
    pub fn read() -> super::Shpr3 {
        super::Shpr3 {
            raw: unsafe { *((0xE000ED00 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Shpr3) {
       unsafe { *((0xE000ED00 + 0x20) as *mut u32) = val.raw; }
    }
}

pub struct Shcrs {
   raw: u32,
}

impl Shcrs {
    #[inline(always)]
    pub fn memfaultact_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn memfaultact_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn busfaultact_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn busfaultact_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn usgfaultact_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usgfaultact_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn svcallact_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn svcallact_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn monitoract_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn monitoract_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn pendsvact_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendsvact_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn systickact_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn systickact_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn usgfaultpended_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usgfaultpended_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn memfaultpended_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn memfaultpended_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn busfaultpended_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn busfaultpended_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn svcallpended_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn svcallpended_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn memfaultena_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn memfaultena_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn busfaultena_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn busfaultena_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn usgfaultena_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usgfaultena_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

}

pub mod shcrs {
    #[inline(always)]
    pub fn read() -> super::Shcrs {
        super::Shcrs {
            raw: unsafe { *((0xE000ED00 + 0x24) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Shcrs) {
       unsafe { *((0xE000ED00 + 0x24) as *mut u32) = val.raw; }
    }
}

pub struct Cfsr_ufsr_bfsr_mmfsr {
   raw: u32,
}

impl Cfsr_ufsr_bfsr_mmfsr {
    #[inline(always)]
    pub fn iaccviol_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iaccviol_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

    #[inline(always)]
    pub fn daccviol_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn daccviol_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn munstkerr_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn munstkerr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn mstkerr_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mstkerr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn mlsperr_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mlsperr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn mmarvalid_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmarvalid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7)
    }

    #[inline(always)]
    pub fn ibuserr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ibuserr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn preciserr_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn preciserr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn impreciserr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn impreciserr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn unstkerr_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn unstkerr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn stkerr_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stkerr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn lsperr_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsperr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn bfarvalid_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bfarvalid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn undefinstr_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn undefinstr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16)
    }

    #[inline(always)]
    pub fn invstate_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn invstate_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17)
    }

    #[inline(always)]
    pub fn invpc_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn invpc_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18)
    }

    #[inline(always)]
    pub fn nocp_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nocp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19)
    }

    #[inline(always)]
    pub fn unaligned_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn unaligned_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24)
    }

    #[inline(always)]
    pub fn divbyzero_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn divbyzero_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25)
    }

}

pub mod cfsr_ufsr_bfsr_mmfsr {
    #[inline(always)]
    pub fn read() -> super::Cfsr_ufsr_bfsr_mmfsr {
        super::Cfsr_ufsr_bfsr_mmfsr {
            raw: unsafe { *((0xE000ED00 + 0x28) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cfsr_ufsr_bfsr_mmfsr) {
       unsafe { *((0xE000ED00 + 0x28) as *mut u32) = val.raw; }
    }
}

pub struct Hfsr {
   raw: u32,
}

impl Hfsr {
    #[inline(always)]
    pub fn vecttbl_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn vecttbl_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn forced_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn forced_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30)
    }

    #[inline(always)]
    pub fn debug_vt_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn debug_vt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31)
    }

}

pub mod hfsr {
    #[inline(always)]
    pub fn read() -> super::Hfsr {
        super::Hfsr {
            raw: unsafe { *((0xE000ED00 + 0x2C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Hfsr) {
       unsafe { *((0xE000ED00 + 0x2C) as *mut u32) = val.raw; }
    }
}

pub struct Mmfar {
   raw: u32,
}

impl Mmfar {
    #[inline(always)]
    pub fn mmfar_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn mmfar_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod mmfar {
    #[inline(always)]
    pub fn read() -> super::Mmfar {
        super::Mmfar {
            raw: unsafe { *((0xE000ED00 + 0x34) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mmfar) {
       unsafe { *((0xE000ED00 + 0x34) as *mut u32) = val.raw; }
    }
}

pub struct Bfar {
   raw: u32,
}

impl Bfar {
    #[inline(always)]
    pub fn bfar_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn bfar_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod bfar {
    #[inline(always)]
    pub fn read() -> super::Bfar {
        super::Bfar {
            raw: unsafe { *((0xE000ED00 + 0x38) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bfar) {
       unsafe { *((0xE000ED00 + 0x38) as *mut u32) = val.raw; }
    }
}

