#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Token { kind }
    }

    pub fn get_kind(&self) -> TokenKind {
        self.kind.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Numerals
    Zero,
    Successor,

    // Variables
    Identifier(char),

    // Punctuation
    Apostrophe,
    LeftParen,
    RightParen,
    Colon,
    LeftBracket,
    RightBracket,

    // Operators
    Plus,
    Times,
    Equals,
    LeftAngleBracket,
    RightAngleBracket,

    // Logical Symbols
    Not,
    And,
    Or,
    Exists,
    ForAll,

    // Two character token
    Implies,

    // End of File
    EOF,
}
