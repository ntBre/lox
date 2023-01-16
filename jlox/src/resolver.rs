use std::{collections::HashMap, ops::Index};

use crate::{
    expr::Expr, interpreter::Interpreter, stmt::Stmt, token::Token, Lox,
};

use stack::Stack;

mod stack;

pub(crate) struct Resolver<'a, 'b> {
    /// interpreter field from java code
    interpreter: &'a mut Interpreter<'b>,

    scopes: Stack<HashMap<String, bool>>,
}

impl<'a, 'b> Resolver<'a, 'b> {
    pub(crate) fn new(interpreter: &'a mut Interpreter<'b>) -> Self {
        Self {
            interpreter,
            scopes: Stack::new(),
        }
    }

    pub(crate) fn resolve(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_stmt(&mut self, statement: &Stmt) {
        match statement {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements);
                self.end_scope();
            }
            Stmt::Expression { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::Function { name, params, body } => {
                self.declare(name);
                self.define(name);
                self.resolve_function(params, body);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);
                if !else_branch.is_null() {
                    self.resolve_stmt(else_branch);
                }
            }
            Stmt::Null => {}
            Stmt::Print { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::Return { keyword: _, value } => {
                if !value.is_null() {
                    self.resolve_expr(value);
                }
            }
            Stmt::Var { name, initializer } => {
                self.declare(name);
                if !initializer.is_null() {
                    self.resolve_expr(initializer);
                }
                self.define(name);
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition);
                self.resolve_stmt(body);
            }
        }
    }

    fn resolve_function(&mut self, params: &Vec<Token>, body: &[Stmt]) {
        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve(body);
        self.end_scope();
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Assign { name, value } => {
                self.resolve_expr(value);
                self.resolve_local(expr, name);
            }
            Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(callee);

                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::Grouping { expression } => {
                self.resolve_expr(expression);
            }
            Expr::Literal(_) => {}
            Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Null => {}
            Expr::Unary { operator: _, right } => {
                self.resolve_expr(right);
            }
            Expr::Variable { name } => {
                if !self.scopes.is_empty() {
                    let test = self.scopes.peek().get(&name.lexeme).unwrap();
                    if test == &false {
                        self.interpreter.lox.parse_error(
                            name.clone(),
                            "Can't read local variable in its own initializer",
                        );
                    }
                }

                self.resolve_local(expr, name);
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push_default();
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.peek();
        scope.insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes.peek().insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(expr.clone(), self.scopes.len() - 1 - i);
                return;
            }
        }
    }
}
