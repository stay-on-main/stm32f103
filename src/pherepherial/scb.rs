pub struct Cpuid {
   raw: u32,
}

impl Cpuid {
    #[inline(always)]
    pub fn revision_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn revision(mut self, val: u32) -> Cpuid {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn partno_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn partno(mut self, val: u32) -> Cpuid {
        self.raw = (self.raw & !(((1 << 12) - 1) << 4)) | ((val & ((1 << 12) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn constant_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn constant(mut self, val: u32) -> Cpuid {
        self.raw = (self.raw & !(((1 << 4) - 1) << 16)) | ((val & ((1 << 4) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn variant_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn variant(mut self, val: u32) -> Cpuid {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn implementer_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn implementer(mut self, val: u32) -> Cpuid {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x0) as *mut u32) = self.raw; }
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
    pub fn vectactive(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 9) - 1) << 0)) | ((val & ((1 << 9) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn rettobase_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rettobase(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn vectpending_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn vectpending(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 7) - 1) << 12)) | ((val & ((1 << 7) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn isrpending_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn isrpending(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn pendstclr_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendstclr(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn pendstset_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendstset(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn pendsvclr_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendsvclr(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn pendsvset_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendsvset(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn nmipendset_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nmipendset(mut self, val: u32) -> Icsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x4) as *mut u32) = self.raw; }
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
    pub fn tbloff(mut self, val: u32) -> Vtor {
        self.raw = (self.raw & !(((1 << 21) - 1) << 9)) | ((val & ((1 << 21) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x8) as *mut u32) = self.raw; }
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
    pub fn vectreset(mut self, val: u32) -> Aircr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn vectclractive_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn vectclractive(mut self, val: u32) -> Aircr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn sysresetreq_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sysresetreq(mut self, val: u32) -> Aircr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn prigroup_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn prigroup(mut self, val: u32) -> Aircr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 8)) | ((val & ((1 << 3) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn endianess_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn endianess(mut self, val: u32) -> Aircr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn vectkeystat_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn vectkeystat(mut self, val: u32) -> Aircr {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0xC) as *mut u32) = self.raw; }
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
    pub fn sleeponexit(mut self, val: u32) -> Scr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn sleepdeep_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sleepdeep(mut self, val: u32) -> Scr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn seveonpend_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn seveonpend(mut self, val: u32) -> Scr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x10) as *mut u32) = self.raw; }
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
    pub fn nonbasethrdena(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn usersetmpend_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usersetmpend(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn unalign__trp_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn unalign__trp(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn div_0_trp_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn div_0_trp(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn bfhfnmign_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bfhfnmign(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn stkalign_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stkalign(mut self, val: u32) -> Ccr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x14) as *mut u32) = self.raw; }
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
    pub fn pri_4(mut self, val: u32) -> Shpr1 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pri_5_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_5(mut self, val: u32) -> Shpr1 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn pri_6_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_6(mut self, val: u32) -> Shpr1 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x18) as *mut u32) = self.raw; }
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
    pub fn pri_11(mut self, val: u32) -> Shpr2 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x1C) as *mut u32) = self.raw; }
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
    pub fn pri_14(mut self, val: u32) -> Shpr3 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn pri_15_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn pri_15(mut self, val: u32) -> Shpr3 {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x20) as *mut u32) = self.raw; }
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
    pub fn memfaultact(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn busfaultact_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn busfaultact(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn usgfaultact_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usgfaultact(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn svcallact_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn svcallact(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn monitoract_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn monitoract(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn pendsvact_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pendsvact(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn systickact_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn systickact(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn usgfaultpended_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usgfaultpended(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn memfaultpended_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn memfaultpended(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn busfaultpended_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn busfaultpended(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn svcallpended_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn svcallpended(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn memfaultena_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn memfaultena(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn busfaultena_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn busfaultena(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn usgfaultena_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usgfaultena(mut self, val: u32) -> Shcrs {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x24) as *mut u32) = self.raw; }
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
    pub fn iaccviol(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn daccviol_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn daccviol(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn munstkerr_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn munstkerr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn mstkerr_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mstkerr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn mlsperr_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mlsperr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn mmarvalid_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mmarvalid(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ibuserr_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ibuserr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn preciserr_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn preciserr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn impreciserr_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn impreciserr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn unstkerr_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn unstkerr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stkerr_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stkerr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn lsperr_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsperr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn bfarvalid_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bfarvalid(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn undefinstr_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn undefinstr(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn invstate_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn invstate(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn invpc_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn invpc(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn nocp_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nocp(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn unaligned_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn unaligned(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn divbyzero_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn divbyzero(mut self, val: u32) -> Cfsr_ufsr_bfsr_mmfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x28) as *mut u32) = self.raw; }
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
    pub fn vecttbl(mut self, val: u32) -> Hfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn forced_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn forced(mut self, val: u32) -> Hfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn debug_vt_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn debug_vt(mut self, val: u32) -> Hfsr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x2C) as *mut u32) = self.raw; }
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
    pub fn mmfar(mut self, val: u32) -> Mmfar {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x34) as *mut u32) = self.raw; }
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
    pub fn bfar(mut self, val: u32) -> Bfar {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE000ED00 + 0x38) as *mut u32) = self.raw; }
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

