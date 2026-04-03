use crate::token::Token;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn display_tokens(&self) {
        for token in self.tokens.iter() {
            println!("{:?}", token);
        }
    }

    pub fn scan_tokens(&mut self) {
        for ch in self.source.chars() {
            let token = self.parse_token(ch);
            self.tokens.push(token);
        }
    }

    fn parse_token(&self, ch: char) -> Token {
        match ch {
            '0' => Token::new(crate::token::TokenKind::Zero),
            'S' => Token::new(crate::token::TokenKind::Successor),
            'a' => Token::new(crate::token::TokenKind::VarA),
            'b' => Token::new(crate::token::TokenKind::VarB),
            'c' => Token::new(crate::token::TokenKind::VarC),
            'd' => Token::new(crate::token::TokenKind::VarD),
            'e' => Token::new(crate::token::TokenKind::VarE),
            '\'' => Token::new(crate::token::TokenKind::Apostrophe),
            '(' => Token::new(crate::token::TokenKind::LeftParen),
            ')' => Token::new(crate::token::TokenKind::RightParen),
            ':' => Token::new(crate::token::TokenKind::Colon),
            '[' => Token::new(crate::token::TokenKind::LeftBracket),
            ']' => Token::new(crate::token::TokenKind::RightBracket),
            '+' => Token::new(crate::token::TokenKind::Plus),
            '-' => Token::new(crate::token::TokenKind::Minus),
            '=' => Token::new(crate::token::TokenKind::Equals),
            '<' => Token::new(crate::token::TokenKind::LessThan),
            '>' => Token::new(crate::token::TokenKind::GreaterThan),
            '~' => Token::new(crate::token::TokenKind::Not),
            '&' => Token::new(crate::token::TokenKind::And),
            '|' => Token::new(crate::token::TokenKind::Or),
            '?' => Token::new(crate::token::TokenKind::Implies),
            'A' => Token::new(crate::token::TokenKind::ForAll),

            '\n' => Token::new(crate::token::TokenKind::EOF),
            _ => panic!("Invalid or Unregonized Token {ch}"),
        }
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
