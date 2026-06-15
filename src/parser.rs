use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::lexer::{Lexer, LexerError, Token, TokenKind};
use crate::span::Span;

pub type Expr = Span<ExprKind>;
pub type Stmt = Span<StmtKind>;

#[derive(Debug)]
pub enum ExprKind {
    Integer(i64),
    String(String),
    Word8FromInt(Box<Expr>),
    PosixExit(Box<Expr>),
}

#[derive(Debug)]
pub enum StmtKind {
    Print(Expr),
    Exit(Expr),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
    pub result: i32,
}

#[derive(Debug, Error, Diagnostic)]
#[error("{message}")]
#[diagnostic(code(nassau::parser))]
pub struct ParseError {
    pub message: String,
    #[label("could not parse this source")]
    pub span: SourceSpan,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn from_source(source: &str, file: &str) -> Result<Self, LexerError> {
        Ok(Self::new(Lexer::new(source, file).tokenize()?))
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        let token = self.tokens.get(self.index).or_else(|| self.tokens.last());
        let span = token
            .map(Span::source_span)
            .unwrap_or_else(|| (0, 0).into());
        ParseError {
            message: message.into(),
            span,
        }
    }

    fn expect(&mut self, expected: TokenKind, message: &str) -> Result<Token, ParseError> {
        let Some(token) = self.tokens.get(self.index) else {
            return Err(self.error(message));
        };
        if token.value != expected {
            return Err(self.error(message));
        }
        self.index += 1;
        Ok(token.clone())
    }

    pub fn parse(mut self) -> Result<Program, ParseError> {
        if self.tokens.len() == 1 {
            if let TokenKind::Integer(value) = self.tokens[0].value {
                let result = i32::try_from(value)
                    .map_err(|_| self.error("integer literal does not fit in i32"))?;
                return Ok(Program {
                    statements: Vec::new(),
                    result,
                });
            }
        }
        let mut statements = Vec::new();
        while self.index < self.tokens.len() {
            if self.tokens[self.index].value == TokenKind::Semicolon {
                self.index += 1;
                continue;
            }
            let start = self
                .expect(TokenKind::Val, "expected a val declaration")?
                .start;
            self.expect(
                TokenKind::Underscore,
                "expected wildcard pattern '_' after val",
            )?;
            self.expect(TokenKind::Equals, "expected '=' after val pattern")?;
            let identifier = self
                .tokens
                .get(self.index)
                .ok_or_else(|| self.error("expected print or Posix.Process.exit"))?
                .clone();
            let kind = match &identifier.value {
                TokenKind::Identifier(name) if name == "print" => {
                    self.index += 1;
                    let token = self.expect_string("print expects a string literal")?;
                    let TokenKind::String(value) = token.value else {
                        unreachable!()
                    };
                    StmtKind::Print(Span::new(token.start, token.end, ExprKind::String(value)))
                }
                TokenKind::Identifier(name) if name == "Posix" => {
                    let value = self.parse_posix_exit()?;
                    StmtKind::Exit(value)
                }
                _ => return Err(self.error("expected print or Posix.Process.exit")),
            };
            let end = self.tokens[self.index - 1].end.clone();
            statements.push(Span::new(start, end, kind));
        }
        if statements.is_empty() {
            return Err(self.error("expected a program"));
        }
        Ok(Program {
            statements,
            result: 0,
        })
    }

    fn expect_string(&mut self, message: &str) -> Result<Token, ParseError> {
        let Some(token) = self.tokens.get(self.index) else {
            return Err(self.error(message));
        };
        if !matches!(token.value, TokenKind::String(_)) {
            return Err(self.error(message));
        }
        self.index += 1;
        Ok(token.clone())
    }

    fn parse_posix_exit(&mut self) -> Result<Expr, ParseError> {
        let start = self
            .expect(
                TokenKind::Identifier("Posix".into()),
                "expected Posix.Process.exit",
            )?
            .start;
        self.expect(TokenKind::Dot, "expected '.' after Posix")?;
        self.expect(
            TokenKind::Identifier("Process".into()),
            "expected Process after Posix.",
        )?;
        self.expect(TokenKind::Dot, "expected '.' after Posix.Process")?;
        self.expect(
            TokenKind::Identifier("exit".into()),
            "expected exit after Posix.Process.",
        )?;
        self.expect(TokenKind::LeftParen, "expected '(' before exit status")?;
        let word8_start = self
            .expect(
                TokenKind::Identifier("Word8".into()),
                "expected Word8.fromInt",
            )?
            .start;
        self.expect(TokenKind::Dot, "expected '.' after Word8")?;
        self.expect(
            TokenKind::Identifier("fromInt".into()),
            "expected fromInt after Word8.",
        )?;
        let integer = self
            .tokens
            .get(self.index)
            .ok_or_else(|| self.error("Word8.fromInt expects an integer"))?
            .clone();
        let TokenKind::Integer(value) = integer.value else {
            return Err(self.error("Word8.fromInt expects an integer"));
        };
        self.index += 1;
        let inner = Span::new(
            integer.start.clone(),
            integer.end.clone(),
            ExprKind::Integer(value),
        );
        self.expect(
            TokenKind::RightParen,
            "expected ')' after Word8.fromInt argument",
        )?;
        let word8 = Span::new(
            word8_start,
            integer.end,
            ExprKind::Word8FromInt(Box::new(inner)),
        );
        Ok(Span::new(
            start,
            self.tokens[self.index - 1].end.clone(),
            ExprKind::PosixExit(Box::new(word8)),
        ))
    }
}
