use std::fmt::Display;

use crate::token::{Literal, Token};

#[derive(Debug)]
pub(crate) enum Expr {
    Null,
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Literal),
    Variable {
        name: Token,
    },
}

impl Expr {
    pub(crate) fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Self::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub(crate) fn grouping(expression: Expr) -> Self {
        Self::Grouping {
            expression: Box::new(expression),
        }
    }

    pub(crate) fn literal(l: Literal) -> Self {
        Self::Literal(l)
    }

    pub(crate) fn unary(operator: Token, right: Expr) -> Self {
        Self::Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub(crate) fn variable(name: Token) -> Self {
	Self::Variable { name }
    }

    /// Returns `true` if the expr is [`Null`].
    ///
    /// [`Null`]: Expr::Null
    #[must_use]
    pub(crate) fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", operator.lexeme, left, right),
            Expr::Grouping { expression } => write!(f, "(group {expression})"),
            Expr::Literal(l) => write!(f, "{l}"),
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
            Expr::Null => write!(f, "nil"),
            Expr::Variable { name } => write!(f, "{name}"),
        }
    }
}
