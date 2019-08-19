pub struct Bcr1 {
   raw: u32,
}

impl Bcr1 {
    #[inline(always)]
    pub fn cburstrw_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cburstrw_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19)
    }

    #[inline(always)]
    pub fn asyncwait_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn asyncwait_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn extmod_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn extmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn waiten_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waiten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn wren_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wren_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn waitcfg_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitcfg_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn waitpol_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitpol_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn bursten_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bursten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn faccen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn faccen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn mwid_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mwid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4)
    }

    #[inline(always)]
    pub fn mtyp_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mtyp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2)
    }

    #[inline(always)]
    pub fn muxen_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn muxen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn mbken_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mbken_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod bcr1 {
    #[inline(always)]
    pub fn read() -> super::Bcr1 {
        super::Bcr1 {
            raw: unsafe { *((0xA0000000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bcr1) {
       unsafe { *((0xA0000000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Btr1 {
   raw: u32,
}

impl Btr1 {
    #[inline(always)]
    pub fn accmod_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn accmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28)
    }

    #[inline(always)]
    pub fn datlat_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn datlat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn clkdiv_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn busturn_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn busturn_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 16)) | ((val & ((1 << 4) - 1)) << 16)
    }

    #[inline(always)]
    pub fn datast_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn datast_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn addhld_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addhld_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod btr1 {
    #[inline(always)]
    pub fn read() -> super::Btr1 {
        super::Btr1 {
            raw: unsafe { *((0xA0000000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Btr1) {
       unsafe { *((0xA0000000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Bcr2 {
   raw: u32,
}

impl Bcr2 {
    #[inline(always)]
    pub fn cburstrw_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cburstrw_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19)
    }

    #[inline(always)]
    pub fn asyncwait_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn asyncwait_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn extmod_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn extmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn waiten_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waiten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn wren_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wren_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn waitcfg_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitcfg_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn wrapmod_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wrapmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn waitpol_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitpol_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn bursten_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bursten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn faccen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn faccen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn mwid_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mwid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4)
    }

    #[inline(always)]
    pub fn mtyp_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mtyp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2)
    }

    #[inline(always)]
    pub fn muxen_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn muxen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn mbken_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mbken_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod bcr2 {
    #[inline(always)]
    pub fn read() -> super::Bcr2 {
        super::Bcr2 {
            raw: unsafe { *((0xA0000000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bcr2) {
       unsafe { *((0xA0000000 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Btr2 {
   raw: u32,
}

impl Btr2 {
    #[inline(always)]
    pub fn accmod_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn accmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28)
    }

    #[inline(always)]
    pub fn datlat_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn datlat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn clkdiv_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn busturn_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn busturn_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 16)) | ((val & ((1 << 4) - 1)) << 16)
    }

    #[inline(always)]
    pub fn datast_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn datast_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn addhld_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addhld_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod btr2 {
    #[inline(always)]
    pub fn read() -> super::Btr2 {
        super::Btr2 {
            raw: unsafe { *((0xA0000000 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Btr2) {
       unsafe { *((0xA0000000 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Bcr3 {
   raw: u32,
}

impl Bcr3 {
    #[inline(always)]
    pub fn cburstrw_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cburstrw_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19)
    }

    #[inline(always)]
    pub fn asyncwait_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn asyncwait_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn extmod_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn extmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn waiten_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waiten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn wren_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wren_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn waitcfg_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitcfg_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn wrapmod_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wrapmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn waitpol_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitpol_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn bursten_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bursten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn faccen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn faccen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn mwid_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mwid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4)
    }

    #[inline(always)]
    pub fn mtyp_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mtyp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2)
    }

    #[inline(always)]
    pub fn muxen_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn muxen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn mbken_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mbken_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod bcr3 {
    #[inline(always)]
    pub fn read() -> super::Bcr3 {
        super::Bcr3 {
            raw: unsafe { *((0xA0000000 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bcr3) {
       unsafe { *((0xA0000000 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Btr3 {
   raw: u32,
}

impl Btr3 {
    #[inline(always)]
    pub fn accmod_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn accmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28)
    }

    #[inline(always)]
    pub fn datlat_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn datlat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn clkdiv_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn busturn_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn busturn_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 16)) | ((val & ((1 << 4) - 1)) << 16)
    }

    #[inline(always)]
    pub fn datast_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn datast_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn addhld_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addhld_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod btr3 {
    #[inline(always)]
    pub fn read() -> super::Btr3 {
        super::Btr3 {
            raw: unsafe { *((0xA0000000 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Btr3) {
       unsafe { *((0xA0000000 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Bcr4 {
   raw: u32,
}

impl Bcr4 {
    #[inline(always)]
    pub fn cburstrw_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cburstrw_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19)
    }

    #[inline(always)]
    pub fn asyncwait_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn asyncwait_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15)
    }

    #[inline(always)]
    pub fn extmod_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn extmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14)
    }

    #[inline(always)]
    pub fn waiten_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waiten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13)
    }

    #[inline(always)]
    pub fn wren_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wren_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12)
    }

    #[inline(always)]
    pub fn waitcfg_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitcfg_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11)
    }

    #[inline(always)]
    pub fn wrapmod_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wrapmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10)
    }

    #[inline(always)]
    pub fn waitpol_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitpol_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9)
    }

    #[inline(always)]
    pub fn bursten_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bursten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8)
    }

    #[inline(always)]
    pub fn faccen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn faccen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn mwid_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mwid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4)
    }

    #[inline(always)]
    pub fn mtyp_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn mtyp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2)
    }

    #[inline(always)]
    pub fn muxen_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn muxen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn mbken_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn mbken_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod bcr4 {
    #[inline(always)]
    pub fn read() -> super::Bcr4 {
        super::Bcr4 {
            raw: unsafe { *((0xA0000000 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bcr4) {
       unsafe { *((0xA0000000 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Btr4 {
   raw: u32,
}

impl Btr4 {
    #[inline(always)]
    pub fn accmod_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn accmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28)
    }

    #[inline(always)]
    pub fn datlat_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn datlat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn clkdiv_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn busturn_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn busturn_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 16)) | ((val & ((1 << 4) - 1)) << 16)
    }

    #[inline(always)]
    pub fn datast_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn datast_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn addhld_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addhld_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod btr4 {
    #[inline(always)]
    pub fn read() -> super::Btr4 {
        super::Btr4 {
            raw: unsafe { *((0xA0000000 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Btr4) {
       unsafe { *((0xA0000000 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Pcr2 {
   raw: u32,
}

impl Pcr2 {
    #[inline(always)]
    pub fn eccps_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn eccps_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 17)) | ((val & ((1 << 3) - 1)) << 17)
    }

    #[inline(always)]
    pub fn tar_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn tar_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 13)) | ((val & ((1 << 4) - 1)) << 13)
    }

    #[inline(always)]
    pub fn tclr_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn tclr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 9)) | ((val & ((1 << 4) - 1)) << 9)
    }

    #[inline(always)]
    pub fn eccen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eccen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn pwid_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pwid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4)
    }

    #[inline(always)]
    pub fn ptyp_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ptyp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn pbken_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pbken_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn pwaiten_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwaiten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

}

pub mod pcr2 {
    #[inline(always)]
    pub fn read() -> super::Pcr2 {
        super::Pcr2 {
            raw: unsafe { *((0xA0000000 + 0x60) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pcr2) {
       unsafe { *((0xA0000000 + 0x60) as *mut u32) = val.raw; }
    }
}

pub struct Sr2 {
   raw: u32,
}

impl Sr2 {
    #[inline(always)]
    pub fn fempt_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fempt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn ifen_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ifen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn ilen_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ilen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn iren_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iren_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn ifs_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ifs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn ils_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ils_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn irs_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn irs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod sr2 {
    #[inline(always)]
    pub fn read() -> super::Sr2 {
        super::Sr2 {
            raw: unsafe { *((0xA0000000 + 0x64) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr2) {
       unsafe { *((0xA0000000 + 0x64) as *mut u32) = val.raw; }
    }
}

pub struct Pmem2 {
   raw: u32,
}

impl Pmem2 {
    #[inline(always)]
    pub fn memhizx_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memhizx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

    #[inline(always)]
    pub fn memholdx_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memholdx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

    #[inline(always)]
    pub fn memwaitx_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memwaitx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn memsetx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memsetx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod pmem2 {
    #[inline(always)]
    pub fn read() -> super::Pmem2 {
        super::Pmem2 {
            raw: unsafe { *((0xA0000000 + 0x68) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pmem2) {
       unsafe { *((0xA0000000 + 0x68) as *mut u32) = val.raw; }
    }
}

pub struct Patt2 {
   raw: u32,
}

impl Patt2 {
    #[inline(always)]
    pub fn atthizx_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn atthizx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

    #[inline(always)]
    pub fn attholdx_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attholdx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

    #[inline(always)]
    pub fn attwaitx_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attwaitx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn attsetx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attsetx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod patt2 {
    #[inline(always)]
    pub fn read() -> super::Patt2 {
        super::Patt2 {
            raw: unsafe { *((0xA0000000 + 0x6C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Patt2) {
       unsafe { *((0xA0000000 + 0x6C) as *mut u32) = val.raw; }
    }
}

pub struct Eccr2 {
   raw: u32,
}

impl Eccr2 {
    #[inline(always)]
    pub fn eccx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn eccx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod eccr2 {
    #[inline(always)]
    pub fn read() -> super::Eccr2 {
        super::Eccr2 {
            raw: unsafe { *((0xA0000000 + 0x74) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Eccr2) {
       unsafe { *((0xA0000000 + 0x74) as *mut u32) = val.raw; }
    }
}

pub struct Pcr3 {
   raw: u32,
}

impl Pcr3 {
    #[inline(always)]
    pub fn eccps_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn eccps_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 17)) | ((val & ((1 << 3) - 1)) << 17)
    }

    #[inline(always)]
    pub fn tar_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn tar_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 13)) | ((val & ((1 << 4) - 1)) << 13)
    }

    #[inline(always)]
    pub fn tclr_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn tclr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 9)) | ((val & ((1 << 4) - 1)) << 9)
    }

    #[inline(always)]
    pub fn eccen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eccen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn pwid_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pwid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4)
    }

    #[inline(always)]
    pub fn ptyp_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ptyp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn pbken_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pbken_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn pwaiten_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwaiten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

}

pub mod pcr3 {
    #[inline(always)]
    pub fn read() -> super::Pcr3 {
        super::Pcr3 {
            raw: unsafe { *((0xA0000000 + 0x80) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pcr3) {
       unsafe { *((0xA0000000 + 0x80) as *mut u32) = val.raw; }
    }
}

pub struct Sr3 {
   raw: u32,
}

impl Sr3 {
    #[inline(always)]
    pub fn fempt_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fempt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn ifen_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ifen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn ilen_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ilen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn iren_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iren_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn ifs_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ifs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn ils_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ils_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn irs_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn irs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod sr3 {
    #[inline(always)]
    pub fn read() -> super::Sr3 {
        super::Sr3 {
            raw: unsafe { *((0xA0000000 + 0x84) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr3) {
       unsafe { *((0xA0000000 + 0x84) as *mut u32) = val.raw; }
    }
}

pub struct Pmem3 {
   raw: u32,
}

impl Pmem3 {
    #[inline(always)]
    pub fn memhizx_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memhizx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

    #[inline(always)]
    pub fn memholdx_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memholdx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

    #[inline(always)]
    pub fn memwaitx_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memwaitx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn memsetx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memsetx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod pmem3 {
    #[inline(always)]
    pub fn read() -> super::Pmem3 {
        super::Pmem3 {
            raw: unsafe { *((0xA0000000 + 0x88) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pmem3) {
       unsafe { *((0xA0000000 + 0x88) as *mut u32) = val.raw; }
    }
}

pub struct Patt3 {
   raw: u32,
}

impl Patt3 {
    #[inline(always)]
    pub fn atthizx_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn atthizx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

    #[inline(always)]
    pub fn attholdx_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attholdx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

    #[inline(always)]
    pub fn attwaitx_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attwaitx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn attsetx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attsetx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod patt3 {
    #[inline(always)]
    pub fn read() -> super::Patt3 {
        super::Patt3 {
            raw: unsafe { *((0xA0000000 + 0x8C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Patt3) {
       unsafe { *((0xA0000000 + 0x8C) as *mut u32) = val.raw; }
    }
}

pub struct Eccr3 {
   raw: u32,
}

impl Eccr3 {
    #[inline(always)]
    pub fn eccx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn eccx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0)
    }

}

pub mod eccr3 {
    #[inline(always)]
    pub fn read() -> super::Eccr3 {
        super::Eccr3 {
            raw: unsafe { *((0xA0000000 + 0x94) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Eccr3) {
       unsafe { *((0xA0000000 + 0x94) as *mut u32) = val.raw; }
    }
}

pub struct Pcr4 {
   raw: u32,
}

impl Pcr4 {
    #[inline(always)]
    pub fn eccps_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn eccps_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 3) - 1) << 17)) | ((val & ((1 << 3) - 1)) << 17)
    }

    #[inline(always)]
    pub fn tar_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn tar_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 13)) | ((val & ((1 << 4) - 1)) << 13)
    }

    #[inline(always)]
    pub fn tclr_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn tclr_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 9)) | ((val & ((1 << 4) - 1)) << 9)
    }

    #[inline(always)]
    pub fn eccen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn eccen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn pwid_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pwid_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4)
    }

    #[inline(always)]
    pub fn ptyp_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ptyp_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn pbken_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pbken_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn pwaiten_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwaiten_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

}

pub mod pcr4 {
    #[inline(always)]
    pub fn read() -> super::Pcr4 {
        super::Pcr4 {
            raw: unsafe { *((0xA0000000 + 0xA0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pcr4) {
       unsafe { *((0xA0000000 + 0xA0) as *mut u32) = val.raw; }
    }
}

pub struct Sr4 {
   raw: u32,
}

impl Sr4 {
    #[inline(always)]
    pub fn fempt_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fempt_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6)
    }

    #[inline(always)]
    pub fn ifen_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ifen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5)
    }

    #[inline(always)]
    pub fn ilen_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ilen_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4)
    }

    #[inline(always)]
    pub fn iren_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iren_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3)
    }

    #[inline(always)]
    pub fn ifs_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ifs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2)
    }

    #[inline(always)]
    pub fn ils_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ils_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1)
    }

    #[inline(always)]
    pub fn irs_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn irs_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0)
    }

}

pub mod sr4 {
    #[inline(always)]
    pub fn read() -> super::Sr4 {
        super::Sr4 {
            raw: unsafe { *((0xA0000000 + 0xA4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sr4) {
       unsafe { *((0xA0000000 + 0xA4) as *mut u32) = val.raw; }
    }
}

pub struct Pmem4 {
   raw: u32,
}

impl Pmem4 {
    #[inline(always)]
    pub fn memhizx_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memhizx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

    #[inline(always)]
    pub fn memholdx_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memholdx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

    #[inline(always)]
    pub fn memwaitx_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memwaitx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn memsetx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn memsetx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod pmem4 {
    #[inline(always)]
    pub fn read() -> super::Pmem4 {
        super::Pmem4 {
            raw: unsafe { *((0xA0000000 + 0xA8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pmem4) {
       unsafe { *((0xA0000000 + 0xA8) as *mut u32) = val.raw; }
    }
}

pub struct Patt4 {
   raw: u32,
}

impl Patt4 {
    #[inline(always)]
    pub fn atthizx_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn atthizx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

    #[inline(always)]
    pub fn attholdx_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attholdx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

    #[inline(always)]
    pub fn attwaitx_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attwaitx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn attsetx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn attsetx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod patt4 {
    #[inline(always)]
    pub fn read() -> super::Patt4 {
        super::Patt4 {
            raw: unsafe { *((0xA0000000 + 0xAC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Patt4) {
       unsafe { *((0xA0000000 + 0xAC) as *mut u32) = val.raw; }
    }
}

pub struct Pio4 {
   raw: u32,
}

impl Pio4 {
    #[inline(always)]
    pub fn iohizx_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn iohizx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 24)) | ((val & ((1 << 8) - 1)) << 24)
    }

    #[inline(always)]
    pub fn ioholdx_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn ioholdx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 16)) | ((val & ((1 << 8) - 1)) << 16)
    }

    #[inline(always)]
    pub fn iowaitx_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn iowaitx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn iosetx_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn iosetx_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0)
    }

}

pub mod pio4 {
    #[inline(always)]
    pub fn read() -> super::Pio4 {
        super::Pio4 {
            raw: unsafe { *((0xA0000000 + 0xB0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Pio4) {
       unsafe { *((0xA0000000 + 0xB0) as *mut u32) = val.raw; }
    }
}

pub struct Bwtr1 {
   raw: u32,
}

impl Bwtr1 {
    #[inline(always)]
    pub fn accmod_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn accmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28)
    }

    #[inline(always)]
    pub fn datlat_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn datlat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn clkdiv_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn datast_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn datast_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn addhld_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addhld_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod bwtr1 {
    #[inline(always)]
    pub fn read() -> super::Bwtr1 {
        super::Bwtr1 {
            raw: unsafe { *((0xA0000000 + 0x104) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bwtr1) {
       unsafe { *((0xA0000000 + 0x104) as *mut u32) = val.raw; }
    }
}

pub struct Bwtr2 {
   raw: u32,
}

impl Bwtr2 {
    #[inline(always)]
    pub fn accmod_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn accmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28)
    }

    #[inline(always)]
    pub fn datlat_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn datlat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn clkdiv_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn datast_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn datast_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn addhld_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addhld_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod bwtr2 {
    #[inline(always)]
    pub fn read() -> super::Bwtr2 {
        super::Bwtr2 {
            raw: unsafe { *((0xA0000000 + 0x10C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bwtr2) {
       unsafe { *((0xA0000000 + 0x10C) as *mut u32) = val.raw; }
    }
}

pub struct Bwtr3 {
   raw: u32,
}

impl Bwtr3 {
    #[inline(always)]
    pub fn accmod_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn accmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28)
    }

    #[inline(always)]
    pub fn datlat_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn datlat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn clkdiv_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn datast_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn datast_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn addhld_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addhld_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod bwtr3 {
    #[inline(always)]
    pub fn read() -> super::Bwtr3 {
        super::Bwtr3 {
            raw: unsafe { *((0xA0000000 + 0x114) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bwtr3) {
       unsafe { *((0xA0000000 + 0x114) as *mut u32) = val.raw; }
    }
}

pub struct Bwtr4 {
   raw: u32,
}

impl Bwtr4 {
    #[inline(always)]
    pub fn accmod_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn accmod_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 2) - 1) << 28)) | ((val & ((1 << 2) - 1)) << 28)
    }

    #[inline(always)]
    pub fn datlat_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn datlat_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 24)) | ((val & ((1 << 4) - 1)) << 24)
    }

    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn clkdiv_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 20)) | ((val & ((1 << 4) - 1)) << 20)
    }

    #[inline(always)]
    pub fn datast_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn datast_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8)
    }

    #[inline(always)]
    pub fn addhld_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addhld_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4)
    }

    #[inline(always)]
    pub fn addset_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn addset_set(&mut self, val: u32) {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0)
    }

}

pub mod bwtr4 {
    #[inline(always)]
    pub fn read() -> super::Bwtr4 {
        super::Bwtr4 {
            raw: unsafe { *((0xA0000000 + 0x11C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bwtr4) {
       unsafe { *((0xA0000000 + 0x11C) as *mut u32) = val.raw; }
    }
}

