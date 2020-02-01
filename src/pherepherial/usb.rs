pub mod ep0r {
    pub mod ea {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C00u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_kind {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_type {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 9) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFF9FFu32;
                reg |= (val & 0x3) << 9;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod setup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C00u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C00u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C00u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ep1r {
    pub mod ea {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C04u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_kind {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_type {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 9) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFF9FFu32;
                reg |= (val & 0x3) << 9;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod setup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C04u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C04u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C04u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ep2r {
    pub mod ea {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C08u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_kind {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_type {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 9) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFF9FFu32;
                reg |= (val & 0x3) << 9;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod setup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C08u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C08u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C08u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ep3r {
    pub mod ea {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C0Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_kind {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_type {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 9) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFF9FFu32;
                reg |= (val & 0x3) << 9;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod setup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C0Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C0Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C0Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod ep4r {
    pub mod ea {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C10u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_kind {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_type {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 9) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFF9FFu32;
                reg |= (val & 0x3) << 9;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod setup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C10u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C10u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C10u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ep5r {
    pub mod ea {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C14u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_kind {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_type {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 9) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFF9FFu32;
                reg |= (val & 0x3) << 9;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod setup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C14u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C14u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C14u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ep6r {
    pub mod ea {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C18u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_kind {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_type {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 9) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFF9FFu32;
                reg |= (val & 0x3) << 9;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod setup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C18u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C18u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C18u32 as *mut u32, reg);
            }
        }
    }
}

pub mod ep7r {
    pub mod ea {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C1Cu32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 4) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFFFCFu32;
                reg |= (val & 0x3) << 4;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 6) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFFFBFu32;
                reg |= (val & 0x1) << 6;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_tx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_kind {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ep_type {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 9) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFF9FFu32;
                reg |= (val & 0x3) << 9;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod setup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod stat_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 12) & 0x3
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFCFFFu32;
                reg |= (val & 0x3) << 12;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod dtog_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr_rx {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C1Cu32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C1Cu32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C1Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod cntr {
    pub mod fres {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C40u32 as *const u32) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFFFFEu32;
                reg |= val & 0x1;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod pdwn {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 1) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFFFFDu32;
                reg |= (val & 0x1) << 1;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod lpmode {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 2) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFFFFBu32;
                reg |= (val & 0x1) << 2;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod fsusp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 3) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFFFF7u32;
                reg |= (val & 0x1) << 3;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod resume {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod esofm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod sofm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod resetm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod suspm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod wkupm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod errm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod pmaovrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctrm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C40u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C40u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C40u32 as *mut u32, reg);
            }
        }
    }
}

pub mod istr {
    pub mod ep_id {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C44u32 as *const u32) & 0xF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFFFF0u32;
                reg |= val & 0xF;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod dir {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 4) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFFFEFu32;
                reg |= (val & 0x1) << 4;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod esof {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 8) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFFEFFu32;
                reg |= (val & 0x1) << 8;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod sof {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 9) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFFDFFu32;
                reg |= (val & 0x1) << 9;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod reset {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 10) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFFBFFu32;
                reg |= (val & 0x1) << 10;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod susp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 11) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFF7FFu32;
                reg |= (val & 0x1) << 11;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod wkup {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 12) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFEFFFu32;
                reg |= (val & 0x1) << 12;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod err {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 13) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFDFFFu32;
                reg |= (val & 0x1) << 13;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod pmaovr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 14) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFFBFFFu32;
                reg |= (val & 0x1) << 14;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
    pub mod ctr {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C44u32 as *const u32) >> 15) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C44u32 as *const u32);
                reg &= 0xFFFF7FFFu32;
                reg |= (val & 0x1) << 15;
                core::ptr::write_volatile(0x40005C44u32 as *mut u32, reg);
            }
        }
    }
}

pub mod fnr {
    pub mod fn {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C48u32 as *const u32) & 0x7FF
            }
        }

    }
    pub mod lsof {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C48u32 as *const u32) >> 11) & 0x3
            }
        }

    }
    pub mod lck {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C48u32 as *const u32) >> 13) & 0x1
            }
        }

    }
    pub mod rxdm {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C48u32 as *const u32) >> 14) & 0x1
            }
        }

    }
    pub mod rxdp {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C48u32 as *const u32) >> 15) & 0x1
            }
        }

    }
}

pub mod daddr {
    pub mod add {
        pub fn get() -> u32 {
            unsafe {
                core::ptr::read_volatile(0x40005C4Cu32 as *const u32) & 0x7F
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C4Cu32 as *const u32);
                reg &= 0xFFFFFF80u32;
                reg |= val & 0x7F;
                core::ptr::write_volatile(0x40005C4Cu32 as *mut u32, reg);
            }
        }
    }
    pub mod ef {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C4Cu32 as *const u32) >> 7) & 0x1
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C4Cu32 as *const u32);
                reg &= 0xFFFFFF7Fu32;
                reg |= (val & 0x1) << 7;
                core::ptr::write_volatile(0x40005C4Cu32 as *mut u32, reg);
            }
        }
    }
}

pub mod btable {
    pub mod btable {
        pub fn get() -> u32 {
            unsafe {
                (core::ptr::read_volatile(0x40005C50u32 as *const u32) >> 3) & 0x1FFF
            }
        }

        pub fn set(val: u32) {
            unsafe {
                let mut reg = core::ptr::read_volatile(0x40005C50u32 as *const u32);
                reg &= 0xFFFF0007u32;
                reg |= (val & 0x1FFF) << 3;
                core::ptr::write_volatile(0x40005C50u32 as *mut u32, reg);
            }
        }
    }
}

