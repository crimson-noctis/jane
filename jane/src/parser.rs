use crate::ast::Formula;
use crate::ast::Term;
use crate::token::Token;
use crate::token::TokenKind;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {}

fn parse_term(t: &[Token]) -> Term {
    let (token, rest) = t.split_first().unwrap();
    match token.get_kind() {
        TokenKind::Zero => Term::Zero,
        TokenKind::Successor => {
            let child = parse_term(rest);
            Term::Succ {
                child: Box::new(child),
            }
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::Term,
        parser::parse_term,
        token::{Token, TokenKind},
    };

    #[test]
    fn test_parse_token() {
        let tokens = vec![Token::new(TokenKind::Zero)];
        let term = parse_term(&tokens);
        let expected = Term::Zero;

        assert_eq!(term, expected);
    }
}
