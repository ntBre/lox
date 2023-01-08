use std::fmt::Display;

use crate::{expr::Expr, token::Token};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expression: Expr,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    Null,
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Expr,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub(crate) fn block(statements: Vec<Stmt>) -> Self {
        Self::Block { statements }
    }

    pub(crate) fn function(
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    ) -> Self {
        Self::Function { name, params, body }
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

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Block { statements } => {
                writeln!(f, "(progn")?;
                for s in statements {
                    writeln!(f, "\t({s})")?;
                }
                writeln!(f, ")")
            }
            Stmt::Expression { expression } => writeln!(f, "{expression}"),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => writeln!(
                f,
                "(if {condition}
\t{then_branch}
\t{else_branch})"
            ),
            Stmt::Null => writeln!(f, "nil"),
            Stmt::Print { expression } => writeln!(f, "(print {expression})"),
            Stmt::Var { name, initializer } => {
                writeln!(f, "(setf {name} {initializer})")
            }
            Stmt::While { condition, body } => writeln!(
                f,
                "(while {condition}
\t{body})"
            ),
            Stmt::Function { name, params, body } => {
                write!(f, "(defun {name} (")?;
                for param in params {
                    write!(f, " {param}")?;
                }

                for (i, stmt) in body.iter().enumerate() {
                    write!(f, "\t{stmt}")?;
                    if i < body.len() - 1 {
                        writeln!(f)?;
                    }
                }

                writeln!(f, ")")
            }
        }
    }
}
