use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use crate::Lox;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]);
}

pub(crate) struct Scanner<'a> {
    // this is a bad idea, looking a lot like algae, but here we go. as long as
    // the scanner runs before any other phase of interpretation, it might be
    // okay. if the scanner and parser need references at the same time, we'll
    // be in trouble again. right now we're only holding this so we can set
    // `had_error`, so we might be able to get away with bubbling up a Result
    // instead
    lox: &'a mut Lox,
    // I keep calling chars everywhere, so it might be better to keep it as a
    // Vec<char> from the start
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

/// approximates java's ternary operator specifically for potentially
/// multi-character lexemes like != vs !
macro_rules! operator {
    ($self:ident, $want:expr, $then:expr, $else:expr) => {{
        let t = if $self.matches($want) { $then } else { $else };
        $self.add_token(t, Literal::Null);
    }};
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(source: String, lox: &'a mut Lox) -> Self {
        Self {
            source,
            lox,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_owned(),
            Literal::Null,
            self.line,
        ));

        // unclear if we need self.tokens after this. if so, derive Clone and
        // clone it. actually, it's not clear that tokens needs to be a field on
        // Scanner either. maybe it's just internal to scan_tokens itself
        std::mem::take(&mut self.tokens)
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Null),
            ')' => self.add_token(TokenType::RightParen, Literal::Null),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Null),
            '}' => self.add_token(TokenType::RightBrace, Literal::Null),
            ',' => self.add_token(TokenType::Comma, Literal::Null),
            '.' => self.add_token(TokenType::Dot, Literal::Null),
            '-' => self.add_token(TokenType::Minus, Literal::Null),
            '+' => self.add_token(TokenType::Plus, Literal::Null),
            ';' => self.add_token(TokenType::Semicolon, Literal::Null),
            '*' => self.add_token(TokenType::Star, Literal::Null),
            '!' => operator!(self, '=', TokenType::BangEqual, TokenType::Bang),
            '=' => {
                operator!(self, '=', TokenType::EqualEqual, TokenType::Equal)
            }
            '<' => operator!(self, '=', TokenType::LessEqual, TokenType::Less),
            '>' => operator!(
                self,
                '=',
                TokenType::GreaterEqual,
                TokenType::Greater
            ),
            '/' => {
                if self.matches('/') {
                    // comment
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Null)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    self.lox.error(self.line, "Unexpected character.");
                }
            }
        }
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        let typ = match KEYWORDS.get(text.as_str()) {
            Some(typ) => *typ,
            None => TokenType::Identifier,
        };

        self.add_token(typ, Literal::Null);
    }

    fn number(&mut self) {
        // assume we're sticking with base 10, but could vary this
        const RADIX: u32 = 10;
        while self.peek().is_digit(RADIX) {
            self.advance();
        }

        // look for fractional part
        if self.peek() == '.' && self.peek_next().is_digit(RADIX) {
            self.advance();
            while self.peek().is_digit(RADIX) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Literal::Number(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            ),
        );
    }

    /// consume characters from self until a closing " or EOF. escape sequences
    /// are not supported
    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            self.lox.error(self.line, "Unterminated string.");
            return;
        }

        self.advance(); // closing "

        self.add_token(
            TokenType::String,
            Literal::String(
                self.source
                    .chars()
                    .skip(self.start + 1)
                    // distribute the negative
                    .take(self.current - 1 - self.start - 1)
                    .collect(),
            ),
        );
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, typ: TokenType, literal: Literal) {
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token::new(typ, text, literal, self.line));
    }

    fn matches(&mut self, arg: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != arg {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    // clearly this and peek are versions of the same thing, where the number of
    // characters to lookahead is a variable. the book addresses this in an
    // aside, saying that this version emphasizes that we only look ahead a
    // maximum of 2 characters
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.chars().count() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
}

// NOTE that we use the built-in is_digit but our own is_alpha throughout
fn is_alphanumeric(c: char) -> bool {
    c.is_ascii_digit() || is_alpha(c)
}
