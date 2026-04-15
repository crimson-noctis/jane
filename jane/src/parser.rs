use crate::ast::Formula;
use crate::ast::Term;
use crate::ast::new_succ;
use crate::ast::new_var;
use crate::ast::new_zero;
use crate::token::Token;
use crate::token::TokenKind;

pub struct Parser {
    formulas: Vec<Formula>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            formulas: Vec::new(),
            current: 0,
        }
    }

    pub fn formulas(&self) -> &Vec<Formula> {
        &self.formulas
    }
}

impl Parser {
    pub fn parse_term(&mut self, tokens: &[Token]) -> Term {
        let token = &tokens[self.current];
        self.current += 1;

        match token.kind() {
            TokenKind::Zero => new_zero(),

            TokenKind::Variable(c) => new_var(c),

            TokenKind::Successor => {
                let child = self.parse_term(tokens);
                new_succ(child)
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

        let mut parser = Parser::new();

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

        let mut parser = Parser::new();

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
