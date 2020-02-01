use super::pherepherial::stk;

#[used]
static mut SYS_TICK_COUNTER_MS: u32 = 0;

#[no_mangle]
pub fn systick_handler_interrupt() {
    unsafe {
        SYS_TICK_COUNTER_MS += 1;
    }
}

pub fn init() {
    unsafe {
        SYS_TICK_COUNTER_MS = 0;
    }
    stk::load_::reload::set(72000u32);
    stk::ctrl::clksource::set(1);
    stk::ctrl::tickint::set(1);
    stk::ctrl::enable::set(1);
}

pub fn get_ms() -> u32 {
    unsafe {
        let b: *const u32 = &SYS_TICK_COUNTER_MS as *const u32;
        core::ptr::read_volatile(b)
    }
}


pub fn delay_ms(delay: u32) {
    let start_time = get_ms();

    while (start_time + delay) > get_ms() {

    }
}
