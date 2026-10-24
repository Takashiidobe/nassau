fn tagged(value: i64) -> i64 {
    (value << 1) | 1
}

#[cfg(unix)]
mod zone {
    pub fn lookup(seconds: i64) -> (i64, bool) {
        let time = seconds as libc::time_t;
        let mut tm: libc::tm = unsafe { core::mem::zeroed() };
        if unsafe { libc::localtime_r(&time, &mut tm) }.is_null() {
            return (0, false);
        }
        (tm.tm_gmtoff as i64, tm.tm_isdst > 0)
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
