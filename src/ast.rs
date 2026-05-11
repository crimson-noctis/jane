use std::{fmt::Display, ops};

#[derive(Clone, Debug, PartialEq)]
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

impl ops::Add<Term> for Term {
    type Output = Term;

    fn add(self, rhs: Term) -> Self::Output {
        Term::Sum {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl ops::Mul<Term> for Term {
    type Output = Term;

    fn mul(self, rhs: Term) -> Self::Output {
        Term::Product {
            left: Box::new(self),
            right: Box::new(rhs),
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

#[derive(Clone, Debug, PartialEq)]
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
    ForAll {
        var: char,
        body: Box<Formula>,
    },
}

impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom { left, right } => write!(f, "({} = {})", left, right),
            Self::Negation { child } => write!(f, "¬{}", child),
            Self::And { left, right } => write!(f, "({} ∧ {})", left, right),
            Self::Or { left, right } => write!(f, "({} ∨ {})", left, right),
            Self::Implies { left, right } => write!(f, "({} -> {})", left, right),
            Self::ForAll { var, body } => write!(f, "∀{}: {}", var, body),
            Self::Exists { var, body } => write!(f, "∃{}: {}", var, body),
        }
    }
}

#[inline]
pub fn new_atom(left: Term, right: Term) -> Formula {
    Formula::Atom { left, right }
}

#[inline]
pub fn new_negation(child: Formula) -> Formula {
    Formula::Negation {
        child: Box::new(child),
    }
}

#[inline]
pub fn new_and(left: Formula, right: Formula) -> Formula {
    Formula::And {
        left: Box::new(left),
        right: Box::new(right),
    }
}

#[inline]
pub fn new_or(left: Formula, right: Formula) -> Formula {
    Formula::Or {
        left: Box::new(left),
        right: Box::new(right),
    }
}

#[inline]
pub fn new_implies(left: Formula, right: Formula) -> Formula {
    Formula::Implies {
        left: Box::new(left),
        right: Box::new(right),
    }
}

#[inline]
pub fn new_exists(var: char, body: Formula) -> Formula {
    Formula::Exists {
        var,
        body: Box::new(body),
    }
}

#[inline]
pub fn new_forall(var: char, body: Formula) -> Formula {
    Formula::ForAll {
        var,
        body: Box::new(body),
    }
}

pub fn intro_conjunction(p: Formula, q: Formula) -> Formula {
    new_and(p, q)
}

#[derive(Debug, Clone, Copy)]
pub enum Choice {
    Left,
    Right,
}

pub fn elim_conjunction(f: Formula, n: Choice) -> Result<Formula, String> {
    if let Formula::And { left, right } = f {
        match n {
            Choice::Left => Ok(*left),
            Choice::Right => Ok(*right),
        }
    } else {
        Err("Elim conjunction: Formula f must be in the form (p ∧ q)".to_string())
    }
}

pub fn intro_implication(p: Formula, q: Formula) -> Formula {
    new_implies(p, q)
}

fn term_equal(a: Term, b: Term) -> bool {
    a == b
}

fn formula_equal(a: Formula, b: Formula) -> bool {
    a == b
}

pub fn elim_implication(p: Formula, f: Formula) -> Result<Formula, String> {
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

pub fn intro_symmetry(p: Formula) -> Result<Formula, String> {
    if let Formula::Atom { left, right } = p {
        Ok(Formula::Atom {
            left: right,
            right: left,
        })
    } else {
        Err("Symmetry: Formula must be an atom of the form (p = q)".to_string())
    }
}

pub fn intro_transitivity(p: Formula, q: Formula) -> Result<Formula, String> {
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

pub fn intro_succ(p: Formula) -> Result<Formula, String> {
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

pub fn elim_succ(p: Formula) -> Result<Formula, String> {
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

pub fn intro_contrapositive(p: Formula) -> Result<Formula, String> {
    if let Formula::Implies { left, right } = p {
        Ok(new_implies(invert(*right), invert(*left)))
    } else {
        Err("Intro contrapositive: Formula must be an implication".to_string())
    }
}

// Helper function that returns ~p when passed p and p when passed ~p
fn invert(p: Formula) -> Formula {
    if let Formula::Negation { child: inner } = p {
        *inner
    } else {
        new_negation(p)
    }
}

pub fn intro_de_morgan(mut p: Formula) -> Result<Formula, String> {
    let mut was_inverted = false;
    if let Formula::Negation { child: inner } = p {
        p = *inner;
        was_inverted = true;
    }

    p = match p {
        Formula::And { left, right } => new_or(invert(*left), invert(*right)),
        Formula::Or { left, right } => new_and(invert(*left), invert(*right)),
        _ => return Err("Fail.".to_string()),
    };

    if !was_inverted {
        p = new_negation(p)
    }
    Ok(p)
}

pub fn intro_material_implication(p: Formula) -> Result<Formula, String> {
    if let Formula::Or { left, right } = p {
        Ok(new_implies(invert(*left), *right))
    } else if let Formula::Implies { left, right } = p {
        Ok(new_or(invert(*left), *right))
    } else {
        Err("err".to_string())
    }
}

// `p` is the base case and `q` is the inductive step
pub fn intro_induction(p: Formula, q: Formula) -> Result<Formula, String> {
    let Formula::ForAll { var, body } = q else {
        return Err("Formula s must be of the form ∀a: ⟨p ⊃ q⟩".to_string());
    };
    let Formula::Implies { left, right } = *body else {
        return Err("Body must be an implication".to_string());
    };
    Ok(new_forall(var, *left))
}

#[rustfmt::skip]
pub fn intro_axiom(n: usize) -> Result<Formula, String> {
    // ∀a: ¬(Sa = 0)
    let axiom_one = new_forall('a',
                      new_negation(
                        new_atom(
                          new_succ(
                            new_var('a')), 
                          new_zero())));

    // ∀a: (a + 0) = a
    let axiom_two = new_forall('a', 
                      new_atom(
                        new_sum(
                          new_var('a'), 
                          new_zero()), 
                        new_var('a')));
    
    // ∀a: ∀b: (a + Sb) = S(a + b)
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
    
    // ∀a: (a × 0) = 0
    let axiom_four = new_forall('a', 
                       new_atom(
                         new_product(
                           new_var('a'), 
                           new_zero()),
                         new_zero()));
    
    // ∀a: ∀b: (a × Sb) = ((a × b) + a)
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

pub fn list_axioms() -> Vec<String> {
    let mut vec = Vec::new();
    for i in 1..=5 {
        let axiom = intro_axiom(i).expect("This should be a valid axiom index");
        vec.push(format!("{}: {}", i, axiom));
    }
    vec
}

fn replace_term_with_var_in_term(t: Term, v: char, p: Term) -> Term {
    // TODO: Check that variables in term are free.
    if t == p {
        new_var(v)
    } else {
        match p {
            Term::Zero => Term::Zero,
            Term::Var { var } => Term::Var { var },
            Term::Succ { child } => new_succ(replace_term_with_var_in_term(t, v, *child)),
            Term::Sum { left, right } => new_sum(
                replace_term_with_var_in_term(t.clone(), v, *left),
                replace_term_with_var_in_term(t, v, *right),
            ),
            Term::Product { left, right } => new_product(
                replace_term_with_var_in_term(t.clone(), v, *left),
                replace_term_with_var_in_term(t, v, *right),
            ),
        }
    }
}

fn replace_term_with_var_in_formula(t: Term, v: char, p: Formula) -> Formula {
    match p {
        Formula::Atom { left, right } => new_atom(
            replace_term_with_var_in_term(t.clone(), v, left),
            replace_term_with_var_in_term(t.clone(), v, right),
        ),
        Formula::Negation { child } => new_negation(replace_term_with_var_in_formula(t, v, *child)),
        Formula::And { left, right } => new_and(
            replace_term_with_var_in_formula(t.clone(), v, *left),
            replace_term_with_var_in_formula(t.clone(), v, *right),
        ),
        Formula::Or { left, right } => new_or(
            replace_term_with_var_in_formula(t.clone(), v, *left),
            replace_term_with_var_in_formula(t.clone(), v, *right),
        ),
        Formula::Implies { left, right } => new_implies(
            replace_term_with_var_in_formula(t.clone(), v, *left),
            replace_term_with_var_in_formula(t.clone(), v, *right),
        ),
        Formula::Exists { var, body } => {
            new_exists(var, replace_term_with_var_in_formula(t, v, *body))
        }
        Formula::ForAll { var, body } => {
            new_forall(var, replace_term_with_var_in_formula(t, v, *body))
        }
    }
}

fn is_var_free_in_formula(v: char, f: Formula) -> bool {
    match f {
        Formula::Atom { .. } => true,
        Formula::Negation { child } => is_var_free_in_formula(v, *child),
        Formula::And { left, right }
        | Formula::Or { left, right }
        | Formula::Implies { left, right } => {
            is_var_free_in_formula(v, *left) && is_var_free_in_formula(v, *right)
        }
        Formula::Exists { var, body } | Formula::ForAll { var, body } => v != var,
    }
}

fn get_vars_in_term(t: Term) -> Vec<char> {
    match t {
        Term::Zero => Vec::new(),
        Term::Var { var } => vec![var],
        Term::Succ { child } => get_vars_in_term(*child),
        Term::Sum { left, right } | Term::Product { left, right } => {
            let mut a = get_vars_in_term(*left);
            let b = get_vars_in_term(*right);
            a.extend(b);
            a
        }
    }
}

fn is_var_in_term(v: char, t: Term) -> bool {
    match t {
        Term::Zero => false,
        Term::Var { var } => var == v,
        Term::Succ { child } => is_var_in_term(v, *child),
        Term::Sum { left, right } | Term::Product { left, right } => {
            is_var_in_term(v, *left) || is_var_in_term(v, *right)
        }
    }
}

fn is_var_in_formula(v: char, f: Formula) -> bool {
    match f {
        Formula::Negation { child } => is_var_in_formula(v, *child),
        Formula::Atom { left, right } => is_var_in_term(v, left) || is_var_in_term(v, right),
        Formula::And { left, right }
        | Formula::Or { left, right }
        | Formula::Implies { left, right } => {
            is_var_in_formula(v, *left) || is_var_in_formula(v, *right)
        }
        Formula::Exists { var, body } => {
            if var == v {
                return true;
            }
            is_var_in_formula(v, *body)
        }
        Formula::ForAll { var, body } => {
            if var == v {
                return true;
            }
            is_var_in_formula(v, *body)
        }
    }
}

fn intro_exists(t: Term, v: char, p: Formula) -> Result<Formula, String> {
    // Check that variables in term t are free in p.
    let vars = get_vars_in_term(t.clone());
    for var in vars {
        if !is_var_free_in_formula(v, p.clone()) {
            return Err(format!("Var {} is not free", v));
        }
    }
    // TODO: Check that variable v does not appear in p
    Ok(new_exists(v, replace_term_with_var_in_formula(t, v, p)))
}

pub fn elim_exists(p: Formula) -> Formula {
    todo!();
}

pub fn intro_forall(p: Formula) -> Formula {
    todo!();
}

fn replace_var_in_term(p: Term, from: char, to: Term) -> Term {
    match p {
        Term::Zero => new_zero(),
        Term::Var { var } => {
            if var == from {
                to.clone()
            } else {
                new_var(var)
            }
        }
        Term::Succ { child } => new_succ(replace_var_in_term(*child, from, to)),
        Term::Sum { left, right } => new_sum(
            replace_var_in_term(*left, from, to.clone()),
            replace_var_in_term(*right, from, to),
        ),
        Term::Product { left, right } => new_product(
            replace_var_in_term(*left, from, to.clone()),
            replace_var_in_term(*right, from, to),
        ),
    }
}

fn replace_var_in_formula(p: Formula, from: char, to: Term) -> Formula {
    match p {
        Formula::Atom { left, right } => new_atom(
            replace_var_in_term(left, from, to.clone()),
            replace_var_in_term(right, from, to),
        ),
        Formula::Negation { child } => new_negation(replace_var_in_formula(*child, from, to)),
        Formula::And { left, right } => new_and(
            replace_var_in_formula(*left, from, to.clone()),
            replace_var_in_formula(*right, from, to),
        ),
        Formula::Or { left, right } => new_or(
            replace_var_in_formula(*left, from, to.clone()),
            replace_var_in_formula(*right, from, to),
        ),
        Formula::Implies { left, right } => new_implies(
            replace_var_in_formula(*left, from, to.clone()),
            replace_var_in_formula(*right, from, to),
        ),
        Formula::Exists { var, body } => new_exists(var, replace_var_in_formula(*body, from, to)),
        Formula::ForAll { var, body } => new_forall(var, replace_var_in_formula(*body, from, to)),
    }
}

pub fn elim_forall(p: Formula, t: Term) -> Result<Formula, String> {
    if let Formula::ForAll { var, body } = p {
        Ok(replace_var_in_formula(*body, var, t))
    } else {
        Err("Error.".to_string())
    }
}

pub fn intro_interchange(mut p: Formula) -> Result<Formula, String> {
    let mut was_inverted = false;
    if let Formula::Negation { child: inner } = p {
        p = *inner;
        was_inverted = true;
    }

    p = match p {
        Formula::ForAll { var, body } => new_exists(var, invert(*body)),
        Formula::Exists { var, body } => new_forall(var, invert(*body)),
        _ => return Err("Fail.".to_string()),
    };

    if !was_inverted {
        p = new_negation(p)
    }
    Ok(p)
}

fn new_self_equal_atom(ch: char) -> Formula {
    new_atom(new_var(ch), new_var(ch))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod printing {
        use super::*;

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
        fn test_print_sum() {
            let sum_term = new_sum(new_var('a'), new_var('b'));
            let expected = "(a + b)".to_string();
            let actual = sum_term.to_string();

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_print_product() {
            let product_term = new_product(new_var('a'), new_var('b'));
            let expected = "(a × b)".to_string();
            let actual = product_term.to_string();

            assert_eq!(actual, expected);
        }
        #[test]
        fn test_print_sum_of_zero_and_one() {
            let sum_term = new_sum(new_zero(), new_succ(new_zero()));

            let expected = "(0 + S0)".to_string();
            let actual = sum_term.to_string();

            assert_eq!(actual, expected)
        }

        #[test]
        fn test_print_product_of_one_and_one() {
            let prod_term = new_product(new_succ(new_zero()), new_succ(new_zero()));

            let expected = "(S0 × S0)".to_string();
            let actual = prod_term.to_string();

            assert_eq!(actual, expected)
        }
    }

    mod transistivity {
        use crate::ast::*;

        fn test_intro_transitivity_success() {
            // (a = Sb)
            let form_one = new_atom(new_var('a'), new_succ(new_var('b')));

            // (Sb = c)
            let form_two = new_atom(new_succ(new_var('b')), new_var('c'));

            let expected = Ok(new_atom(new_var('a'), new_var('c')));

            // (a = c)
            let actual = intro_transitivity(form_one, form_two);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_transitivity_fail() {
            // (a = Sb)
            let form_one = new_atom(new_var('a'), new_succ(new_var('b')));

            // (b = c)
            let form_two = new_atom(new_var('b'), new_var('c'));

            let actual = intro_transitivity(form_one, form_two);

            assert!(actual.is_err());
        }
    }

    mod succ {
        use crate::ast::*;

        #[test]
        fn test_intro_succ_success() {
            // (a = Sb)
            let form_one = new_atom(new_var('a'), new_succ(new_var('b')));

            // (Sa = SSb)
            let expected = Ok(new_atom(
                new_succ(new_var('a')),
                new_succ(new_succ(new_var('b'))),
            ));

            let actual = intro_succ(form_one);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_elim_succ_success() {
            // (Sa = SSb)
            let form_one = new_atom(new_succ(new_var('a')), new_succ(new_succ(new_var('b'))));

            // (a = Sb)
            let expected = Ok(new_atom(new_var('a'), new_succ(new_var('b'))));

            let actual = elim_succ(form_one);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_elim_succ_fail() {
            // (a = Sb)
            let form_one = new_atom(new_var('a'), new_succ(new_var('b')));

            let actual = elim_succ(form_one);

            assert!(actual.is_err());
        }
    }

    mod contrapositive {
        use crate::ast::*;

        #[test]
        fn test_intro_contrapositive_one() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // x ->  y
            let formula = new_implies(atom_x.clone(), atom_y.clone());

            // ~y -> ~x
            let expected = Ok(new_implies(new_negation(atom_y), new_negation(atom_x)));

            let actual = intro_contrapositive(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_contrapositive_two() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // ~x ->  y
            let formula = new_implies(new_negation(atom_x.clone()), atom_y.clone());

            // ~y ->  x
            let expected = Ok(new_implies(new_negation(atom_y), atom_x));

            let actual = intro_contrapositive(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_contrapositive_three() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // x -> ~y
            let formula = new_implies(atom_x.clone(), new_negation(atom_y.clone()));

            // y -> ~x
            let expected = Ok(new_implies(atom_y, new_negation(atom_x)));

            let actual = intro_contrapositive(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_contrapositive_four() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // ~x -> ~y
            let formula = new_implies(new_negation(atom_x.clone()), new_negation(atom_y.clone()));

            // y ->  x
            let expected = Ok(new_implies(atom_y, atom_x));

            let actual = intro_contrapositive(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_contrapositive_fail() {
            let atom_a = new_self_equal_atom('a');
            let atom_b = new_self_equal_atom('b');
            let formula = new_and(atom_b, new_negation(atom_a));

            let actual = intro_contrapositive(formula);

            assert!(actual.is_err());
        }
    }

    mod material_implication {
        use crate::ast::*;

        #[test]
        fn test_material_implication_one() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // x | y
            let formula = new_or(atom_x.clone(), atom_y.clone());
            // ~x -> y
            let expected = Ok(new_implies(new_negation(atom_x), atom_y));
            let actual = intro_material_implication(formula);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_material_implication_two() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // ~x | y
            let formula = new_or(new_negation(atom_x.clone()), atom_y.clone());
            // x -> y
            let expected = Ok(new_implies(atom_x, atom_y));
            let actual = intro_material_implication(formula);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_material_implication_three() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // x | ~y
            let formula = new_or(atom_x.clone(), new_negation(atom_y.clone()));
            // ~x -> ~y
            let expected = Ok(new_implies(new_negation(atom_x), new_negation(atom_y)));
            let actual = intro_material_implication(formula);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_material_implication_four() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // ~x | ~y
            let formula = new_or(new_negation(atom_x.clone()), new_negation(atom_y.clone()));
            // x -> ~y
            let expected = Ok(new_implies(atom_x, new_negation(atom_y)));
            let actual = intro_material_implication(formula);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_material_implication_reverse_one() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // ~x -> y
            let formula = new_implies(new_negation(atom_x.clone()), atom_y.clone());
            // x | y
            let expected = Ok(new_or(atom_x, atom_y));
            let actual = intro_material_implication(formula);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_material_implication_reverse_two() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // x -> y
            let formula = new_implies(atom_x.clone(), atom_y.clone());
            // ~x | y
            let expected = Ok(new_or(new_negation(atom_x), atom_y));
            let actual = intro_material_implication(formula);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_material_implication_reverse_three() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // ~x -> ~y
            let formula = new_implies(new_negation(atom_x.clone()), new_negation(atom_y.clone()));
            // x | ~y
            let expected = Ok(new_or(atom_x, new_negation(atom_y)));
            let actual = intro_material_implication(formula);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_material_implication_reverse_four() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // x -> ~y
            let formula = new_implies(atom_x.clone(), new_negation(atom_y.clone()));
            // ~x | ~y
            let expected = Ok(new_or(new_negation(atom_x), new_negation(atom_y)));
            let actual = intro_material_implication(formula);
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_material_implication_fail() {
            let formula = new_self_equal_atom('x');
            let actual = intro_material_implication(formula);
            assert!(actual.is_err());
        }
    }

    mod demorgan {
        use crate::ast::*;

        #[test]
        fn test_intro_de_morgan_one() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');

            // ~x ∧ ~y
            let formula = new_and(new_negation(atom_x.clone()), new_negation(atom_y.clone()));
            // ~(x ∨ y)
            let expected = Ok(new_negation(new_or(atom_x, atom_y)));

            let actual = intro_de_morgan(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_de_morgan_two() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');

            // x ∧ ~y
            let formula = new_and(atom_x.clone(), new_negation(atom_y.clone()));
            // ~(~x ∨ y)
            let expected = Ok(new_negation(new_or(new_negation(atom_x), atom_y)));

            let actual = intro_de_morgan(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_de_morgan_three() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');

            // ~x ∧ y
            let formula = new_and(new_negation(atom_x.clone()), atom_y.clone());
            // ~(x ∨ ~y)
            let expected = Ok(new_negation(new_or(atom_x, new_negation(atom_y))));

            let actual = intro_de_morgan(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_de_morgan_four() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');

            // x ∧ y
            let formula = new_and(atom_x.clone(), atom_y.clone());
            // ~(~x ∨ ~y)
            let expected = Ok(new_negation(new_or(
                new_negation(atom_x),
                new_negation(atom_y),
            )));

            let actual = intro_de_morgan(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_de_morgan_reverse_one() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');

            // ~(x ∨ y)
            let formula = new_negation(new_or(atom_x.clone(), atom_y.clone()));
            // ~x ∧ ~y
            let expected = Ok(new_and(
                new_negation(atom_x.clone()),
                new_negation(atom_y.clone()),
            ));

            let actual = intro_de_morgan(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_de_morgan_reverse_two() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');

            // ~(~x ∨ y)
            let formula = new_negation(new_or(new_negation(atom_x.clone()), atom_y.clone()));
            // x ∧ ~y
            let expected = Ok(new_and(atom_x, new_negation(atom_y)));

            let actual = intro_de_morgan(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_de_morgan_reverse_three() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');

            // ~(x ∨ ~y)
            let formula = new_negation(new_or(atom_x.clone(), new_negation(atom_y.clone())));
            // ~x ∧ y
            let expected = Ok(new_and(new_negation(atom_x), atom_y));

            let actual = intro_de_morgan(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_de_morgan_reverse_four() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');

            // ~(~x ∨ ~y)
            let formula = new_negation(new_or(
                new_negation(atom_x.clone()),
                new_negation(atom_y.clone()),
            ));
            // x ∧ y
            let expected = Ok(new_and(atom_x, atom_y));

            let actual = intro_de_morgan(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_intro_de_morgan_fail() {
            let atom_x = new_self_equal_atom('x');
            let atom_y = new_self_equal_atom('y');
            // x -> y
            let formula = new_implies(atom_x, atom_y);

            let actual = intro_de_morgan(formula);

            assert!(actual.is_err());
        }
    }

    mod interchange {
        use crate::ast::*;

        #[test]
        fn test_interchange_not_exists_to_forall_not_success() {
            let formula = new_negation(new_exists('a', new_self_equal_atom('a')));
            let actual = intro_interchange(formula);

            let expected = Ok(new_forall('a', new_negation(new_self_equal_atom('a'))));

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_interchange_not_forall_to_exists_not_success() {
            let formula = new_negation(new_forall('a', new_self_equal_atom('a')));
            let actual = intro_interchange(formula);

            let expected = Ok(new_exists('a', new_negation(new_self_equal_atom('a'))));

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_interchange_exists_not_to_not_forall_success() {
            let formula = new_exists('a', new_negation(new_self_equal_atom('a')));
            let actual = intro_interchange(formula);

            let expected = Ok(new_negation(new_forall('a', new_self_equal_atom('a'))));

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_interchange_forall_not_to_not_exists_success() {
            let formula = new_forall('a', new_negation(new_self_equal_atom('a')));
            let actual = intro_interchange(formula);

            let expected = Ok(new_negation(new_exists('a', new_self_equal_atom('a'))));

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_interchange_fail() {
            let formula = new_self_equal_atom('a');
            let actual = intro_interchange(formula);
            assert!(actual.is_err())
        }
    }

    mod exists {
        use super::*;

        #[test]
        fn test_intro_exists() {
            // Va: -Sa=O
            let axiom = intro_axiom(1).unwrap();
            // Eb: Va: -Sa=b
            let expected = Ok(new_exists(
                'b',
                new_forall(
                    'a',
                    new_negation(new_atom(new_succ(new_var('a')), new_var('b'))),
                ),
            ));
            let actual = intro_exists(Term::Zero, 'b', axiom);
            assert_eq!(actual, expected);
        }
    }

    mod other {
        use crate::ast::*;

        #[test]
        fn test_elim_forall_success() {
            // ∀a:∀b:(a + Sb) = S(a + b)
            let formula = new_forall(
                'a',
                new_forall(
                    'b',
                    new_atom(
                        new_sum(new_var('a'), new_succ(new_var('b'))),
                        new_succ(new_sum(new_var('a'), new_var('b'))),
                    ),
                ),
            );

            // 0
            let term = Term::Zero;

            // ∀b:(0 + Sb) = S(0 + b)
            let expected = Ok(new_forall(
                'b',
                new_atom(
                    new_sum(new_zero(), new_succ(new_var('b'))),
                    new_succ(new_sum(new_zero(), new_var('b'))),
                ),
            ));

            let actual = elim_forall(formula, term);

            println!("{}", actual.clone().unwrap());
            println!("{}", expected.clone().unwrap());
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_elim_forall_two_success() {
            // ∀b:(0 + Sb) = S(0 + b)
            let formula = new_forall(
                'b',
                new_atom(
                    new_sum(new_zero(), new_succ(new_var('b'))),
                    new_succ(new_sum(new_zero(), new_var('b'))),
                ),
            );

            // (b + c)
            let term = new_var('b') + new_var('c');

            // (0 + S(b + c)) = S(0 + (b + c))
            let expected = Ok(new_atom(
                new_sum(new_zero(), new_succ(new_sum(new_var('b'), new_var('c')))),
                new_succ(new_sum(new_zero(), new_sum(new_var('b'), new_var('c')))),
            ));

            let actual = elim_forall(formula, term);

            println!("{}", actual.clone().unwrap());
            println!("{}", expected.clone().unwrap());
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_specification_restriction() {
            // The term which replaces u must not contain any variable that is quantified in x.
            // todo!();
        }

        #[test]
        fn test_generalization_restriction() {
            // Restriction: No generalization is allowed in a subproof on any
            // variable which appeared free in the subproof's premise.
            // todo!();
        }

        #[test]
        fn test_intro_symmetry() {
            // 0 = S0
            let formula = new_atom(new_zero(), new_succ(new_zero()));
            // S0 = 0
            let expected = Ok(new_atom(new_succ(new_zero()), new_zero()));

            let actual = intro_symmetry(formula);

            assert_eq!(actual, expected);
        }

        #[test]
        fn test_induction() {
            // Ac: (c + 0) = (0 + c)
            let formula_one = new_forall(
                'c',
                new_atom(
                    new_sum(new_var('c'), new_zero()),
                    new_sum(new_zero(), new_var('c')),
                ),
            );

            // ∀d: (Vc: (c + d) = (d + c) -> Vc: (c + Sd) = (Sd + c))
            let formula_two = new_forall(
                'd',
                new_implies(
                    new_forall(
                        'c',
                        new_atom(
                            new_sum(new_var('c'), new_var('d')),
                            new_sum(new_var('d'), new_var('c')),
                        ),
                    ),
                    new_forall(
                        'c',
                        new_atom(
                            new_sum(new_var('c'), new_succ(new_var('d'))),
                            new_sum(new_succ(new_var('d')), new_var('c')),
                        ),
                    ),
                ),
            );

            // ∀d: (Vc: (c + d) = (d + c))
            let expected = Ok(new_forall(
                'd',
                new_forall(
                    'c',
                    new_atom(
                        new_sum(new_var('c'), new_var('d')),
                        new_sum(new_var('d'), new_var('c')),
                    ),
                ),
            ));

            let actual = intro_induction(formula_one, formula_two);

            assert_eq!(actual, expected);
        }
    }
}
