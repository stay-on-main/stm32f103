pub struct Idcode {
   raw: u32,
}

impl Idcode {
    #[inline(always)]
    pub fn dev_id_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 12) - 1)
    }

    #[inline(always)]
    pub fn dev_id(mut self, val: u32) -> Idcode {
        self.raw = (self.raw & !(((1 << 12) - 1) << 0)) | ((val & ((1 << 12) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn rev_id_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 16) - 1)
    }

    #[inline(always)]
    pub fn rev_id(mut self, val: u32) -> Idcode {
        self.raw = (self.raw & !(((1 << 16) - 1) << 16)) | ((val & ((1 << 16) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE0042000 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod idcode {
    #[inline(always)]
    pub fn read() -> super::Idcode {
        super::Idcode {
            raw: unsafe { *((0xE0042000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Idcode) {
       unsafe { *((0xE0042000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn dbg_sleep_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_sleep(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn dbg_stop_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn dbg_standby_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_standby(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn trace_ioen_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn trace_ioen(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn trace_mode_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn trace_mode(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 6)) | ((val & ((1 << 2) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn dbg_iwdg_stop_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_iwdg_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn dbg_wwdg_stop_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_wwdg_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dbg_tim1_stop_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_tim1_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn dbg_tim2_stop_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_tim2_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn dbg_tim3_stop_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_tim3_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn dbg_tim4_stop_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_tim4_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn dbg_can1_stop_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_can1_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn dbg_tim8_stop_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_tim8_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn dbg_tim5_stop_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_tim5_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn dbg_tim6_stop_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_tim6_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn dbg_tim7_stop_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_tim7_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn dbg_can2_stop_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbg_can2_stop(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0xE0042000 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0xE0042000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0xE0042000 + 0x4) as *mut u32) = val.raw; }
    }
}

