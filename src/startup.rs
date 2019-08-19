use core::panic::PanicInfo;

extern "C" {
    static mut _ebss: u32;
    static mut _sbss: u32;
}

extern {
    fn main();
}

extern "C" fn hardfault_handler() {
    loop { }
}

extern "C" fn nmi_handler() {
    loop { }
}

extern "C" fn default_handler() {
    loop { }
}

extern "C" fn reset_handler() {
    unsafe {
        let mut bss_start:u32 = (&_sbss as *const _) as u32;
        let end_bss: u32 = (&_ebss as *const _) as u32;

        while bss_start != end_bss {
            *(bss_start as *mut u32) = 0;
            bss_start += 4;
        }
    }

	/*
    let mut reg = mem_read(SYSAHBCLKCTRL);
    reg |= 1 << 6;
    mem_write(SYSAHBCLKCTRL, reg);
	mem_write(SYST_RVR, 6000);
	mem_write(SYST_CSR, 3); // clock source is 6MHz
	*/
    unsafe {
    main();
    }
}
/*
extern "C" fn systick_handler() {
	unsafe {
		COUNTER += 1;

        if COUNTER != 0 && COUNTER % 100 == 0  && OS_ENABLE == true {
            mem_write(ICSR, 0x10000000);
        }
    }
}
*/

#[link_section = ".reset_handler"]
#[used]
static RESET_HANDLER: [extern "C" fn(); 15] = [
    reset_handler, 		// 0x00
    nmi_handler,		// 0x04
    hardfault_handler,  // 0x08
    default_handler,	// 0x0C
    default_handler,	// 0x10
    default_handler,	// 0x14
    default_handler,	// 0x18
    default_handler,	// 0x1C
    default_handler,	// 0x20
    default_handler,	// 0x24
    default_handler,	// 0x28
    default_handler,	// 0x2C
    default_handler,	// 0x30
    default_handler,//pend_sv_handler,	// 0x34
    default_handler,//systick_handler,	// 0x38
];

#[panic_handler]
fn panic_impl(_info: &PanicInfo) -> ! {
    loop {

    }
}