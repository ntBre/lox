use crate::expr::Expr;

pub(crate) enum Stmt {
    Expression(Expr),
    Print(Expr),
}
