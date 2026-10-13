//! The shared runtime, linked as a Rust library for JIT calls and an archive for native code.

use crate::native_runtime::{
    nassau_alloc, nassau_array_new, nassau_code_global, nassau_exception, nassau_exit,
    nassau_global_root, nassau_os_chdir, nassau_os_close, nassau_os_error_message,
    nassau_os_file_size, nassau_os_get_dir, nassau_os_getenv, nassau_os_is_dir, nassau_os_mkdir,
    nassau_os_now, nassau_os_open, nassau_os_open_dir, nassau_os_read, nassau_os_read_dir,
    nassau_os_remove, nassau_os_rewind_dir, nassau_os_rmdir, nassau_os_sleep, nassau_os_system,
    nassau_os_write, nassau_print, nassau_raised, nassau_repl, nassau_roots_pop, nassau_roots_push,
    nassau_take_uncaught, nassau_terminate, nassau_uncaught, nassau_vector_from_array,
};

/// The runtime archive, written next to the object file when linking.
pub const ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/libnassau_runtime.a"));

pub const NATIVE_LIBS: &str = env!("NASSAU_RUNTIME_NATIVE_LIBS");

/// Runtime entry points called by generated code, for the JIT's symbol table.
pub fn symbols() -> [(&'static str, *const u8); 32] {
    [
        ("nassau_alloc", nassau_alloc as *const u8),
        ("nassau_array_new", nassau_array_new as *const u8),
        (
            "nassau_vector_from_array",
            nassau_vector_from_array as *const u8,
        ),
        ("nassau_global_root", nassau_global_root as *const u8),
        ("nassau_code_global", nassau_code_global as *const u8),
        ("nassau_roots_push", nassau_roots_push as *const u8),
        ("nassau_roots_pop", nassau_roots_pop as *const u8),
        ("nassau_print", nassau_print as *const u8),
        ("nassau_exit", nassau_exit as *const u8),
        ("nassau_terminate", nassau_terminate as *const u8),
        ("nassau_exception", nassau_exception as *const u8),
        ("nassau_raised", nassau_raised as *const u8),
        ("nassau_uncaught", nassau_uncaught as *const u8),
        ("nassau_os_open", nassau_os_open as *const u8),
        ("nassau_os_read", nassau_os_read as *const u8),
        ("nassau_os_write", nassau_os_write as *const u8),
        ("nassau_os_close", nassau_os_close as *const u8),
        ("nassau_os_remove", nassau_os_remove as *const u8),
        (
            "nassau_os_error_message",
            nassau_os_error_message as *const u8,
        ),
        ("nassau_os_open_dir", nassau_os_open_dir as *const u8),
        ("nassau_os_read_dir", nassau_os_read_dir as *const u8),
        ("nassau_os_rewind_dir", nassau_os_rewind_dir as *const u8),
        ("nassau_os_is_dir", nassau_os_is_dir as *const u8),
        ("nassau_os_file_size", nassau_os_file_size as *const u8),
        ("nassau_os_mkdir", nassau_os_mkdir as *const u8),
        ("nassau_os_rmdir", nassau_os_rmdir as *const u8),
        ("nassau_os_get_dir", nassau_os_get_dir as *const u8),
        ("nassau_os_chdir", nassau_os_chdir as *const u8),
        ("nassau_os_system", nassau_os_system as *const u8),
        ("nassau_os_sleep", nassau_os_sleep as *const u8),
        ("nassau_os_now", nassau_os_now as *const u8),
        ("nassau_os_getenv", nassau_os_getenv as *const u8),
    ]
}

/// Makes an exception that escapes a chunk wait for the REPL to report it,
/// instead of reporting it as a program's uncaught exception.
pub fn enter_repl() {
    // SAFETY: sets a flag the runtime reads when an exception escapes.
    unsafe { nassau_repl() }
}

/// The exception the last chunk left uncaught, if any.
pub fn take_uncaught() -> Option<i64> {
    // SAFETY: reads and clears the runtime's record of it.
    let exception = unsafe { nassau_take_uncaught() };
    (exception != 0).then_some(exception)
}

/// The identity of the built-in exception with index `index`.
pub fn builtin_exception(index: i64) -> i64 {
    // SAFETY: the index names one of the runtime's built-in exceptions.
    unsafe { nassau_exception(crate::value::tagged(index)) }
}

pub fn with_root<T>(value: i64, f: impl FnOnce() -> T) -> T {
    crate::native_runtime::with_root(value, f)
}

pub fn replace_global_roots(addresses: &[usize]) {
    crate::native_runtime::replace_global_roots(addresses);
}

pub fn reset_repl_roots() {
    crate::native_runtime::reset_repl_roots();
}
