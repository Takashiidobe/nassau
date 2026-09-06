//! The runtime representation of SML values; docs/value-representation.md
//! explains the choices.

/// The smallest and largest `int`: 31 bits, as in SML/NJ.
pub const INT_MIN: i64 = -(1 << 30);
pub const INT_MAX: i64 = (1 << 30) - 1;

pub fn int_fits(value: i64) -> bool {
    (INT_MIN..=INT_MAX).contains(&value)
}

/// The word of an immediate value: an `int`, `char`, `bool`, `unit` or
/// nullary constructor, shifted left with the low bit set.
pub const fn tagged(value: i64) -> i64 {
    (value << 1) | 1
}

/// `nil`, `false`, `unit` and `0` share this word.
/// What a function returns when it raises an exception instead: neither an
/// immediate nor a pointer to a block.
pub const RAISED: i64 = 0;

pub const NIL: i64 = tagged(0);
pub const FALSE: i64 = tagged(0);
pub const TRUE: i64 = tagged(1);

/// Heap block kinds, in the low byte of a block's header word; the design
/// note lists the rest. Fields are values: tuples, records, cons cells and
/// constructor payloads.
pub const KIND_RECORD: i64 = 0;
/// Field 0 is a code address; the rest are the free variables' values.
pub const KIND_CLOSURE: i64 = 1;
/// Bytes, NUL-terminated; the header's length counts bytes, not words.
pub const KIND_STRING: i64 = 2;
/// One IEEE double.
pub const KIND_REAL: i64 = 3;
/// One mutable field.
pub const KIND_REF: i64 = 4;

/// A block's header word: its length above the kind byte.
pub const fn header(length: i64, kind: i64) -> i64 {
    (length << 8) | kind
}

/// The basis's exceptions, in the order the runtime numbers them
/// (`nassau_exception`).
pub const BUILTIN_EXCEPTIONS: &[&str] = &[
    "Div",
    "Overflow",
    "Match",
    "Bind",
    "Fail",
    "Subscript",
    "Empty",
    "Size",
];

/// The index of the built-in exception `name`.
pub fn builtin_exception(name: &str) -> Option<i64> {
    let index = BUILTIN_EXCEPTIONS.iter().position(|known| *known == name)?;
    i64::try_from(index).ok()
}
