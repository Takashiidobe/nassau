const NEAREST: i64 = 0;

#[cfg(not(target_arch = "wasm32"))]
mod hardware {
    use core::ffi::c_int;

    unsafe extern "C" {
        fn fegetround() -> c_int;
        fn fesetround(mode: c_int) -> c_int;
    }

    #[cfg(target_arch = "x86_64")]
    const MODES: [c_int; 4] = [0, 0x400, 0x800, 0xc00];
    #[cfg(target_arch = "aarch64")]
    const MODES: [c_int; 4] = [0, 0x80_0000, 0x40_0000, 0xc0_0000];

    pub fn get() -> Option<i64> {
        let current = unsafe { fegetround() };
        MODES
            .iter()
            .position(|mode| *mode == current)
            .map(|index| index as i64)
    }

    pub fn set(mode: i64) -> bool {
        unsafe { fesetround(MODES[mode as usize]) == 0 }
    }
}

#[cfg(target_arch = "wasm32")]
mod hardware {
    pub fn get() -> Option<i64> {
        Some(super::NEAREST)
    }

    pub fn set(mode: i64) -> bool {
        mode == super::NEAREST
    }
}

fn tagged(value: i64) -> i64 {
    (value << 1) | 1
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_fp_get_round() -> i64 {
    tagged(hardware::get().unwrap_or(NEAREST))
}

// modes are nearest, neginf, posinf, zero; returns 1 when the target cannot use the mode
#[unsafe(no_mangle)]
pub extern "C" fn nassau_fp_set_round(mode: i64) -> i64 {
    let mode = mode >> 1;
    let unsupported = !(0..=3).contains(&mode) || !hardware::set(mode);
    tagged(i64::from(unsupported))
}
