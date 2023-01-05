#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum TokenType {
    // Single-character tokens
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
    // One- or two-character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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
    //
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

    /// Returns `true` if the token type is [`Or`].
    ///
    /// [`Or`]: TokenType::Or
    #[must_use]
    pub(crate) fn is_or(&self) -> bool {
        matches!(self, Self::Or)
    }
}
