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
    terms: Vec<Term>,
    pos: usize,
}

impl Parser {
    pub fn new(token: Vec<Token>) -> Self {
        Self {
            token,
            terms: Vec::new(),
            pos: 0,
        }
    }
}

impl Parser {
    pub fn peek(&self) -> TokenKind {
        if self.pos >= self.token.len() {
            TokenKind::EOF
        } else {
            self.token[self.pos].kind()
        }
    }
    pub fn terms(&self) -> &Vec<Term> {
        &self.terms
    }

    pub fn advance(&mut self) -> TokenKind {
        let t = self.token[self.pos].kind();
        self.pos += 1;
        t
    }

    pub fn is_end(&self) -> bool {
        self.token[self.pos].kind() == TokenKind::EOF
    }

    pub fn push(&mut self, term: Term) {
        self.terms.push(term);
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

    pub fn parse_formula(&mut self) -> Formula {
        match self.peek() {
            TokenKind::OpenParen => {
                self.advance();

                let left = self.parse_term();

                let formula = self.advance();

                let right = self.parse_term();

                self.expect(TokenKind::CloseParen);

                match formula {
                    TokenKind::Equals => Formula::Atom { left, right },
                    _ => panic!("Invalid Formula"),
                }
            }

            TokenKind::Not => {
                self.advance();

                let child = self.parse_formula();

                Formula::Negation {
                    child: Box::new(child),
                }
            }

            TokenKind::Exists => {
                self.advance();

                let var_term = self.parse_term();

                let var = match var_term {
                    Term::Var { var } => var,
                    _ => panic!("Expected Variable After E"),
                };

                let colon = self.advance();

                let body = self.parse_formula();

                Formula::Exists {
                    var: var,
                    body: Box::new(body),
                }
            }

            TokenKind::ForAll => {
                self.advance();

                let var_term = self.parse_term();

                let var = match var_term {
                    Term::Var { var } => var,
                    _ => panic!("Expected Variable After E"),
                };

                let colon = self.advance();

                let body = self.parse_formula();

                Formula::ForAll {
                    var: var,
                    body: Box::new(body),
                }
            }

            _ => {
                println!("{:#?}", self.peek());
                panic!("ERROR");
            }
        }
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
        let tokens = vec![Token::new(TokenKind::Zero), Token::new(TokenKind::EOF)];

        let mut parser = Parser::new(tokens.clone());

        let term = parser.parse_term();
        let expected = Term::Zero;

        assert_eq!(term, expected);
    }

    #[test]
    fn test_parse_ss0() {
        let tokens = vec![
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::EOF),
        ];

        let mut parser = Parser::new(tokens.clone());

        let term = parser.parse_term();

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
