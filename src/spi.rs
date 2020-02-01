
use super::pherepherial::gpiob;
use super::pherepherial::rcc;
use super::pherepherial::spi1;
use super::pherepherial::afio;

pub fn init()
{
    rcc::apb2enr::afioen::set(1);
    // SWJ _CFG [2:0]: 010 JTAG-DP Disabled and SW-DP Enabled
    afio::mapr::swj_cfg::set(0b10);
    // SPI1_REMAP = 1
    afio::mapr::spi1_remap::set(1);

    rcc::apb2enr::iopben::set(1);
    // pb3 scl
    // 11: Output mode, max speed 50 MHz
    gpiob::crl::mode3::set(0b11);
    // 10: Alternate function output Push-pull
    gpiob::crl::cnf3::set(0b10); 
    // pb5 dio
    // 11: Output mode, max speed 50 MHz
    gpiob::crl::mode5::set(0b11);
    // 10: Alternate function output Push-pull
    gpiob::crl::cnf5::set(0b10); 
    // enable clock on spi
    rcc::apb2enr::spi1en::set(1);
    
    // BR[2:0]: Baud rate control
    // 111: fPCLK/256
    // 001: fPCLK/4
    //spi1::cr1::br::set(0b011); 
    spi1::cr1::br::set(0b111); 
    // set clock settings
    spi1::cr1::cpol::set(0);
    spi1::cr1::cpha::set(0);
    // DFF: Data frame format
    // 0: 8-bit data frame format is selected for transmission/reception
    spi1::cr1::dff::set(0);
    // LSBFIRST: Frame format
    // 0: MSB transmitted first
    spi1::cr1::lsbfirst::set(0);
    // If the NSS pin is required in output mode, the SSOE bit only should be set. 
    spi1::cr2::ssoe::set(1);
    // MSTR: Master selection
    // 1: Master configuration
    spi1::cr1::mstr::set(1);
    spi1::cr1::spe::set(1);
}

pub fn write(byte: u8) -> u8 {
    while spi1::sr::txe::get() == 0
        {}
    
    unsafe {
        core::ptr::write_volatile(0x4001300Cu32 as *mut u32, byte as u32);
    }

    //while spi1::sr::rxne::get() == 0
    //    {}
    while spi1::sr::bsy::get() == 1
        {}
    
        0
    /*
    // 0: Tx buffer not empty
    while spi1::sr::txe::get() == 0
        {}
    unsafe {
        core::ptr::write_volatile(0x4001300Cu32 as *mut u32, byte as u32);
    }
        //spi1::dr::dr::set(byte as u32);

    while spi1::sr::rxne::get() == 0
        {}

    spi1::dr::dr::get() as u8
    */
}

pub fn write_bytes(bytes: &[u8]) {
    for byte in bytes {
        while spi1::sr::txe::get() == 0
        {}
    
        unsafe {
            core::ptr::write_volatile(0x4001300Cu32 as *mut u32, *byte as u32);
        }

        //while spi1::sr::rxne::get() == 0
        //    {}
        while spi1::sr::bsy::get() == 1
        {}
    }
}