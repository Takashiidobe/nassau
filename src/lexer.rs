use std::path::{Path, PathBuf};

use crate::error::{LexerError, LexerErrorKind};
use crate::span::{Loc, Span};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Integer(String),
    Word(String),
    Real(String),
    Character(char),
    String(String),
    Val,
    If,
    Then,
    Else,
    True,
    False,
    Identifier(String),
    TypeVariable(String),
    Reserved(String),
    SymbolicIdentifier(String),
    Underscore,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Div,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    NotEquals,
    Tilde,
    Colon,
    Bar,
    Hash,
    Arrow,
    FatArrow,
    OpaqueAscription,
    Ellipsis,
    Semicolon,
    Dot,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
}

pub type Token = Span<TokenKind>;

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

    fn error(&self, start: Loc, kind: LexerErrorKind) -> LexerError {
        Span::new(start, self.loc(), kind)
    }

    fn consume(&mut self, text: &str) -> bool {
        if self.source[self.offset..].starts_with(text) {
            for _ in text.chars() {
                self.bump();
            }
            true
        } else {
            false
        }
    }

    fn digits(&mut self, radix: u32) -> String {
        let mut digits = String::new();
        while self.peek().is_some_and(|ch| ch.is_digit(radix)) {
            digits.push(self.bump().unwrap());
        }
        digits
    }

    fn scan_number(&mut self, start: Loc, negative: bool) -> Result<TokenKind, LexerError> {
        if self.consume("0wx") || self.consume("0wX") {
            if negative {
                return Err(self.error(start, LexerErrorKind::InvalidWordLiteral));
            }
            let digits = self.digits(16);
            if digits.is_empty() {
                return Err(self.error(start, LexerErrorKind::InvalidWordLiteral));
            }
            return Ok(TokenKind::Word(
                self.source[start.offset..self.offset].to_string(),
            ));
        }
        if self.consume("0w") {
            if negative {
                return Err(self.error(start, LexerErrorKind::InvalidWordLiteral));
            }
            let digits = self.digits(10);
            if digits.is_empty() {
                return Err(self.error(start, LexerErrorKind::InvalidWordLiteral));
            }
            return Ok(TokenKind::Word(
                self.source[start.offset..self.offset].to_string(),
            ));
        }
        if self.consume("0x") || self.consume("0X") {
            let digits = self.digits(16);
            if digits.is_empty() {
                return Err(self.error(start, LexerErrorKind::InvalidIntegerLiteral));
            }
            return Ok(TokenKind::Integer(
                self.source[start.offset..self.offset].to_string(),
            ));
        }

        let integer = self.digits(10);
        let mut literal = integer.clone();
        let mut real = false;
        if self.peek() == Some('.') && self.peek_next().is_some_and(|ch| ch.is_ascii_digit()) {
            real = true;
            literal.push(self.bump().unwrap());
            literal.push_str(&self.digits(10));
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            let exponent_tail = self.source[self.offset + 1..].trim_start_matches('~');
            if exponent_tail.starts_with(|ch: char| ch.is_ascii_digit()) {
                self.bump();
                if self.peek() == Some('~') {
                    self.bump();
                }
                let exponent = self.digits(10);
                real = true;
                literal.push_str(&exponent);
            }
        }
        if real {
            Ok(TokenKind::Real(
                self.source[start.offset..self.offset].to_string(),
            ))
        } else {
            let literal = self.source[start.offset..self.offset].to_string();
            Ok(TokenKind::Integer(literal))
        }
    }

    fn scan_string(&mut self, start: Loc, character: bool) -> Result<TokenKind, LexerError> {
        let mut value = String::new();
        loop {
            match self.bump() {
                Some('"') => break,
                Some('\\') => match self.bump() {
                    Some('a') => value.push('\u{7}'),
                    Some('b') => value.push('\u{8}'),
                    Some('t') => value.push('\t'),
                    Some('n') => value.push('\n'),
                    Some('v') => value.push('\u{b}'),
                    Some('f') => value.push('\u{c}'),
                    Some('r') => value.push('\r'),
                    Some('"') => value.push('"'),
                    Some('\\') => value.push('\\'),
                    Some('^') => match self.bump() {
                        Some(ch @ '@'..='_') => {
                            value.push(char::from_u32(ch as u32 & 0x1f).unwrap())
                        }
                        _ => return Err(self.error(start, LexerErrorKind::UnsupportedStringEscape)),
                    },
                    Some(ch @ '0'..='9') => {
                        let digits = [ch, self.bump().unwrap_or('\0'), self.bump().unwrap_or('\0')];
                        if !digits.iter().all(char::is_ascii_digit) {
                            return Err(self.error(start, LexerErrorKind::UnsupportedStringEscape));
                        }
                        let byte =
                            digits
                                .iter()
                                .collect::<String>()
                                .parse::<u8>()
                                .map_err(|_| {
                                    self.error(
                                        start.clone(),
                                        LexerErrorKind::UnsupportedStringEscape,
                                    )
                                })?;
                        value.push(char::from(byte));
                    }
                    Some(ch) if ch.is_whitespace() => {
                        while self.peek().is_some_and(char::is_whitespace) {
                            self.bump();
                        }
                        if self.bump() != Some('\\') {
                            return Err(self.error(start, LexerErrorKind::UnsupportedStringEscape));
                        }
                    }
                    Some(_) => {
                        return Err(self.error(start, LexerErrorKind::UnsupportedStringEscape));
                    }
                    None => return Err(self.error(start, LexerErrorKind::UnterminatedStringEscape)),
                },
                Some(ch) => {
                    let mut bytes = [0; 4];
                    for &byte in ch.encode_utf8(&mut bytes).as_bytes() {
                        value.push(char::from(byte));
                    }
                }
                None => return Err(self.error(start, LexerErrorKind::UnterminatedString)),
            }
        }
        if character {
            let mut chars = value.chars();
            let Some(ch) = chars.next() else {
                return Err(self.error(start, LexerErrorKind::InvalidCharacterLiteral));
            };
            if chars.next().is_some() {
                return Err(self.error(start, LexerErrorKind::InvalidCharacterLiteral));
            }
            Ok(TokenKind::Character(ch))
        } else {
            Ok(TokenKind::String(value))
        }
    }

    fn symbolic(ch: char) -> bool {
        matches!(
            ch,
            '!' | '%'
                | '&'
                | '$'
                | '#'
                | '+'
                | '-'
                | '/'
                | ':'
                | '<'
                | '='
                | '>'
                | '?'
                | '@'
                | '\\'
                | '~'
                | '`'
                | '^'
                | '|'
                | '*'
        )
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.bump();
                continue;
            }
            if ch == '(' && self.peek_next() == Some('*') {
                let start = self.loc();
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
                        (None, _) => {
                            return Err(self.error(start, LexerErrorKind::UnterminatedComment));
                        }
                    }
                }
                continue;
            }
            let start = self.loc();
            if ch == '~' && self.peek_next().is_some_and(|next| next.is_ascii_digit()) {
                self.bump();
                let kind = self.scan_number(start.clone(), true)?;
                tokens.push(Span::new(start, self.loc(), kind));
                continue;
            }
            if ch.is_ascii_digit() {
                let kind = self.scan_number(start.clone(), false)?;
                tokens.push(Span::new(start, self.loc(), kind));
                continue;
            }
            if ch == '#' && self.peek_next() == Some('"') {
                self.bump();
                self.bump();
                let kind = self.scan_string(start.clone(), true)?;
                tokens.push(Span::new(start, self.loc(), kind));
                continue;
            }
            if ch == '.' && self.source[self.offset..].starts_with("...") {
                self.consume("...");
                tokens.push(Span::new(start, self.loc(), TokenKind::Ellipsis));
                continue;
            }
            if Self::symbolic(ch) {
                let mut symbol = String::new();
                while self.peek().is_some_and(Self::symbolic) {
                    symbol.push(self.bump().unwrap());
                }
                let kind = match symbol.as_str() {
                    "=" => TokenKind::Equals,
                    "+" => TokenKind::Plus,
                    "-" => TokenKind::Minus,
                    "*" => TokenKind::Star,
                    "/" => TokenKind::Slash,
                    ">" => TokenKind::Greater,
                    ">=" => TokenKind::GreaterEqual,
                    "<" => TokenKind::Less,
                    "<=" => TokenKind::LessEqual,
                    "<>" => TokenKind::NotEquals,
                    "~" => TokenKind::Tilde,
                    ":" => TokenKind::Colon,
                    "|" => TokenKind::Bar,
                    "#" => TokenKind::Hash,
                    "=>" => TokenKind::FatArrow,
                    "->" => TokenKind::Arrow,
                    ":>" => TokenKind::OpaqueAscription,
                    "..." => TokenKind::Ellipsis,
                    _ => TokenKind::SymbolicIdentifier(symbol),
                };
                tokens.push(Span::new(start, self.loc(), kind));
                continue;
            }
            let first = self.bump().unwrap();
            let kind = match first {
                '_' => TokenKind::Underscore,
                '=' => TokenKind::Equals,
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                ';' => TokenKind::Semicolon,
                '.' => TokenKind::Dot,
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '[' => TokenKind::LeftBracket,
                ']' => TokenKind::RightBracket,
                ',' => TokenKind::Comma,
                '"' => self.scan_string(start.clone(), false)?,
                '{' => TokenKind::LeftBrace,
                '}' => TokenKind::RightBrace,
                ':' => TokenKind::Colon,
                '|' => TokenKind::Bar,
                '#' => TokenKind::Hash,
                '~' => TokenKind::Tilde,
                ch if ch == '\'' => {
                    let mut variable = String::from(ch);
                    while self.peek().is_some_and(|next| {
                        next.is_ascii_alphanumeric() || next == '_' || next == '\''
                    }) {
                        variable.push(self.bump().unwrap());
                    }
                    TokenKind::TypeVariable(variable)
                }
                ch if ch.is_ascii_alphabetic() => {
                    let mut word = String::from(ch);
                    while self.peek().is_some_and(|next| {
                        next.is_ascii_alphanumeric() || next == '_' || next == '\''
                    }) {
                        word.push(self.bump().unwrap());
                    }
                    match word.as_str() {
                        "val" => TokenKind::Val,
                        "div" => TokenKind::Div,
                        "if" => TokenKind::If,
                        "then" => TokenKind::Then,
                        "else" => TokenKind::Else,
                        "true" => TokenKind::True,
                        "false" => TokenKind::False,
                        "abstype" | "and" | "andalso" | "as" | "case" | "datatype" | "do"
                        | "end" | "exception" | "fn" | "fun" | "functor" | "handle" | "in"
                        | "infix" | "infixr" | "let" | "local" | "nonfix" | "of" | "op"
                        | "eqtype" | "include" | "open" | "orelse" | "raise" | "rec"
                        | "sharing" | "sig" | "signature" | "struct" | "structure" | "type"
                        | "where" | "while" | "with" | "withtype" => TokenKind::Reserved(word),
                        _ => TokenKind::Identifier(word),
                    }
                }
                _ => return Err(self.error(start, LexerErrorKind::UnexpectedCharacter(first))),
            };
            tokens.push(Span::new(start, self.loc(), kind));
        }
        Ok(tokens)
    }
}
