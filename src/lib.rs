#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod constructors;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod core;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod error;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod infer;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod lexer;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod lower;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod matching;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod parser;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod prelude;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod printing;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod scope;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod session;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod span;
pub mod value;
#[cfg(any(not(feature = "runtime-archive"), feature = "codegen"))]
pub mod walk;

#[cfg(feature = "web")]
mod web;

#[cfg(feature = "codegen")]
pub mod codegen;
#[cfg(feature = "native")]
pub mod runtime;

#[cfg(any(feature = "native", feature = "runtime-archive"))]
#[path = "../runtime/nassau_runtime.rs"]
mod native_runtime;

#[cfg(all(feature = "runtime-archive", not(feature = "native")))]
pub use native_runtime::*;
