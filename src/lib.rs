pub mod constructors;
pub mod core;
pub mod error;
pub mod infer;
pub mod lexer;
pub mod lower;
pub mod matching;
pub mod parser;
pub mod prelude;
pub mod printing;
pub mod scope;
pub mod session;
pub mod span;
pub mod value;
pub mod walk;

#[cfg(feature = "web")]
mod web;

#[cfg(feature = "codegen")]
pub mod codegen;
#[cfg(feature = "native")]
pub mod runtime;
