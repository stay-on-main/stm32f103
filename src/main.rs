#![no_main]
#![no_std]

mod startup;
pub mod pherepherial;

use pherepherial::gpiob;
use pherepherial::gpioa;
use pherepherial::rcc;
use pherepherial::usart1;

fn usart_init() {
    // PA9, PA10
    let mut rcc = rcc::apb2enr::read();
    rcc.iopaen_set(1);
    rcc.afioen_set(1);
    rcc::apb2enr::write(&rcc);

    let mut crh = gpioa::crh::read(); // tx
    crh.mode9_set(1); // output 50 MHz
    crh.cnf9_set(2); // Alternate Function output
    gpioa::crh::write(&crh);

    let mut crh = gpioa::crh::read(); // tx
    crh.mode10_set(0);
    crh.cnf10_set(1); // Input floating
    gpioa::crh::write(&crh);

    let mut rcc = rcc::apb2enr::read();
    rcc.usart1en_set(1);
    rcc::apb2enr::write(&rcc);
    
    // Enable the USART by writing the UE bit in USART_CR1 register to 1.
    let mut cr1 = usart1::cr1::read();
    cr1.ue_set(1);
    usart1::cr1::write(&cr1);
    // Program the M bit in USART_CR1 to define the word length.
    cr1.m_set(0); // 0: 1 Start bit, 8 Data bits, n Stop bit
    usart1::cr1::write(&cr1);
    // Program the number of stop bits in USART_CR2.
    let mut cr2 = usart1::cr2::read();
    cr2.stop_set(0); // 00: 1 Stop bit
    usart1::cr2::write(&cr2);
    // Select the desired baud rate using the USART_BRR register
    let mut brr = usart1::brr::read();
    brr.div_mantissa_set(27);
    brr.div_fraction_set(12);
    usart1::brr::write(&brr);
    // Set the TE bit in USART_CR1 to send an idle frame as first transmission
    let mut cr1 = usart1::cr1::read();
    cr1.te_set(1);
    usart1::cr1::write(&cr1);
    // Write the data to send in the USART_DR register (this clears the TXE bit).
    // Repeat this for each data to be transmitted in case of single buffer.
    let mut dr = usart1::dr::read();
    dr.dr_set(0x0f);
    usart1::dr::write(&dr);
}

fn usart_send(byte: u8) {
    while usart1::sr::read().txe_get() == 0 {
        break;
    }

    let mut dr = usart1::dr::read();
    dr.dr_set(byte as u32);
    usart1::dr::write(&dr);
}

#[no_mangle]
pub fn main() {
    let mut rcc = rcc::apb2enr::read();
    rcc.iopben_set(1);
    rcc::apb2enr::write(&rcc);

    let mut crh = gpiob::crh::read();
    crh.mode12_set(1); // output 50 MHz
    crh.cnf12_set(0); // general purpose push-pull
    gpiob::crh::write(&crh);

    let mut odr = gpiob::odr::read();
    odr.odr12_set(0);
    gpiob::odr::write(&odr);

    usart_init();

    let mut count: u32 = 0;
    loop {
        if count % 100000 == 0 {
            usart_send(0x55);
        }

        count += 1;

        /*
        let mut bsrr = pherepherial::gpiob::bsrr::read();
        //bsrr.bs4_set(1);
        //pherepherial::gpiob::bsrr::write(&bsrr);

        //bsrr.bs4_set(0);
        bsrr.br4_set(1);
        pherepherial::gpiob::bsrr::write(&bsrr);
        */
    }
}