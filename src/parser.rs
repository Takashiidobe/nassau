use std::collections::HashSet;

use crate::error::{LexerError, ParseError};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::span::Span;

pub type Expr = Span<ExprKind>;
pub type Stmt = Span<StmtKind>;

#[derive(Debug)]
pub enum ExprKind {
    Integer(i64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    String(String),
    Word8FromInt(Box<Expr>),
    PosixExit(Box<Expr>),
}

#[derive(Debug)]
pub enum StmtKind {
    Val(String, Expr),
    Print(Expr),
    Exit(Expr),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
    pub result: i32,
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    allow_implicit_val: bool,
    variables: HashSet<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            allow_implicit_val: false,
            variables: HashSet::new(),
        }
    }

    pub fn from_source(source: &str, file: &str) -> Result<Self, LexerError> {
        Ok(Self::new(Lexer::new(source, file).tokenize()?))
    }

    pub fn from_repl_source(source: &str, file: &str) -> Result<Self, LexerError> {
        let mut parser = Self::new(Lexer::new(source, file).tokenize()?);
        parser.allow_implicit_val = true;
        Ok(parser)
    }

    pub fn from_repl_source_with_variables(
        source: &str,
        file: &str,
        variables: impl IntoIterator<Item = String>,
    ) -> Result<Self, LexerError> {
        let mut parser = Self::from_repl_source(source, file)?;
        parser.variables.extend(variables);
        Ok(parser)
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
            let start = if self.tokens[self.index].value == TokenKind::Val {
                let start = self
                    .expect(TokenKind::Val, "expected a val declaration")?
                    .start;
                start
            } else if self.allow_implicit_val {
                self.tokens[self.index].start.clone()
            } else {
                return Err(self.error("expected a val declaration"));
            };
            if matches!(
                self.tokens.get(self.index).map(|token| &token.value),
                Some(TokenKind::Underscore)
            ) {
                self.index += 1;
                self.expect(TokenKind::Equals, "expected '=' after val pattern")?;
                let kind = self.parse_effect()?;
                let end = self.tokens[self.index - 1].end.clone();
                statements.push(Span::new(start, end, kind));
                continue;
            }
            let name = match self.tokens.get(self.index).map(|token| &token.value) {
                Some(TokenKind::Identifier(name)) => name.clone(),
                _ => return Err(self.error("expected a variable name or wildcard pattern")),
            };
            self.index += 1;
            self.expect(TokenKind::Equals, "expected '=' after val name")?;
            let expr = self.parse_integer_expr()?;
            let end = expr.end.clone();
            self.variables.insert(name.clone());
            statements.push(Span::new(start, end, StmtKind::Val(name, expr)));
        }
        if statements.is_empty() {
            return Err(self.error("expected a program"));
        }
        Ok(Program {
            statements,
            result: 0,
        })
    }

    fn parse_effect(&mut self) -> Result<StmtKind, ParseError> {
        let identifier = self
            .tokens
            .get(self.index)
            .ok_or_else(|| self.error("expected print or Posix.Process.exit"))?
            .clone();
        match &identifier.value {
            TokenKind::Identifier(name) if name == "print" => {
                self.index += 1;
                let token = self.expect_string("print expects a string literal")?;
                let TokenKind::String(value) = token.value else {
                    unreachable!()
                };
                Ok(StmtKind::Print(Span::new(
                    token.start,
                    token.end,
                    ExprKind::String(value),
                )))
            }
            TokenKind::Identifier(name) if name == "Posix" => {
                Ok(StmtKind::Exit(self.parse_posix_exit()?))
            }
            _ => Err(self.error("expected print or Posix.Process.exit")),
        }
    }

    fn parse_integer_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_atom()?;
        while self
            .tokens
            .get(self.index)
            .is_some_and(|token| token.value == TokenKind::Plus)
        {
            self.index += 1;
            let rhs = self.parse_atom()?;
            let start = expr.start.clone();
            let end = rhs.end.clone();
            expr = Span::new(start, end, ExprKind::Add(Box::new(expr), Box::new(rhs)));
        }
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        let token = self
            .tokens
            .get(self.index)
            .ok_or_else(|| self.error("expected an integer expression"))?
            .clone();
        self.index += 1;
        match token.value {
            TokenKind::Integer(value) => {
                Ok(Span::new(token.start, token.end, ExprKind::Integer(value)))
            }
            TokenKind::Identifier(name) if self.variables.contains(&name) => {
                Ok(Span::new(token.start, token.end, ExprKind::Variable(name)))
            }
            TokenKind::Identifier(name) => Err(self.error(format!("unbound variable '{name}'"))),
            _ => Err(self.error("expected an integer expression")),
        }
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
        let integer = self.parse_integer_expr()?;
        let word8_end = integer.end.clone();
        let word8 = Span::new(
            word8_start,
            word8_end,
            ExprKind::Word8FromInt(Box::new(integer)),
        );
        let end = self
            .expect(TokenKind::RightParen, "expected ')' after exit status")?
            .end;
        Ok(Span::new(start, end, ExprKind::PosixExit(Box::new(word8))))
    }
}
