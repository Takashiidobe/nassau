use std::path::{Path, PathBuf};

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::span::{Loc, Span};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Integer(i64),
    String(String),
    Val,
    Identifier(String),
    Underscore,
    Equals,
    Semicolon,
    Dot,
    LeftParen,
    RightParen,
}

pub type Token = Span<TokenKind>;

#[derive(Debug, Error, Diagnostic)]
#[error("{message}")]
#[diagnostic(code(nassau::lexer))]
pub struct LexerError {
    pub message: String,
    #[label("invalid source here")]
    pub span: SourceSpan,
}

pub struct Lexer<'a> {
    source: &'a str,
    file: PathBuf,
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, file: impl AsRef<Path>) -> Self {
        Self {
            source,
            file: file.as_ref().to_path_buf(),
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    fn loc(&self) -> Loc {
        Loc {
            file: self.file.clone(),
            line: self.line,
            column: self.column,
            offset: self.offset,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.offset..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.offset..].chars();
        chars.next()?;
        chars.next()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.offset += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn error(&self, start: usize, message: impl Into<String>) -> LexerError {
        LexerError {
            message: message.into(),
            span: (start, self.offset.saturating_sub(start).max(1)).into(),
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.bump();
                continue;
            }
            if ch == '(' && self.peek_next() == Some('*') {
                let start = self.offset;
                self.bump();
                self.bump();
                let mut depth = 1;
                while depth > 0 {
                    match (self.peek(), self.peek_next()) {
                        (Some('('), Some('*')) => {
                            self.bump();
                            self.bump();
                            depth += 1;
                        }
                        (Some('*'), Some(')')) => {
                            self.bump();
                            self.bump();
                            depth -= 1;
                        }
                        (Some(_), _) => {
                            self.bump();
                        }
                        (None, _) => return Err(self.error(start, "unterminated comment")),
                    }
                }
                continue;
            }
            let start = self.loc();
            let first = self.bump().unwrap();
            let kind = match first {
                '_' => TokenKind::Underscore,
                '=' => TokenKind::Equals,
                ';' => TokenKind::Semicolon,
                '.' => TokenKind::Dot,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '"' => {
                    let mut value = String::new();
                    loop {
                        match self.bump() {
                            Some('"') => break,
                            Some('\\') => match self.bump() {
                                Some('n') => value.push('\n'),
                                Some('t') => value.push('\t'),
                                Some('"') => value.push('"'),
                                Some('\\') => value.push('\\'),
                                Some(_) => {
                                    return Err(
                                        self.error(start.offset, "unsupported string escape")
                                    );
                                }
                                None => {
                                    return Err(
                                        self.error(start.offset, "unterminated string escape")
                                    );
                                }
                            },
                            Some(ch) => value.push(ch),
                            None => {
                                return Err(self.error(start.offset, "unterminated string literal"));
                            }
                        }
                    }
                    TokenKind::String(value)
                }
                ch if ch.is_ascii_digit() || ch == '-' => {
                    let mut value = String::from(ch);
                    while self.peek().is_some_and(|next| next.is_ascii_digit()) {
                        value.push(self.bump().unwrap());
                    }
                    TokenKind::Integer(
                        value
                            .parse()
                            .map_err(|_| self.error(start.offset, "invalid integer literal"))?,
                    )
                }
                ch if ch.is_ascii_alphabetic() => {
                    let mut word = String::from(ch);
                    while self
                        .peek()
                        .is_some_and(|next| next.is_ascii_alphanumeric() || next == '_')
                    {
                        word.push(self.bump().unwrap());
                    }
                    match word.as_str() {
                        "val" => TokenKind::Val,
                        _ => TokenKind::Identifier(word),
                    }
                }
                _ => return Err(self.error(start.offset, format!("unexpected character: {first}"))),
            };
            tokens.push(Span::new(start, self.loc(), kind));
        }
        Ok(tokens)
    }
}
