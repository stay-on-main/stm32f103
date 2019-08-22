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
	rcc::apb2enr::read().iopaen(3).afioen(1).write();
    gpioa::crh::read().mode9(1).cnf9(2).write();
    gpioa::crh::read().mode10(0).cnf10(1).write();

    rcc::apb2enr::read().usart1en(1).write();
    // Enable the USART by writing the UE bit in USART_CR1 register to 1.
    usart1::cr1::read().ue(1).write();
    // Program the M bit in USART_CR1 to define the word length.
    usart1::cr2::read().stop(0).write();
    // Program the number of stop bits in USART_CR2.
    // Select the desired baud rate using the USART_BRR register
    usart1::brr::read().div_mantissa(27).div_fraction(12).write();
    // Set the TE bit in USART_CR1 to send an idle frame as first transmission
    usart1::cr1::read().te(1).write();
    // Write the data to send in the USART_DR register (this clears the TXE bit).
    // Repeat this for each data to be transmitted in case of single buffer.
    usart1::dr::read().dr(0x0f).write();
}

fn usart_send(byte: u8) {
    while usart1::sr::read().txe_get() == 0 {
        break;
    }

    usart1::dr::read().dr(byte as u32).write();
}

fn system_init()
{
    // turn on high speed external osicilator
    rcc::cr::read().hseon(1).write();
    // waiting while HSE unstable
    while rcc::cr::read().hserdy_get() == 0 {

    }

    flash::acr::read().prftbe(1).latency(2).write();

    rcc::cfgr::read().hpre(0).ppre2(0).ppre1(0).pllmul(0b111).pllxtpre(0).pllsrc(1).write(); // SYSCLK not divided, HCLK not divided,
    // Caution: The PLL output frequency must not exceed 72 MHz
    rcc::cr::read().pllon(1).write();

    while rcc::cr::read().pllrdy_get() == 0 {

    }
    // PLL selected as system clock
    rcc::cfgr::read().sw(2).write(); 

    while rcc::cfgr::read().sws_get() != 2 {

    }
}

#[no_mangle]
pub fn main() {
    system_init();
    

    rcc::apb2enr::read().iopben(1).write();
    // output 50 MHz general purpose push-pull
    gpiob::crh::read().mode12(1).cnf12(0).write(); 
    gpiob::odr::read().odr12(0).write();

    usart_init();

    stk::load_::read().reload(7200000u32).write();
    stk::ctrl::read().clksource(1).tickint(1).enable(1).write();


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
	usart_send(0x55);
}