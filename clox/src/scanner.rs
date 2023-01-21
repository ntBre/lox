pub(crate) mod token_type;
pub(crate) use token_type::*;

pub(crate) struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

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

impl Scanner {
    pub(crate) fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub(crate) fn scan_token(&mut self) -> Token {
        self.start = self.current;

        if self.at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

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
        self.current == self.source.len()
    }

    fn advance(&mut self) -> char {
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
}
