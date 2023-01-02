use std::fmt::Display;

use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub(crate) enum Literal {
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Null => write!(f, "nil"),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Token {
    pub(crate) typ: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Literal,
    pub(crate) line: usize,
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
