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

#[cfg(unix)]
const SIGNALS: [libc::c_int; 20] = [
    libc::SIGHUP,
    libc::SIGINT,
    libc::SIGQUIT,
    libc::SIGILL,
    libc::SIGABRT,
    libc::SIGFPE,
    libc::SIGKILL,
    libc::SIGPIPE,
    libc::SIGALRM,
    libc::SIGTERM,
    libc::SIGSEGV,
    libc::SIGBUS,
    libc::SIGUSR1,
    libc::SIGUSR2,
    libc::SIGCHLD,
    libc::SIGCONT,
    libc::SIGSTOP,
    libc::SIGTSTP,
    libc::SIGTTIN,
    libc::SIGTTOU,
];

// wasi-libc declares no signals, so the browser reports the Linux numbers
#[cfg(not(unix))]
const SIGNALS: [libc::c_int; 20] = [
    1, 2, 3, 4, 6, 8, 9, 13, 14, 15, 11, 7, 10, 12, 17, 18, 19, 20, 21, 22,
];

#[cfg(unix)]
mod sys {
    use core::ffi::{c_char, c_int};
    use std::ffi::{CStr, OsStr, OsString};
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    use std::os::unix::process::CommandExt;
    use std::process::Command;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicI64, Ordering};

    use super::write_stdout;

    const EINVAL: i64 = libc::EINVAL as i64;

    static STATUS_KIND: AtomicI64 = AtomicI64::new(0);
    static STATUS_VALUE: AtomicI64 = AtomicI64::new(0);
    static TIMES: Mutex<[f64; 5]> = Mutex::new([0.0; 5]);

    unsafe extern "C" {
        fn ctermid(buffer: *mut c_char) -> *const c_char;
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
        const NAMES: [c_int; 10] = [
            libc::_SC_ARG_MAX,
            libc::_SC_CHILD_MAX,
            libc::_SC_CLK_TCK,
            libc::_SC_NGROUPS_MAX,
            libc::_SC_OPEN_MAX,
            libc::_SC_STREAM_MAX,
            libc::_SC_TZNAME_MAX,
            libc::_SC_JOB_CONTROL,
            libc::_SC_SAVED_IDS,
            libc::_SC_VERSION,
        ];
        NAMES.get(usize::try_from(index).ok()?).copied()
    }

    fn record_status(raw: c_int) {
        let (kind, value) = if libc::WIFEXITED(raw) {
            (0, libc::WEXITSTATUS(raw))
        } else if libc::WIFSTOPPED(raw) {
            (2, libc::WSTOPSIG(raw))
        } else {
            (1, libc::WTERMSIG(raw))
        };
        STATUS_KIND.store(kind, Ordering::Relaxed);
        STATUS_VALUE.store(i64::from(value), Ordering::Relaxed);
    }

    fn ticks() -> f64 {
        unsafe { libc::sysconf(sysconf_name(2).unwrap_or(2)) }.max(1) as f64
    }

    pub fn call(op: i64, a: i64, b: i64) -> i64 {
        let (pid, id) = (a as c_int, a as u32);
        match op {
            0 => i64::from(unsafe { libc::getpid() }),
            1 => i64::from(unsafe { libc::getppid() }),
            2 => i64::from(unsafe { libc::getuid() }),
            3 => i64::from(unsafe { libc::geteuid() }),
            4 => i64::from(unsafe { libc::getgid() }),
            5 => i64::from(unsafe { libc::getegid() }),
            6 => check(unsafe { libc::setuid(id) }),
            7 => check(unsafe { libc::setgid(id) }),
            8 => i64::from(unsafe { libc::getpgrp() }),
            9 => check(unsafe { libc::setsid() }),
            10 => check(unsafe { libc::setpgid(pid, b as c_int) }),
            11 => {
                unsafe { write_stdout(&[]) };
                check(unsafe { libc::fork() })
            }
            12 => check(unsafe { libc::kill(pid, b as c_int) }),
            13 => i64::from(unsafe { libc::alarm(a as u32) }),
            14 => {
                unsafe { libc::pause() };
                0
            }
            15 => i64::from(unsafe { libc::isatty(pid) }),
            16 => match sysconf_name(a) {
                Some(name) => {
                    let value = unsafe { libc::sysconf(name) };
                    if value < 0 { -EINVAL } else { value }
                }
                None => -EINVAL,
            },
            17 => {
                let mut raw: c_int = 0;
                let found = unsafe { libc::waitpid(pid, &mut raw, b as c_int) };
                if found > 0 {
                    record_status(raw);
                }
                check(found)
            }
            18 => STATUS_KIND.load(Ordering::Relaxed),
            19 => STATUS_VALUE.load(Ordering::Relaxed),
            20 => super::SIGNALS
                .get(a as usize)
                .map_or(-EINVAL, |signal| i64::from(*signal)),
            21 => check(unsafe { libc::getgroups(0, core::ptr::null_mut()) }),
            22 => {
                let mut list = vec![0u32; a.max(1) as usize];
                let found = unsafe { libc::getgroups(list.len() as c_int, list.as_mut_ptr()) };
                if found < 0 {
                    -errno()
                } else {
                    list.get(b as usize).map_or(-EINVAL, |g| i64::from(*g))
                }
            }
            23 => i64::from(unsafe { libc::sleep(a as u32) }),
            24 => super::SIGNALS
                .iter()
                .position(|signal| i64::from(*signal) == a)
                .map_or(-EINVAL, |i| i as i64),
            _ => -EINVAL,
        }
    }

    pub fn real(op: i64) -> f64 {
        let mut stored = TIMES.lock().unwrap_or_else(|p| p.into_inner());
        match op {
            0 => {
                let mut buffer = libc::tms {
                    tms_utime: 0,
                    tms_stime: 0,
                    tms_cutime: 0,
                    tms_cstime: 0,
                };
                let elapsed = unsafe { libc::times(&mut buffer) };
                let rate = ticks();
                *stored = [
                    elapsed as f64 / rate,
                    buffer.tms_utime as f64 / rate,
                    buffer.tms_stime as f64 / rate,
                    buffer.tms_cutime as f64 / rate,
                    buffer.tms_cstime as f64 / rate,
                ];
                stored[0]
            }
            1..=4 => stored[op as usize],
            10 => (unsafe { libc::time(core::ptr::null_mut()) }) as f64,
            _ => 0.0,
        }
    }

    fn owned(pointer: *const c_char) -> Option<Vec<u8>> {
        (!pointer.is_null()).then(|| unsafe { CStr::from_ptr(pointer) }.to_bytes().to_vec())
    }

    pub fn query(op: i64, a: i64) -> Result<Vec<u8>, i64> {
        match op {
            0 => {
                let mut names: libc::utsname = unsafe { core::mem::zeroed() };
                if unsafe { libc::uname(&mut names) } < 0 {
                    return Err(errno());
                }
                let mut joined = Vec::new();
                for field in [
                    &names.sysname[..],
                    &names.nodename,
                    &names.release,
                    &names.version,
                    &names.machine,
                ] {
                    joined.extend(field.iter().take_while(|c| **c != 0).map(|c| *c as u8));
                    joined.push(0);
                }
                Ok(joined)
            }
            1 => owned(unsafe { libc::getlogin() }).ok_or_else(errno),
            2 => owned(unsafe { ctermid(core::ptr::null_mut()) }).ok_or(EINVAL),
            3 => owned(unsafe { libc::ttyname(a as c_int) }).ok_or_else(errno),
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
    const ENOSYS: i64 = libc::ENOSYS as i64;
    const EINVAL: i64 = libc::EINVAL as i64;

    pub fn call(op: i64, a: i64, _b: i64) -> i64 {
        match op {
            20 => super::SIGNALS
                .get(a as usize)
                .map_or(-EINVAL, |signal| i64::from(*signal)),
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
