//! The runtime library (runtime/nassau_runtime.c), which build.rs compiles.
//! Compiled programs link the archive; the JIT calls the copy linked into
//! the compiler.

/// The runtime archive, written next to the object file when linking.
pub const ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/libnassau_runtime.a"));

unsafe extern "C" {
    fn nassau_alloc(fields: i64) -> *mut i64;
    fn nassau_print(string: i64) -> i64;
    fn nassau_concat(lhs: i64, rhs: i64) -> i64;
    fn nassau_int_to_string(integer: i64) -> i64;
    fn nassau_equal(lhs: i64, rhs: i64) -> i64;
    fn nassau_exit(status: i64);
    fn nassau_exception(index: i64) -> i64;
    fn nassau_raise_exception(exception: i64, location: i64);
    fn nassau_raise_builtin(index: i64, location: i64);
    fn nassau_caught() -> i64;
    fn nassau_uncaught() -> i32;
    fn nassau_repl();
    fn nassau_take_uncaught() -> i64;
}

/// Every runtime entry point, for the JIT's symbol table.
pub fn symbols() -> [(&'static str, *const u8); 11] {
    [
        ("nassau_alloc", nassau_alloc as *const u8),
        ("nassau_print", nassau_print as *const u8),
        ("nassau_concat", nassau_concat as *const u8),
        ("nassau_int_to_string", nassau_int_to_string as *const u8),
        ("nassau_equal", nassau_equal as *const u8),
        ("nassau_exit", nassau_exit as *const u8),
        ("nassau_exception", nassau_exception as *const u8),
        (
            "nassau_raise_exception",
            nassau_raise_exception as *const u8,
        ),
        ("nassau_raise_builtin", nassau_raise_builtin as *const u8),
        ("nassau_caught", nassau_caught as *const u8),
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
