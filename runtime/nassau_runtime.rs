use core::ffi::{c_char, c_int, c_void};
use core::ptr;

mod float;
pub use float::*;
mod gc;
mod math;
pub use math::*;
mod time;
pub use time::*;
mod os;
pub use os::*;
mod posix;
pub use posix::*;

use crate::value;

use value::{BUILTIN_EXCEPTIONS, KIND_ARRAY, KIND_REF, KIND_STRING, KIND_VECTOR, NIL, header};

unsafe extern "C" {
    fn fwrite(bytes: *const u8, size: usize, count: usize, stream: *mut c_void) -> usize;
    fn fflush(stream: *mut c_void) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn _Exit(status: c_int) -> !;
    #[cfg_attr(target_vendor = "apple", link_name = "__stdoutp")]
    static mut stdout: *mut c_void;
    #[cfg_attr(target_vendor = "apple", link_name = "__stderrp")]
    static mut stderr: *mut c_void;
}

static mut IDENTITIES: [i64; BUILTIN_EXCEPTIONS.len()] = [0; BUILTIN_EXCEPTIONS.len()];
static mut RAISED: i64 = 0;
static mut REPL: bool = false;
static mut UNCAUGHT: i64 = 0;

unsafe fn alloc(length: i64, kind: i64) -> *mut i64 {
    let bytes = gc::physical_size(length, kind).unwrap_or_else(|| out_of_memory());
    gc::allocate(bytes, header(length, kind))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_alloc(length: i64, kind: i64) -> i64 {
    unsafe { alloc(length, kind) as i64 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_array_new(length: i64, initial: i64) -> i64 {
    let length = usize::try_from(length >> 1).unwrap_or_else(|_| out_of_memory());
    gc::with_roots([initial as u64], || unsafe {
        let array = alloc(length as i64, KIND_ARRAY);
        for index in 0..length {
            array.add(index + 1).write(initial);
        }
        array as i64
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_vector_from_array(array: i64) -> i64 {
    let array = array as *const i64;
    gc::with_roots([array as u64], || unsafe {
        let header = array.read();
        let length = (header >> 8) as usize;
        let vector = alloc(length as i64, KIND_VECTOR);
        ptr::copy_nonoverlapping(array.add(1), vector.add(1), length);
        vector as i64
    })
}

fn out_of_memory() -> ! {
    unsafe {
        fprintf(stderr, c"nassau: out of memory\n".as_ptr());
        exit(1);
    }
}

unsafe fn string(bytes: &[u8]) -> i64 {
    unsafe {
        let block = alloc(bytes.len() as i64, KIND_STRING);
        ptr::copy_nonoverlapping(bytes.as_ptr(), block.add(1).cast(), bytes.len());
        block.add(1).cast::<u8>().add(bytes.len()).write(0);
        block as i64
    }
}

unsafe fn string_bytes(string: i64) -> *const u8 {
    unsafe { (string as *const i64).add(1).cast() }
}

unsafe fn string_length(string: i64) -> usize {
    unsafe { ((string as *const i64).read() >> 8) as usize }
}

unsafe fn write_stdout(bytes: &[u8]) {
    unsafe {
        fwrite(bytes.as_ptr(), 1, bytes.len(), stdout);
        fflush(stdout);
    }
}

unsafe fn write_stderr(bytes: &[u8]) {
    unsafe {
        fwrite(bytes.as_ptr(), 1, bytes.len(), stderr);
        fflush(stderr);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_print(string: i64) -> i64 {
    unsafe {
        write_stdout(core::slice::from_raw_parts(
            string_bytes(string),
            string_length(string),
        ))
    };
    NIL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_exception(index: i64) -> i64 {
    unsafe {
        let slot = (index >> 1) as usize;
        let identity = (&raw mut IDENTITIES).cast::<i64>().add(slot);
        if identity.read() == 0 {
            let name = BUILTIN_EXCEPTIONS.get_unchecked(slot);
            let block = alloc(1, KIND_REF);
            gc::with_roots([block as u64], || {
                block.add(1).write(string(name.as_bytes()))
            });
            identity.write(block as i64);
        }
        identity.read()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_raised() -> i64 {
    (&raw mut RAISED) as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_repl() {
    unsafe { REPL = true }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_take_uncaught() -> i64 {
    unsafe {
        let exception = UNCAUGHT;
        UNCAUGHT = 0;
        exception
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_uncaught() -> c_int {
    unsafe {
        let exception = RAISED as *const i64;
        RAISED = 0;
        if REPL {
            UNCAUGHT = exception as i64;
            return 0;
        }
        let identity = exception.add(1).read() as *const i64;
        let name = identity.add(1).read();
        let argument = exception.add(2).read();
        let location = exception.add(3).read();
        fflush(stdout);
        fprintf(
            stderr,
            c"/usr/lib/smlnj/bin/sml: Fatal error -- Uncaught exception %.*s with ".as_ptr(),
            string_length(name) as c_int,
            string_bytes(name),
        );
        if argument & 1 != 0 {
            fprintf(
                stderr,
                c"%lld\n raised at %.*s\n\n".as_ptr(),
                argument >> 1,
                string_length(location) as c_int,
                string_bytes(location),
            );
        } else if (argument as *const i64).read() & 0xff == KIND_STRING {
            fprintf(
                stderr,
                c"\"%.*s\" raised at %.*s\n\n".as_ptr(),
                string_length(argument) as c_int,
                string_bytes(argument),
                string_length(location) as c_int,
                string_bytes(location),
            );
        } else {
            fprintf(
                stderr,
                c"<unknown> raised at %.*s\n\n".as_ptr(),
                string_length(location) as c_int,
                string_bytes(location),
            );
        }
        1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_exit(status: i64) {
    unsafe {
        fflush(stdout);
        exit(((status >> 1) & 0xff) as c_int);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_terminate(status: i64) {
    unsafe { _Exit(((status >> 1) & 0xff) as c_int) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_roots_push(frame: i64, count: i64, code: i64) {
    unsafe { gc::push_roots(frame as usize as *mut u64, count as u64, code as u64) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_roots_pop(frame: i64) {
    unsafe { gc::pop_roots(frame as usize as *mut u64) }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_host_root_push(value: i64) {
    gc::push_host_root(value as u64);
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_host_root_pop() {
    gc::pop_host_root();
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_reset_frames() {
    gc::reset_frames();
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_clear_globals() {
    gc::clear_globals();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_static_alloc(size: i64, align: i64) -> i64 {
    let layout = std::alloc::Layout::from_size_align(size.max(8) as usize, align as usize)
        .unwrap_or_else(|_| out_of_memory());
    let address = unsafe { std::alloc::alloc_zeroed(layout) };
    if address.is_null() {
        out_of_memory();
    }
    address as i64
}

pub fn with_root<T>(value: i64, f: impl FnOnce() -> T) -> T {
    gc::with_roots([value as u64], f)
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_global_root(address: i64) {
    gc::register_global(address as usize);
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_code_global(code: i64, address: i64) {
    gc::register_code_global(code as usize, address as usize);
}

pub fn replace_global_roots(addresses: &[usize]) {
    gc::replace_globals(addresses);
}

pub fn reset_repl_roots() {
    gc::reset_repl_roots();
}
