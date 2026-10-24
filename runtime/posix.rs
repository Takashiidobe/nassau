use super::{string, string_bytes, string_length, write_stdout};
use crate::value::tagged;

unsafe fn text(word: i64) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(string_bytes(word), string_length(word)).to_vec() }
}

fn terminated(bytes: &[u8]) -> Vec<Vec<u8>> {
    bytes
        .split(|byte| *byte == 0)
        .map(<[u8]>::to_vec)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .collect()
}

fn reply(outcome: Result<Vec<u8>, i64>) -> i64 {
    let bytes = match outcome {
        Ok(mut found) => {
            found.insert(0, b'=');
            found
        }
        Err(code) => code.to_string().into_bytes(),
    };
    unsafe { string(&bytes) }
}

const LINUX: bool = cfg!(any(target_os = "linux", target_os = "wasi"));

const SIGNALS: [(i32, i32); 20] = [
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
    (6, 6),
    (8, 8),
    (9, 9),
    (13, 13),
    (14, 14),
    (15, 15),
    (11, 11),
    (7, if LINUX { 7 } else { 10 }),
    (10, if LINUX { 10 } else { 30 }),
    (12, if LINUX { 12 } else { 31 }),
    (17, if LINUX { 17 } else { 20 }),
    (18, if LINUX { 18 } else { 19 }),
    (19, if LINUX { 19 } else { 17 }),
    (20, if LINUX { 20 } else { 18 }),
    (21, 21),
    (22, 22),
];

#[cfg(unix)]
mod sys {
    use core::ffi::{c_char, c_int, c_long};
    use std::ffi::{CStr, OsStr, OsString};
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    use std::os::unix::process::CommandExt;
    use std::process::Command;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicI64, Ordering};

    use super::write_stdout;

    const EINVAL: i64 = 22;

    static STATUS_KIND: AtomicI64 = AtomicI64::new(0);
    static STATUS_VALUE: AtomicI64 = AtomicI64::new(0);
    static TIMES: Mutex<[f64; 5]> = Mutex::new([0.0; 5]);

    #[repr(C)]
    struct Tms {
        utime: c_long,
        stime: c_long,
        cutime: c_long,
        cstime: c_long,
    }

    unsafe extern "C" {
        fn getpid() -> c_int;
        fn getppid() -> c_int;
        fn getuid() -> u32;
        fn geteuid() -> u32;
        fn getgid() -> u32;
        fn getegid() -> u32;
        fn setuid(id: u32) -> c_int;
        fn setgid(id: u32) -> c_int;
        fn getpgrp() -> c_int;
        fn setsid() -> c_int;
        fn setpgid(pid: c_int, group: c_int) -> c_int;
        fn fork() -> c_int;
        fn kill(pid: c_int, signal: c_int) -> c_int;
        fn alarm(seconds: u32) -> u32;
        fn pause() -> c_int;
        fn sleep(seconds: u32) -> u32;
        fn isatty(fd: c_int) -> c_int;
        fn sysconf(name: c_int) -> c_long;
        fn waitpid(pid: c_int, status: *mut c_int, flags: c_int) -> c_int;
        fn getgroups(size: c_int, list: *mut u32) -> c_int;
        fn getlogin() -> *const c_char;
        fn ctermid(buffer: *mut c_char) -> *const c_char;
        fn ttyname(fd: c_int) -> *const c_char;
        fn uname(buffer: *mut c_char) -> c_int;
        fn times(buffer: *mut Tms) -> c_long;
        fn time(out: *mut i64) -> i64;
    }

    fn check(value: c_int) -> i64 {
        if value < 0 {
            -errno()
        } else {
            i64::from(value)
        }
    }

    fn errno() -> i64 {
        i64::from(std::io::Error::last_os_error().raw_os_error().unwrap_or(5))
    }

    fn sysconf_name(index: i64) -> Option<c_int> {
        let linux = [0, 1, 2, 3, 4, 5, 6, 7, 8, 29];
        let mac = [1, 2, 3, 4, 5, 26, 27, 6, 7, 8];
        let table = if cfg!(target_os = "linux") {
            linux
        } else {
            mac
        };
        table.get(usize::try_from(index).ok()?).copied()
    }

    fn record_status(raw: c_int) {
        let (kind, value) = if raw & 0x7f == 0 {
            (0, (raw >> 8) & 0xff)
        } else if raw & 0xff == 0x7f {
            (2, (raw >> 8) & 0xff)
        } else {
            (1, raw & 0x7f)
        };
        STATUS_KIND.store(kind, Ordering::Relaxed);
        STATUS_VALUE.store(i64::from(value), Ordering::Relaxed);
    }

    fn ticks() -> f64 {
        unsafe { sysconf(sysconf_name(2).unwrap_or(2)) }.max(1) as f64
    }

    pub fn call(op: i64, a: i64, b: i64) -> i64 {
        let (pid, id) = (a as c_int, a as u32);
        match op {
            0 => i64::from(unsafe { getpid() }),
            1 => i64::from(unsafe { getppid() }),
            2 => i64::from(unsafe { getuid() }),
            3 => i64::from(unsafe { geteuid() }),
            4 => i64::from(unsafe { getgid() }),
            5 => i64::from(unsafe { getegid() }),
            6 => check(unsafe { setuid(id) }),
            7 => check(unsafe { setgid(id) }),
            8 => i64::from(unsafe { getpgrp() }),
            9 => check(unsafe { setsid() }),
            10 => check(unsafe { setpgid(pid, b as c_int) }),
            11 => {
                unsafe { write_stdout(&[]) };
                check(unsafe { fork() })
            }
            12 => check(unsafe { kill(pid, b as c_int) }),
            13 => i64::from(unsafe { alarm(a as u32) }),
            14 => {
                unsafe { pause() };
                0
            }
            15 => i64::from(unsafe { isatty(pid) }),
            16 => match sysconf_name(a) {
                Some(name) => {
                    let value = unsafe { sysconf(name) };
                    if value < 0 { -EINVAL } else { value }
                }
                None => -EINVAL,
            },
            17 => {
                let mut raw: c_int = 0;
                let found = unsafe { waitpid(pid, &mut raw, b as c_int) };
                if found > 0 {
                    record_status(raw);
                }
                check(found)
            }
            18 => STATUS_KIND.load(Ordering::Relaxed),
            19 => STATUS_VALUE.load(Ordering::Relaxed),
            20 => super::SIGNALS
                .get(a as usize)
                .map_or(-EINVAL, |s| i64::from(s.1)),
            21 => check(unsafe { getgroups(0, core::ptr::null_mut()) }),
            22 => {
                let mut list = vec![0u32; a.max(1) as usize];
                let found = unsafe { getgroups(list.len() as c_int, list.as_mut_ptr()) };
                if found < 0 {
                    -errno()
                } else {
                    list.get(b as usize).map_or(-EINVAL, |g| i64::from(*g))
                }
            }
            23 => i64::from(unsafe { sleep(a as u32) }),
            24 => super::SIGNALS
                .iter()
                .position(|s| i64::from(s.1) == a)
                .map_or(-EINVAL, |i| i as i64),
            _ => -EINVAL,
        }
    }

    pub fn real(op: i64) -> f64 {
        let mut stored = TIMES.lock().unwrap_or_else(|p| p.into_inner());
        match op {
            0 => {
                let mut buffer = Tms {
                    utime: 0,
                    stime: 0,
                    cutime: 0,
                    cstime: 0,
                };
                let elapsed = unsafe { times(&mut buffer) };
                let rate = ticks();
                *stored = [
                    elapsed as f64 / rate,
                    buffer.utime as f64 / rate,
                    buffer.stime as f64 / rate,
                    buffer.cutime as f64 / rate,
                    buffer.cstime as f64 / rate,
                ];
                stored[0]
            }
            1..=4 => stored[op as usize],
            10 => (unsafe { time(core::ptr::null_mut()) }) as f64,
            _ => 0.0,
        }
    }

    fn owned(pointer: *const c_char) -> Option<Vec<u8>> {
        (!pointer.is_null()).then(|| unsafe { CStr::from_ptr(pointer) }.to_bytes().to_vec())
    }

    pub fn query(op: i64, a: i64) -> Result<Vec<u8>, i64> {
        match op {
            0 => {
                let width = if cfg!(target_os = "linux") { 65 } else { 256 };
                let mut buffer = vec![0u8; width * 6];
                let status = unsafe { uname(buffer.as_mut_ptr().cast()) };
                if status < 0 {
                    return Err(errno());
                }
                let mut joined = Vec::new();
                for field in buffer.chunks(width).take(5) {
                    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
                    joined.extend_from_slice(&field[..end]);
                    joined.push(0);
                }
                Ok(joined)
            }
            1 => owned(unsafe { getlogin() }).ok_or_else(errno),
            2 => owned(unsafe { ctermid(core::ptr::null_mut()) }).ok_or(EINVAL),
            3 => owned(unsafe { ttyname(a as c_int) }).ok_or_else(errno),
            4 => {
                let mut joined = Vec::new();
                for (key, value) in std::env::vars_os() {
                    joined.extend_from_slice(key.as_bytes());
                    joined.push(b'=');
                    joined.extend_from_slice(value.as_bytes());
                    joined.push(0);
                }
                Ok(joined)
            }
            _ => Err(EINVAL),
        }
    }

    pub fn exec(path: &[u8], args: &[Vec<u8>], env: &[Vec<u8>], mode: i64) -> i64 {
        unsafe { write_stdout(&[]) };
        let mut target = path.to_vec();
        if mode != 2 && !target.contains(&b'/') {
            target.splice(0..0, *b"./");
        }
        let mut command = Command::new(OsStr::from_bytes(&target));
        if let Some((first, rest)) = args.split_first() {
            command.arg0(OsStr::from_bytes(first));
            command.args(rest.iter().map(|arg| OsStr::from_bytes(arg)));
        }
        if mode == 1 {
            command.env_clear();
            for entry in env {
                let split = entry.iter().position(|b| *b == b'=').unwrap_or(entry.len());
                let value = entry.get(split + 1..).unwrap_or(&[]);
                command.env(
                    OsString::from_vec(entry[..split].to_vec()),
                    OsString::from_vec(value.to_vec()),
                );
            }
        }
        -i64::from(command.exec().raw_os_error().unwrap_or(5))
    }
}

#[cfg(not(unix))]
mod sys {
    const ENOSYS: i64 = if cfg!(target_os = "wasi") { 52 } else { 38 };
    const EINVAL: i64 = if cfg!(target_os = "wasi") { 28 } else { 22 };

    pub fn call(op: i64, a: i64, _b: i64) -> i64 {
        match op {
            20 => super::SIGNALS
                .get(a as usize)
                .map_or(-EINVAL, |s| i64::from(s.1)),
            _ => -ENOSYS,
        }
    }

    pub fn real(_op: i64) -> f64 {
        0.0
    }

    pub fn query(_op: i64, _a: i64) -> Result<Vec<u8>, i64> {
        Err(ENOSYS)
    }

    pub fn exec(_path: &[u8], _args: &[Vec<u8>], _env: &[Vec<u8>], _mode: i64) -> i64 {
        -ENOSYS
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_posix_int(op: i64, a: i64, b: i64) -> i64 {
    tagged(sys::call(op >> 1, a >> 1, b >> 1))
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_posix_real(op: i64) -> f64 {
    sys::real(op >> 1)
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_posix_text(op: i64, a: i64) -> i64 {
    reply(sys::query(op >> 1, a >> 1))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_posix_exec(path: i64, args: i64, env: i64, mode: i64) -> i64 {
    let (path, args, env) = unsafe { (text(path), text(args), text(env)) };
    tagged(sys::exec(
        &path,
        &terminated(&args),
        &terminated(&env),
        mode >> 1,
    ))
}
