use crate::{
    expr::Expr,
    stmt::Stmt,
    token::{Literal, Token},
    token_type::TokenType,
    Lox,
};

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
        let r = if self.matches(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        if r.is_err() {
            self.synchronize();
        }
        r
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
        if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression { expression: value })
    }

    /// expression → equality
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
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

        self.primary()
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
