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
pub const NIL: i64 = tagged(0);

/// Heap block kinds, in the low byte of a block's header word (the rest are
/// listed in the design note). Fields are values: tuples, records, cons cells and constructor payloads.
pub const KIND_RECORD: i64 = 0;
/// One IEEE double.
pub const KIND_REAL: i64 = 3;

/// A block's header word: its length above the kind byte.
pub const fn header(length: i64, kind: i64) -> i64 {
    (length << 8) | kind
}
