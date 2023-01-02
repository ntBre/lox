use crate::{expr::Expr, token::Token};

pub(crate) enum Stmt {
    Print { expression: Expr },
    Expression { expression: Expr },
    Var { name: Token, initializer: Expr },
}

impl Stmt {
    pub(crate) fn var(name: Token, initializer: Expr) -> Self {
        Self::Var { name, initializer }
    }
}
