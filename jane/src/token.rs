#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Token { kind }
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
    Minus,
    Equals,
    LessThan,
    GreaterThan,

    // Logical Symbols
    Not,
    And,
    Or,
    Implies,
    Exists,
    ForAll,

    // End of File
    EOF,
}
