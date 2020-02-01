#![no_main]
#![no_std]

mod startup;
pub mod pherepherial;
pub mod spi;
pub mod uart;
pub mod system;
pub mod display;
pub mod time;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn to_u16(&self) -> u16 {
        let mut c = 0;
        c |= ((self.r >> 3) as u16) << 11;
        c |= ((self.g >> 2) as u16) << 5;
        c |= (self.b >> 3) as u16;
        c
    }
}


#[no_mangle]
pub fn main() {
    system::init();
    time::init();
    display::init();
    
    //uart::init();

    let mut count: usize = 0;

    loop {
        time::delay_ms(1000);
            
        let colors = [
            Color { r: 255, g: 0, b: 0},
            Color { r: 0, g: 255, b: 0},
            Color { r: 0, g: 0, b: 255},
        ];

        let color = colors[count % 3].to_u16();
        count += 1;
        
        for y in 0..4 {
            for x in 0..4 {
                display::set_pixel(x, y, color);
            }
        }
        
        /*
        if time::get_ms() >= (count + 1000) {
            count = time::get_ms();
           
        }
        */
        
        //display::put();
        /*
        if count % 1000 == 0 {
            display::put();
            //usart_send(0x55);
        }

        count += 1;
        */
    }
}



// if register write only - no need to read it