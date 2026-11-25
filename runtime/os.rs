use std::fs::{File, OpenOptions, ReadDir};
use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
#[cfg(unix)]
use std::os::fd::AsRawFd;
#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};
use std::slice;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(any(unix, windows))]
use socket2::{Domain, Protocol, SockAddr, Socket as Datagram, Type};
#[cfg(windows)]
use windows_sys::Win32::Networking::WinSock;

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
    #[cfg(any(unix, windows))]
    Udp(Datagram),
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
fn identity(path: &str) -> io::Result<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    let meta = std::fs::metadata(path)?;
    Ok((meta.dev(), meta.ino()))
}

#[cfg(target_os = "wasi")]
fn identity(path: &str) -> io::Result<(u64, u64)> {
    use std::os::wasi::fs::MetadataExt;
    let meta = std::fs::metadata(path)?;
    Ok((meta.dev(), meta.ino()))
}

#[cfg(windows)]
fn identity(path: &str) -> io::Result<(u64, u64)> {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, FILE_FLAG_BACKUP_SEMANTICS, GetFileInformationByHandle,
    };

    let file = OpenOptions::new()
        .access_mode(0)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?;
    let mut info = std::mem::MaybeUninit::<BY_HANDLE_FILE_INFORMATION>::uninit();
    if unsafe { GetFileInformationByHandle(file.as_raw_handle(), info.as_mut_ptr()) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let info = unsafe { info.assume_init() };
    let index = (u64::from(info.nFileIndexHigh) << 32) | u64::from(info.nFileIndexLow);
    Ok((u64::from(info.dwVolumeSerialNumber), index))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_file_id(path: i64) -> i64 {
    text_result(
        identity(&unsafe { text(path) })
            .map(|(device, inode)| format!("{device:016x}{inode:016x}")),
    )
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
        #[cfg(any(unix, windows))]
        Socket::Udp(socket) => socket.bind(&udp_address(address)?),
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
        Socket::TcpListener(listener) => listener.try_clone().map(Socket::TcpListener),
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
        #[cfg(any(unix, windows))]
        if let Socket::Udp(socket) = socket {
            if socket.peer_addr().is_ok() {
                return Err(udp_already_connected());
            }
            return socket.connect(&udp_address(address)?);
        }
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
        #[cfg(any(unix, windows))]
        Socket::Udp(socket) => {
            let address = if peer {
                socket.peer_addr()
            } else {
                socket.local_addr()
            }?;
            Ok(address_text(&Address::Inet(udp_inet(address)?)))
        }
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

#[cfg(any(unix, windows))]
fn udp_address(address: Address) -> io::Result<SockAddr> {
    match address {
        Address::Inet(SocketAddr::V4(address)) => Ok(address.into()),
        _ => Err(io::ErrorKind::InvalidInput.into()),
    }
}

#[cfg(any(unix, windows))]
fn udp_inet(address: SockAddr) -> io::Result<SocketAddr> {
    address.as_socket().ok_or(io::ErrorKind::InvalidData.into())
}

#[cfg(any(unix, windows))]
fn udp_already_connected() -> io::Error {
    #[cfg(unix)]
    let code = libc::EISCONN;
    #[cfg(windows)]
    let code = WinSock::WSAEISCONN;
    io::Error::from_raw_os_error(code)
}

#[cfg(any(unix, windows))]
fn udp_socket(handle: i64) -> io::Result<Datagram> {
    with_socket(handle, |socket| match socket {
        Socket::Udp(socket) => socket.try_clone(),
        _ => Err(io::ErrorKind::InvalidInput.into()),
    })
}

#[cfg(any(unix, windows))]
fn udp_flags(flags: i64, receive: bool) -> i32 {
    #[cfg(windows)]
    use WinSock::{MSG_DONTROUTE, MSG_OOB, MSG_PEEK};
    #[cfg(unix)]
    use libc::{MSG_DONTROUTE, MSG_OOB, MSG_PEEK};

    let mut native = 0;
    if flags & 1 != 0 {
        native |= MSG_OOB;
    }
    if flags & 2 != 0 {
        native |= if receive { MSG_PEEK } else { MSG_DONTROUTE };
    }
    #[cfg(unix)]
    if flags & 4 != 0 {
        native |= libc::MSG_DONTWAIT;
    }
    native
}

#[cfg(any(unix, windows))]
fn udp_io<T>(
    socket: &Datagram,
    receive: bool,
    flags: i64,
    mut operation: impl FnMut(i32) -> io::Result<T>,
) -> io::Result<Option<T>> {
    let native = udp_flags(flags, receive);
    let nonblocking = flags & 4 != 0;
    #[cfg(unix)]
    let _ = socket;
    loop {
        match operation(native) {
            Ok(value) => return Ok(Some(value)),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                if nonblocking {
                    return Ok(None);
                }
                #[cfg(windows)]
                udp_wait(socket, receive)?;
                #[cfg(unix)]
                return Err(error);
            }
            Err(error) => return Err(error),
        }
    }
}

#[cfg(windows)]
fn udp_wait(socket: &Datagram, receive: bool) -> io::Result<()> {
    use std::os::windows::io::AsRawSocket;

    let mut descriptor = WinSock::WSAPOLLFD {
        fd: socket.as_raw_socket() as WinSock::SOCKET,
        events: if receive {
            WinSock::POLLIN
        } else {
            WinSock::POLLOUT
        },
        revents: 0,
    };
    if unsafe { WinSock::WSAPoll(&mut descriptor, 1, -1) } == WinSock::SOCKET_ERROR {
        return Err(io::Error::from_raw_os_error(unsafe {
            WinSock::WSAGetLastError()
        }));
    }
    Ok(())
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_udp_create(protocol: i64) -> i64 {
    #[cfg(any(unix, windows))]
    {
        let Ok(protocol) = i32::try_from(protocol >> 1) else {
            return failure(&io::ErrorKind::InvalidInput.into());
        };
        let outcome = Datagram::new(Domain::IPV4, Type::DGRAM, Some(Protocol::from(protocol)));
        #[cfg(windows)]
        let outcome = outcome.and_then(|socket| {
            socket.set_nonblocking(true)?;
            Ok(socket)
        });
        match outcome {
            Ok(socket) => insert(Stream::Socket(Socket::Udp(socket))),
            Err(error) => failure(&error),
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = protocol;
        failure(&unsupported())
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_udp_send_to(
    handle: i64,
    address: i64,
    data: i64,
    flags: i64,
) -> i64 {
    #[cfg(any(unix, windows))]
    {
        let outcome = (|| {
            let address =
                parse_address(&unsafe { text(address) }).ok_or(io::ErrorKind::InvalidInput)?;
            let address = udp_address(address)?;
            let socket = udp_socket(handle)?;
            let peer = socket.peer_addr().ok();
            if let Some(peer) = &peer
                && peer.as_socket() != address.as_socket()
            {
                return Err(udp_already_connected());
            }
            let data = unsafe { bytes(data) };
            udp_io(&socket, false, flags >> 1, |native| {
                if peer.is_some() {
                    socket.send_with_flags(data, native)
                } else {
                    socket.send_to_with_flags(data, &address, native)
                }
            })
        })();
        match outcome {
            Ok(sent) => tagged(i64::from(sent.is_some())),
            Err(error) => failure(&error),
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (handle, address, data, flags);
        failure(&unsupported())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_udp_recv_from(handle: i64, max: i64, flags: i64) -> i64 {
    #[cfg(any(unix, windows))]
    {
        let outcome = (|| {
            let socket = udp_socket(handle)?;
            let peer = socket.peer_addr().ok();
            let max = (max >> 1).clamp(0, 65535) as usize;
            #[cfg(unix)]
            let capacity = max;
            #[cfg(windows)]
            let capacity = 65535;
            let mut data = vec![std::mem::MaybeUninit::<u8>::uninit(); capacity];
            let received = udp_io(&socket, true, flags >> 1, |native| {
                if let Some(peer) = &peer {
                    socket
                        .recv_with_flags(&mut data, native)
                        .map(|count| (count, peer.clone()))
                } else {
                    socket.recv_from_with_flags(&mut data, native)
                }
            })?;
            let Some((count, peer)) = received else {
                return Ok(None);
            };
            let peer = udp_inet(peer)?;
            let mut reply = format!("=inet:{peer}\n").into_bytes();
            let data = unsafe { slice::from_raw_parts(data.as_ptr().cast::<u8>(), count.min(max)) };
            reply.extend_from_slice(data);
            Ok(Some(reply))
        })();
        match outcome {
            Ok(Some(reply)) => unsafe { string(&reply) },
            Ok(None) => unsafe { string(b"") },
            Err(error) => text_result(Err(error)),
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (handle, max, flags);
        text_result(Err(unsupported()))
    }
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

#[cfg(unix)]
fn raw_descriptor(stream: &Stream) -> Option<i32> {
    match stream {
        Stream::Stdin => Some(0),
        Stream::Stdout => Some(1),
        Stream::Stderr => Some(2),
        Stream::File(file) => Some(file.as_raw_fd()),
        Stream::Dir { .. } => None,
        Stream::Socket(Socket::TcpListener(socket)) => Some(socket.as_raw_fd()),
        Stream::Socket(Socket::TcpStream(socket)) => Some(socket.as_raw_fd()),
        Stream::Socket(Socket::Udp(socket)) => Some(socket.as_raw_fd()),
        Stream::Socket(Socket::UnixListener(socket, _)) => Some(socket.as_raw_fd()),
        Stream::Socket(Socket::UnixStream(socket)) => Some(socket.as_raw_fd()),
        Stream::Socket(_) => None,
    }
}

#[cfg(any(unix, windows))]
const KIND_SOCKET: i64 = 5;
#[cfg(unix)]
const EBADF: i64 = 9;

#[cfg(unix)]
#[unsafe(no_mangle)]
pub extern "C" fn nassau_os_io_kind(handle: i64) -> i64 {
    use std::mem::ManuallyDrop;
    use std::os::fd::FromRawFd;
    use std::os::unix::fs::FileTypeExt;

    const KIND_FILE: i64 = 0;
    const KIND_DIR: i64 = 1;
    const KIND_TTY: i64 = 3;
    const KIND_PIPE: i64 = 4;
    const KIND_DEVICE: i64 = 6;

    let streams = streams();
    let Some(Some(stream)) = streams.get((handle >> 1) as usize) else {
        return tagged(-EBADF);
    };
    if matches!(stream, Stream::Dir { .. }) {
        return tagged(KIND_DIR);
    }
    if matches!(stream, Stream::Socket(_)) {
        return tagged(KIND_SOCKET);
    }
    let Some(fd) = raw_descriptor(stream) else {
        return tagged(-EBADF);
    };
    let file = ManuallyDrop::new(unsafe { File::from_raw_fd(fd) });
    let Ok(metadata) = file.metadata() else {
        return tagged(-EBADF);
    };
    let kind = metadata.file_type();
    tagged(if kind.is_socket() {
        KIND_SOCKET
    } else if kind.is_fifo() {
        KIND_PIPE
    } else if kind.is_dir() {
        KIND_DIR
    } else if kind.is_file() {
        KIND_FILE
    } else if kind.is_char_device() && unsafe { libc::isatty(fd) } == 1 {
        KIND_TTY
    } else {
        KIND_DEVICE
    })
}

#[cfg(windows)]
#[unsafe(no_mangle)]
pub extern "C" fn nassau_os_io_kind(handle: i64) -> i64 {
    match streams().get((handle >> 1) as usize) {
        Some(Some(Stream::Socket(_))) => tagged(KIND_SOCKET),
        _ => tagged(-ENOSYS),
    }
}

#[cfg(not(any(unix, windows)))]
#[unsafe(no_mangle)]
pub extern "C" fn nassau_os_io_kind(_handle: i64) -> i64 {
    tagged(-ENOSYS)
}

#[cfg(unix)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_poll(spec: i64, timeout: i64) -> i64 {
    let spec = unsafe { text(spec) };
    let mut wanted = Vec::new();
    {
        let streams = streams();
        for entry in spec.split(',').filter(|entry| !entry.is_empty()) {
            let (handle, events) = entry.split_once(':').unwrap_or((entry, "0"));
            let found = handle
                .parse::<usize>()
                .ok()
                .and_then(|handle| streams.get(handle)?.as_ref())
                .and_then(raw_descriptor);
            let Some(fd) = found else {
                return unsafe { string(EBADF.to_string().as_bytes()) };
            };
            wanted.push(libc::pollfd {
                fd,
                events: events.parse().unwrap_or(0),
                revents: 0,
            });
        }
    }
    unsafe { write_stdout(&[]) };
    let count = unsafe {
        libc::poll(
            wanted.as_mut_ptr(),
            wanted.len() as libc::nfds_t,
            (timeout >> 1) as i32,
        )
    };
    if count < 0 {
        let code = io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(EIO as i32);
        return unsafe { string(code.to_string().as_bytes()) };
    }
    let mut reply = vec![b'='];
    let found: Vec<String> = wanted
        .iter()
        .map(|entry| entry.revents.to_string())
        .collect();
    reply.extend_from_slice(found.join(",").as_bytes());
    unsafe { string(&reply) }
}

#[cfg(not(unix))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_poll(_spec: i64, _timeout: i64) -> i64 {
    unsafe { string(ENOSYS.to_string().as_bytes()) }
}
