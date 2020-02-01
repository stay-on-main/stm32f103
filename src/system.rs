use super::pherepherial::rcc;
use super::pherepherial::flash;

pub fn init() {
    // turn on high speed external osicilator
    rcc::cr::hseon::set(1);
    // waiting while HSE unstable
    while rcc::cr::hserdy::get() == 0 {

    }

    flash::acr::prftbe::set(1);
    flash::acr::latency::set(2);

    rcc::cfgr::hpre::set(0);
    rcc::cfgr::ppre2::set(0);
    rcc::cfgr::ppre1::set(0);
    rcc::cfgr::pllmul::set(0b111);
    rcc::cfgr::pllxtpre::set(0);
    rcc::cfgr::pllsrc::set(1);
    // SYSCLK not divided, HCLK not divided,
    // Caution: The PLL output frequency must not exceed 72 MHz
    rcc::cr::pllon::set(1);

    while rcc::cr::pllrdy::get() == 0 {

    }
    // PLL selected as system clock
    rcc::cfgr::sw::set(2); 

    while rcc::cfgr::sws::get() != 2 {

    }
}