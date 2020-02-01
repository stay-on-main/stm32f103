use super::pherepherial::gpioa;
use super::pherepherial::rcc;
use super::pherepherial::usart1;

pub fn init() {
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

pub fn send(byte: u8) {
    while usart1::sr::txe::get() == 0 {
        break;
    }

    usart1::dr::dr::set(byte as u32);
}
