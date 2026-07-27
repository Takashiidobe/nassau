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
    fn nassau_raise(name: *const u8, location: *const u8);
    fn nassau_exit(status: i64);
}

/// Every runtime entry point, for the JIT's symbol table.
pub fn symbols() -> [(&'static str, *const u8); 7] {
    [
        ("nassau_alloc", nassau_alloc as *const u8),
        ("nassau_print", nassau_print as *const u8),
        ("nassau_concat", nassau_concat as *const u8),
        ("nassau_int_to_string", nassau_int_to_string as *const u8),
        ("nassau_equal", nassau_equal as *const u8),
        ("nassau_raise", nassau_raise as *const u8),
        ("nassau_exit", nassau_exit as *const u8),
    ]
}
