use crate::{expr::Expr, token::Token};

pub(crate) enum Stmt {
    Block { statements: Vec<Stmt> },
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
}

impl Stmt {
    pub(crate) fn block(statements: Vec<Stmt>) -> Self {
        Self::Block { statements }
    }

    pub(crate) fn var(name: Token, initializer: Expr) -> Self {
        Self::Var { name, initializer }
    }
}
