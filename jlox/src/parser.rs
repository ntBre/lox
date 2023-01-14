use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token},
    token_type::TokenType,
    Lox,
};

const ARG_LIMIT: usize = 255;

#[derive(Debug)]
struct ParseError;

pub(crate) struct Parser<'a> {
    lox: &'a mut Lox,
    tokens: Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: Vec<Token>, lox: &'a mut Lox) -> Self {
        Self {
            lox,
            tokens,
            current: 0,
        }
    }

    pub(crate) fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.at_end() {
            if let Ok(s) = self.declaration() {
                statements.push(s)
            }
            // the error has already been reported, so skip the err case
        }
        statements
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        let r = if self.matches(&[TokenType::Fun]) {
            self.function("function")
        } else if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        if r.is_err() {
            self.synchronize();
        }
        r
    }

    /// TODO consider making `kind` an enum that implements Display, so the
    /// actual kinds are encoded in the types. it's only used for error
    /// messages, so it's not really a big deal though
    fn function(&mut self, kind: &str) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expect {kind} name."))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {kind} name."),
        )?;
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= ARG_LIMIT {
                    self.error(
                        self.peek(),
                        &format!(
                            "Can't have more than {ARG_LIMIT} parameters."
                        ),
                    );
                }
                params.push(self.consume(
                    TokenType::Identifier,
                    "Expect parameter name.",
                )?);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {kind} body."),
        )?;
        let body = self.block()?;
        Ok(Stmt::function(name, params, body))
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name =
            self.consume(TokenType::Identifier, "Expect variable name.")?;
        let mut initializer = Expr::Null;
        if self.matches(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.matches(&[TokenType::For]) {
            self.for_statement()
        } else if self.matches(&[TokenType::If]) {
            self.if_statement()
        } else if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else if self.matches(&[TokenType::Return]) {
            self.return_statement()
        } else if self.matches(&[TokenType::While]) {
            self.while_statement()
        } else if self.matches(&[TokenType::LeftBrace]) {
            Ok(Stmt::block(self.block()?))
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous();
        let mut value = Expr::Null;
        if !self.check(TokenType::Semicolon) {
            value = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword, value })
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;
        let initializer = if self.matches(&[TokenType::Semicolon]) {
            Stmt::Null
        } else if self.matches(&[TokenType::Var]) {
            self.var_declaration()?
        } else {
            self.expression_statement()?
        };

        let condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Null
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(TokenType::RightParen) {
            self.expression()?
        } else {
            Expr::Null
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if !increment.is_null() {
            body = Stmt::block(vec![
                body,
                Stmt::Expression {
                    expression: increment,
                },
            ]);
        }

        let condition = if condition.is_null() {
            Expr::Literal(Literal::True)
        } else {
            condition
        };

        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if !initializer.is_null() {
            body = Stmt::block(vec![initializer, body]);
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after while condition.",
        )?;
        let body = self.statement()?;
        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;
        let then_branch = self.statement()?;
        let else_branch = if self.matches(&[TokenType::Else]) {
            self.statement()?
        } else {
            Stmt::Null
        };
        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression { expression: value })
    }

    /// expression → equality
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable { name } = expr {
                return Ok(Expr::assign(name, value));
            }
            self.error(equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        while self.matches(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right);
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.matches(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::logical(expr, operator, right);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// term → TODO ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// factor → TODO ( ( ">" | ">=" | "<" | "<=" ) term )*
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// unary → ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;
        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    /// finish a call expression by consuming argument expressions until a right
    /// paren
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            // emulating `do` loop
            loop {
                if arguments.len() >= ARG_LIMIT {
                    self.error(
                        self.peek(),
                        &format!("Can't have more than {ARG_LIMIT} arguments."),
                    );
                }
                arguments.push(self.expression()?);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        let paren =
            self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::call(callee, paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::literal(Literal::False));
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Expr::literal(Literal::True));
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::literal(Literal::Null));
        }

        if self.matches(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::literal(self.previous().literal));
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::variable(self.previous()));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.",
            )?;
            return Ok(Expr::grouping(expr));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for &typ in types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(
        &mut self,
        typ: TokenType,
        message: &str,
    ) -> Result<Token, ParseError> {
        if self.check(typ) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn check(&self, typ: TokenType) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().typ == typ
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().typ.is_eof()
    }

    // TODO these could probably be references, but we'll see
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error(&mut self, token: Token, message: &str) -> ParseError {
        self.lox.parse_error(token, message);
        ParseError
    }

    /// try to recover from an error by looking for a synchronization point (end
    /// of statement, denoted by ;)
    #[allow(unused)]
    fn synchronize(&mut self) {
        use TokenType::*;
        self.advance();
        while !self.at_end() {
            if self.previous().typ == Semicolon {
                return;
            }

            if matches!(
                self.peek().typ,
                Class | For | Fun | If | Print | Return | Var | While
            ) {
                return;
            }

            self.advance();
        }
    }
}
