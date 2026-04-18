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
            TokenKind::Lt => {
                self.advance();

                let left = self.parse_formula();
                let op = self.advance();
                let right = self.parse_formula();

                self.expect(TokenKind::Gt);

                match op {
                    TokenKind::And => Formula::And {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    TokenKind::Or => Formula::Or {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    TokenKind::Implies => Formula::Implies {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    _ => panic!("Expected binary connective, got {:?}", op),
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

                self.expect(TokenKind::Colon);

                let body = self.parse_formula();

                Formula::Exists {
                    var,
                    body: Box::new(body),
                }
            }

            TokenKind::ForAll => {
                self.advance();

                let var_term = self.parse_term();

                let var = match var_term {
                    Term::Var { var } => var,
                    _ => panic!("Expected Variable After A"),
                };

                self.expect(TokenKind::Colon);

                let body = self.parse_formula();

                Formula::ForAll {
                    var,
                    body: Box::new(body),
                }
            }

            _ => {
                let left = self.parse_term();

                match self.peek() {
                    TokenKind::Equals => {
                        self.advance();

                        let right = self.parse_term();

                        Formula::Atom { left, right }
                    }
                    _ => panic!("Expected =, got {:?}", self.peek()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Formula, Term, new_succ, new_sum, new_var, new_zero};
    use crate::parser::Parser;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_parse_zero() {
        let tokens = vec![Token::new(TokenKind::Zero), Token::new(TokenKind::EOF)];
        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();
        assert_eq!(term, Term::Zero);
    }

    #[test]
    fn test_parse_ss0() {
        let tokens = vec![
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::EOF),
        ];
        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();
        let expected = Term::Succ {
            child: Box::new(Term::Succ {
                child: Box::new(Term::Zero),
            }),
        };
        assert_eq!(term, expected);
    }

    #[test]
    fn test_parse_var() {
        let tokens = vec![
            Token::new(TokenKind::Variable('a')),
            Token::new(TokenKind::EOF),
        ];
        let mut parser = Parser::new(tokens);
        let term = parser.parse_term();
        assert_eq!(term, Term::Var { var: 'a' });
    }

    #[test]
    fn test_parse_atom() {
        // S0 = 0
        let tokens = vec![
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Equals),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::EOF),
        ];
        let mut parser = Parser::new(tokens);
        let formula = parser.parse_formula();
        let expected = Formula::Atom {
            left: new_succ(new_zero()),
            right: new_zero(),
        };
        assert_eq!(formula, expected);
    }

    #[test]
    fn test_parse_negation() {
        // ~S0 = 0
        let tokens = vec![
            Token::new(TokenKind::Not),
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Equals),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::EOF),
        ];
        let mut parser = Parser::new(tokens);
        let formula = parser.parse_formula();
        let expected = Formula::Negation {
            child: Box::new(Formula::Atom {
                left: new_succ(new_zero()),
                right: new_zero(),
            }),
        };
        assert_eq!(formula, expected);
    }

    #[test]
    fn test_parse_and() {
        // <S0 = 0 & 0 = 0>
        let tokens = vec![
            Token::new(TokenKind::Lt),
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Equals),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::And),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Equals),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Gt),
            Token::new(TokenKind::EOF),
        ];
        let mut parser = Parser::new(tokens);
        let formula = parser.parse_formula();
        let expected = Formula::And {
            left: Box::new(Formula::Atom {
                left: new_succ(new_zero()),
                right: new_zero(),
            }),
            right: Box::new(Formula::Atom {
                left: new_zero(),
                right: new_zero(),
            }),
        };
        assert_eq!(formula, expected);
    }

    #[test]
    fn test_parse_implies() {
        // <0 = 0 -> S0 = 0>
        let tokens = vec![
            Token::new(TokenKind::Lt),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Equals),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Implies),
            Token::new(TokenKind::Successor),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Equals),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::Gt),
            Token::new(TokenKind::EOF),
        ];
        let mut parser = Parser::new(tokens);
        let formula = parser.parse_formula();
        let expected = Formula::Implies {
            left: Box::new(Formula::Atom {
                left: new_zero(),
                right: new_zero(),
            }),
            right: Box::new(Formula::Atom {
                left: new_succ(new_zero()),
                right: new_zero(),
            }),
        };
        assert_eq!(formula, expected);
    }

    #[test]
    fn test_parse_exists() {
        // Ea: a = 0
        let tokens = vec![
            Token::new(TokenKind::Exists),
            Token::new(TokenKind::Variable('a')),
            Token::new(TokenKind::Colon),
            Token::new(TokenKind::Variable('a')),
            Token::new(TokenKind::Equals),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::EOF),
        ];
        let mut parser = Parser::new(tokens);
        let formula = parser.parse_formula();
        let expected = Formula::Exists {
            var: 'a',
            body: Box::new(Formula::Atom {
                left: new_var('a'),
                right: new_zero(),
            }),
        };
        assert_eq!(formula, expected);
    }

    #[test]
    fn test_parse_forall() {
        // Aa: a = 0
        let tokens = vec![
            Token::new(TokenKind::ForAll),
            Token::new(TokenKind::Variable('a')),
            Token::new(TokenKind::Colon),
            Token::new(TokenKind::Variable('a')),
            Token::new(TokenKind::Equals),
            Token::new(TokenKind::Zero),
            Token::new(TokenKind::EOF),
        ];
        let mut parser = Parser::new(tokens);
        let formula = parser.parse_formula();
        let expected = Formula::ForAll {
            var: 'a',
            body: Box::new(Formula::Atom {
                left: new_var('a'),
                right: new_zero(),
            }),
        };
        assert_eq!(formula, expected);
    }
}
