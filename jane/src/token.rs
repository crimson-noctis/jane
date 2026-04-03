#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Token { kind }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Numerals
    Zero,
    Successor,

    // Variables
    VarA,
    VarB,
    VarC,
    VarD,
    VarE,

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

    // Whitespace
    Whitespace,

    // End of File
    EOF,
}
