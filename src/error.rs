pub use thiserror::Error as ThisError;

use miette::{Diagnostic, SourceSpan};

#[derive(Debug, ThisError, Diagnostic)]
#[error("{message}")]
#[diagnostic(code(nassau::lexer))]
pub struct LexerError {
    pub message: String,
    #[label("invalid source here")]
    pub span: SourceSpan,
}

#[derive(Debug, ThisError, Diagnostic)]
#[error("{message}")]
#[diagnostic(code(nassau::parser))]
pub struct ParseError {
    pub message: String,
    #[label("could not parse this source")]
    pub span: SourceSpan,
}

#[derive(Debug, ThisError, Diagnostic)]
#[diagnostic(code(nassau::codegen))]
pub enum CodegenError {
    #[error("code generation failed: {0}")]
    Backend(String),
    #[error("I/O failed: {0}")]
    Io(String),
    #[error("external tool failed: {0}")]
    Tool(String),
    #[error("linking failed: {0}")]
    Linker(String),
    #[error("{0}")]
    Message(String),
}

impl From<String> for CodegenError {
    fn from(message: String) -> Self {
        Self::Message(message)
    }
}
