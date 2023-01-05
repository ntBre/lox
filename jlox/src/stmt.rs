use crate::{expr::Expr, token::Token};

pub(crate) enum Stmt {
    Block { statements: Vec<Stmt> },
    Expression { expression: Expr },
    If { condition: Expr, then_branch: Box<Stmt>, else_branch: Box<Stmt> },
    Null,
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

    /// Returns `true` if the stmt is [`Null`].
    ///
    /// [`Null`]: Stmt::Null
    #[must_use]
    pub(crate) fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}
