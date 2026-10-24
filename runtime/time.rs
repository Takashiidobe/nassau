fn tagged(value: i64) -> i64 {
    (value << 1) | 1
}

#[cfg(unix)]
mod zone {
    use core::ffi::{c_char, c_int, c_long};

    #[repr(C)]
    struct Tm {
        fields: [c_int; 9],
        gmtoff: c_long,
        zone: *const c_char,
    }

    unsafe extern "C" {
        fn localtime_r(time: *const i64, out: *mut Tm) -> *mut Tm;
    }

    pub fn lookup(seconds: i64) -> (i64, bool) {
        let mut tm = Tm {
            fields: [0; 9],
            gmtoff: 0,
            zone: core::ptr::null(),
        };
        if unsafe { localtime_r(&seconds, &mut tm) }.is_null() {
            return (0, false);
        }
        (tm.gmtoff as i64, tm.fields[8] > 0)
    }
}

#[cfg(not(unix))]
mod zone {
    pub fn lookup(_seconds: i64) -> (i64, bool) {
        (0, false)
    }
}

// seconds east of UTC, doubled, with the daylight-saving flag in the low bit
#[unsafe(no_mangle)]
pub extern "C" fn nassau_time_zone(seconds: f64) -> i64 {
    let (offset, dst) = zone::lookup(seconds as i64);
    tagged(offset * 2 + i64::from(dst))
}
