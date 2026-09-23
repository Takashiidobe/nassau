pub use thiserror::Error as ThisError;

use miette::{Diagnostic, SourceSpan};

use crate::span::Span;

#[derive(Debug, ThisError, Diagnostic)]
#[error("{error}")]
#[diagnostic(code(nassau::source))]
pub struct SourceError<E: std::error::Error + 'static> {
    #[source]
    pub error: E,
    #[label("error occurs here")]
    pub span: SourceSpan,
}

impl<E: std::error::Error + 'static> SourceError<E> {
    pub fn new(error: E, span: SourceSpan) -> Self {
        Self { error, span }
    }

    pub fn from_span(error: Span<E>) -> Self {
        let span = error.source_span();
        Self::new(error.value, span)
    }
}

#[derive(Clone, Copy, Debug, ThisError)]
pub enum LexerErrorKind {
    #[error("unterminated comment")]
    UnterminatedComment,
    #[error("unsupported string escape")]
    UnsupportedStringEscape,
    #[error("unterminated string escape")]
    UnterminatedStringEscape,
    #[error("unterminated string literal")]
    UnterminatedString,
    #[error("invalid real literal")]
    InvalidRealLiteral,
    #[error("invalid integer literal")]
    InvalidIntegerLiteral,
    #[error("unexpected character: {0}")]
    UnexpectedCharacter(char),
}

pub type LexerError = Span<LexerErrorKind>;

#[derive(Debug, ThisError)]
pub enum ParseErrorKind {
    #[error("expected {0}")]
    Expect(String),
    #[error("integer literal does not fit in i32")]
    IntegerOutOfRange,
}

pub type ParseError = Span<ParseErrorKind>;

#[derive(Debug, ThisError)]
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
