use crate::token::Token;
use crate::token::TokenKind;

pub struct Scanner {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.scan_token().unwrap();
        }
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let ch = self.advance();

        if matches!(ch, ' ' | '\t' | '\r') {
            return Ok(());
        }

        let token = match ch {
            '0' => Token::new(TokenKind::Zero),
            'S' => Token::new(TokenKind::Successor),
            '\'' => Token::new(TokenKind::Apostrophe),
            '(' => Token::new(TokenKind::LeftParen),
            ')' => Token::new(TokenKind::RightParen),
            ':' => Token::new(TokenKind::Colon),
            '[' => Token::new(TokenKind::LeftBracket),
            ']' => Token::new(TokenKind::RightBracket),
            '+' => Token::new(TokenKind::Plus),
            '-' => Token::new(TokenKind::Minus),
            '=' => Token::new(TokenKind::Equals),
            '<' => Token::new(TokenKind::LessThan),
            '>' => Token::new(TokenKind::GreaterThan),
            '~' => Token::new(TokenKind::Not),
            '&' => Token::new(TokenKind::And),
            '|' => Token::new(TokenKind::Or),
            '?' => Token::new(TokenKind::Implies),
            'A' => Token::new(TokenKind::ForAll),
            '\n' => Token::new(TokenKind::EOF),

            ch if ch.is_ascii_lowercase() => Token::new(TokenKind::Identifier(ch)),

            _ => return Err(format!("Unrecognized token or invalid expression: {ch}")),
        };

        self.tokens.push(token);
        Ok(())
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;

    #[test]
    fn test_token() {
        let mut scanner = Scanner::new("Hello".to_string());

        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens,
            vec![
                crate::token::Token::new(crate::token::TokenKind::Successor),
                crate::token::Token::new(crate::token::TokenKind::Successor),
                crate::token::Token::new(crate::token::TokenKind::Zero)
            ]
        )
    }
}
