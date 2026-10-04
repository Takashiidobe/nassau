use core::ffi::{c_char, c_int, c_void};
use core::ptr;

mod gc;

#[expect(
    dead_code,
    reason = "the shared value layout also defines compiler-only helpers"
)]
#[path = "../src/value.rs"]
mod value;

use value::{BUILTIN_EXCEPTIONS, KIND_REF, KIND_STRING, NIL, header};

unsafe extern "C" {
    fn fwrite(bytes: *const u8, size: usize, count: usize, stream: *mut c_void) -> usize;
    fn fflush(stream: *mut c_void) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    static mut stdout: *mut c_void;
    static mut stderr: *mut c_void;
}

static mut IDENTITIES: [i64; BUILTIN_EXCEPTIONS.len()] = [0; BUILTIN_EXCEPTIONS.len()];
static mut RAISED: i64 = 0;
static mut REPL: bool = false;
static mut UNCAUGHT: i64 = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_alloc(length: i64, kind: i64) -> *mut i64 {
    let bytes = gc::physical_size(length, kind).unwrap_or_else(|| out_of_memory());
    gc::allocate(bytes, header(length, kind))
}

fn out_of_memory() -> ! {
    unsafe {
        fprintf(stderr, c"nassau: out of memory\n".as_ptr());
        exit(1);
    }
}

unsafe fn string(bytes: &[u8]) -> i64 {
    unsafe {
        let block = nassau_alloc(bytes.len() as i64, KIND_STRING);
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_print(string: i64) -> i64 {
    unsafe {
        fwrite(string_bytes(string), 1, string_length(string), stdout);
        fflush(stdout);
    }
    NIL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_exception(index: i64) -> i64 {
    unsafe {
        let slot = (index >> 1) as usize;
        let identity = (&raw mut IDENTITIES).cast::<i64>().add(slot);
        if identity.read() == 0 {
            let name = BUILTIN_EXCEPTIONS.get_unchecked(slot);
            let block = nassau_alloc(1, KIND_REF);
            gc::with_roots([block as usize], || {
                block.add(1).write(string(name.as_bytes()))
            });
            identity.write(block as i64);
        }
        identity.read()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_raised() -> *mut i64 {
    &raw mut RAISED
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
pub unsafe extern "C" fn nassau_roots_push(frame: *mut usize, count: usize, code: usize) {
    unsafe { gc::push_roots(frame, count, code) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nassau_roots_pop(frame: *mut usize) {
    unsafe { gc::pop_roots(frame) }
}

pub fn with_root<T>(value: i64, f: impl FnOnce() -> T) -> T {
    gc::with_roots([value as usize], f)
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_global_root(address: usize) {
    gc::register_global(address);
}

#[unsafe(no_mangle)]
pub extern "C" fn nassau_code_global(code: usize, address: usize) {
    gc::register_code_global(code, address);
}

pub fn replace_global_roots(addresses: &[usize]) {
    gc::replace_globals(addresses);
}

pub fn reset_repl_roots() {
    gc::reset_repl_roots();
}
