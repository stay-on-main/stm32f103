#![no_main]
#![no_std]

mod startup;
pub mod pherepherial;

use pherepherial::gpiob;
use pherepherial::gpioa;
use pherepherial::rcc;
use pherepherial::usart1;
use pherepherial::flash;
use pherepherial::stk;

fn usart_init() {
    // PA9, PA10
	rcc::apb2enr::iopaen::set(1);
    rcc::apb2enr::afioen::set(1);

    gpioa::crh::mode9::set(1);
    gpioa::crh::cnf9::set(2);

    gpioa::crh::mode10::set(0);
    gpioa::crh::cnf10::set(1);

    rcc::apb2enr::usart1en::set(1);
    // Enable the USART by writing the UE bit in USART_CR1 register to 1.
    usart1::cr1::ue::set(1);
    // Program the M bit in USART_CR1 to define the word length.
    usart1::cr2::stop::set(0);
    // Program the number of stop bits in USART_CR2.
    // Select the desired baud rate using the USART_BRR register
    usart1::brr::div_mantissa::set(0xEA);
    usart1::brr::div_fraction::set(0x6);
    // Set the TE bit in USART_CR1 to send an idle frame as first transmission
    usart1::cr1::te::set(1);
    // Write the data to send in the USART_DR register (this clears the TXE bit).
    // Repeat this for each data to be transmitted in case of single buffer.
    usart1::dr::dr::set(0xfff);
}

fn usart_send(byte: u8) {
    while usart1::sr::txe::get() == 0 {
        break;
    }

    usart1::dr::dr::set(byte as u32);
}

fn system_init()
{
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
    rcc::cfgr::pllsrc::set(1); // SYSCLK not divided, HCLK not divided,
    // Caution: The PLL output frequency must not exceed 72 MHz
    rcc::cr::pllon::set(1);

    while rcc::cr::pllrdy::get() == 0 {

    }
    // PLL selected as system clock
    rcc::cfgr::sw::set(2); 

    while rcc::cfgr::sws::get() != 2 {

    }
}

#[no_mangle]
pub fn main() {
    system_init();
    
    rcc::apb2enr::iopben::set(1);
    // output 50 MHz general purpose push-pull
    gpiob::crh::mode12::set(1);
    gpiob::crh::cnf12::set(0); 
    gpiob::odr::odr12::set(0);

    usart_init();

    stk::load_::reload::set(7200000u32);
    stk::ctrl::clksource::set(1);
    stk::ctrl::tickint::set(1);
    stk::ctrl::enable::set(1);

    let mut count: u32 = 0;
    loop {
        if count % 100000 == 0 {
            //usart_send(0x55);
        }

        count += 1;
    }
}

#[no_mangle]
pub fn systick_handler_interrupt() {
	usart_send(0x36);
}