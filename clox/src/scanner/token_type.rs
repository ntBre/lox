use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    #[default]
    Error,
    Eof,
}

impl TokenType {
    /// Returns `true` if the token type is [`Eof`].
    ///
    /// [`Eof`]: TokenType::Eof
    #[must_use]
    pub(crate) fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }

    /// Returns `true` if the token type is [`Error`].
    ///
    /// [`Error`]: TokenType::Error
    #[must_use]
    pub(crate) fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (*self) as u8)
    }
}
