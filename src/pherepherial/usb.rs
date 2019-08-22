pub struct Ep0r {
   raw: u32,
}

impl Ep0r {
    #[inline(always)]
    pub fn ea_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ea(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stat_tx_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_tx(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dtog_tx_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_tx(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ctr_tx_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_tx(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ep_kind_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ep_kind(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ep_type_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ep_type(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 9)) | ((val & ((1 << 2) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn setup_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn setup(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stat_rx_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_rx(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dtog_rx_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_rx(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_rx_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_rx(mut self, val: u32) -> Ep0r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod ep0r {
    #[inline(always)]
    pub fn read() -> super::Ep0r {
        super::Ep0r {
            raw: unsafe { *((0x40005C00 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ep0r) {
       unsafe { *((0x40005C00 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Ep1r {
   raw: u32,
}

impl Ep1r {
    #[inline(always)]
    pub fn ea_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ea(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stat_tx_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_tx(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dtog_tx_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_tx(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ctr_tx_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_tx(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ep_kind_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ep_kind(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ep_type_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ep_type(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 9)) | ((val & ((1 << 2) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn setup_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn setup(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stat_rx_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_rx(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dtog_rx_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_rx(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_rx_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_rx(mut self, val: u32) -> Ep1r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod ep1r {
    #[inline(always)]
    pub fn read() -> super::Ep1r {
        super::Ep1r {
            raw: unsafe { *((0x40005C00 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ep1r) {
       unsafe { *((0x40005C00 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Ep2r {
   raw: u32,
}

impl Ep2r {
    #[inline(always)]
    pub fn ea_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ea(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stat_tx_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_tx(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dtog_tx_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_tx(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ctr_tx_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_tx(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ep_kind_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ep_kind(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ep_type_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ep_type(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 9)) | ((val & ((1 << 2) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn setup_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn setup(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stat_rx_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_rx(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dtog_rx_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_rx(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_rx_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_rx(mut self, val: u32) -> Ep2r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod ep2r {
    #[inline(always)]
    pub fn read() -> super::Ep2r {
        super::Ep2r {
            raw: unsafe { *((0x40005C00 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ep2r) {
       unsafe { *((0x40005C00 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Ep3r {
   raw: u32,
}

impl Ep3r {
    #[inline(always)]
    pub fn ea_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ea(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stat_tx_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_tx(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dtog_tx_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_tx(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ctr_tx_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_tx(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ep_kind_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ep_kind(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ep_type_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ep_type(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 9)) | ((val & ((1 << 2) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn setup_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn setup(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stat_rx_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_rx(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dtog_rx_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_rx(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_rx_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_rx(mut self, val: u32) -> Ep3r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod ep3r {
    #[inline(always)]
    pub fn read() -> super::Ep3r {
        super::Ep3r {
            raw: unsafe { *((0x40005C00 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ep3r) {
       unsafe { *((0x40005C00 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Ep4r {
   raw: u32,
}

impl Ep4r {
    #[inline(always)]
    pub fn ea_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ea(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stat_tx_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_tx(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dtog_tx_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_tx(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ctr_tx_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_tx(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ep_kind_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ep_kind(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ep_type_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ep_type(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 9)) | ((val & ((1 << 2) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn setup_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn setup(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stat_rx_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_rx(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dtog_rx_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_rx(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_rx_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_rx(mut self, val: u32) -> Ep4r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod ep4r {
    #[inline(always)]
    pub fn read() -> super::Ep4r {
        super::Ep4r {
            raw: unsafe { *((0x40005C00 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ep4r) {
       unsafe { *((0x40005C00 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Ep5r {
   raw: u32,
}

impl Ep5r {
    #[inline(always)]
    pub fn ea_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ea(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stat_tx_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_tx(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dtog_tx_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_tx(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ctr_tx_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_tx(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ep_kind_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ep_kind(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ep_type_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ep_type(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 9)) | ((val & ((1 << 2) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn setup_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn setup(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stat_rx_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_rx(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dtog_rx_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_rx(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_rx_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_rx(mut self, val: u32) -> Ep5r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod ep5r {
    #[inline(always)]
    pub fn read() -> super::Ep5r {
        super::Ep5r {
            raw: unsafe { *((0x40005C00 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ep5r) {
       unsafe { *((0x40005C00 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Ep6r {
   raw: u32,
}

impl Ep6r {
    #[inline(always)]
    pub fn ea_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ea(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stat_tx_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_tx(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dtog_tx_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_tx(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ctr_tx_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_tx(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ep_kind_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ep_kind(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ep_type_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ep_type(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 9)) | ((val & ((1 << 2) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn setup_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn setup(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stat_rx_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_rx(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dtog_rx_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_rx(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_rx_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_rx(mut self, val: u32) -> Ep6r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod ep6r {
    #[inline(always)]
    pub fn read() -> super::Ep6r {
        super::Ep6r {
            raw: unsafe { *((0x40005C00 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ep6r) {
       unsafe { *((0x40005C00 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Ep7r {
   raw: u32,
}

impl Ep7r {
    #[inline(always)]
    pub fn ea_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ea(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn stat_tx_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_tx(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 4)) | ((val & ((1 << 2) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn dtog_tx_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_tx(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn ctr_tx_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_tx(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn ep_kind_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ep_kind(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ep_type_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn ep_type(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 9)) | ((val & ((1 << 2) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn setup_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn setup(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn stat_rx_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn stat_rx(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 2) - 1) << 12)) | ((val & ((1 << 2) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dtog_rx_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtog_rx(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_rx_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr_rx(mut self, val: u32) -> Ep7r {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x1C) as *mut u32) = self.raw; }
    }
}

pub mod ep7r {
    #[inline(always)]
    pub fn read() -> super::Ep7r {
        super::Ep7r {
            raw: unsafe { *((0x40005C00 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ep7r) {
       unsafe { *((0x40005C00 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Cntr {
   raw: u32,
}

impl Cntr {
    #[inline(always)]
    pub fn fres_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fres(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn pdwn_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pdwn(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn lpmode_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lpmode(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn fsusp_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsusp(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn resume_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn resume(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn esofm_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn esofm(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn sofm_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sofm(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn resetm_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn resetm(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn suspm_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn suspm(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn wkupm_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wkupm(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn errm_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn errm(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn pmaovrm_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pmaovrm(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctrm_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctrm(mut self, val: u32) -> Cntr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x40) as *mut u32) = self.raw; }
    }
}

pub mod cntr {
    #[inline(always)]
    pub fn read() -> super::Cntr {
        super::Cntr {
            raw: unsafe { *((0x40005C00 + 0x40) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cntr) {
       unsafe { *((0x40005C00 + 0x40) as *mut u32) = val.raw; }
    }
}

pub struct Istr {
   raw: u32,
}

impl Istr {
    #[inline(always)]
    pub fn ep_id_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn ep_id(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 4) - 1) << 0)) | ((val & ((1 << 4) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn dir_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dir(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn esof_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn esof(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn sof_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sof(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn reset_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn reset(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn susp_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn susp(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn wkup_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wkup(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn err_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn err(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn pmaovr_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pmaovr(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn ctr_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctr(mut self, val: u32) -> Istr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x44) as *mut u32) = self.raw; }
    }
}

pub mod istr {
    #[inline(always)]
    pub fn read() -> super::Istr {
        super::Istr {
            raw: unsafe { *((0x40005C00 + 0x44) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Istr) {
       unsafe { *((0x40005C00 + 0x44) as *mut u32) = val.raw; }
    }
}

pub struct Fnr {
   raw: u32,
}

impl Fnr {
    #[inline(always)]
    pub fn fn_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 11) - 1)
    }

    #[inline(always)]
    pub fn fn(mut self, val: u32) -> Fnr {
        self.raw = (self.raw & !(((1 << 11) - 1) << 0)) | ((val & ((1 << 11) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn lsof_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn lsof(mut self, val: u32) -> Fnr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 11)) | ((val & ((1 << 2) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn lck_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lck(mut self, val: u32) -> Fnr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn rxdm_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxdm(mut self, val: u32) -> Fnr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn rxdp_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxdp(mut self, val: u32) -> Fnr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x48) as *mut u32) = self.raw; }
    }
}

pub mod fnr {
    #[inline(always)]
    pub fn read() -> super::Fnr {
        super::Fnr {
            raw: unsafe { *((0x40005C00 + 0x48) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fnr) {
       unsafe { *((0x40005C00 + 0x48) as *mut u32) = val.raw; }
    }
}

pub struct Daddr {
   raw: u32,
}

impl Daddr {
    #[inline(always)]
    pub fn add_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 7) - 1)
    }

    #[inline(always)]
    pub fn add(mut self, val: u32) -> Daddr {
        self.raw = (self.raw & !(((1 << 7) - 1) << 0)) | ((val & ((1 << 7) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ef_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ef(mut self, val: u32) -> Daddr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x4C) as *mut u32) = self.raw; }
    }
}

pub mod daddr {
    #[inline(always)]
    pub fn read() -> super::Daddr {
        super::Daddr {
            raw: unsafe { *((0x40005C00 + 0x4C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Daddr) {
       unsafe { *((0x40005C00 + 0x4C) as *mut u32) = val.raw; }
    }
}

pub struct Btable {
   raw: u32,
}

impl Btable {
    #[inline(always)]
    pub fn btable_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 13) - 1)
    }

    #[inline(always)]
    pub fn btable(mut self, val: u32) -> Btable {
        self.raw = (self.raw & !(((1 << 13) - 1) << 3)) | ((val & ((1 << 13) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40005C00 + 0x50) as *mut u32) = self.raw; }
    }
}

pub mod btable {
    #[inline(always)]
    pub fn read() -> super::Btable {
        super::Btable {
            raw: unsafe { *((0x40005C00 + 0x50) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Btable) {
       unsafe { *((0x40005C00 + 0x50) as *mut u32) = val.raw; }
    }
}

