use std::path::PathBuf;

use crate::error::{LexerError, ParseError, ParseErrorKind};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::span::Span;

pub type Expr = Span<ExprKind>;
pub type Stmt = Span<StmtKind>;
pub type Pat = Span<PatKind>;
pub type Ty = Span<TyKind>;
/// One `pattern => expression` arm of a `case`, `fn` or `handle`.
pub type Rule = (Pat, Expr);
/// A `val` binding inside `let`; `None` is the wildcard pattern.
pub type Binding = (Option<String>, Expr);

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
    Character(char),
    Word(String),
    Unit,
    Tuple(Vec<Expr>),
    Record(Vec<(String, Expr)>),
    Selector(String),
    /// Juxtaposition, left associative: `f x y` is `Apply(Apply(f, x), y)`.
    Apply(Box<Expr>, Box<Expr>),
    /// An infix operator that has no dedicated variant, such as `::` or `mod`.
    Infix(String, Box<Expr>, Box<Expr>),
    AndAlso(Box<Expr>, Box<Expr>),
    OrElse(Box<Expr>, Box<Expr>),
    Sequence(Vec<Expr>),
    Let(Vec<Binding>, Box<Expr>),
    Case(Box<Expr>, Vec<Rule>),
    Fn(Vec<Rule>),
    While(Box<Expr>, Box<Expr>),
    Raise(Box<Expr>),
    Handle(Box<Expr>, Vec<Rule>),
    Typed(Box<Expr>, Ty),
}

#[derive(Debug)]
pub enum PatKind {
    Wildcard,
    Variable(String),
    Integer(i64),
    String(String),
    Character(char),
    Boolean(bool),
    Unit,
    Tuple(Vec<Pat>),
    List(Vec<Pat>),
    Cons(Box<Pat>, Box<Pat>),
    Typed(Box<Pat>, Ty),
}

#[derive(Debug)]
pub enum TyKind {
    Variable(String),
    Constructor(String, Vec<Ty>),
    Tuple(Vec<Ty>),
    Arrow(Box<Ty>, Box<Ty>),
    Record(Vec<(String, Ty)>),
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
            let expr = self.parse_expr()?;
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

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.index).map(|token| &token.value)
    }

    fn peek_at(&self, offset: usize) -> Option<&TokenKind> {
        self.tokens
            .get(self.index + offset)
            .map(|token| &token.value)
    }

    fn at(&self, kind: &TokenKind) -> bool {
        self.peek() == Some(kind)
    }

    fn at_reserved(&self, word: &str) -> bool {
        matches!(self.peek(), Some(TokenKind::Reserved(reserved)) if reserved == word)
    }

    fn eat(&mut self, kind: &TokenKind) -> bool {
        let found = self.at(kind);
        if found {
            self.index += 1;
        }
        found
    }

    fn expect_reserved(&mut self, word: &str) -> Result<Token, ParseError> {
        self.expect(
            TokenKind::Reserved(word.into()),
            ParseErrorKind::Expect(word.into()),
        )
    }

    fn previous_end(&self) -> crate::span::Loc {
        self.tokens[self.index - 1].end.clone()
    }

    fn label(&mut self) -> Result<String, ParseError> {
        let label = match self.peek() {
            Some(TokenKind::Identifier(name)) => name.clone(),
            Some(TokenKind::Integer(number)) => number.clone(),
            _ => return Err(self.error(ParseErrorKind::Expect("a record label".into()))),
        };
        self.index += 1;
        Ok(label)
    }

    /// The name, precedence and right-associativity of a default infix operator.
    fn infix_info(kind: &TokenKind) -> Option<(String, u8, bool)> {
        let (name, precedence, right) = match kind {
            TokenKind::Star => ("*", 7, false),
            TokenKind::Slash => ("/", 7, false),
            TokenKind::Div => ("div", 7, false),
            TokenKind::Plus => ("+", 6, false),
            TokenKind::Minus => ("-", 6, false),
            TokenKind::Equals => ("=", 4, false),
            TokenKind::NotEquals => ("<>", 4, false),
            TokenKind::Greater => (">", 4, false),
            TokenKind::GreaterEqual => (">=", 4, false),
            TokenKind::Less => ("<", 4, false),
            TokenKind::LessEqual => ("<=", 4, false),
            TokenKind::SymbolicIdentifier(name) => match name.as_str() {
                "::" => ("::", 5, true),
                "@" => ("@", 5, true),
                "^" => ("^", 6, false),
                ":=" => (":=", 3, false),
                _ => return None,
            },
            TokenKind::Identifier(name) => match name.as_str() {
                "mod" => ("mod", 7, false),
                "o" => ("o", 3, false),
                "before" => ("before", 0, false),
                _ => return None,
            },
            _ => return None,
        };
        Some((name.to_owned(), precedence, right))
    }

    fn build_infix(name: String, lhs: Expr, rhs: Expr) -> Expr {
        let (start, end) = (lhs.start.clone(), rhs.end.clone());
        let (lhs, rhs) = (Box::new(lhs), Box::new(rhs));
        let kind = match name.as_str() {
            "+" => ExprKind::Add(lhs, rhs),
            "-" => ExprKind::Subtract(lhs, rhs),
            "*" => ExprKind::Multiply(lhs, rhs),
            "/" => ExprKind::Divide(lhs, rhs),
            "div" => ExprKind::IntDivide(lhs, rhs),
            ">" => ExprKind::Greater(lhs, rhs),
            ">=" => ExprKind::GreaterEqual(lhs, rhs),
            "<" => ExprKind::Less(lhs, rhs),
            "<=" => ExprKind::LessEqual(lhs, rhs),
            "=" => ExprKind::Equal(lhs, rhs),
            "<>" => ExprKind::NotEqual(lhs, rhs),
            _ => ExprKind::Infix(name, lhs, rhs),
        };
        Span::new(start, end, kind)
    }

    fn binary(lhs: Expr, rhs: Expr, kind: fn(Box<Expr>, Box<Expr>) -> ExprKind) -> Expr {
        let (start, end) = (lhs.start.clone(), rhs.end.clone());
        Span::new(start, end, kind(Box::new(lhs), Box::new(rhs)))
    }

    /// `exp`: the loosest level, where `handle` applies to everything on its left.
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_orelse()?;
        while self.at_reserved("handle") {
            self.index += 1;
            let rules = self.parse_rules()?;
            let start = expr.start.clone();
            let end = rules.last().expect("at least one rule").1.end.clone();
            expr = Span::new(start, end, ExprKind::Handle(Box::new(expr), rules));
        }
        Ok(expr)
    }

    fn parse_orelse(&mut self) -> Result<Expr, ParseError> {
        let lhs = self.parse_andalso()?;
        if self.at_reserved("orelse") {
            self.index += 1;
            let rhs = self.parse_orelse()?;
            return Ok(Self::binary(lhs, rhs, ExprKind::OrElse));
        }
        Ok(lhs)
    }

    fn parse_andalso(&mut self) -> Result<Expr, ParseError> {
        let lhs = self.parse_typed()?;
        if self.at_reserved("andalso") {
            self.index += 1;
            let rhs = self.parse_andalso()?;
            return Ok(Self::binary(lhs, rhs, ExprKind::AndAlso));
        }
        Ok(lhs)
    }

    fn parse_typed(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_infix(0)?;
        while self.eat(&TokenKind::Colon) {
            let ty = self.parse_ty()?;
            let (start, end) = (expr.start.clone(), ty.end.clone());
            expr = Span::new(start, end, ExprKind::Typed(Box::new(expr), ty));
        }
        Ok(expr)
    }

    /// Precedence climbing over the default fixities of the Definition.
    fn parse_infix(&mut self, minimum: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_application()?;
        while let Some((name, precedence, right)) = self.peek().and_then(Self::infix_info) {
            if precedence < minimum {
                break;
            }
            self.index += 1;
            let rhs = self.parse_infix(if right { precedence } else { precedence + 1 })?;
            lhs = Self::build_infix(name, lhs, rhs);
        }
        Ok(lhs)
    }

    fn parse_application(&mut self) -> Result<Expr, ParseError> {
        if let Some(expr) = self.parse_extending()? {
            return Ok(expr);
        }
        let mut expr = self.parse_first_atom()?;
        while self.starts_atexp() {
            let argument = self.parse_atexp()?;
            let (start, end) = (expr.start.clone(), argument.end.clone());
            expr = Span::new(
                start,
                end,
                ExprKind::Apply(Box::new(expr), Box::new(argument)),
            );
        }
        Ok(expr)
    }

    /// `if`, `case`, `fn`, `while` and `raise` extend as far right as possible.
    fn parse_extending(&mut self) -> Result<Option<Expr>, ParseError> {
        let Some(token) = self.tokens.get(self.index).cloned() else {
            return Ok(None);
        };
        let kind = match &token.value {
            TokenKind::If => {
                self.index += 1;
                let condition = self.parse_expr()?;
                self.expect(TokenKind::Then, ParseErrorKind::Expect("then".into()))?;
                let consequent = self.parse_expr()?;
                self.expect(TokenKind::Else, ParseErrorKind::Expect("else".into()))?;
                let alternative = self.parse_expr()?;
                ExprKind::If(
                    Box::new(condition),
                    Box::new(consequent),
                    Box::new(alternative),
                )
            }
            TokenKind::Reserved(word) if word == "case" => {
                self.index += 1;
                let scrutinee = self.parse_expr()?;
                self.expect_reserved("of")?;
                ExprKind::Case(Box::new(scrutinee), self.parse_rules()?)
            }
            TokenKind::Reserved(word) if word == "fn" => {
                self.index += 1;
                ExprKind::Fn(self.parse_rules()?)
            }
            TokenKind::Reserved(word) if word == "while" => {
                self.index += 1;
                let condition = self.parse_expr()?;
                self.expect_reserved("do")?;
                ExprKind::While(Box::new(condition), Box::new(self.parse_expr()?))
            }
            TokenKind::Reserved(word) if word == "raise" => {
                self.index += 1;
                ExprKind::Raise(Box::new(self.parse_expr()?))
            }
            _ => return Ok(None),
        };
        Ok(Some(Span::new(token.start, self.previous_end(), kind)))
    }

    /// The head of an application. A `-` directly before a number is kept as a
    /// negative literal, as before general expressions existed.
    fn parse_first_atom(&mut self) -> Result<Expr, ParseError> {
        if !self.at(&TokenKind::Minus) {
            return self.parse_atexp();
        }
        let minus = self.tokens[self.index].clone();
        self.index += 1;
        let Some(number) = self.tokens.get(self.index).cloned() else {
            return Err(self.error(ParseErrorKind::Expect("a number after unary '-'".into())));
        };
        self.index += 1;
        match number.value {
            TokenKind::Integer(value) => Ok(Span::new(
                minus.start,
                number.end,
                ExprKind::Integer(
                    Self::integer_value(&value)
                        .and_then(i64::checked_neg)
                        .ok_or_else(|| self.error(ParseErrorKind::IntegerOutOfRange))?,
                ),
            )),
            TokenKind::Real(value) => {
                let value = Self::real_value(&value).map_err(|kind| self.error(kind))?;
                Ok(Span::new(minus.start, number.end, ExprKind::Real(-value)))
            }
            _ => Err(self.error(ParseErrorKind::Expect("a number after unary '-'".into()))),
        }
    }

    fn starts_atexp(&self) -> bool {
        match self.peek() {
            Some(
                TokenKind::Integer(_)
                | TokenKind::Word(_)
                | TokenKind::Real(_)
                | TokenKind::Character(_)
                | TokenKind::String(_)
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Tilde
                | TokenKind::LeftParen
                | TokenKind::LeftBracket
                | TokenKind::LeftBrace
                | TokenKind::Hash,
            ) => true,
            Some(TokenKind::Identifier(_)) => self.peek().and_then(Self::infix_info).is_none(),
            Some(TokenKind::Reserved(word)) => word == "op" || word == "let",
            _ => false,
        }
    }

    fn parse_atexp(&mut self) -> Result<Expr, ParseError> {
        let Some(token) = self.tokens.get(self.index).cloned() else {
            return Err(self.error(ParseErrorKind::Expect("an expression".into())));
        };
        self.index += 1;
        let (start, mut end) = (token.start.clone(), token.end.clone());
        let kind = match token.value {
            TokenKind::Integer(value) => ExprKind::Integer(
                Self::integer_value(&value)
                    .ok_or_else(|| self.error(ParseErrorKind::IntegerOutOfRange))?,
            ),
            TokenKind::Real(value) => {
                ExprKind::Real(Self::real_value(&value).map_err(|kind| self.error(kind))?)
            }
            TokenKind::Word(value) => ExprKind::Word(value),
            TokenKind::Character(value) => ExprKind::Character(value),
            TokenKind::String(value) => ExprKind::String(value),
            TokenKind::True => ExprKind::Boolean(true),
            TokenKind::False => ExprKind::Boolean(false),
            TokenKind::Tilde => ExprKind::Variable("~".into()),
            TokenKind::Identifier(mut name) => {
                while self.at(&TokenKind::Dot)
                    && let Some(TokenKind::Identifier(part)) = self.peek_at(1)
                {
                    name = format!("{name}.{part}");
                    self.index += 2;
                    end = self.previous_end();
                }
                ExprKind::Variable(name)
            }
            TokenKind::Reserved(word) if word == "op" => {
                let Some(operator) = self.tokens.get(self.index).cloned() else {
                    return Err(self.error(ParseErrorKind::Expect("an operator after op".into())));
                };
                let name = match &operator.value {
                    TokenKind::Tilde => "~".to_owned(),
                    TokenKind::Identifier(name) => name.clone(),
                    other => match Self::infix_info(other) {
                        Some((name, _, _)) => name,
                        None => {
                            return Err(
                                self.error(ParseErrorKind::Expect("an operator after op".into()))
                            );
                        }
                    },
                };
                self.index += 1;
                end = operator.end;
                ExprKind::Variable(name)
            }
            TokenKind::Reserved(word) if word == "let" => return self.parse_let(start),
            TokenKind::LeftParen => return self.parse_paren(start),
            TokenKind::LeftBracket => {
                let mut elements = Vec::new();
                if !self.eat(&TokenKind::RightBracket) {
                    loop {
                        elements.push(self.parse_expr()?);
                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(
                        TokenKind::RightBracket,
                        ParseErrorKind::Expect("] after list elements".into()),
                    )?;
                }
                end = self.previous_end();
                ExprKind::List(elements)
            }
            TokenKind::LeftBrace => {
                let mut fields = Vec::new();
                if !self.eat(&TokenKind::RightBrace) {
                    loop {
                        let label = self.label()?;
                        self.expect(
                            TokenKind::Equals,
                            ParseErrorKind::Expect("= after record label".into()),
                        )?;
                        fields.push((label, self.parse_expr()?));
                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(
                        TokenKind::RightBrace,
                        ParseErrorKind::Expect("} after record fields".into()),
                    )?;
                }
                end = self.previous_end();
                if fields.is_empty() {
                    ExprKind::Unit
                } else {
                    ExprKind::Record(fields)
                }
            }
            TokenKind::Hash => {
                let label = self.label()?;
                end = self.previous_end();
                ExprKind::Selector(label)
            }
            _ => {
                self.index -= 1;
                return Err(self.error(ParseErrorKind::Expect("an expression".into())));
            }
        };
        Ok(Span::new(start, end, kind))
    }

    /// After `(`: unit, a parenthesised expression, a tuple, or a sequence.
    fn parse_paren(&mut self, start: crate::span::Loc) -> Result<Expr, ParseError> {
        if self.eat(&TokenKind::RightParen) {
            return Ok(Span::new(start, self.previous_end(), ExprKind::Unit));
        }
        let first = self.parse_expr()?;
        let separator = if self.at(&TokenKind::Comma) {
            Some(TokenKind::Comma)
        } else if self.at(&TokenKind::Semicolon) {
            Some(TokenKind::Semicolon)
        } else {
            None
        };
        let mut items = vec![first];
        if let Some(separator) = &separator {
            while self.eat(separator) {
                items.push(self.parse_expr()?);
            }
        }
        self.expect(
            TokenKind::RightParen,
            ParseErrorKind::Expect(") to close the parenthesis".into()),
        )?;
        let end = self.previous_end();
        Ok(match separator {
            None => items.pop().expect("one expression"),
            Some(TokenKind::Comma) => Span::new(start, end, ExprKind::Tuple(items)),
            Some(_) => Span::new(start, end, ExprKind::Sequence(items)),
        })
    }

    fn parse_let(&mut self, start: crate::span::Loc) -> Result<Expr, ParseError> {
        let mut bindings = Vec::new();
        while self.eat(&TokenKind::Val) {
            let name = match self.peek() {
                Some(TokenKind::Underscore) => None,
                Some(TokenKind::Identifier(name)) => Some(name.clone()),
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
            bindings.push((name, self.parse_expr()?));
            self.eat(&TokenKind::Semicolon);
        }
        self.expect_reserved("in")?;
        let mut body = vec![self.parse_expr()?];
        while self.eat(&TokenKind::Semicolon) {
            body.push(self.parse_expr()?);
        }
        self.expect_reserved("end")?;
        let end = self.previous_end();
        let body = if body.len() == 1 {
            body.pop().expect("one expression")
        } else {
            let (first, last) = (body[0].start.clone(), body[body.len() - 1].end.clone());
            Span::new(first, last, ExprKind::Sequence(body))
        };
        Ok(Span::new(
            start,
            end,
            ExprKind::Let(bindings, Box::new(body)),
        ))
    }

    fn parse_rules(&mut self) -> Result<Vec<Rule>, ParseError> {
        let mut rules = Vec::new();
        loop {
            let pattern = self.parse_pat()?;
            self.expect(
                TokenKind::FatArrow,
                ParseErrorKind::Expect("=> after pattern".into()),
            )?;
            rules.push((pattern, self.parse_expr()?));
            if !self.eat(&TokenKind::Bar) {
                return Ok(rules);
            }
        }
    }

    fn parse_pat(&mut self) -> Result<Pat, ParseError> {
        let mut pattern = self.parse_cons_pat()?;
        while self.eat(&TokenKind::Colon) {
            let ty = self.parse_ty()?;
            let (start, end) = (pattern.start.clone(), ty.end.clone());
            pattern = Span::new(start, end, PatKind::Typed(Box::new(pattern), ty));
        }
        Ok(pattern)
    }

    fn parse_cons_pat(&mut self) -> Result<Pat, ParseError> {
        let head = self.parse_atpat()?;
        if matches!(self.peek(), Some(TokenKind::SymbolicIdentifier(name)) if name == "::") {
            self.index += 1;
            let tail = self.parse_cons_pat()?;
            let (start, end) = (head.start.clone(), tail.end.clone());
            return Ok(Span::new(
                start,
                end,
                PatKind::Cons(Box::new(head), Box::new(tail)),
            ));
        }
        Ok(head)
    }

    fn parse_atpat(&mut self) -> Result<Pat, ParseError> {
        let Some(token) = self.tokens.get(self.index).cloned() else {
            return Err(self.error(ParseErrorKind::Expect("a pattern".into())));
        };
        self.index += 1;
        let start = token.start.clone();
        let kind = match token.value {
            TokenKind::Underscore => PatKind::Wildcard,
            TokenKind::Identifier(name) => PatKind::Variable(name),
            TokenKind::Integer(value) => PatKind::Integer(
                Self::integer_value(&value)
                    .ok_or_else(|| self.error(ParseErrorKind::IntegerOutOfRange))?,
            ),
            TokenKind::String(value) => PatKind::String(value),
            TokenKind::Character(value) => PatKind::Character(value),
            TokenKind::True => PatKind::Boolean(true),
            TokenKind::False => PatKind::Boolean(false),
            TokenKind::LeftParen => {
                if self.eat(&TokenKind::RightParen) {
                    PatKind::Unit
                } else {
                    let mut items = vec![self.parse_pat()?];
                    let tuple = self.at(&TokenKind::Comma);
                    while self.eat(&TokenKind::Comma) {
                        items.push(self.parse_pat()?);
                    }
                    self.expect(
                        TokenKind::RightParen,
                        ParseErrorKind::Expect(") to close the pattern".into()),
                    )?;
                    if !tuple {
                        return Ok(items.pop().expect("one pattern"));
                    }
                    PatKind::Tuple(items)
                }
            }
            TokenKind::LeftBracket => {
                let mut items = Vec::new();
                if !self.eat(&TokenKind::RightBracket) {
                    loop {
                        items.push(self.parse_pat()?);
                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(
                        TokenKind::RightBracket,
                        ParseErrorKind::Expect("] after list pattern".into()),
                    )?;
                }
                PatKind::List(items)
            }
            _ => {
                self.index -= 1;
                return Err(self.error(ParseErrorKind::Expect("a pattern".into())));
            }
        };
        Ok(Span::new(start, self.previous_end(), kind))
    }

    fn parse_ty(&mut self) -> Result<Ty, ParseError> {
        let lhs = self.parse_tuple_ty()?;
        if self.eat(&TokenKind::Arrow) {
            let rhs = self.parse_ty()?;
            let (start, end) = (lhs.start.clone(), rhs.end.clone());
            return Ok(Span::new(
                start,
                end,
                TyKind::Arrow(Box::new(lhs), Box::new(rhs)),
            ));
        }
        Ok(lhs)
    }

    fn parse_tuple_ty(&mut self) -> Result<Ty, ParseError> {
        let first = self.parse_app_ty()?;
        if !self.at(&TokenKind::Star) {
            return Ok(first);
        }
        let start = first.start.clone();
        let mut items = vec![first];
        while self.eat(&TokenKind::Star) {
            items.push(self.parse_app_ty()?);
        }
        Ok(Span::new(start, self.previous_end(), TyKind::Tuple(items)))
    }

    /// `atty` followed by any number of postfix type constructors: `int list list`.
    fn parse_app_ty(&mut self) -> Result<Ty, ParseError> {
        let Some(token) = self.tokens.get(self.index).cloned() else {
            return Err(self.error(ParseErrorKind::Expect("a type".into())));
        };
        self.index += 1;
        let start = token.start.clone();
        let mut ty = match token.value {
            TokenKind::TypeVariable(name) => {
                Span::new(start.clone(), token.end, TyKind::Variable(name))
            }
            TokenKind::Identifier(name) => Span::new(
                start.clone(),
                self.previous_end(),
                TyKind::Constructor(name, Vec::new()),
            ),
            TokenKind::LeftParen => {
                let mut items = vec![self.parse_ty()?];
                while self.eat(&TokenKind::Comma) {
                    items.push(self.parse_ty()?);
                }
                self.expect(
                    TokenKind::RightParen,
                    ParseErrorKind::Expect(") to close the type".into()),
                )?;
                if items.len() == 1 {
                    items.pop().expect("one type")
                } else {
                    let Some(TokenKind::Identifier(name)) = self.peek().cloned() else {
                        return Err(self.error(ParseErrorKind::Expect(
                            "a type constructor after the type arguments".into(),
                        )));
                    };
                    self.index += 1;
                    Span::new(
                        start.clone(),
                        self.previous_end(),
                        TyKind::Constructor(name, items),
                    )
                }
            }
            TokenKind::LeftBrace => {
                let mut fields = Vec::new();
                if !self.eat(&TokenKind::RightBrace) {
                    loop {
                        let label = self.label()?;
                        self.expect(
                            TokenKind::Colon,
                            ParseErrorKind::Expect(": after record label".into()),
                        )?;
                        fields.push((label, self.parse_ty()?));
                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(
                        TokenKind::RightBrace,
                        ParseErrorKind::Expect("} after record type".into()),
                    )?;
                }
                Span::new(start.clone(), self.previous_end(), TyKind::Record(fields))
            }
            _ => {
                self.index -= 1;
                return Err(self.error(ParseErrorKind::Expect("a type".into())));
            }
        };
        while let Some(TokenKind::Identifier(name)) = self.peek().cloned() {
            self.index += 1;
            ty = Span::new(
                start.clone(),
                self.previous_end(),
                TyKind::Constructor(name, vec![ty]),
            );
        }
        Ok(ty)
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
        let integer = self.parse_expr()?;
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
