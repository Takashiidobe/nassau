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
pub type Decl = Span<DeclKind>;

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
    Let(Vec<Decl>, Box<Expr>),
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
    /// A variable, or a nullary constructor such as `nil`; the two are only
    /// told apart once constructors are known.
    Variable(String),
    Integer(i64),
    Word(String),
    String(String),
    Character(char),
    Boolean(bool),
    Unit,
    Tuple(Vec<Pat>),
    List(Vec<Pat>),
    /// `{a = p, b, ...}`; the flag is true when the row list ends in `...`.
    Record(Vec<(String, Pat)>, bool),
    /// A constructor applied to an argument pattern.
    Constructor(String, Box<Pat>),
    Cons(Box<Pat>, Box<Pat>),
    /// `x as p` or `x : ty as p`.
    Layered(String, Option<Ty>, Box<Pat>),
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
    /// Any other declaration; the backend does not handle these yet.
    Declaration(Decl),
}

#[derive(Debug)]
pub enum DeclKind {
    /// `val pat = exp and ...`. Without `rec`, the right-hand sides cannot see
    /// the names the patterns bind; with `rec` (whose right-hand sides must be
    /// `fn`), they see their own.
    Val {
        recursive: bool,
        bindings: Vec<(Pat, Expr)>,
    },
    /// `fun f p = e | f q = e' and g ...`: an `and` group of mutually recursive
    /// functions, each with one or more clauses.
    Fun(Vec<FunBinding>),
    Type(Vec<TypeBinding>),
    /// `local private in public end`: only `public` is visible afterwards.
    Local(Vec<Decl>, Vec<Decl>),
    Fixity {
        kind: FixityKind,
        precedence: u8,
        names: Vec<String>,
    },
}

#[derive(Debug)]
pub struct FunBinding {
    pub name: String,
    pub clauses: Vec<FunClause>,
}

/// One clause; `fun f x : t = e` keeps its result type as `e : t`.
#[derive(Debug)]
pub struct FunClause {
    pub parameters: Vec<Pat>,
    pub body: Expr,
}

#[derive(Debug)]
pub struct TypeBinding {
    pub parameters: Vec<String>,
    pub name: String,
    pub ty: Ty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixityKind {
    Infix,
    Infixr,
    Nonfix,
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
    /// Precedence and right-associativity of every infix name in scope.
    fixity: std::collections::HashMap<String, (u8, bool)>,
}

impl Parser {
    fn with_file(tokens: Vec<Token>, file: PathBuf) -> Self {
        Self {
            tokens,
            index: 0,
            allow_implicit_val: false,
            file,
            fixity: Self::default_fixity(),
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
            if self.eat(&TokenKind::Semicolon) {
                continue;
            }
            let declaration = if self.starts_decl() {
                self.parse_decl()?
            } else if self.allow_implicit_val {
                let start = self.tokens[self.index].start.clone();
                self.parse_val_decl(start)?
            } else {
                return Err(self.error(ParseErrorKind::Expect("a declaration".into())));
            };
            statements.push(Self::lower_statement(declaration));
        }
        if statements.is_empty() {
            return Err(self.error(ParseErrorKind::Expect("a program".into())));
        }
        Ok(Program {
            statements,
            result: 0,
        })
    }

    /// Recognises the declarations the backend already handles (`val x = e`,
    /// `val _ = print "..."` and `val _ = Posix.Process.exit (Word8.fromInt e)`)
    /// and keeps every other declaration as it was parsed.
    fn lower_statement(declaration: Decl) -> Stmt {
        let (start, end) = (declaration.start.clone(), declaration.end.clone());
        if let DeclKind::Val {
            recursive: false,
            bindings,
        } = &declaration.value
            && let [(pattern, expr)] = bindings.as_slice()
        {
            let simple = match &pattern.value {
                PatKind::Variable(name) => !name.contains('.'),
                PatKind::Wildcard => Self::is_effect(expr),
                _ => false,
            };
            if simple {
                let DeclKind::Val { mut bindings, .. } = declaration.value else {
                    unreachable!()
                };
                let (pattern, expr) = bindings.pop().expect("one binding");
                let kind = match pattern.value {
                    PatKind::Variable(name) => StmtKind::Val(name, expr),
                    _ => Self::lower_effect(expr),
                };
                return Span::new(start, end, kind);
            }
        }
        Span::new(start, end, StmtKind::Declaration(declaration))
    }

    fn is_effect(expr: &Expr) -> bool {
        let ExprKind::Apply(function, argument) = &expr.value else {
            return false;
        };
        match (&function.value, &argument.value) {
            (ExprKind::Variable(name), ExprKind::String(_)) => name == "print",
            (ExprKind::Variable(name), ExprKind::Apply(inner, _)) => {
                name == "Posix.Process.exit"
                    && matches!(&inner.value, ExprKind::Variable(n) if n == "Word8.fromInt")
            }
            _ => false,
        }
    }

    /// `print "text"` and `Posix.Process.exit (Word8.fromInt e)` as the
    /// backend's effect statements; the caller has checked `is_effect`.
    fn lower_effect(expr: Expr) -> StmtKind {
        let ExprKind::Apply(function, argument) = expr.value else {
            unreachable!()
        };
        if matches!(argument.value, ExprKind::String(_)) {
            return StmtKind::Print(*argument);
        }
        let Expr {
            start,
            end,
            value: ExprKind::Apply(_, integer),
        } = *argument
        else {
            unreachable!()
        };
        let word8 = Span::new(start, end.clone(), ExprKind::Word8FromInt(integer));
        StmtKind::Exit(Span::new(
            function.start,
            end,
            ExprKind::PosixExit(Box::new(word8)),
        ))
    }

    fn starts_decl(&self) -> bool {
        match self.peek() {
            Some(TokenKind::Val) => true,
            Some(TokenKind::Reserved(word)) => matches!(
                word.as_str(),
                "fun"
                    | "type"
                    | "local"
                    | "infix"
                    | "infixr"
                    | "nonfix"
                    | "datatype"
                    | "exception"
                    | "structure"
                    | "signature"
                    | "functor"
                    | "open"
                    | "abstype"
                    | "eqtype"
            ),
            _ => false,
        }
    }

    /// One declaration, with fixity changes made by `infix`/`nonfix` recorded.
    fn parse_decl(&mut self) -> Result<Decl, ParseError> {
        let Some(token) = self.tokens.get(self.index).cloned() else {
            return Err(self.error(ParseErrorKind::Expect("a declaration".into())));
        };
        self.index += 1;
        let start = token.start.clone();
        let kind = match token.value {
            TokenKind::Val => return self.parse_val_decl(start),
            TokenKind::Reserved(word) => match word.as_str() {
                "fun" => self.parse_fun_decl()?,
                "type" => self.parse_type_decl()?,
                "local" => {
                    let saved = self.fixity.clone();
                    let private = self.parse_decls_until("in")?;
                    self.expect_reserved("in")?;
                    let public = self.parse_decls_until("end")?;
                    self.expect_reserved("end")?;
                    // Fixity set in the private part scopes over the public part only.
                    self.fixity = saved;
                    DeclKind::Local(private, public)
                }
                "infix" | "infixr" | "nonfix" => self.parse_fixity_decl(&word)?,
                "datatype" | "abstype" => {
                    self.index -= 1;
                    return Err(self.error(ParseErrorKind::Unsupported("datatype declarations")));
                }
                "exception" => {
                    self.index -= 1;
                    return Err(self.error(ParseErrorKind::Unsupported("exception declarations")));
                }
                _ => {
                    self.index -= 1;
                    return Err(self.error(ParseErrorKind::Unsupported("module declarations")));
                }
            },
            _ => {
                self.index -= 1;
                return Err(self.error(ParseErrorKind::Expect("a declaration".into())));
            }
        };
        Ok(Span::new(start, self.previous_end(), kind))
    }

    /// Declarations up to (not including) the reserved word `terminator`.
    fn parse_decls_until(&mut self, terminator: &str) -> Result<Vec<Decl>, ParseError> {
        let mut declarations = Vec::new();
        loop {
            if self.eat(&TokenKind::Semicolon) {
                continue;
            }
            if self.at_reserved(terminator) {
                return Ok(declarations);
            }
            if !self.starts_decl() {
                return Err(self.error(ParseErrorKind::Expect(format!(
                    "a declaration or {terminator}"
                ))));
            }
            declarations.push(self.parse_decl()?);
        }
    }

    /// After `val` (or at the start of an implicit REPL `val`).
    fn parse_val_decl(&mut self, start: crate::span::Loc) -> Result<Decl, ParseError> {
        let recursive = self.at_reserved("rec");
        if recursive {
            self.index += 1;
        }
        let mut bindings = Vec::new();
        loop {
            let pattern = self.parse_pat()?;
            self.expect(
                TokenKind::Equals,
                ParseErrorKind::Expect("= after val pattern".into()),
            )?;
            let expr = self.parse_expr()?;
            if recursive && !matches!(expr.value, ExprKind::Fn(_)) {
                self.index -= 1;
                return Err(self.error(ParseErrorKind::Expect(
                    "a fn expression after val rec".into(),
                )));
            }
            bindings.push((pattern, expr));
            if !self.at_reserved("and") {
                break;
            }
            self.index += 1;
        }
        Ok(Span::new(
            start,
            self.previous_end(),
            DeclKind::Val {
                recursive,
                bindings,
            },
        ))
    }

    /// After `fun`: `f p1 p2 = e | f q1 q2 = e' and g ...`.
    fn parse_fun_decl(&mut self) -> Result<DeclKind, ParseError> {
        let mut bindings = Vec::new();
        loop {
            let mut name: Option<String> = None;
            let mut clauses: Vec<FunClause> = Vec::new();
            loop {
                let (clause_name, parameters) = self.parse_fun_head()?;
                let mut body_ty = None;
                if self.eat(&TokenKind::Colon) {
                    body_ty = Some(self.parse_ty()?);
                }
                self.expect(
                    TokenKind::Equals,
                    ParseErrorKind::Expect("= after function parameters".into()),
                )?;
                let mut body = self.parse_expr()?;
                if let Some(ty) = body_ty {
                    let (start, end) = (body.start.clone(), ty.end.clone());
                    body = Span::new(start, end, ExprKind::Typed(Box::new(body), ty));
                }
                match &name {
                    Some(previous) if *previous != clause_name => {
                        return Err(self.error(ParseErrorKind::Expect(
                            "every clause to define the same function".into(),
                        )));
                    }
                    _ => name = Some(clause_name),
                }
                if let Some(first) = clauses.first()
                    && first.parameters.len() != parameters.len()
                {
                    return Err(self.error(ParseErrorKind::Expect(
                        "every clause to have the same number of parameters".into(),
                    )));
                }
                clauses.push(FunClause { parameters, body });
                if !self.eat(&TokenKind::Bar) {
                    break;
                }
            }
            bindings.push(FunBinding {
                name: name.expect("at least one clause"),
                clauses,
            });
            if !self.at_reserved("and") {
                return Ok(DeclKind::Fun(bindings));
            }
            self.index += 1;
        }
    }

    /// The left of a `fun` clause: `f p1 .. pn`, `p1 ++ p2`, or `(p1 ++ p2) p3 ..`.
    fn parse_fun_head(&mut self) -> Result<(String, Vec<Pat>), ParseError> {
        if self.at(&TokenKind::LeftParen) {
            let saved = self.index;
            match self.parse_parenthesised_infix_head() {
                Ok(head) => return Ok(head),
                Err(_) => self.index = saved,
            }
        }
        if self.eat_reserved("op") {
            let name = self.function_name()?;
            return Ok((name, self.parse_parameters()?));
        }
        let first = self.parse_atpat()?;
        if let Some(name) = self.peek().and_then(|kind| self.infix_name(kind)) {
            self.index += 1;
            let second = self.parse_atpat()?;
            return Ok((name, vec![Self::tuple_pat(first, second)]));
        }
        let PatKind::Variable(name) = first.value else {
            return Err(self.error(ParseErrorKind::Expect("a function name".into())));
        };
        Ok((name, self.parse_parameters()?))
    }

    fn parse_parenthesised_infix_head(&mut self) -> Result<(String, Vec<Pat>), ParseError> {
        self.expect(TokenKind::LeftParen, ParseErrorKind::Expect("(".into()))?;
        let first = self.parse_atpat()?;
        let Some(name) = self.peek().and_then(|kind| self.infix_name(kind)) else {
            return Err(self.error(ParseErrorKind::Expect("an infix function name".into())));
        };
        self.index += 1;
        let second = self.parse_atpat()?;
        self.expect(
            TokenKind::RightParen,
            ParseErrorKind::Expect(") after infix function head".into()),
        )?;
        let mut parameters = vec![Self::tuple_pat(first, second)];
        while self.starts_atpat() {
            parameters.push(self.parse_atpat()?);
        }
        Ok((name, parameters))
    }

    fn tuple_pat(first: Pat, second: Pat) -> Pat {
        let (start, end) = (first.start.clone(), second.end.clone());
        Span::new(start, end, PatKind::Tuple(vec![first, second]))
    }

    fn function_name(&mut self) -> Result<String, ParseError> {
        let name = match self.peek() {
            Some(TokenKind::Identifier(name) | TokenKind::SymbolicIdentifier(name)) => name.clone(),
            Some(kind) => match Self::token_name(kind) {
                Some(name) => name,
                None => return Err(self.error(ParseErrorKind::Expect("a function name".into()))),
            },
            None => return Err(self.error(ParseErrorKind::Expect("a function name".into()))),
        };
        self.index += 1;
        Ok(name)
    }

    fn parse_parameters(&mut self) -> Result<Vec<Pat>, ParseError> {
        let mut parameters = Vec::new();
        while self.starts_atpat() {
            parameters.push(self.parse_atpat()?);
        }
        if parameters.is_empty() {
            return Err(self.error(ParseErrorKind::Expect(
                "at least one function parameter".into(),
            )));
        }
        Ok(parameters)
    }

    /// After `type`: `type ('a, 'b) pair = 'a * 'b and ...`.
    fn parse_type_decl(&mut self) -> Result<DeclKind, ParseError> {
        let mut bindings = Vec::new();
        loop {
            let mut parameters = Vec::new();
            match self.peek().cloned() {
                Some(TokenKind::TypeVariable(name)) => {
                    self.index += 1;
                    parameters.push(name);
                }
                Some(TokenKind::LeftParen) => {
                    self.index += 1;
                    loop {
                        let Some(TokenKind::TypeVariable(name)) = self.peek().cloned() else {
                            return Err(
                                self.error(ParseErrorKind::Expect("a type variable".into()))
                            );
                        };
                        self.index += 1;
                        parameters.push(name);
                        if !self.eat(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(
                        TokenKind::RightParen,
                        ParseErrorKind::Expect(") after type parameters".into()),
                    )?;
                }
                _ => {}
            }
            let Some(TokenKind::Identifier(name)) = self.peek().cloned() else {
                return Err(self.error(ParseErrorKind::Expect("a type name".into())));
            };
            self.index += 1;
            self.expect(
                TokenKind::Equals,
                ParseErrorKind::Expect("= after type name".into()),
            )?;
            let ty = self.parse_ty()?;
            bindings.push(TypeBinding {
                parameters,
                name,
                ty,
            });
            if !self.at_reserved("and") {
                return Ok(DeclKind::Type(bindings));
            }
            self.index += 1;
        }
    }

    /// After `infix`, `infixr` or `nonfix`: `infix 6 ++ --`.
    fn parse_fixity_decl(&mut self, word: &str) -> Result<DeclKind, ParseError> {
        let kind = match word {
            "infix" => FixityKind::Infix,
            "infixr" => FixityKind::Infixr,
            _ => FixityKind::Nonfix,
        };
        let mut precedence = 0;
        if kind != FixityKind::Nonfix
            && let Some(TokenKind::Integer(digits)) = self.peek().cloned()
        {
            precedence = digits
                .parse::<u8>()
                .ok()
                .filter(|precedence| *precedence <= 9)
                .ok_or_else(|| {
                    self.error(ParseErrorKind::Expect("a precedence from 0 to 9".into()))
                })?;
            self.index += 1;
        }
        let mut names = Vec::new();
        while let Some(name) = self.peek().and_then(Self::token_name) {
            self.index += 1;
            names.push(name);
        }
        if names.is_empty() {
            return Err(self.error(ParseErrorKind::Expect("an operator name".into())));
        }
        for name in &names {
            match kind {
                FixityKind::Nonfix => {
                    self.fixity.remove(name);
                }
                _ => {
                    self.fixity
                        .insert(name.clone(), (precedence, kind == FixityKind::Infixr));
                }
            }
        }
        Ok(DeclKind::Fixity {
            kind,
            precedence,
            names,
        })
    }

    fn eat_reserved(&mut self, word: &str) -> bool {
        let found = self.at_reserved(word);
        if found {
            self.index += 1;
        }
        found
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

    /// The fixities the Definition's initial basis declares.
    fn default_fixity() -> std::collections::HashMap<String, (u8, bool)> {
        [
            ("*", 7, false),
            ("/", 7, false),
            ("div", 7, false),
            ("mod", 7, false),
            ("+", 6, false),
            ("-", 6, false),
            ("^", 6, false),
            ("::", 5, true),
            ("@", 5, true),
            ("=", 4, false),
            ("<>", 4, false),
            (">", 4, false),
            (">=", 4, false),
            ("<", 4, false),
            ("<=", 4, false),
            (":=", 3, false),
            ("o", 3, false),
            ("before", 0, false),
        ]
        .into_iter()
        .map(|(name, precedence, right)| (name.to_owned(), (precedence, right)))
        .collect()
    }

    /// The name a token spells when used as an operator or identifier.
    fn token_name(kind: &TokenKind) -> Option<String> {
        Some(
            match kind {
                TokenKind::Star => "*",
                TokenKind::Slash => "/",
                TokenKind::Div => "div",
                TokenKind::Plus => "+",
                TokenKind::Minus => "-",
                TokenKind::Equals => "=",
                TokenKind::NotEquals => "<>",
                TokenKind::Greater => ">",
                TokenKind::GreaterEqual => ">=",
                TokenKind::Less => "<",
                TokenKind::LessEqual => "<=",
                TokenKind::Tilde => "~",
                TokenKind::SymbolicIdentifier(name) | TokenKind::Identifier(name) => name,
                _ => return None,
            }
            .to_owned(),
        )
    }

    /// The name, precedence and right-associativity of `kind` if it is infix here.
    fn infix_info(&self, kind: &TokenKind) -> Option<(String, u8, bool)> {
        let name = Self::token_name(kind)?;
        let (precedence, right) = *self.fixity.get(&name)?;
        Some((name, precedence, right))
    }

    fn infix_name(&self, kind: &TokenKind) -> Option<String> {
        self.infix_info(kind).map(|(name, _, _)| name)
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
        while let Some((name, precedence, right)) =
            self.peek().and_then(|kind| self.infix_info(kind))
        {
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
            Some(TokenKind::Identifier(_) | TokenKind::SymbolicIdentifier(_)) => {
                self.peek().and_then(|kind| self.infix_info(kind)).is_none()
            }
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
            TokenKind::SymbolicIdentifier(name) | TokenKind::Identifier(name)
                if self.fixity.contains_key(&name) =>
            {
                self.index -= 1;
                return Err(self.error(ParseErrorKind::Expect(
                    "an expression; an infix identifier needs op".into(),
                )));
            }
            TokenKind::SymbolicIdentifier(name) => ExprKind::Variable(name),
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
                let Some(name) = Self::token_name(&operator.value) else {
                    return Err(self.error(ParseErrorKind::Expect("an operator after op".into())));
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
        // Fixity declared inside `let` ends with it.
        let saved = self.fixity.clone();
        let declarations = self.parse_decls_until("in")?;
        self.expect_reserved("in")?;
        let mut body = vec![self.parse_expr()?];
        while self.eat(&TokenKind::Semicolon) {
            body.push(self.parse_expr()?);
        }
        self.expect_reserved("end")?;
        self.fixity = saved;
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
            ExprKind::Let(declarations, Box::new(body)),
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

    /// `pat`: the loosest pattern level, where `: ty` and layering apply.
    fn parse_pat(&mut self) -> Result<Pat, ParseError> {
        let mut pattern = self.parse_layered_pat()?;
        while self.eat(&TokenKind::Colon) {
            let ty = self.parse_ty()?;
            let start = pattern.start.clone();
            if self.at_reserved("as")
                && let PatKind::Variable(name) = &pattern.value
            {
                let name = name.clone();
                self.index += 1;
                let inner = self.parse_pat()?;
                let end = inner.end.clone();
                return Ok(Span::new(
                    start,
                    end,
                    PatKind::Layered(name, Some(ty), Box::new(inner)),
                ));
            }
            let end = ty.end.clone();
            pattern = Span::new(start, end, PatKind::Typed(Box::new(pattern), ty));
        }
        Ok(pattern)
    }

    /// `vid as pat`, which extends as far right as possible.
    fn parse_layered_pat(&mut self) -> Result<Pat, ParseError> {
        if let Some(TokenKind::Identifier(name)) = self.peek().cloned()
            && matches!(self.peek_at(1), Some(TokenKind::Reserved(word)) if word == "as")
        {
            let start = self.tokens[self.index].start.clone();
            self.index += 2;
            let inner = self.parse_pat()?;
            let end = inner.end.clone();
            return Ok(Span::new(
                start,
                end,
                PatKind::Layered(name, None, Box::new(inner)),
            ));
        }
        self.parse_cons_pat()
    }

    /// `::` is the only infix constructor until fixity declarations exist.
    fn parse_cons_pat(&mut self) -> Result<Pat, ParseError> {
        let head = self.parse_app_pat()?;
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

    /// A constructor applied to one atomic pattern: `SOME x`, `Fail _`.
    fn parse_app_pat(&mut self) -> Result<Pat, ParseError> {
        let head = self.parse_atpat()?;
        if let PatKind::Variable(name) = &head.value
            && self.starts_atpat()
        {
            let name = name.clone();
            let argument = self.parse_atpat()?;
            let (start, end) = (head.start.clone(), argument.end.clone());
            return Ok(Span::new(
                start,
                end,
                PatKind::Constructor(name, Box::new(argument)),
            ));
        }
        Ok(head)
    }

    fn starts_atpat(&self) -> bool {
        match self.peek() {
            Some(
                TokenKind::Underscore
                | TokenKind::Identifier(_)
                | TokenKind::Integer(_)
                | TokenKind::Word(_)
                | TokenKind::String(_)
                | TokenKind::Character(_)
                | TokenKind::True
                | TokenKind::False
                | TokenKind::LeftParen
                | TokenKind::LeftBracket
                | TokenKind::LeftBrace,
            ) => true,
            Some(TokenKind::Reserved(word)) => word == "op",
            _ => false,
        }
    }

    fn parse_atpat(&mut self) -> Result<Pat, ParseError> {
        let Some(token) = self.tokens.get(self.index).cloned() else {
            return Err(self.error(ParseErrorKind::Expect("a pattern".into())));
        };
        self.index += 1;
        let start = token.start.clone();
        let kind = match token.value {
            TokenKind::Underscore => PatKind::Wildcard,
            TokenKind::Identifier(mut name) => {
                if self.fixity.contains_key(&name) {
                    self.index -= 1;
                    return Err(self.error(ParseErrorKind::Expect(
                        "a pattern; an infix identifier needs op".into(),
                    )));
                }
                while self.at(&TokenKind::Dot)
                    && let Some(TokenKind::Identifier(part)) = self.peek_at(1)
                {
                    name = format!("{name}.{part}");
                    self.index += 2;
                }
                PatKind::Variable(name)
            }
            TokenKind::Reserved(word) if word == "op" => {
                let Some(name) = self.peek().and_then(Self::token_name) else {
                    return Err(self.error(ParseErrorKind::Expect("a name after op".into())));
                };
                self.index += 1;
                PatKind::Variable(name)
            }
            TokenKind::Integer(value) => PatKind::Integer(
                Self::integer_value(&value)
                    .ok_or_else(|| self.error(ParseErrorKind::IntegerOutOfRange))?,
            ),
            TokenKind::Word(value) => PatKind::Word(value),
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
            TokenKind::LeftBrace => self.parse_record_pat()?,
            TokenKind::Real(_) => {
                self.index -= 1;
                return Err(self.error(ParseErrorKind::Expect(
                    "a pattern; real constants cannot be patterns".into(),
                )));
            }
            _ => {
                self.index -= 1;
                return Err(self.error(ParseErrorKind::Expect("a pattern".into())));
            }
        };
        Ok(Span::new(start, self.previous_end(), kind))
    }

    /// After `{`: rows of `label = pat` or punned `label [: ty] [as pat]`, then optional `...`.
    fn parse_record_pat(&mut self) -> Result<PatKind, ParseError> {
        let mut fields: Vec<(String, Pat)> = Vec::new();
        let mut flexible = false;
        if !self.eat(&TokenKind::RightBrace) {
            loop {
                if self.eat(&TokenKind::Ellipsis) {
                    flexible = true;
                    break;
                }
                let start = self.tokens.get(self.index).map(|token| token.start.clone());
                let punned = matches!(self.peek(), Some(TokenKind::Identifier(_)))
                    && !matches!(self.peek_at(1), Some(TokenKind::Equals));
                let label = self.label()?;
                let pattern = if punned {
                    let start = start.expect("label token exists");
                    let mut pattern = Span::new(
                        start.clone(),
                        self.previous_end(),
                        PatKind::Variable(label.clone()),
                    );
                    let mut annotation = None;
                    if self.eat(&TokenKind::Colon) {
                        annotation = Some(self.parse_ty()?);
                    }
                    if self.at_reserved("as") {
                        self.index += 1;
                        let inner = self.parse_pat()?;
                        let end = inner.end.clone();
                        pattern = Span::new(
                            start,
                            end,
                            PatKind::Layered(label.clone(), annotation, Box::new(inner)),
                        );
                    } else if let Some(ty) = annotation {
                        let end = ty.end.clone();
                        pattern = Span::new(start, end, PatKind::Typed(Box::new(pattern), ty));
                    }
                    pattern
                } else {
                    self.expect(
                        TokenKind::Equals,
                        ParseErrorKind::Expect("= after record label".into()),
                    )?;
                    self.parse_pat()?
                };
                fields.push((label, pattern));
                if !self.eat(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(
                TokenKind::RightBrace,
                ParseErrorKind::Expect("} after record pattern".into()),
            )?;
        }
        Ok(if fields.is_empty() && !flexible {
            PatKind::Unit
        } else {
            PatKind::Record(fields, flexible)
        })
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
}
