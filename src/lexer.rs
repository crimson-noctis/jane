use crate::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

impl Lexer {
    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

impl Lexer {
    pub fn tokenize(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            self.generate_token()?;
        }

        self.tokens.push(Token::new(TokenKind::EOF));

        Ok(())
    }

    fn generate_token(&mut self) -> Result<(), String> {
        let ch = self.advance();

        if self.is_whitespace(ch) || ch == '\n' {
            return Ok(());
        }

        let kind = match ch {
            // Terms
            '0' => Some(TokenKind::Zero),
            'S' => Some(TokenKind::Successor),

            // Variables
            ch if ch.is_ascii_lowercase() => Some(TokenKind::Variable(ch)),

            // Syntax
            '(' => Some(TokenKind::OpenParen),
            ')' => Some(TokenKind::CloseParen),
            '[' => Some(TokenKind::OpenBracket),
            ']' => Some(TokenKind::CloseBracket),
            '<' => Some(TokenKind::Lt),
            '>' => Some(TokenKind::Gt),
            '\'' => Some(TokenKind::Apostrophe),
            ':' => Some(TokenKind::Colon),

            // Operations
            '+' => Some(TokenKind::Add),
            '*' => Some(TokenKind::Mult),

            // Logical Operators
            '~' => Some(TokenKind::Not),
            '&' => Some(TokenKind::And),
            '|' => Some(TokenKind::Or),
            '-' => match self.peek() {
                Some('>') => {
                    self.advance();
                    Some(TokenKind::Implies)
                }
                _ => None,
            },

            // Quantifiers
            'A' => Some(TokenKind::ForAll),
            'E' => Some(TokenKind::Exists),

            // Equality
            '=' => Some(TokenKind::Equals),

            _ => None,
        };

        match kind {
            Some(token_kind) => {
                self.tokens.push(Token::new(token_kind));
                Ok(())
            }
            None => Err(format!("Unrecognized Token: {}", ch)),
        }
    }

    fn is_whitespace(&self, ch: char) -> bool {
        matches!(ch, ' ' | '\t' | '\r')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    fn lex(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(input.to_string());
        lexer.tokenize().unwrap();
        lexer.tokens.into_iter().map(|t| t.kind()).collect()
    }

    #[test]
    fn test_single_char_tokens() {
        let tokens = lex("()+*=");

        assert_eq!(
            tokens,
            vec![
                TokenKind::OpenParen,
                TokenKind::CloseParen,
                TokenKind::Add,
                TokenKind::Mult,
                TokenKind::Equals,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_keywords() {
        let tokens = lex("0SAE");

        assert_eq!(
            tokens,
            vec![
                TokenKind::Zero,
                TokenKind::Successor,
                TokenKind::ForAll,
                TokenKind::Exists,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let tokens = lex("abc");

        assert_eq!(
            tokens,
            vec![
                TokenKind::Variable('a'),
                TokenKind::Variable('b'),
                TokenKind::Variable('c'),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_implies() {
        let tokens = lex("->");

        assert_eq!(tokens, vec![TokenKind::Implies, TokenKind::EOF,]);
    }

    #[test]
    fn test_invalid_implies() {
        let mut lexer = Lexer::new("-".to_string());
        let result = lexer.tokenize();

        assert!(result.is_err());
    }

    #[test]
    fn test_whitespace() {
        let tokens = lex("a b\tc\r");

        assert_eq!(
            tokens,
            vec![
                TokenKind::Variable('a'),
                TokenKind::Variable('b'),
                TokenKind::Variable('c'),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_mixed_expression() {
        let tokens = lex("A x -> S y");

        assert_eq!(
            tokens,
            vec![
                TokenKind::ForAll,
                TokenKind::Variable('x'),
                TokenKind::Implies,
                TokenKind::Successor,
                TokenKind::Variable('y'),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_unknown_token() {
        let mut lexer = Lexer::new("@".to_string());
        let result = lexer.tokenize();

        assert!(result.is_err());
    }
}
