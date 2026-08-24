//! The shared runtime, linked as a Rust library for JIT calls and an archive for native code.

use nassau_runtime::{
    nassau_alloc, nassau_code_global, nassau_exception, nassau_exit, nassau_global_root,
    nassau_print, nassau_raised, nassau_repl, nassau_roots_pop, nassau_roots_push,
    nassau_take_uncaught, nassau_uncaught,
};

/// The runtime archive, written next to the object file when linking.
pub const ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/libnassau_runtime.a"));

pub const NATIVE_LIBS: &str = env!("NASSAU_RUNTIME_NATIVE_LIBS");

/// Runtime entry points called by generated code, for the JIT's symbol table.
pub fn symbols() -> [(&'static str, *const u8); 10] {
    [
        ("nassau_alloc", nassau_alloc as *const u8),
        ("nassau_global_root", nassau_global_root as *const u8),
        ("nassau_code_global", nassau_code_global as *const u8),
        ("nassau_roots_push", nassau_roots_push as *const u8),
        ("nassau_roots_pop", nassau_roots_pop as *const u8),
        ("nassau_print", nassau_print as *const u8),
        ("nassau_exit", nassau_exit as *const u8),
        ("nassau_exception", nassau_exception as *const u8),
        ("nassau_raised", nassau_raised as *const u8),
        ("nassau_uncaught", nassau_uncaught as *const u8),
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
    nassau_runtime::with_root(value, f)
}

pub fn replace_global_roots(addresses: &[usize]) {
    nassau_runtime::replace_global_roots(addresses);
}
