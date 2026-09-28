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
    #[error("invalid integer literal")]
    InvalidIntegerLiteral,
    #[error("invalid word literal")]
    InvalidWordLiteral,
    #[error("invalid character literal")]
    InvalidCharacterLiteral,
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
    #[error("invalid real literal")]
    InvalidRealLiteral,
    #[error("{0} are not supported yet")]
    Unsupported(&'static str),
}

pub type ParseError = Span<ParseErrorKind>;

#[derive(Clone, Copy, Debug, ThisError)]
pub enum MatchErrorKind {
    #[error("match redundant")]
    Redundant,
}

#[derive(Clone, Debug, ThisError)]
#[expect(
    clippy::enum_variant_names,
    reason = "each variant names what was declared twice"
)]
pub enum ScopeErrorKind {
    #[error("duplicate variable '{0}' in pattern")]
    DuplicateVariable(String),
    #[error("duplicate function name '{0}'")]
    DuplicateFunction(String),
    #[error("duplicate constructor name '{0}' in datatype declaration")]
    DuplicateConstructor(String),
    #[error("duplicate type name '{0}' in type declaration")]
    DuplicateType(String),
    #[error("duplicate exception name '{0}' in exception declaration")]
    DuplicateException(String),
}

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
