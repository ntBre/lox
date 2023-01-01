use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Debug)]
pub(crate) enum Literal {
    String(String),
    Number(f64),
    Null,
}

pub(crate) struct Token {
    typ: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Token {
    pub(crate) fn new(
        typ: TokenType,
        lexeme: String,
        literal: Literal,
        line: usize,
    ) -> Self {
        Self {
            typ,
            lexeme,
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.typ, self.lexeme, self.literal)
    }
}
