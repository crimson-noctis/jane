use crate::ast::Formula;
use crate::ast::Term;
use crate::ast::new_product;
use crate::ast::new_succ;
use crate::ast::new_sum;
use crate::ast::new_var;
use crate::ast::new_zero;
use crate::token::Token;
use crate::token::TokenKind;

pub struct Parser {
    token: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(token: Vec<Token>) -> Self {
        Self { token, pos: 0 }
    }
}

impl Parser {
    pub fn peek(&self) -> TokenKind {
        self.token[self.pos].kind()
    }

    pub fn advance(&mut self) -> TokenKind {
        let t = self.token[self.pos].kind();
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: TokenKind) {
        let actual = self.advance();

        if actual != expected {
            panic!("Expected {:?}, got {:?}", expected, actual);
        }
    }
}

impl Parser {
    pub fn parse_term(&mut self) -> Term {
        match self.peek() {
            TokenKind::Zero => {
                self.advance();
                new_zero()
            }

            TokenKind::Variable(ch) => {
                self.advance();
                new_var(ch)
            }

            TokenKind::Successor => {
                self.advance();
                let child = self.parse_term();
                new_succ(child)
            }

            TokenKind::OpenParen => {
                self.advance();

                let left = self.parse_term();

                let op = self.advance();

                let right = self.parse_term();

                self.expect(TokenKind::CloseParen);

                match op {
                    TokenKind::Add => new_sum(left, right),
                    TokenKind::Mult => new_product(left, right),

                    _ => panic!("Invalid Expression"),
                }
            }

            _ => panic!("Unexpected Token in Term"),
        }
    }

    pub fn parse_formula(&mut self, tokens: &[Token]) -> Formula {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Term, new_succ},
        parser::Parser,
        token::{Token, TokenKind},
    };

    #[test]
    fn test_parse_zero() {
        let tokens = vec![Token::new(TokenKind::Zero)];

        let mut parser = Parser::new(tokens.clone());

        let term = parser.parse_term(&tokens);
        let expected = Term::Zero;

        assert_eq!(term, expected);
    }

    #[test]
    fn test_parse_ss0() {
        let tokens = vec![
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Zero),
        ];

        let mut parser = Parser::new(tokens.clone());

        let term = parser.parse_term(&tokens);

        let expected = Term::Succ {
            child: Box::new(Term::Succ {
                child: Box::new(Term::Zero),
            }),
        };

        assert_eq!(term, expected)
    }

    #[test]
    fn test_parse_var() {}
}
