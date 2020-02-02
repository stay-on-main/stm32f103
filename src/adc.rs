use super::pherepherial::adc1;
use super::pherepherial::rcc;
use super::pherepherial::gpiob;

pub fn init()
{
    rcc::apb2enr::iopben::set(1);
    // pb1 ADC input
    // 00: Input mode (reset state)
    gpiob::crl::mode1::set(0b00);
    // 00: Analog mode
    gpiob::crl::cnf1::set(0b00);
    // 11: PCLK2 divided by 8
    rcc::cfgr::adcpre::set(0b00);

    // enable clock on ADC
    rcc::apb2enr::adc1en::set(1);
    // The ADC can be powered-on by setting the ADON bit in the ADC_CR2 register
    adc1::cr2::adon::set(1);
    // start auto-calibration
    adc1::cr2::cal::set(1);

    while adc1::cr2::cal::get() != 0
        {}
    // one shot
    adc1::cr2::cont::set(0);
    // SMPx[2:0]: Channel x Sample time selection
    // 010: 13.5 cycles
    adc1::smpr2::smp9::set(0b010);
    // EXTSEL[2:0]: SWSTART Software control bit 111
    adc1::cr2::extsel::set(0b111);
    adc1::cr2::exttrig::set(1);
    adc1::sqr3::sq1::set(9);
}

pub fn get() -> u32 {
    adc1::cr2::swstart::set(1);

    while adc1::sr::eoc::get() == 0 
        {}

    adc1::dr::data::get()
}

