

use super::pherepherial::gpiob;
use super::pherepherial::rcc;
use super::spi;
use super::time;
use super::pherepherial::spi1;

fn cs_init() {
    rcc::apb2enr::iopben::set(1);
    // pb8 cs
    // 11: Output mode, max speed 50 MHz
    gpiob::crh::mode8::set(0b11);
    // 00: General purpose output push-pull
    gpiob::crh::cnf8::set(0b00);
}

fn cs_set(value: bool) {
    if value {
        gpiob::bsrr::bs8::set(1);
    } else {
        gpiob::bsrr::br8::set(1);
    }
}

fn rs_init() {
    rcc::apb2enr::iopben::set(1);
    // pb6 rs
    // 11: Output mode, max speed 50 MHz
    gpiob::crl::mode6::set(0b11);
    // 00: General purpose output push-pull
    gpiob::crl::cnf6::set(0b00);
}

#[inline]
fn rs_set(value: bool) {
    if value {
        gpiob::bsrr::bs6::set(1);
    } else {
        gpiob::bsrr::br6::set(1);
    }
}

fn rst_init() {
    rcc::apb2enr::iopben::set(1);
    // pb7 rsr
    // 11: Output mode, max speed 50 MHz
    gpiob::crl::mode7::set(0b11);
    // 00: General purpose output push-pull
    gpiob::crl::cnf7::set(0b00);
}

fn rst_set(value: bool) {
    if value {
        gpiob::odr::odr7::set(1);
    } else {
        gpiob::odr::odr7::set(0);
    }
}

fn write_data(data: u8) {
    spi::write(data);
}

fn write_cmd(cmd: u8) {
    // If D/CX is “low”, the transmission byte is interpreted as a command byte
    rs_set(false);
    cs_set(false);
    write_data(cmd);
    rs_set(true);
}

pub fn init() {
    rst_init();
    rst_set(false);

    spi::init();
    
    cs_init();
    cs_set(true);
    
    rs_init();
    rs_set(true);

    rst_set(true);

    time::delay_ms(500);

    let init_sequence = [
        DataType::Cmd(0x11),//Sleep exit
        DataType::Delay(250),
        //ST7735R Frame Rate
        DataType::Cmd(0xB1),
        DataType::Data(0x01),
        DataType::Data(0x2C),
        DataType::Data(0x2D),
        DataType::Cmd(0xB2),
        DataType::Data(0x01),
        DataType::Data(0x2C),
        DataType::Data(0x2D),
        DataType::Cmd(0xB3),
        DataType::Data(0x01),
        DataType::Data(0x2C),
        DataType::Data(0x2D),
        DataType::Data(0x01),
        DataType::Data(0x2C),
        DataType::Data(0x2D),
    
        DataType::Cmd(0xB4), //Column inversion 
        DataType::Data(0x07),
     
        //ST7735R Power Sequence
        DataType::Cmd(0xC0),
        DataType::Data(0xA2),
        DataType::Data(0x02),
        DataType::Data(0x84),
        DataType::Cmd(0xC1),
        DataType::Data(0xC5),
        DataType::Cmd(0xC2),
        DataType::Data(0x0A),
        DataType::Data(0x00),
        DataType::Cmd(0xC3),
        DataType::Data(0x8A),
        DataType::Data(0x2A),
        DataType::Cmd(0xC4),
        DataType::Data(0x8A),
        DataType::Data(0xEE),
     
        DataType::Cmd(0xC5), //VCOM 
        DataType::Data(0x0E), 
     
        DataType::Cmd(0x36), //MX, MY, RGB mode 
        DataType::Data(0xC8), 
    
        //ST7735R Gamma Sequence
        DataType::Cmd(0xe0), 
        DataType::Data(0x0f),
        DataType::Data(0x1a), 
        DataType::Data(0x0f),
        DataType::Data(0x18),
        DataType::Data(0x2f),
        DataType::Data(0x28),
        DataType::Data(0x20),
        DataType::Data(0x22),
        DataType::Data(0x1f),
        DataType::Data(0x1b),
        DataType::Data(0x23),
        DataType::Data(0x37),
        DataType::Data(0x00),
    
        DataType::Data(0x07),
        DataType::Data(0x02),
        DataType::Data(0x10),
        DataType::Cmd(0xe1),
        DataType::Data(0x0f),
        DataType::Data(0x1b),
        DataType::Data(0x0f),
        DataType::Data(0x17),
        DataType::Data(0x33),
        DataType::Data(0x2c),
        DataType::Data(0x29),
        DataType::Data(0x2e),
        DataType::Data(0x30),
        DataType::Data(0x30),
        DataType::Data(0x39),
        DataType::Data(0x3f),
        DataType::Data(0x00),
        DataType::Data(0x07),
        DataType::Data(0x03),
        DataType::Data(0x10),
    
        DataType::Cmd(0x2a),
        DataType::Data(0x00),
        DataType::Data(0x00),
        DataType::Data(0x00),
        DataType::Data(0x7f),
        DataType::Cmd(0x2b),
        DataType::Data(0x00),
        DataType::Data(0x00),
        DataType::Data(0x00),
        DataType::Data(0x9f),
    
        DataType::Cmd(0xF0), //Enable test command  
        DataType::Data(0x01), 
        DataType::Cmd(0xF6), //Disable ram power save mode 
        DataType::Data(0x00), 
     
        DataType::Cmd(0x3A), //65k mode 
        DataType::Data(0x05), 
        DataType::Cmd(0x29),//Display on

    ];

    for byte in init_sequence.iter() {
        match byte {
            DataType::Cmd(cmd) => write_cmd(*cmd),
            DataType::Data(data) => write_data(*data),
            DataType::Delay(delay) => time::delay_ms(*delay),
        }
    }

    cs_set(true);
    time::delay_ms(1);
}

pub fn set_pixel(x: u8, y: u8, color: u16) {
    write_cmd(0x2a); // Column address set
    
    let data = [0, x, 0, x + 1];
    spi::write_bytes(&data);

    write_cmd(0x2b); // Column address set

    let data = [0, y, 0, y + 1];
    spi::write_bytes(&data);

    write_cmd(0x2c); // Memory write

    write_data((color >> 8) as u8);
    write_data(color as u8);
    cs_set(true);
}

pub fn fill(color: u16)
{
    write_cmd(0x2a); // Column address set
    
    let data = [0, 0, 0, 128];
    spi::write_bytes(&data);
    write_cmd(0x2b); // Column address set

    let data = [0, 0, 0, 160];
    spi::write_bytes(&data);

    write_cmd(0x2c); // Memory write

    for _ in 0..128 * 160 {

        while spi1::sr::txe::get() == 0
            {}
    
        unsafe {
            core::ptr::write_volatile(0x4001300Cu32 as *mut u32, (color >> 8) as u32);
        }

        while spi1::sr::txe::get() == 0
            {}
    
        unsafe {
            core::ptr::write_volatile(0x4001300Cu32 as *mut u32, (color & 0xff) as u32);
        }
    }

    while spi1::sr::rxne::get() == 0
        {}

    while spi1::sr::bsy::get() == 1
        {}
    
}

pub fn set_window(x0: u8, y0: u8, x1: u8, y1: u8)
{
    write_cmd(0x2a); // Column address set
    
    let data = [0, x0, 0, x1];
    spi::write_bytes(&data);

    write_cmd(0x2b); // Column address set
    let data = [0, y0, 0, y1];
    spi::write_bytes(&data);
    write_cmd(0x2c); // Memory write
    cs_set(true);
}

pub fn write(colors: &[u16]) {
    
    cs_set(false);
    for color in colors {
        write_data((*color >> 8) as u8);
        write_data(*color as u8);
    }
    cs_set(true);
}

enum DataType {
    Cmd(u8),
    Data(u8),
    Delay(u32),
}


// itdb02-1.8sp