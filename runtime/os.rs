use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::slice;
use std::sync::{Mutex, MutexGuard};

use super::{string, string_bytes, string_length, write_stderr, write_stdout};
use crate::value::tagged;

enum Stream {
    Stdin,
    Stdout,
    Stderr,
    File(File),
}

static STREAMS: Mutex<Vec<Option<Stream>>> = Mutex::new(Vec::new());

const READ: i64 = 0;
const WRITE: i64 = 1;
const APPEND: i64 = 2;
const EIO: i64 = 5;

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
        Ok(file) => {
            let mut streams = streams();
            let handle = streams.iter().position(Option::is_none).unwrap_or_else(|| {
                streams.push(None);
                streams.len() - 1
            });
            streams[handle] = Some(Stream::File(file));
            tagged(handle as i64)
        }
        Err(error) => failure(&error),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_os_read(handle: i64, max: i64) -> i64 {
    let mut buffer = vec![0; (max >> 1).max(0) as usize];
    let count = match streams().get_mut((handle >> 1) as usize) {
        Some(Some(Stream::Stdin)) => io::stdin().read(&mut buffer),
        Some(Some(Stream::File(file))) => file.read(&mut buffer),
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
