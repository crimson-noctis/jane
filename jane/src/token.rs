#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Terms
    Zero,      // 0
    Successor, // S

    // Variables (Examples: a..z or a', b'', c''')
    Variable(char),

    // Syntax
    OpenParen,    // (
    CloseParen,   // )
    Gt,           // >
    Lt,           // <
    OpenBracket,  // [
    CloseBracket, // ]
    Apostrophe,   // '
    Colon,        // :

    // Operations
    Add,  // +
    Mult, // •

    // Logical operators
    Not,     // ¬
    And,     // ∧
    Or,      // ∨
    Implies, // ⊃

    // Quantifiers
    ForAll, // ∀
    Exists, // ∃

    // Equality
    Equals, // =

    // End markers
    EOF,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Self { kind }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}
