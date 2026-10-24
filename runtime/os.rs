use std::fs::{File, OpenOptions, ReadDir};
use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};
use std::slice;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::{string, string_bytes, string_length, write_stderr, write_stdout};
use crate::value::tagged;

enum Stream {
    Stdin,
    Stdout,
    Stderr,
    File(File),
    Dir { path: String, entries: ReadDir },
    Socket(Socket),
}

enum Address {
    Inet(SocketAddr),
    Unix(String),
}

enum Socket {
    Fresh,
    Bound(Address),
    TcpListener(TcpListener),
    TcpStream(TcpStream),
    #[cfg(unix)]
    UnixListener(UnixListener, String),
    #[cfg(unix)]
    UnixStream(UnixStream),
}

static STREAMS: Mutex<Vec<Option<Stream>>> = Mutex::new(Vec::new());

const READ: i64 = 0;
const WRITE: i64 = 1;
const APPEND: i64 = 2;
const EIO: i64 = 5;
const ENOSYS: i64 = if cfg!(target_os = "wasi") { 52 } else { 38 };

fn streams() -> MutexGuard<'static, Vec<Option<Stream>>> {
    let mut streams = STREAMS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if streams.is_empty() {
        streams.extend([Stream::Stdin, Stream::Stdout, Stream::Stderr].map(Some));
    }
    streams
}

unsafe fn bytes<'a>(word: i64) -> &'a [u8] {
    unsafe { slice::from_raw_parts(string_bytes(word), string_length(word)) }
}

unsafe fn text(word: i64) -> String {
    unsafe { String::from_utf8_lossy(bytes(word)).into_owned() }
}

fn failure(error: &io::Error) -> i64 {
    tagged(-i64::from(error.raw_os_error().unwrap_or(EIO as i32)))
}

fn result(outcome: io::Result<()>) -> i64 {
    outcome.map_or_else(|error| failure(&error), |()| tagged(0))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_open(path: i64, mode: i64) -> i64 {
    let path = unsafe { text(path) };
    let mut options = OpenOptions::new();
    match mode >> 1 {
        READ => options.read(true),
        WRITE => options.write(true).create(true).truncate(true),
        APPEND => options.append(true).create(true),
        _ => return tagged(-EIO),
    };
    match options.open(path) {
        Ok(file) => insert(Stream::File(file)),
        Err(error) => failure(&error),
    }
}

fn insert(stream: Stream) -> i64 {
    let mut streams = streams();
    let handle = streams.iter().position(Option::is_none).unwrap_or_else(|| {
        streams.push(None);
        streams.len() - 1
    });
    streams[handle] = Some(stream);
    tagged(handle as i64)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_open_dir(path: i64) -> i64 {
    let path = unsafe { text(path) };
    match std::fs::read_dir(&path) {
        Ok(entries) => insert(Stream::Dir { path, entries }),
        Err(error) => failure(&error),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_read_dir(handle: i64) -> i64 {
    let name = match streams().get_mut((handle >> 1) as usize) {
        Some(Some(Stream::Dir { entries, .. })) => entries
            .next()
            .and_then(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned()),
        _ => None,
    };
    unsafe { string(name.unwrap_or_default().as_bytes()) }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_os_rewind_dir(handle: i64) -> i64 {
    match streams().get_mut((handle >> 1) as usize) {
        Some(Some(Stream::Dir { path, entries })) => match std::fs::read_dir(&*path) {
            Ok(fresh) => {
                *entries = fresh;
                tagged(0)
            }
            Err(error) => failure(&error),
        },
        _ => tagged(-EIO),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_is_dir(path: i64) -> i64 {
    std::fs::metadata(unsafe { text(path) }).map_or_else(
        |error| failure(&error),
        |meta| tagged(i64::from(meta.is_dir())),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_file_size(path: i64) -> i64 {
    std::fs::metadata(unsafe { text(path) })
        .map_or_else(|error| failure(&error), |meta| tagged(meta.len() as i64))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_mkdir(path: i64) -> i64 {
    result(std::fs::create_dir(unsafe { text(path) }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_rmdir(path: i64) -> i64 {
    result(std::fs::remove_dir(unsafe { text(path) }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_read(handle: i64, max: i64) -> i64 {
    let mut buffer = vec![0; (max >> 1).max(0) as usize];
    let count = match streams().get_mut((handle >> 1) as usize) {
        Some(Some(Stream::Stdin)) => io::stdin().read(&mut buffer),
        Some(Some(Stream::File(file))) => file.read(&mut buffer),
        Some(Some(Stream::Socket(Socket::TcpStream(stream)))) => stream.read(&mut buffer),
        #[cfg(unix)]
        Some(Some(Stream::Socket(Socket::UnixStream(stream)))) => stream.read(&mut buffer),
        _ => Ok(0),
    }
    .unwrap_or(0);
    unsafe { string(&buffer[..count]) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_write(handle: i64, data: i64) -> i64 {
    let data = unsafe { bytes(data) };
    result(match streams().get_mut((handle >> 1) as usize) {
        Some(Some(Stream::Stdout)) => {
            unsafe { write_stdout(data) };
            Ok(())
        }
        Some(Some(Stream::Stderr)) => {
            unsafe { write_stderr(data) };
            Ok(())
        }
        Some(Some(Stream::File(file))) => file.write_all(data),
        Some(Some(Stream::Socket(Socket::TcpStream(stream)))) => stream.write_all(data),
        #[cfg(unix)]
        Some(Some(Stream::Socket(Socket::UnixStream(stream)))) => stream.write_all(data),
        _ => Err(io::Error::from_raw_os_error(EIO as i32)),
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_os_close(handle: i64) -> i64 {
    let handle = (handle >> 1) as usize;
    let mut streams = streams();
    if handle >= 3
        && let Some(slot) = streams.get_mut(handle)
    {
        *slot = None;
    }
    tagged(0)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_remove(path: i64) -> i64 {
    result(std::fs::remove_file(unsafe { text(path) }))
}

const ACCESS_WRITE: i64 = 2;
const ACCESS_EXEC: i64 = 4;

fn text_result(outcome: io::Result<String>) -> i64 {
    let reply = match outcome {
        Ok(value) => format!("={value}"),
        Err(error) => (error.raw_os_error().unwrap_or(EIO as i32)).to_string(),
    };
    unsafe { string(reply.as_bytes()) }
}

fn executable(meta: &std::fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        meta.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        meta.is_dir()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_access(path: i64, mode: i64) -> i64 {
    let path = unsafe { text(path) };
    let mode = mode >> 1;
    let Ok(meta) = std::fs::metadata(&path) else {
        return tagged(0);
    };
    let writable = mode & ACCESS_WRITE == 0
        || if meta.is_dir() {
            !meta.permissions().readonly()
        } else {
            OpenOptions::new().write(true).open(&path).is_ok()
        };
    let granted =
        File::open(&path).is_ok() && writable && (mode & ACCESS_EXEC == 0 || executable(&meta));
    tagged(i64::from(granted))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_rename(old: i64, new: i64) -> i64 {
    result(std::fs::rename(unsafe { text(old) }, unsafe { text(new) }))
}

fn micros(time: SystemTime) -> i64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(after) => after.as_micros() as i64,
        Err(before) => -(before.duration().as_micros() as i64),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_mod_time(path: i64) -> i64 {
    std::fs::metadata(unsafe { text(path) })
        .and_then(|meta| meta.modified())
        .map_or_else(|error| failure(&error), |time| tagged(micros(time)))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_set_time(path: i64, stamp: i64, use_now: i64) -> i64 {
    let time = if use_now >> 1 == 1 {
        SystemTime::now()
    } else {
        let stamp = stamp >> 1;
        let offset = Duration::from_micros(stamp.unsigned_abs());
        if stamp < 0 {
            UNIX_EPOCH - offset
        } else {
            UNIX_EPOCH + offset
        }
    };
    result(File::open(unsafe { text(path) }).and_then(|file| file.set_modified(time)))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_is_link(path: i64) -> i64 {
    std::fs::symlink_metadata(unsafe { text(path) }).map_or_else(
        |error| failure(&error),
        |meta| tagged(i64::from(meta.file_type().is_symlink())),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_read_link(path: i64) -> i64 {
    text_result(
        std::fs::read_link(unsafe { text(path) })
            .map(|target| target.to_string_lossy().into_owned()),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_full_path(path: i64) -> i64 {
    let path = unsafe { text(path) };
    text_result(canonical(&path))
}

#[cfg(not(target_os = "wasi"))]
fn canonical(path: &str) -> io::Result<String> {
    std::fs::canonicalize(path).map(|full| full.to_string_lossy().into_owned())
}

#[cfg(target_os = "wasi")]
fn canonical(path: &str) -> io::Result<String> {
    std::fs::metadata(path)?;
    let mut parts: Vec<&str> = Vec::new();
    let base = if path.starts_with('/') {
        String::new()
    } else {
        normalized_dir()
    };
    for part in base.split('/').chain(path.split('/')) {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part),
        }
    }
    Ok(format!("/{}", parts.join("/")))
}

#[cfg(unix)]
fn identity(meta: &std::fs::Metadata) -> (u64, u64) {
    use std::os::unix::fs::MetadataExt;
    (meta.dev(), meta.ino())
}

#[cfg(target_os = "wasi")]
fn identity(meta: &std::fs::Metadata) -> (u64, u64) {
    use std::os::wasi::fs::MetadataExt;
    (meta.dev(), meta.ino())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_file_id(path: i64) -> i64 {
    text_result(std::fs::metadata(unsafe { text(path) }).map(|meta| {
        let (device, inode) = identity(&meta);
        format!("{device:016x}{inode:016x}")
    }))
}

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn nassau_os_tmp_name() -> i64 {
    let directory =
        std::path::PathBuf::from(std::env::var_os("TMPDIR").unwrap_or_else(|| "/tmp".into()));
    if cfg!(target_os = "wasi") {
        let _ = std::fs::create_dir_all(&directory);
    }
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    loop {
        let name = directory.join(format!(
            "nassau{seed:x}-{}",
            TMP_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        match OpenOptions::new().write(true).create_new(true).open(&name) {
            Ok(_) => return text_result(Ok(name.to_string_lossy().into_owned())),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return text_result(Err(error)),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_error_message(code: i64) -> i64 {
    let message = io::Error::from_raw_os_error((code >> 1) as i32).to_string();
    let message = message.split(" (os error").next().unwrap_or(&message);
    unsafe { string(message.as_bytes()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_getenv(name: i64) -> i64 {
    let value = std::env::var_os(unsafe { text(name) });
    let mut found = Vec::new();
    if let Some(value) = value {
        found.push(b'=');
        found.extend_from_slice(value.to_string_lossy().as_bytes());
    }
    unsafe { string(&found) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_args() -> i64 {
    let arguments: Vec<String> = std::env::args_os()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect();
    unsafe { string(arguments.join("\0").as_bytes()) }
}

fn normalized_dir() -> String {
    let mut parts: Vec<String> = Vec::new();
    let directory = std::env::current_dir().unwrap_or_else(|_| "/".into());
    for part in directory.to_string_lossy().split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part.to_owned()),
        }
    }
    format!("/{}", parts.join("/"))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_get_dir() -> i64 {
    unsafe { string(normalized_dir().as_bytes()) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_chdir(path: i64) -> i64 {
    result(
        std::env::set_current_dir(unsafe { text(path) })
            .and_then(|()| std::env::set_current_dir(normalized_dir())),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_system(command: i64) -> i64 {
    let command = unsafe { text(command) };
    unsafe { write_stdout(&[]) };
    match std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
    {
        Ok(status) => tagged(i64::from(status.code().unwrap_or(1))),
        Err(error) if error.kind() == io::ErrorKind::Unsupported => tagged(-ENOSYS),
        Err(error) => failure(&error),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_os_sleep(seconds: f64) {
    if seconds > 0.0 {
        std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_os_now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0.0, |elapsed| elapsed.as_secs_f64())
}

fn parse_address(text: &str) -> Option<Address> {
    if let Some(path) = text.strip_prefix("unix:") {
        return Some(Address::Unix(path.to_owned()));
    }
    text.strip_prefix("inet:")?.parse().ok().map(Address::Inet)
}

fn address_text(address: &Address) -> String {
    match address {
        Address::Inet(address) => format!("inet:{address}"),
        Address::Unix(path) => format!("unix:{path}"),
    }
}

fn unsupported() -> io::Error {
    io::Error::from_raw_os_error(ENOSYS as i32)
}

fn with_socket<T>(handle: i64, action: impl FnOnce(&mut Socket) -> io::Result<T>) -> io::Result<T> {
    match streams().get_mut((handle >> 1) as usize) {
        Some(Some(Stream::Socket(socket))) => action(socket),
        _ => Err(io::Error::from_raw_os_error(EIO as i32)),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_sock_create() -> i64 {
    insert(Stream::Socket(Socket::Fresh))
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_sock_pair() -> i64 {
    #[cfg(unix)]
    {
        match UnixStream::pair() {
            Ok((left, right)) => {
                let left = insert(Stream::Socket(Socket::UnixStream(left))) >> 1;
                let right = insert(Stream::Socket(Socket::UnixStream(right))) >> 1;
                tagged(left * 16384 + right)
            }
            Err(error) => failure(&error),
        }
    }
    #[cfg(not(unix))]
    failure(&unsupported())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_sock_bind(handle: i64, address: i64) -> i64 {
    let Some(address) = parse_address(&unsafe { text(address) }) else {
        return failure(&io::Error::from_raw_os_error(22));
    };
    result(with_socket(handle, |socket| match socket {
        Socket::Fresh => {
            *socket = Socket::Bound(address);
            Ok(())
        }
        _ => Err(io::Error::from_raw_os_error(22)),
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_sock_listen(handle: i64) -> i64 {
    result(with_socket(handle, |socket| {
        let Socket::Bound(address) = socket else {
            return Err(io::Error::from_raw_os_error(22));
        };
        *socket = match address {
            Address::Inet(address) => Socket::TcpListener(TcpListener::bind(*address)?),
            #[cfg(unix)]
            Address::Unix(path) => Socket::UnixListener(UnixListener::bind(&*path)?, path.clone()),
            #[cfg(not(unix))]
            Address::Unix(_) => return Err(unsupported()),
        };
        Ok(())
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_sock_accept(handle: i64) -> i64 {
    // the table lock is released while blocked so other handles keep working
    let accepted = with_socket(handle, |socket| match socket {
        Socket::TcpListener(listener) => {
            listener.try_clone().map(Socket::TcpListener)
        }
        #[cfg(unix)]
        Socket::UnixListener(listener, path) => listener
            .try_clone()
            .map(|clone| Socket::UnixListener(clone, path.clone())),
        _ => Err(io::Error::from_raw_os_error(22)),
    });
    let outcome = accepted.and_then(|listener| match listener {
        Socket::TcpListener(listener) => listener
            .accept()
            .map(|(stream, _)| Socket::TcpStream(stream)),
        #[cfg(unix)]
        Socket::UnixListener(listener, _) => listener
            .accept()
            .map(|(stream, _)| Socket::UnixStream(stream)),
        _ => Err(unsupported()),
    });
    match outcome {
        Ok(socket) => insert(Stream::Socket(socket)),
        Err(error) => failure(&error),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_sock_connect(handle: i64, address: i64) -> i64 {
    let Some(address) = parse_address(&unsafe { text(address) }) else {
        return failure(&io::Error::from_raw_os_error(22));
    };
    result(with_socket(handle, |socket| {
        if !matches!(socket, Socket::Fresh | Socket::Bound(_)) {
            return Err(io::Error::from_raw_os_error(22));
        }
        *socket = match address {
            Address::Inet(address) => Socket::TcpStream(TcpStream::connect(address)?),
            #[cfg(unix)]
            Address::Unix(path) => Socket::UnixStream(UnixStream::connect(path)?),
            #[cfg(not(unix))]
            Address::Unix(_) => return Err(unsupported()),
        };
        Ok(())
    }))
}

// peer selects the remote end rather than the local one
#[unsafe(no_mangle)]
pub extern "C" fn nassau_sock_name(handle: i64, peer: i64) -> i64 {
    let peer = peer >> 1 != 0;
    let name = with_socket(handle, |socket| match socket {
        Socket::Bound(address) if !peer => Ok(address_text(address)),
        Socket::TcpListener(listener) if !peer => listener
            .local_addr()
            .map(|address| address_text(&Address::Inet(address))),
        Socket::TcpStream(stream) => if peer {
            stream.peer_addr()
        } else {
            stream.local_addr()
        }
        .map(|address| address_text(&Address::Inet(address))),
        #[cfg(unix)]
        Socket::UnixListener(_, path) if !peer => Ok(format!("unix:{path}")),
        #[cfg(unix)]
        Socket::UnixStream(stream) => {
            let address = if peer {
                stream.peer_addr()
            } else {
                stream.local_addr()
            }?;
            Ok(format!(
                "unix:{}",
                address
                    .as_pathname()
                    .map(|path| path.to_string_lossy().into_owned())
                    .unwrap_or_default()
            ))
        }
        _ => Err(io::Error::from_raw_os_error(107)),
    });
    text_result(name)
}

const SHUT_READ: i64 = 0;
const SHUT_WRITE: i64 = 1;

#[unsafe(no_mangle)]
pub extern "C" fn nassau_sock_shutdown(handle: i64, how: i64) -> i64 {
    let how = match how >> 1 {
        SHUT_READ => Shutdown::Read,
        SHUT_WRITE => Shutdown::Write,
        _ => Shutdown::Both,
    };
    result(with_socket(handle, |socket| match socket {
        Socket::TcpStream(stream) => stream.shutdown(how),
        #[cfg(unix)]
        Socket::UnixStream(stream) => stream.shutdown(how),
        _ => Err(io::Error::from_raw_os_error(107)),
    }))
}
