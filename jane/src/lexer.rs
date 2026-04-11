use crate::token::Token;
use crate::token::TokenKind;

pub struct Lexer {
    source: Vec<char>,
    pub tokens: Vec<Token>,
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

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.scan_token().unwrap();
        }

        self.tokens.push(Token::new(TokenKind::EOF));
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
            '*' => Token::new(TokenKind::Times),
            '=' => Token::new(TokenKind::Equals),
            '<' => Token::new(TokenKind::LeftAngleBracket),
            '>' => Token::new(TokenKind::RightAngleBracket),
            '~' => Token::new(TokenKind::Not),
            '&' => Token::new(TokenKind::And),
            '|' => Token::new(TokenKind::Or),
            'A' => Token::new(TokenKind::ForAll),
            'E' => Token::new(TokenKind::Exists),
            '\n' => Token::new(TokenKind::Newline),
            '-' => {
                let next_token = self.advance();
                if next_token != '>' {
                    return Err("Unexpected character".to_string());
                }
                Token::new(TokenKind::Implies)
            }

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
    use crate::{lexer::Lexer, token::Token, token::TokenKind};

    #[test]
    fn test_token() {
        let mut lexer = Lexer::new("SS0".to_string());

        lexer.scan_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token::new(TokenKind::Successor),
                Token::new(TokenKind::Successor),
                Token::new(TokenKind::Zero),
                Token::new(TokenKind::EOF)
            ]
        )
    }

    #[test]
    fn test_imples() {
        let mut lexer = Lexer::new("(0 = 0) -> (S0 = S0)".to_string());

        lexer.scan_tokens();

        assert_eq!(
            lexer.tokens,
            vec![
                Token::new(TokenKind::LeftParen),
                Token::new(TokenKind::Zero),
                Token::new(TokenKind::Equals),
                Token::new(TokenKind::Zero),
                Token::new(TokenKind::RightParen),
                Token::new(TokenKind::Implies),
                Token::new(TokenKind::LeftParen),
                Token::new(TokenKind::Successor),
                Token::new(TokenKind::Zero),
                Token::new(TokenKind::Equals),
                Token::new(TokenKind::Successor),
                Token::new(TokenKind::Zero),
                Token::new(TokenKind::RightParen),
                Token::new(TokenKind::EOF),
            ]
        )
    }
}
