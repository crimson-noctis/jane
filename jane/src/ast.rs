use std::{fmt::Display, num::FpCategory};

#[derive(Debug, PartialEq)]
pub enum Term {
    Zero,
    Var { var: char },
    Succ { child: Box<Term> },
    Sum { left: Box<Term>, right: Box<Term> },
    Product { left: Box<Term>, right: Box<Term> },
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "0"),
            Self::Var { var } => write!(f, "{}", var),
            Self::Succ { child } => write!(f, "S{}", child),
            Self::Sum { left, right } => write!(f, "({} + {})", left, right),
            Self::Product { left, right } => write!(f, "({} × {})", left, right),
        }
    }
}

#[inline]
pub fn new_zero() -> Term {
    Term::Zero
}

#[inline]
pub fn new_var(var: char) -> Term {
    Term::Var { var }
}

#[inline]
pub fn new_succ(child: Term) -> Term {
    Term::Succ {
        child: Box::new(child),
    }
}

#[inline]
pub fn new_sum(left: Term, right: Term) -> Term {
    Term::Sum {
        left: Box::new(left),
        right: Box::new(right),
    }
}

#[inline]
pub fn new_product(left: Term, right: Term) -> Term {
    Term::Product {
        left: Box::new(left),
        right: Box::new(right),
    }
}

#[derive(Debug, PartialEq)]
pub enum Formula {
    Atom {
        left: Term,
        right: Term,
    },
    Negation {
        child: Box<Formula>,
    },
    And {
        left: Box<Formula>,
        right: Box<Formula>,
    },
    Or {
        left: Box<Formula>,
        right: Box<Formula>,
    },
    Implies {
        left: Box<Formula>,
        right: Box<Formula>,
    },
    Exists {
        var: char,
        body: Box<Formula>,
    },
    Forall {
        var: char,
        body: Box<Formula>,
    },
}

impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom { left, right } => write!(f, "({} = {})", left, right),
            Self::Negation { child } => write!(f, "¬({})", child),
            Self::And { left, right } => write!(f, "({} ∧ {})", left, right),
            Self::Or { left, right } => write!(f, "({} ∨ {})", left, right),
            Self::Implies { left, right } => write!(f, "({} -> {})", left, right),
            Self::Exists { var, body } => write!(f, "(∀{}: {})", var, body),
            Self::Forall { var, body } => write!(f, "(∃{}: {})", var, body),
        }
    }
}

fn new_atom(left: Term, right: Term) -> Formula {
    Formula::Atom { left, right }
}

fn new_negation(child: Formula) -> Formula {
    Formula::Negation {
        child: Box::new(child),
    }
}

fn new_and(left: Formula, right: Formula) -> Formula {
    Formula::And {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn new_or(left: Formula, right: Formula) -> Formula {
    Formula::Or {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn new_implies(left: Formula, right: Formula) -> Formula {
    Formula::Implies {
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn new_exists(var: char, body: Formula) -> Formula {
    Formula::Exists {
        var,
        body: Box::new(body),
    }
}

fn new_forall(var: char, body: Formula) -> Formula {
    Formula::Forall {
        var,
        body: Box::new(body),
    }
}

fn intro_conjunction(p: Formula, q: Formula) -> Formula {
    new_and(p, q)
}

enum Choice {
    Left,
    Right,
}

fn elim_conjunction(f: Formula, n: Choice) -> Result<Formula, String> {
    if let Formula::And { left, right } = f {
        match n {
            Choice::Left => Ok(*left),
            Choice::Right => Ok(*right),
        }
    } else {
        Err("Elim conjunction: Formula f must be in the form (p ∧ q)".to_string())
    }
}

fn intro_implication() {
    todo!();
}

fn term_equal(a: Term, b: Term) -> bool {
    a == b
}

fn formula_equal(a: Formula, b: Formula) -> bool {
    a == b
}

fn elim_implication(p: Formula, f: Formula) -> Result<Formula, String> {
    if let Formula::Implies { left, right } = f {
        if formula_equal(p, *left) {
            Ok(*right)
        } else {
            Err("Elim implies: Premise does not match antecedent".to_string())
        }
    } else {
        Err("Elim implies: Formula f is not an implication".to_string())
    }
}

fn intro_symmetry(p: Formula) -> Result<Formula, String> {
    if let Formula::Atom { left, right } = p {
        Ok(Formula::Atom { right, left })
    } else {
        Err("Symmetry: Formula must be an atom of the form (p = q)".to_string())
    }
}

fn intro_transitivity(p: Formula, q: Formula) -> Result<Formula, String> {
    if let (
        Formula::Atom {
            left: p_left,
            right: p_right,
        },
        Formula::Atom {
            left: q_left,
            right: q_right,
        },
    ) = (p, q)
    {
        if p_right == q_left {
            Ok(Formula::Atom {
                left: p_left,
                right: q_right,
            })
        } else {
            Err("Transitivity: The RHS of p and the LHS of q are not equal".to_string())
        }
    } else {
        Err("Transivity: Formula p and q must be an atom of the form (p = q)".to_string())
    }
}

fn intro_succ(p: Formula) -> Result<Formula, String> {
    if let Formula::Atom { left, right } = p {
        Ok(Formula::Atom {
            left: Term::Succ {
                child: Box::new(left),
            },
            right: Term::Succ {
                child: Box::new(right),
            },
        })
    } else {
        Err("Fail".to_string())
    }
}

fn elim_succ(p: Formula) -> Result<Formula, String> {
    if let Formula::Atom {
        left: Term::Succ { child: left_child },
        right: Term::Succ { child: right_child },
    } = p
    {
        Ok(Formula::Atom {
            left: *left_child,
            right: *right_child,
        })
    } else {
        Err("Fail".to_string())
    }
}

#[rustfmt::skip]
fn intro_contrapositive(p: Formula) -> Result<Formula, String> {
    if let Formula::Implies { left, right } = p {
        match (*left, *right) {
            // (¬p -> ¬q) becomes (q -> p)
            (Formula::Negation { child: left }, Formula::Negation { child: right }) => Ok(new_implies(*right, *left)),
            // (p -> ¬q) becomes (q -> ¬p)
            (left, Formula::Negation { child: right }) => Ok(new_implies(*right, new_negation(left))),
            // (¬p -> q) becomes (¬q -> p)
            (Formula::Negation { child: left }, right) => Ok(new_implies(new_negation(right), *left)),
            // (p -> q) becomes (¬q -> ¬p)
            (left, right) => Ok(new_implies(new_negation(right), new_negation(left))),
        }
    } else {
        Err("Intro contrapositive: Formula must be an implication".to_string())
    }
}

#[rustfmt::skip]
fn intro_axiom(n: usize) -> Result<Formula, String> {
    let axiom_one = new_forall('a',
                      new_negation(
                        new_atom(
                          new_succ(
                            new_var('a')), 
                          new_zero())));

    let axiom_two = new_forall('a', 
                      new_atom(
                        new_sum(
                          new_var('a'), 
                          new_zero()), 
                        new_var('a')));
    
    let axiom_three = new_forall('a',
                        new_forall('b',
                          new_atom(
                            new_sum(
                              new_var('a'), 
                              new_succ(new_var('b'))),
                            new_succ(
                              new_sum(
                                new_var('a'),
                                new_var('b'))))));
    
    let axiom_four = new_forall('a', 
                       new_atom(
                         new_product(
                           new_var('a'), 
                           new_zero()),
                         new_zero()));
    
    let axiom_five = new_forall('a',
                       new_forall('b',
                         new_atom(
                           new_product(
                             new_var('a'),
                             new_succ(
                               new_var('b'))),
                           new_sum(
                             new_product(
                               new_var('a'),
                               new_var('b')),
                             new_var('a')))));
        
    match n {
        1 => Ok(axiom_one),
        2 => Ok(axiom_two),
        3 => Ok(axiom_three),
        4 => Ok(axiom_four),
        5 => Ok(axiom_five),
        _ => Err("Intro axiom: n is out of range".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Formula;
    use crate::ast::Term;
    use crate::ast::elim_succ;
    use crate::ast::intro_contrapositive;
    use crate::ast::intro_succ;
    use crate::ast::intro_symmetry;
    use crate::ast::intro_transitivity;
    use crate::ast::new_atom;
    use crate::ast::new_implies;
    use crate::ast::new_negation;
    use crate::ast::new_var;

    #[test]
    fn test_print_zero() {
        let zero_term = Term::Zero;

        let expected = "0".to_string();
        let actual = zero_term.to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_print_succ_zero() {
        let succ_zero_term = Term::Succ {
            child: Box::new(Term::Zero),
        };

        let expected = "S0".to_string();
        let actual = succ_zero_term.to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_print_var() {
        let var_term = Term::Var { var: 'j' };

        let expected = "j".to_string();
        let actual = var_term.to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_print_succ_var() {
        let succ_var_term = Term::Succ {
            child: Box::new(Term::Var { var: 'j' }),
        };

        let expected = "Sj".to_string();
        let actual = succ_var_term.to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_print_sum_of_zero_and_one() {
        let sum_term = Term::Sum {
            left: Box::new(Term::Zero),
            right: Box::new(Term::Succ {
                child: Box::new(Term::Zero),
            }),
        };

        let expected = "(0 + S0)".to_string();
        let actual = sum_term.to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_print_product_of_one_and_one() {
        let prod_term = Term::Product {
            left: Box::new(Term::Succ {
                child: Box::new(Term::Zero),
            }),

            right: Box::new(Term::Succ {
                child: Box::new(Term::Zero),
            }),
        };

        let expected = "(S0 × S0)".to_string();
        let actual = prod_term.to_string();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_equality_of_formulas() {
        let form_one = Formula::Atom {
            left: Term::Zero,
            right: Term::Succ {
                child: Box::new(Term::Zero),
            },
        };

        let form_two = Formula::Atom {
            left: Term::Zero,
            right: Term::Zero,
        };

        let expected = false;
        let actual = form_one == form_two;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intro_transitivity_success() {
        // (a = Sb)
        let form_one = Formula::Atom {
            left: Term::Var { var: 'a' },
            right: Term::Succ {
                child: Box::new(Term::Var { var: 'b' }),
            },
        };

        // (Sb = c)
        let form_two = Formula::Atom {
            left: Term::Succ {
                child: Box::new(Term::Var { var: 'b' }),
            },
            right: Term::Var { var: 'c' },
        };

        let expected = Ok(Formula::Atom {
            left: Term::Var { var: 'a' },
            right: Term::Var { var: 'c' },
        });

        // (a = c)
        let actual = intro_transitivity(form_one, form_two);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intro_transitivity_fail() {
        // (a = Sb)
        let form_one = Formula::Atom {
            left: Term::Var { var: 'a' },
            right: Term::Succ {
                child: Box::new(Term::Var { var: 'b' }),
            },
        };

        // (b = c)
        let form_two = Formula::Atom {
            left: Term::Var { var: 'b' },
            right: Term::Var { var: 'c' },
        };

        let actual = intro_transitivity(form_one, form_two);

        assert!(actual.is_err());
    }

    #[test]
    fn test_intro_succ_success() {
        // (a = Sb)
        let form_one = Formula::Atom {
            left: Term::Var { var: 'a' },
            right: Term::Succ {
                child: Box::new(Term::Var { var: 'b' }),
            },
        };

        // (Sa = SSb)
        let expected = Ok(Formula::Atom {
            left: Term::Succ {
                child: Box::new(Term::Var { var: 'a' }),
            },
            right: Term::Succ {
                child: Box::new(Term::Succ {
                    child: Box::new(Term::Var { var: 'b' }),
                }),
            },
        });

        let actual = intro_succ(form_one);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_elim_succ_success() {
        // (Sa = SSb)
        let form_one = Formula::Atom {
            left: Term::Succ {
                child: Box::new(Term::Var { var: 'a' }),
            },
            right: Term::Succ {
                child: Box::new(Term::Succ {
                    child: Box::new(Term::Var { var: 'b' }),
                }),
            },
        };

        // (a = Sb)
        let expected = Ok(Formula::Atom {
            left: Term::Var { var: 'a' },
            right: Term::Succ {
                child: Box::new(Term::Var { var: 'b' }),
            },
        });

        let actual = elim_succ(form_one);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_elim_succ_fail() {
        // (a = Sb)
        let form_one = Formula::Atom {
            left: Term::Var { var: 'a' },
            right: Term::Succ {
                child: Box::new(Term::Var { var: 'b' }),
            },
        };

        let actual = elim_succ(form_one);

        assert!(actual.is_err());
    }

    #[test]
    fn test_intro_contrapositive_one_success() {
        let form_one = new_implies(
            new_atom(new_var('a'), new_var('a')),
            new_atom(new_var('b'), new_var('b')),
        );

        let expected = Ok(new_implies(
            new_negation(new_atom(new_var('b'), new_var('b'))),
            new_negation(new_atom(new_var('a'), new_var('a'))),
        ));

        let actual = intro_contrapositive(form_one);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intro_contrapositive_two_success() {
        let form_one = new_implies(
            new_negation(new_atom(new_var('b'), new_var('b'))),
            new_negation(new_atom(new_var('a'), new_var('a'))),
        );

        let expected = Ok(new_implies(
            new_atom(new_var('a'), new_var('a')),
            new_atom(new_var('b'), new_var('b')),
        ));

        let actual = intro_contrapositive(form_one);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intro_contrapositive_three_success() {
        let form_one = new_implies(
            new_atom(new_var('b'), new_var('b')),
            new_negation(new_atom(new_var('a'), new_var('a'))),
        );

        let expected = Ok(new_implies(
            new_atom(new_var('a'), new_var('a')),
            new_negation(new_atom(new_var('b'), new_var('b'))),
        ));

        let actual = intro_contrapositive(form_one);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intro_contrapositive_four_success() {
        let form_one = new_implies(
            new_negation(new_atom(new_var('b'), new_var('b'))),
            new_atom(new_var('a'), new_var('a')),
        );

        let expected = Ok(new_implies(
            new_negation(new_atom(new_var('a'), new_var('a'))),
            new_atom(new_var('b'), new_var('b')),
        ));

        let actual = intro_contrapositive(form_one);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_intro_contrapositive_success_fail() {}
}
