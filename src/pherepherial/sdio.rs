pub struct Power {
   raw: u32,
}

impl Power {
    #[inline(always)]
    pub fn pwrctrl_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn pwrctrl(mut self, val: u32) -> Power {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod power {
    #[inline(always)]
    pub fn read() -> super::Power {
        super::Power {
            raw: unsafe { *((0x40018000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Power) {
       unsafe { *((0x40018000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Clkcr {
   raw: u32,
}

impl Clkcr {
    #[inline(always)]
    pub fn clkdiv_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn clkdiv(mut self, val: u32) -> Clkcr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 0)) | ((val & ((1 << 8) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn clken_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn clken(mut self, val: u32) -> Clkcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn pwrsav_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwrsav(mut self, val: u32) -> Clkcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn bypass_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bypass(mut self, val: u32) -> Clkcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn widbus_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn widbus(mut self, val: u32) -> Clkcr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 11)) | ((val & ((1 << 2) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn negedge_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn negedge(mut self, val: u32) -> Clkcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn hwfc_en_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hwfc_en(mut self, val: u32) -> Clkcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod clkcr {
    #[inline(always)]
    pub fn read() -> super::Clkcr {
        super::Clkcr {
            raw: unsafe { *((0x40018000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Clkcr) {
       unsafe { *((0x40018000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Arg {
   raw: u32,
}

impl Arg {
    #[inline(always)]
    pub fn cmdarg_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn cmdarg(mut self, val: u32) -> Arg {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod arg {
    #[inline(always)]
    pub fn read() -> super::Arg {
        super::Arg {
            raw: unsafe { *((0x40018000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Arg) {
       unsafe { *((0x40018000 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Cmd {
   raw: u32,
}

impl Cmd {
    #[inline(always)]
    pub fn cmdindex_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn cmdindex(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 6) - 1) << 0)) | ((val & ((1 << 6) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn waitresp_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn waitresp(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 2) - 1) << 6)) | ((val & ((1 << 2) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn waitint_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitint(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn waitpend_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn waitpend(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn cpsmen_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cpsmen(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn sdiosuspend_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sdiosuspend(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn encmdcompl_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn encmdcompl(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn nien_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn nien(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn ce_atacmd_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ce_atacmd(mut self, val: u32) -> Cmd {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod cmd {
    #[inline(always)]
    pub fn read() -> super::Cmd {
        super::Cmd {
            raw: unsafe { *((0x40018000 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cmd) {
       unsafe { *((0x40018000 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Respcmd {
   raw: u32,
}

impl Respcmd {
    #[inline(always)]
    pub fn respcmd_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 6) - 1)
    }

    #[inline(always)]
    pub fn respcmd(mut self, val: u32) -> Respcmd {
        self.raw = (self.raw & !(((1 << 6) - 1) << 0)) | ((val & ((1 << 6) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod respcmd {
    #[inline(always)]
    pub fn read() -> super::Respcmd {
        super::Respcmd {
            raw: unsafe { *((0x40018000 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Respcmd) {
       unsafe { *((0x40018000 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Respi1 {
   raw: u32,
}

impl Respi1 {
    #[inline(always)]
    pub fn cardstatus1_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn cardstatus1(mut self, val: u32) -> Respi1 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod respi1 {
    #[inline(always)]
    pub fn read() -> super::Respi1 {
        super::Respi1 {
            raw: unsafe { *((0x40018000 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Respi1) {
       unsafe { *((0x40018000 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Resp2 {
   raw: u32,
}

impl Resp2 {
    #[inline(always)]
    pub fn cardstatus2_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn cardstatus2(mut self, val: u32) -> Resp2 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod resp2 {
    #[inline(always)]
    pub fn read() -> super::Resp2 {
        super::Resp2 {
            raw: unsafe { *((0x40018000 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Resp2) {
       unsafe { *((0x40018000 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Resp3 {
   raw: u32,
}

impl Resp3 {
    #[inline(always)]
    pub fn cardstatus3_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn cardstatus3(mut self, val: u32) -> Resp3 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x1C) as *mut u32) = self.raw; }
    }
}

pub mod resp3 {
    #[inline(always)]
    pub fn read() -> super::Resp3 {
        super::Resp3 {
            raw: unsafe { *((0x40018000 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Resp3) {
       unsafe { *((0x40018000 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Resp4 {
   raw: u32,
}

impl Resp4 {
    #[inline(always)]
    pub fn cardstatus4_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn cardstatus4(mut self, val: u32) -> Resp4 {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x20) as *mut u32) = self.raw; }
    }
}

pub mod resp4 {
    #[inline(always)]
    pub fn read() -> super::Resp4 {
        super::Resp4 {
            raw: unsafe { *((0x40018000 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Resp4) {
       unsafe { *((0x40018000 + 0x20) as *mut u32) = val.raw; }
    }
}

pub struct Dtimer {
   raw: u32,
}

impl Dtimer {
    #[inline(always)]
    pub fn datatime_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn datatime(mut self, val: u32) -> Dtimer {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x24) as *mut u32) = self.raw; }
    }
}

pub mod dtimer {
    #[inline(always)]
    pub fn read() -> super::Dtimer {
        super::Dtimer {
            raw: unsafe { *((0x40018000 + 0x24) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dtimer) {
       unsafe { *((0x40018000 + 0x24) as *mut u32) = val.raw; }
    }
}

pub struct Dlen {
   raw: u32,
}

impl Dlen {
    #[inline(always)]
    pub fn datalength_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 25) - 1)
    }

    #[inline(always)]
    pub fn datalength(mut self, val: u32) -> Dlen {
        self.raw = (self.raw & !(((1 << 25) - 1) << 0)) | ((val & ((1 << 25) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x28) as *mut u32) = self.raw; }
    }
}

pub mod dlen {
    #[inline(always)]
    pub fn read() -> super::Dlen {
        super::Dlen {
            raw: unsafe { *((0x40018000 + 0x28) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dlen) {
       unsafe { *((0x40018000 + 0x28) as *mut u32) = val.raw; }
    }
}

pub struct Dctrl {
   raw: u32,
}

impl Dctrl {
    #[inline(always)]
    pub fn dten_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dten(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn dtdir_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtdir(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn dtmode_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtmode(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn dmaen_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dmaen(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn dblocksize_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn dblocksize(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn pwstart_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwstart(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn pwstop_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwstop(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn rwmod_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rwmod(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn sdioen_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sdioen(mut self, val: u32) -> Dctrl {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x2C) as *mut u32) = self.raw; }
    }
}

pub mod dctrl {
    #[inline(always)]
    pub fn read() -> super::Dctrl {
        super::Dctrl {
            raw: unsafe { *((0x40018000 + 0x2C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dctrl) {
       unsafe { *((0x40018000 + 0x2C) as *mut u32) = val.raw; }
    }
}

pub struct Dcount {
   raw: u32,
}

impl Dcount {
    #[inline(always)]
    pub fn datacount_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 25) - 1)
    }

    #[inline(always)]
    pub fn datacount(mut self, val: u32) -> Dcount {
        self.raw = (self.raw & !(((1 << 25) - 1) << 0)) | ((val & ((1 << 25) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x30) as *mut u32) = self.raw; }
    }
}

pub mod dcount {
    #[inline(always)]
    pub fn read() -> super::Dcount {
        super::Dcount {
            raw: unsafe { *((0x40018000 + 0x30) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Dcount) {
       unsafe { *((0x40018000 + 0x30) as *mut u32) = val.raw; }
    }
}

pub struct Sta {
   raw: u32,
}

impl Sta {
    #[inline(always)]
    pub fn ccrcfail_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ccrcfail(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn dcrcfail_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dcrcfail(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ctimeout_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctimeout(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn dtimeout_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtimeout(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn txunderr_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txunderr(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn rxoverr_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxoverr(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn cmdrend_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmdrend(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn cmdsent_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmdsent(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn dataend_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dataend(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn stbiterr_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stbiterr(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dbckend_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbckend(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cmdact_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmdact(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn txact_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txact(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn rxact_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxact(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn txfifohe_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfifohe(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn rxfifohf_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxfifohf(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn txfifof_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfifof(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn rxfifof_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxfifof(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn txfifoe_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfifoe(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn rxfifoe_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxfifoe(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn txdavl_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txdavl(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn rxdavl_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxdavl(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn sdioit_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sdioit(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn ceataend_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ceataend(mut self, val: u32) -> Sta {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x34) as *mut u32) = self.raw; }
    }
}

pub mod sta {
    #[inline(always)]
    pub fn read() -> super::Sta {
        super::Sta {
            raw: unsafe { *((0x40018000 + 0x34) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Sta) {
       unsafe { *((0x40018000 + 0x34) as *mut u32) = val.raw; }
    }
}

pub struct Icr {
   raw: u32,
}

impl Icr {
    #[inline(always)]
    pub fn ccrcfailc_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ccrcfailc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn dcrcfailc_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dcrcfailc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ctimeoutc_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctimeoutc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn dtimeoutc_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtimeoutc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn txunderrc_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txunderrc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn rxoverrc_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxoverrc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn cmdrendc_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmdrendc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn cmdsentc_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmdsentc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn dataendc_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dataendc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn stbiterrc_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stbiterrc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dbckendc_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbckendc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn sdioitc_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sdioitc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn ceataendc_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ceataendc(mut self, val: u32) -> Icr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x38) as *mut u32) = self.raw; }
    }
}

pub mod icr {
    #[inline(always)]
    pub fn read() -> super::Icr {
        super::Icr {
            raw: unsafe { *((0x40018000 + 0x38) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Icr) {
       unsafe { *((0x40018000 + 0x38) as *mut u32) = val.raw; }
    }
}

pub struct Mask {
   raw: u32,
}

impl Mask {
    #[inline(always)]
    pub fn ccrcfailie_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ccrcfailie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn dcrcfailie_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dcrcfailie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn ctimeoutie_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ctimeoutie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn dtimeoutie_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dtimeoutie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn txunderrie_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txunderrie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn rxoverrie_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxoverrie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn cmdrendie_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmdrendie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn cmdsentie_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmdsentie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn dataendie_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dataendie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn stbiterrie_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn stbiterrie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn dbackendie_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dbackendie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn cmdactie_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cmdactie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn txactie_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txactie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn rxactie_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxactie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn txfifoheie_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfifoheie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn rxfifohfie_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxfifohfie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn txfifofie_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfifofie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn rxfifofie_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxfifofie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn txfifoeie_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txfifoeie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn rxfifoeie_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxfifoeie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn txdavlie_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn txdavlie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn rxdavlie_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rxdavlie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn sdioitie_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sdioitie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn ceatendie_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ceatendie(mut self, val: u32) -> Mask {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x3C) as *mut u32) = self.raw; }
    }
}

pub mod mask {
    #[inline(always)]
    pub fn read() -> super::Mask {
        super::Mask {
            raw: unsafe { *((0x40018000 + 0x3C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Mask) {
       unsafe { *((0x40018000 + 0x3C) as *mut u32) = val.raw; }
    }
}

pub struct Fifocnt {
   raw: u32,
}

impl Fifocnt {
    #[inline(always)]
    pub fn fif0count_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 24) - 1)
    }

    #[inline(always)]
    pub fn fif0count(mut self, val: u32) -> Fifocnt {
        self.raw = (self.raw & !(((1 << 24) - 1) << 0)) | ((val & ((1 << 24) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x48) as *mut u32) = self.raw; }
    }
}

pub mod fifocnt {
    #[inline(always)]
    pub fn read() -> super::Fifocnt {
        super::Fifocnt {
            raw: unsafe { *((0x40018000 + 0x48) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fifocnt) {
       unsafe { *((0x40018000 + 0x48) as *mut u32) = val.raw; }
    }
}

pub struct Fifo {
   raw: u32,
}

impl Fifo {
    #[inline(always)]
    pub fn fifodata_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 32) - 1)
    }

    #[inline(always)]
    pub fn fifodata(mut self, val: u32) -> Fifo {
        self.raw = (self.raw & !(((1 << 32) - 1) << 0)) | ((val & ((1 << 32) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40018000 + 0x80) as *mut u32) = self.raw; }
    }
}

pub mod fifo {
    #[inline(always)]
    pub fn read() -> super::Fifo {
        super::Fifo {
            raw: unsafe { *((0x40018000 + 0x80) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Fifo) {
       unsafe { *((0x40018000 + 0x80) as *mut u32) = val.raw; }
    }
}

