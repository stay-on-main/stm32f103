pub mod stir {
    pub mod intid {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0xE000EF00u32 as *const u32) & 0x1FF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0xE000EF00u32 as *const u32);
                reg &= 0xFFFFFE00u32;
                reg |= val & 0x1FF;
                core::ptr::write_volatile(0xE000EF00u32 as *mut u32, reg);
            }
        }
    }
}

