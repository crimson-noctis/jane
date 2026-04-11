use crate::token::{Token, TokenKind};

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

    pub fn scan_tokens(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenKind::EOF));
        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let ch = self.advance();

        if self.try_whitespace(ch)
            || self.try_newline(ch)
            || self.try_single_char(ch)
            || self.try_keyword(ch)
            || self.try_identifier(ch)
        {
            return Ok(());
        }

        if self.try_implies(ch)? {
            return Ok(());
        }

        Err(format!("Unrecognized Token: {ch}"))
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
    fn try_whitespace(&self, ch: char) -> bool {
        matches!(ch, ' ' | '\t' | '\r')
    }

    fn try_newline(&mut self, ch: char) -> bool {
        if ch == '\n' {
            self.tokens.push(Token::new(TokenKind::Newline));
            true
        } else {
            false
        }
    }

    fn try_single_char(&mut self, ch: char) -> bool {
        let kind = match ch {
            '\'' => Some(TokenKind::Apostrophe),
            '(' => Some(TokenKind::LeftParen),
            ')' => Some(TokenKind::RightParen),
            ':' => Some(TokenKind::Colon),
            '[' => Some(TokenKind::LeftBracket),
            ']' => Some(TokenKind::RightBracket),
            '+' => Some(TokenKind::Plus),
            '*' => Some(TokenKind::Times),
            '=' => Some(TokenKind::Equals),
            '<' => Some(TokenKind::LeftAngleBracket),
            '>' => Some(TokenKind::RightAngleBracket),
            '~' => Some(TokenKind::Not),
            '&' => Some(TokenKind::And),
            '|' => Some(TokenKind::Or),
            _ => None,
        };

        if let Some(kind) = kind {
            self.tokens.push(Token::new(kind));
            true
        } else {
            false
        }
    }

    fn try_keyword(&mut self, ch: char) -> bool {
        let kind = match ch {
            '0' => Some(TokenKind::Zero),
            'S' => Some(TokenKind::Successor),
            'A' => Some(TokenKind::ForAll),
            'E' => Some(TokenKind::Exists),
            _ => None,
        };

        if let Some(kind) = kind {
            self.tokens.push(Token::new(kind));
            true
        } else {
            false
        }
    }

    fn try_implies(&mut self, ch: char) -> Result<bool, String> {
        if ch != '-' {
            return Ok(false);
        }

        match self.peek() {
            Some('>') => {
                self.advance();
                self.tokens.push(Token::new(TokenKind::Implies));
                Ok(true)
            }
            _ => Err("Expected '>' after '-'".to_string()),
        }
    }

    fn try_identifier(&mut self, ch: char) -> bool {
        if ch.is_ascii_lowercase() {
            self.tokens.push(Token::new(TokenKind::Identifier(ch)));
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    fn lex(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(input.to_string());
        lexer.scan_tokens().unwrap();
        lexer.tokens.into_iter().map(|t| t.get_kind()).collect()
    }

    #[test]
    fn test_single_char_tokens() {
        let tokens = lex("()+*=");

        assert_eq!(
            tokens,
            vec![
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::Plus,
                TokenKind::Times,
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
                TokenKind::Identifier('a'),
                TokenKind::Identifier('b'),
                TokenKind::Identifier('c'),
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
        let result = lexer.scan_tokens();

        assert!(result.is_err());
    }

    #[test]
    fn test_newlines() {
        let tokens = lex("\n\n");

        assert_eq!(
            tokens,
            vec![TokenKind::Newline, TokenKind::Newline, TokenKind::EOF,]
        );
    }

    #[test]
    fn test_whitespace() {
        let tokens = lex("a b\tc\r");

        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier('a'),
                TokenKind::Identifier('b'),
                TokenKind::Identifier('c'),
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
                TokenKind::Identifier('x'),
                TokenKind::Implies,
                TokenKind::Successor,
                TokenKind::Identifier('y'),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_unknown_token() {
        let mut lexer = Lexer::new("@".to_string());
        let result = lexer.scan_tokens();

        assert!(result.is_err());
    }
}
