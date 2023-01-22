pub(crate) mod token_type;
pub(crate) use token_type::*;

pub(crate) struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

#[derive(Clone, Default)]
pub(crate) struct Token {
    pub(crate) typ: TokenType,
    pub(crate) start: usize,
    pub(crate) length: usize,
    pub(crate) line: usize,
}

impl Token {
    pub(crate) fn new(
        typ: TokenType,
        start: usize,
        length: usize,
        line: usize,
    ) -> Self {
        Self {
            typ,
            start,
            length,
            line,
        }
    }
}

macro_rules! ternary {
    ($test:expr => $then:expr, $else:expr) => {
        if $test {
            $then
        } else {
            $else
        }
    };
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c == '_'
}

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn get_token(&self, token: &Token) -> String {
        self.source[token.start..token.start + token.length]
            .iter()
            .collect()
    }

    pub(crate) fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        if is_alpha(c) {
            return self.identifier();
        }
        if c.is_ascii_digit() {
            return self.number();
        }

        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => {
                let tok = ternary!(self.matches('=')
			 => TokenType::BangEqual, TokenType::Bang);
                return self.make_token(tok);
            }
            '=' => {
                let tok = ternary!(self.matches('=')
			 => TokenType::EqualEqual, TokenType::Equal);
                return self.make_token(tok);
            }
            '<' => {
                let tok = ternary!(self.matches('=')
			 => TokenType::LessEqual, TokenType::Less);
                return self.make_token(tok);
            }
            '>' => {
                let tok = ternary!(self.matches('=')
			 => TokenType::GreaterEqual, TokenType::Greater);
                return self.make_token(tok);
            }
            '"' => return self.string(),
            _ => {}
        }

        self.error_token("Unexpected character.")
    }

    fn make_token(&mut self, typ: TokenType) -> Token {
        Token::new(typ, self.start, self.current - self.start, self.line)
    }

    fn error_token(&self, arg: &str) -> Token {
        Token::new(TokenType::Error, 0, arg.len(), self.line)
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub(crate) fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        typ: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length {
            let s: String =
                self.source[self.start..self.current].iter().collect();
            if s == rest {
                return typ;
            }
        }
        TokenType::Identifier
    }

    fn identifier_type(&mut self) -> TokenType {
        match self.source[self.start] {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current > self.start + 1 {
                    match self.source[self.start + 1] {
                        'a' => {
                            return self.check_keyword(
                                2,
                                3,
                                "lse",
                                TokenType::False,
                            )
                        }
                        'o' => {
                            return self.check_keyword(
                                2,
                                1,
                                "r",
                                TokenType::For,
                            )
                        }
                        'u' => {
                            return self.check_keyword(
                                2,
                                1,
                                "n",
                                TokenType::Fun,
                            )
                        }
                        _ => {}
                    }
                }
                TokenType::Identifier
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current > self.start + 1 {
                    match self.source[self.start + 1] {
                        'h' => {
                            return self.check_keyword(
                                2,
                                2,
                                "is",
                                TokenType::This,
                            )
                        }
                        'r' => {
                            return self.check_keyword(
                                2,
                                2,
                                "ue",
                                TokenType::True,
                            )
                        }
                        _ => {}
                    }
                }
                TokenType::Identifier
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || self.peek().is_ascii_digit() {
            self.advance();
        }
        let typ = self.identifier_type();
        self.make_token(typ)
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume decimal point
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance(); // closing quote
        self.make_token(TokenType::String)
    }

    fn peek(&self) -> char {
	if self.at_end() {
	    return '\0'
	}
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.at_end() {
            return '\0';
        }
        self.source[self.current + 1]
    }
}
