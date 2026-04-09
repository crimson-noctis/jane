use std::fmt::Display;

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
            Self::Exists { var, body } => write!(f, "(∀{}:{})", var, body),
            Self::Forall { var, body } => write!(f, "(∃{}:{})", var, body),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Term;

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
}
