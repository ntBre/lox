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

impl Eq for Literal {}

// NOTE have to implement this to satisfy clippy that the eq and hash
// implementations are the same
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::String(a), Literal::String(b)) => a.eq(b),
            (Literal::Number(a), Literal::Number(b)) => {
                a.to_bits().eq(&b.to_bits())
            }
            (Literal::True, Literal::True)
            | (Literal::False, Literal::False)
            | (Literal::Null, Literal::Null) => true,
            _ => false,
        }
    }
}

impl std::hash::Hash for Literal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Literal::String(s) => s.hash(state),
            Literal::Number(n) => n.to_bits().hash(state),
            t @ Literal::True => t.hash(state),
            f @ Literal::False => f.hash(state),
            n @ Literal::Null => n.hash(state),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => write!(f, "{n}"),
            Literal::Null => write!(f, "nil"),
            Literal::True => write!(f, "true"),
            Literal::False => write!(f, "false"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
