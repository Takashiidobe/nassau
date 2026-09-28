use std::path::PathBuf;

use crate::error::{LexerError, ParseError, ParseErrorKind};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::span::Span;

pub type Expr = Span<ExprKind>;
pub type Stmt = Span<StmtKind>;

#[derive(Debug)]
pub enum ExprKind {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    IntDivide(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    String(String),
    List(Vec<Expr>),
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

#[derive(Clone, Copy, Debug)]
pub enum NumericValue {
    Integer(i32),
    Real(f64),
    Boolean(bool),
    List(*mut u64),
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    allow_implicit_val: bool,
    file: PathBuf,
}

impl Parser {
    fn with_file(tokens: Vec<Token>, file: PathBuf) -> Self {
        Self {
            tokens,
            index: 0,
            allow_implicit_val: false,
            file,
        }
    }

    pub fn from_source(source: &str, file: &str) -> Result<Self, LexerError> {
        Ok(Self::with_file(
            Lexer::new(source, file).tokenize()?,
            PathBuf::from(file),
        ))
    }

    pub fn from_repl_source(source: &str, file: &str) -> Result<Self, LexerError> {
        let mut parser = Self::with_file(Lexer::new(source, file).tokenize()?, PathBuf::from(file));
        parser.allow_implicit_val = true;
        Ok(parser)
    }

    fn error(&self, kind: ParseErrorKind) -> ParseError {
        self.error_kind(kind)
    }

    fn error_kind(&self, kind: ParseErrorKind) -> ParseError {
        let token = self.tokens.get(self.index).or_else(|| self.tokens.last());
        let (start, end) = token
            .map(|token| (token.start.clone(), token.end.clone()))
            .unwrap_or_else(|| {
                let loc = crate::span::Loc {
                    file: self.file.clone(),
                    line: 1,
                    column: 1,
                    offset: 0,
                };
                (loc.clone(), loc)
            });
        Span::new(start, end, kind)
    }

    fn integer_value(literal: &str) -> Option<i64> {
        let (negative, literal) = literal
            .strip_prefix('~')
            .map_or((false, literal), |literal| (true, literal));
        let (radix, digits) = if let Some(digits) = literal
            .strip_prefix("0x")
            .or_else(|| literal.strip_prefix("0X"))
        {
            (16, digits)
        } else {
            (10, literal)
        };
        let value = u64::from_str_radix(digits, radix).ok()?;
        if negative {
            if value == (i64::MAX as u64) + 1 {
                Some(i64::MIN)
            } else if value <= i64::MAX as u64 {
                Some(-(value as i64))
            } else {
                None
            }
        } else {
            i64::try_from(value).ok()
        }
    }

    fn real_value(literal: &str) -> Result<f64, ParseErrorKind> {
        literal
            .replace('~', "-")
            .parse::<f64>()
            .map_err(|_| ParseErrorKind::InvalidRealLiteral)
    }

    fn expect(&mut self, expected: TokenKind, kind: ParseErrorKind) -> Result<Token, ParseError> {
        let Some(token) = self.tokens.get(self.index) else {
            return Err(self.error(kind));
        };
        if token.value != expected {
            return Err(self.error(kind));
        }
        self.index += 1;
        Ok(token.clone())
    }

    pub fn parse(mut self) -> Result<Program, ParseError> {
        if self.tokens.len() == 1
            && let TokenKind::Integer(ref literal) = self.tokens[0].value
        {
            let value = Self::integer_value(literal)
                .ok_or_else(|| self.error_kind(ParseErrorKind::IntegerOutOfRange))?;
            let result = i32::try_from(value)
                .map_err(|_| self.error_kind(ParseErrorKind::IntegerOutOfRange))?;
            return Ok(Program {
                statements: Vec::new(),
                result,
            });
        }
        let mut statements = Vec::new();
        while self.index < self.tokens.len() {
            if self.tokens[self.index].value == TokenKind::Semicolon {
                self.index += 1;
                continue;
            }
            let start = if self.tokens[self.index].value == TokenKind::Val {
                self.expect(
                    TokenKind::Val,
                    ParseErrorKind::Expect("a val declaration".into()),
                )?
                .start
            } else if self.allow_implicit_val {
                self.tokens[self.index].start.clone()
            } else {
                return Err(self.error(ParseErrorKind::Expect("a val declaration".into())));
            };
            if matches!(
                self.tokens.get(self.index).map(|token| &token.value),
                Some(TokenKind::Underscore)
            ) {
                self.index += 1;
                self.expect(
                    TokenKind::Equals,
                    ParseErrorKind::Expect("= after val pattern".into()),
                )?;
                let kind = self.parse_effect()?;
                let end = self.tokens[self.index - 1].end.clone();
                statements.push(Span::new(start, end, kind));
                continue;
            }
            let name = match self.tokens.get(self.index).map(|token| &token.value) {
                Some(TokenKind::Identifier(name)) => name.clone(),
                _ => {
                    return Err(self.error(ParseErrorKind::Expect(
                        "a variable name or wildcard pattern".into(),
                    )));
                }
            };
            self.index += 1;
            self.expect(
                TokenKind::Equals,
                ParseErrorKind::Expect("= after val name".into()),
            )?;
            let expr = self.parse_integer_expr()?;
            let end = expr.end.clone();
            statements.push(Span::new(start, end, StmtKind::Val(name, expr)));
        }
        if statements.is_empty() {
            return Err(self.error(ParseErrorKind::Expect("a program".into())));
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
            .ok_or_else(|| {
                self.error(ParseErrorKind::Expect("print or Posix.Process.exit".into()))
            })?
            .clone();
        match &identifier.value {
            TokenKind::Identifier(name) if name == "print" => {
                self.index += 1;
                let token = self.expect_string(ParseErrorKind::Expect(
                    "a string literal after print".into(),
                ))?;
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
            _ => Err(self.error(ParseErrorKind::Expect("print or Posix.Process.exit".into()))),
        }
    }

    fn parse_integer_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_additive_expr()?;
        while let Some(operator) = self.tokens.get(self.index).map(|token| &token.value) {
            if !matches!(
                operator,
                TokenKind::Greater
                    | TokenKind::GreaterEqual
                    | TokenKind::Less
                    | TokenKind::LessEqual
                    | TokenKind::Equals
                    | TokenKind::NotEquals
            ) {
                break;
            }
            let operator = operator.clone();
            self.index += 1;
            let rhs = self.parse_additive_expr()?;
            let start = expr.start.clone();
            let end = rhs.end.clone();
            let kind = match operator {
                TokenKind::Greater => ExprKind::Greater(Box::new(expr), Box::new(rhs)),
                TokenKind::GreaterEqual => ExprKind::GreaterEqual(Box::new(expr), Box::new(rhs)),
                TokenKind::Less => ExprKind::Less(Box::new(expr), Box::new(rhs)),
                TokenKind::LessEqual => ExprKind::LessEqual(Box::new(expr), Box::new(rhs)),
                TokenKind::Equals => ExprKind::Equal(Box::new(expr), Box::new(rhs)),
                TokenKind::NotEquals => ExprKind::NotEqual(Box::new(expr), Box::new(rhs)),
                _ => unreachable!(),
            };
            expr = Span::new(start, end, kind);
        }
        Ok(expr)
    }

    fn parse_additive_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_multiplicative_expr()?;
        while let Some(operator) = self.tokens.get(self.index).map(|token| &token.value) {
            if !matches!(operator, TokenKind::Plus | TokenKind::Minus) {
                break;
            }
            let operator = operator.clone();
            self.index += 1;
            let rhs = self.parse_multiplicative_expr()?;
            let start = expr.start.clone();
            let end = rhs.end.clone();
            let kind = match operator {
                TokenKind::Plus => ExprKind::Add(Box::new(expr), Box::new(rhs)),
                TokenKind::Minus => ExprKind::Subtract(Box::new(expr), Box::new(rhs)),
                _ => unreachable!(),
            };
            expr = Span::new(start, end, kind);
        }
        Ok(expr)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_atom()?;
        while let Some(operator) = self.tokens.get(self.index).map(|token| &token.value) {
            if !matches!(
                operator,
                TokenKind::Star | TokenKind::Slash | TokenKind::Div
            ) {
                break;
            }
            let operator = operator.clone();
            self.index += 1;
            let rhs = self.parse_atom()?;
            let start = expr.start.clone();
            let end = rhs.end.clone();
            let kind = match operator {
                TokenKind::Star => ExprKind::Multiply(Box::new(expr), Box::new(rhs)),
                TokenKind::Slash => ExprKind::Divide(Box::new(expr), Box::new(rhs)),
                TokenKind::Div => ExprKind::IntDivide(Box::new(expr), Box::new(rhs)),
                _ => unreachable!(),
            };
            expr = Span::new(start, end, kind);
        }
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        let token = self
            .tokens
            .get(self.index)
            .ok_or_else(|| self.error(ParseErrorKind::Expect("an integer expression".into())))?
            .clone();
        self.index += 1;
        match token.value {
            TokenKind::Minus => {
                let integer = self
                    .tokens
                    .get(self.index)
                    .ok_or_else(|| {
                        self.error(ParseErrorKind::Expect("an integer after unary '-'".into()))
                    })?
                    .clone();
                self.index += 1;
                match integer.value {
                    TokenKind::Integer(value) => Ok(Span::new(
                        token.start,
                        integer.end,
                        ExprKind::Integer(
                            Self::integer_value(&value)
                                .and_then(i64::checked_neg)
                                .ok_or_else(|| self.error(ParseErrorKind::IntegerOutOfRange))?,
                        ),
                    )),
                    TokenKind::Real(value) => {
                        let value = Self::real_value(&value).map_err(|kind| self.error(kind))?;
                        Ok(Span::new(token.start, integer.end, ExprKind::Real(-value)))
                    }
                    _ => Err(self.error(ParseErrorKind::Expect("a number after unary '-'".into()))),
                }
            }
            TokenKind::Integer(value) => {
                let value = Self::integer_value(&value)
                    .ok_or_else(|| self.error(ParseErrorKind::IntegerOutOfRange))?;
                Ok(Span::new(token.start, token.end, ExprKind::Integer(value)))
            }
            TokenKind::Real(value) => {
                let value = Self::real_value(&value).map_err(|kind| self.error(kind))?;
                Ok(Span::new(token.start, token.end, ExprKind::Real(value)))
            }
            TokenKind::True => Ok(Span::new(token.start, token.end, ExprKind::Boolean(true))),
            TokenKind::False => Ok(Span::new(token.start, token.end, ExprKind::Boolean(false))),
            TokenKind::LeftBracket => {
                let mut elements = Vec::new();
                if self
                    .tokens
                    .get(self.index)
                    .is_some_and(|t| t.value == TokenKind::RightBracket)
                {
                    let end = self.tokens[self.index].end.clone();
                    self.index += 1;
                    return Ok(Span::new(token.start, end, ExprKind::List(elements)));
                }
                loop {
                    elements.push(self.parse_integer_expr()?);
                    if self
                        .tokens
                        .get(self.index)
                        .is_some_and(|t| t.value == TokenKind::Comma)
                    {
                        self.index += 1;
                    } else {
                        break;
                    }
                }
                let end = self
                    .expect(
                        TokenKind::RightBracket,
                        ParseErrorKind::Expect("] after list elements".into()),
                    )?
                    .end;
                Ok(Span::new(token.start, end, ExprKind::List(elements)))
            }
            TokenKind::If => {
                let condition = self.parse_integer_expr()?;
                self.expect(TokenKind::Then, ParseErrorKind::Expect("then".into()))?;
                let consequent = self.parse_integer_expr()?;
                self.expect(TokenKind::Else, ParseErrorKind::Expect("else".into()))?;
                let alternative = self.parse_integer_expr()?;
                let end = alternative.end.clone();
                Ok(Span::new(
                    token.start,
                    end,
                    ExprKind::If(
                        Box::new(condition),
                        Box::new(consequent),
                        Box::new(alternative),
                    ),
                ))
            }
            TokenKind::Identifier(name) => {
                Ok(Span::new(token.start, token.end, ExprKind::Variable(name)))
            }
            _ => Err(self.error(ParseErrorKind::Expect("an integer expression".into()))),
        }
    }

    fn expect_string(&mut self, kind: ParseErrorKind) -> Result<Token, ParseError> {
        let Some(token) = self.tokens.get(self.index) else {
            return Err(self.error(kind));
        };
        if !matches!(token.value, TokenKind::String(_)) {
            return Err(self.error(kind));
        }
        self.index += 1;
        Ok(token.clone())
    }

    fn parse_posix_exit(&mut self) -> Result<Expr, ParseError> {
        let start = self
            .expect(
                TokenKind::Identifier("Posix".into()),
                ParseErrorKind::Expect("Posix.Process.exit".into()),
            )?
            .start;
        self.expect(
            TokenKind::Dot,
            ParseErrorKind::Expect(". after Posix".into()),
        )?;
        self.expect(
            TokenKind::Identifier("Process".into()),
            ParseErrorKind::Expect("Process after Posix.".into()),
        )?;
        self.expect(
            TokenKind::Dot,
            ParseErrorKind::Expect(". after Posix.Process".into()),
        )?;
        self.expect(
            TokenKind::Identifier("exit".into()),
            ParseErrorKind::Expect("exit after Posix.Process.".into()),
        )?;
        self.expect(
            TokenKind::LeftParen,
            ParseErrorKind::Expect("( before exit status".into()),
        )?;
        let word8_start = self
            .expect(
                TokenKind::Identifier("Word8".into()),
                ParseErrorKind::Expect("Word8.fromInt".into()),
            )?
            .start;
        self.expect(
            TokenKind::Dot,
            ParseErrorKind::Expect(". after Word8".into()),
        )?;
        self.expect(
            TokenKind::Identifier("fromInt".into()),
            ParseErrorKind::Expect("fromInt after Word8.".into()),
        )?;
        let integer = self.parse_integer_expr()?;
        let word8_end = integer.end.clone();
        let word8 = Span::new(
            word8_start,
            word8_end,
            ExprKind::Word8FromInt(Box::new(integer)),
        );
        let end = self
            .expect(
                TokenKind::RightParen,
                ParseErrorKind::Expect(") after exit status".into()),
            )?
            .end;
        Ok(Span::new(start, end, ExprKind::PosixExit(Box::new(word8))))
    }
}
