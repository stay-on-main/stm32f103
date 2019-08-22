pub struct Cr {
   raw: u32,
}

impl Cr {
    #[inline(always)]
    pub fn hsion_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hsion(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn hsirdy_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hsirdy(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn hsitrim_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 5) - 1)
    }

    #[inline(always)]
    pub fn hsitrim(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 5) - 1) << 3)) | ((val & ((1 << 5) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn hsical_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 8) - 1)
    }

    #[inline(always)]
    pub fn hsical(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 8) - 1) << 8)) | ((val & ((1 << 8) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn hseon_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hseon(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn hserdy_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hserdy(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn hsebyp_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hsebyp(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn csson_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn csson(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn pllon_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pllon(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn pllrdy_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pllrdy(mut self, val: u32) -> Cr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x0) as *mut u32) = self.raw; }
    }
}

pub mod cr {
    #[inline(always)]
    pub fn read() -> super::Cr {
        super::Cr {
            raw: unsafe { *((0x40021000 + 0x0) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cr) {
       unsafe { *((0x40021000 + 0x0) as *mut u32) = val.raw; }
    }
}

pub struct Cfgr {
   raw: u32,
}

impl Cfgr {
    #[inline(always)]
    pub fn sw_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn sw(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 0)) | ((val & ((1 << 2) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn sws_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn sws(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 2)) | ((val & ((1 << 2) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn hpre_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn hpre(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 4) - 1) << 4)) | ((val & ((1 << 4) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn ppre1_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ppre1(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 8)) | ((val & ((1 << 3) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn ppre2_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn ppre2(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 11)) | ((val & ((1 << 3) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn adcpre_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn adcpre(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 14)) | ((val & ((1 << 2) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn pllsrc_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pllsrc(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn pllxtpre_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pllxtpre(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn pllmul_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 4) - 1)
    }

    #[inline(always)]
    pub fn pllmul(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 4) - 1) << 18)) | ((val & ((1 << 4) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn otgfspre_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn otgfspre(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn mco_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 3) - 1)
    }

    #[inline(always)]
    pub fn mco(mut self, val: u32) -> Cfgr {
        self.raw = (self.raw & !(((1 << 3) - 1) << 24)) | ((val & ((1 << 3) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x4) as *mut u32) = self.raw; }
    }
}

pub mod cfgr {
    #[inline(always)]
    pub fn read() -> super::Cfgr {
        super::Cfgr {
            raw: unsafe { *((0x40021000 + 0x4) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cfgr) {
       unsafe { *((0x40021000 + 0x4) as *mut u32) = val.raw; }
    }
}

pub struct Cir {
   raw: u32,
}

impl Cir {
    #[inline(always)]
    pub fn lsirdyf_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsirdyf(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn lserdyf_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lserdyf(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn hsirdyf_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hsirdyf(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn hserdyf_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hserdyf(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn pllrdyf_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pllrdyf(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn cssf_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cssf(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn lsirdyie_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsirdyie(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn lserdyie_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lserdyie(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn hsirdyie_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hsirdyie(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn hserdyie_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hserdyie(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn pllrdyie_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pllrdyie(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn lsirdyc_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsirdyc(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn lserdyc_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lserdyc(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn hsirdyc_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hsirdyc(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn hserdyc_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn hserdyc(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn pllrdyc_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pllrdyc(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn cssc_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn cssc(mut self, val: u32) -> Cir {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x8) as *mut u32) = self.raw; }
    }
}

pub mod cir {
    #[inline(always)]
    pub fn read() -> super::Cir {
        super::Cir {
            raw: unsafe { *((0x40021000 + 0x8) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Cir) {
       unsafe { *((0x40021000 + 0x8) as *mut u32) = val.raw; }
    }
}

pub struct Apb2rstr {
   raw: u32,
}

impl Apb2rstr {
    #[inline(always)]
    pub fn afiorst_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn afiorst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn ioparst_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ioparst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn iopbrst_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopbrst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn iopcrst_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopcrst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn iopdrst_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopdrst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn ioperst_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn ioperst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn iopfrst_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopfrst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn iopgrst_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopgrst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn adc1rst_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc1rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn adc2rst_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc2rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn tim1rst_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim1rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn spi1rst_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn spi1rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn tim8rst_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim8rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn usart1rst_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usart1rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn adc3rst_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc3rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn tim9rst_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim9rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn tim10rst_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim10rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn tim11rst_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim11rst(mut self, val: u32) -> Apb2rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0xC) as *mut u32) = self.raw; }
    }
}

pub mod apb2rstr {
    #[inline(always)]
    pub fn read() -> super::Apb2rstr {
        super::Apb2rstr {
            raw: unsafe { *((0x40021000 + 0xC) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Apb2rstr) {
       unsafe { *((0x40021000 + 0xC) as *mut u32) = val.raw; }
    }
}

pub struct Apb1rstr {
   raw: u32,
}

impl Apb1rstr {
    #[inline(always)]
    pub fn tim2rst_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim2rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tim3rst_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim3rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn tim4rst_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim4rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn tim5rst_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim5rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn tim6rst_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim6rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn tim7rst_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim7rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn tim12rst_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim12rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn tim13rst_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim13rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn tim14rst_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim14rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn wwdgrst_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wwdgrst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn spi2rst_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn spi2rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn spi3rst_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn spi3rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn usart2rst_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usart2rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn usart3rst_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usart3rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn uart4rst_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uart4rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn uart5rst_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uart5rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn i2c1rst_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn i2c1rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn i2c2rst_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn i2c2rst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn usbrst_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usbrst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn canrst_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn canrst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn bkprst_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bkprst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn pwrrst_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwrrst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn dacrst_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dacrst(mut self, val: u32) -> Apb1rstr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x10) as *mut u32) = self.raw; }
    }
}

pub mod apb1rstr {
    #[inline(always)]
    pub fn read() -> super::Apb1rstr {
        super::Apb1rstr {
            raw: unsafe { *((0x40021000 + 0x10) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Apb1rstr) {
       unsafe { *((0x40021000 + 0x10) as *mut u32) = val.raw; }
    }
}

pub struct Ahbenr {
   raw: u32,
}

impl Ahbenr {
    #[inline(always)]
    pub fn dma1en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dma1en(mut self, val: u32) -> Ahbenr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn dma2en_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dma2en(mut self, val: u32) -> Ahbenr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn sramen_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sramen(mut self, val: u32) -> Ahbenr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn flitfen_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn flitfen(mut self, val: u32) -> Ahbenr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn crcen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn crcen(mut self, val: u32) -> Ahbenr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn fsmcen_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn fsmcen(mut self, val: u32) -> Ahbenr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn sdioen_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sdioen(mut self, val: u32) -> Ahbenr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x14) as *mut u32) = self.raw; }
    }
}

pub mod ahbenr {
    #[inline(always)]
    pub fn read() -> super::Ahbenr {
        super::Ahbenr {
            raw: unsafe { *((0x40021000 + 0x14) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Ahbenr) {
       unsafe { *((0x40021000 + 0x14) as *mut u32) = val.raw; }
    }
}

pub struct Apb2enr {
   raw: u32,
}

impl Apb2enr {
    #[inline(always)]
    pub fn afioen_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn afioen(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn iopaen_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopaen(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn iopben_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopben(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn iopcen_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopcen(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn iopden_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopden(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn iopeen_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopeen(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn iopfen_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopfen(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn iopgen_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iopgen(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn adc1en_get(&self) -> u32 {
        (self.raw >> 9) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc1en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 9)) | ((val & ((1 << 1) - 1)) << 9);
        self
    }

    #[inline(always)]
    pub fn adc2en_get(&self) -> u32 {
        (self.raw >> 10) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc2en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 10)) | ((val & ((1 << 1) - 1)) << 10);
        self
    }

    #[inline(always)]
    pub fn tim1en_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim1en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn spi1en_get(&self) -> u32 {
        (self.raw >> 12) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn spi1en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 12)) | ((val & ((1 << 1) - 1)) << 12);
        self
    }

    #[inline(always)]
    pub fn tim8en_get(&self) -> u32 {
        (self.raw >> 13) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim8en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 13)) | ((val & ((1 << 1) - 1)) << 13);
        self
    }

    #[inline(always)]
    pub fn usart1en_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usart1en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn adc3en_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn adc3en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn tim9en_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim9en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn tim10en_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim10en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn tim11en_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim11en(mut self, val: u32) -> Apb2enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x18) as *mut u32) = self.raw; }
    }
}

pub mod apb2enr {
    #[inline(always)]
    pub fn read() -> super::Apb2enr {
        super::Apb2enr {
            raw: unsafe { *((0x40021000 + 0x18) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Apb2enr) {
       unsafe { *((0x40021000 + 0x18) as *mut u32) = val.raw; }
    }
}

pub struct Apb1enr {
   raw: u32,
}

impl Apb1enr {
    #[inline(always)]
    pub fn tim2en_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim2en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn tim3en_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim3en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn tim4en_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim4en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn tim5en_get(&self) -> u32 {
        (self.raw >> 3) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim5en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 3)) | ((val & ((1 << 1) - 1)) << 3);
        self
    }

    #[inline(always)]
    pub fn tim6en_get(&self) -> u32 {
        (self.raw >> 4) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim6en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 4)) | ((val & ((1 << 1) - 1)) << 4);
        self
    }

    #[inline(always)]
    pub fn tim7en_get(&self) -> u32 {
        (self.raw >> 5) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim7en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 5)) | ((val & ((1 << 1) - 1)) << 5);
        self
    }

    #[inline(always)]
    pub fn tim12en_get(&self) -> u32 {
        (self.raw >> 6) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim12en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 6)) | ((val & ((1 << 1) - 1)) << 6);
        self
    }

    #[inline(always)]
    pub fn tim13en_get(&self) -> u32 {
        (self.raw >> 7) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim13en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 7)) | ((val & ((1 << 1) - 1)) << 7);
        self
    }

    #[inline(always)]
    pub fn tim14en_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn tim14en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 8)) | ((val & ((1 << 1) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn wwdgen_get(&self) -> u32 {
        (self.raw >> 11) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wwdgen(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 11)) | ((val & ((1 << 1) - 1)) << 11);
        self
    }

    #[inline(always)]
    pub fn spi2en_get(&self) -> u32 {
        (self.raw >> 14) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn spi2en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 14)) | ((val & ((1 << 1) - 1)) << 14);
        self
    }

    #[inline(always)]
    pub fn spi3en_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn spi3en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn usart2en_get(&self) -> u32 {
        (self.raw >> 17) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usart2en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 17)) | ((val & ((1 << 1) - 1)) << 17);
        self
    }

    #[inline(always)]
    pub fn usart3en_get(&self) -> u32 {
        (self.raw >> 18) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usart3en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 18)) | ((val & ((1 << 1) - 1)) << 18);
        self
    }

    #[inline(always)]
    pub fn uart4en_get(&self) -> u32 {
        (self.raw >> 19) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uart4en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 19)) | ((val & ((1 << 1) - 1)) << 19);
        self
    }

    #[inline(always)]
    pub fn uart5en_get(&self) -> u32 {
        (self.raw >> 20) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn uart5en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 20)) | ((val & ((1 << 1) - 1)) << 20);
        self
    }

    #[inline(always)]
    pub fn i2c1en_get(&self) -> u32 {
        (self.raw >> 21) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn i2c1en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 21)) | ((val & ((1 << 1) - 1)) << 21);
        self
    }

    #[inline(always)]
    pub fn i2c2en_get(&self) -> u32 {
        (self.raw >> 22) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn i2c2en(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 22)) | ((val & ((1 << 1) - 1)) << 22);
        self
    }

    #[inline(always)]
    pub fn usben_get(&self) -> u32 {
        (self.raw >> 23) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn usben(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 23)) | ((val & ((1 << 1) - 1)) << 23);
        self
    }

    #[inline(always)]
    pub fn canen_get(&self) -> u32 {
        (self.raw >> 25) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn canen(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 25)) | ((val & ((1 << 1) - 1)) << 25);
        self
    }

    #[inline(always)]
    pub fn bkpen_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bkpen(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn pwren_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pwren(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn dacen_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn dacen(mut self, val: u32) -> Apb1enr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x1C) as *mut u32) = self.raw; }
    }
}

pub mod apb1enr {
    #[inline(always)]
    pub fn read() -> super::Apb1enr {
        super::Apb1enr {
            raw: unsafe { *((0x40021000 + 0x1C) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Apb1enr) {
       unsafe { *((0x40021000 + 0x1C) as *mut u32) = val.raw; }
    }
}

pub struct Bdcr {
   raw: u32,
}

impl Bdcr {
    #[inline(always)]
    pub fn lseon_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lseon(mut self, val: u32) -> Bdcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn lserdy_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lserdy(mut self, val: u32) -> Bdcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn lsebyp_get(&self) -> u32 {
        (self.raw >> 2) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsebyp(mut self, val: u32) -> Bdcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 2)) | ((val & ((1 << 1) - 1)) << 2);
        self
    }

    #[inline(always)]
    pub fn rtcsel_get(&self) -> u32 {
        (self.raw >> 8) & ((1 << 2) - 1)
    }

    #[inline(always)]
    pub fn rtcsel(mut self, val: u32) -> Bdcr {
        self.raw = (self.raw & !(((1 << 2) - 1) << 8)) | ((val & ((1 << 2) - 1)) << 8);
        self
    }

    #[inline(always)]
    pub fn rtcen_get(&self) -> u32 {
        (self.raw >> 15) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rtcen(mut self, val: u32) -> Bdcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 15)) | ((val & ((1 << 1) - 1)) << 15);
        self
    }

    #[inline(always)]
    pub fn bdrst_get(&self) -> u32 {
        (self.raw >> 16) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn bdrst(mut self, val: u32) -> Bdcr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 16)) | ((val & ((1 << 1) - 1)) << 16);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x20) as *mut u32) = self.raw; }
    }
}

pub mod bdcr {
    #[inline(always)]
    pub fn read() -> super::Bdcr {
        super::Bdcr {
            raw: unsafe { *((0x40021000 + 0x20) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Bdcr) {
       unsafe { *((0x40021000 + 0x20) as *mut u32) = val.raw; }
    }
}

pub struct Csr {
   raw: u32,
}

impl Csr {
    #[inline(always)]
    pub fn lsion_get(&self) -> u32 {
        (self.raw >> 0) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsion(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 0)) | ((val & ((1 << 1) - 1)) << 0);
        self
    }

    #[inline(always)]
    pub fn lsirdy_get(&self) -> u32 {
        (self.raw >> 1) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lsirdy(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 1)) | ((val & ((1 << 1) - 1)) << 1);
        self
    }

    #[inline(always)]
    pub fn rmvf_get(&self) -> u32 {
        (self.raw >> 24) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn rmvf(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 24)) | ((val & ((1 << 1) - 1)) << 24);
        self
    }

    #[inline(always)]
    pub fn pinrstf_get(&self) -> u32 {
        (self.raw >> 26) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn pinrstf(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 26)) | ((val & ((1 << 1) - 1)) << 26);
        self
    }

    #[inline(always)]
    pub fn porrstf_get(&self) -> u32 {
        (self.raw >> 27) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn porrstf(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 27)) | ((val & ((1 << 1) - 1)) << 27);
        self
    }

    #[inline(always)]
    pub fn sftrstf_get(&self) -> u32 {
        (self.raw >> 28) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn sftrstf(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 28)) | ((val & ((1 << 1) - 1)) << 28);
        self
    }

    #[inline(always)]
    pub fn iwdgrstf_get(&self) -> u32 {
        (self.raw >> 29) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn iwdgrstf(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 29)) | ((val & ((1 << 1) - 1)) << 29);
        self
    }

    #[inline(always)]
    pub fn wwdgrstf_get(&self) -> u32 {
        (self.raw >> 30) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn wwdgrstf(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 30)) | ((val & ((1 << 1) - 1)) << 30);
        self
    }

    #[inline(always)]
    pub fn lpwrrstf_get(&self) -> u32 {
        (self.raw >> 31) & ((1 << 1) - 1)
    }

    #[inline(always)]
    pub fn lpwrrstf(mut self, val: u32) -> Csr {
        self.raw = (self.raw & !(((1 << 1) - 1) << 31)) | ((val & ((1 << 1) - 1)) << 31);
        self
    }

    #[inline(always)]
    pub fn write(self) {
       unsafe { *((0x40021000 + 0x24) as *mut u32) = self.raw; }
    }
}

pub mod csr {
    #[inline(always)]
    pub fn read() -> super::Csr {
        super::Csr {
            raw: unsafe { *((0x40021000 + 0x24) as *const u32) }
        }
    }

    #[inline(always)]
    pub fn write(val: & super::Csr) {
       unsafe { *((0x40021000 + 0x24) as *mut u32) = val.raw; }
    }
}

