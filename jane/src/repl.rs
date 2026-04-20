use std::fmt::Display;
use std::io;
use std::io::Read;
use std::io::Write;

use jane::ast::Choice;
use jane::ast::Formula;
use jane::ast::Term;
use jane::ast::elim_forall;
use jane::ast::intro_axiom;
use jane::ast::intro_conjunction;
use jane::ast::new_succ;
use jane::ast::new_zero;

#[derive(Debug)]
enum Justification {
    Axiom,
    IntroConjection(usize, usize),
    ElimConjunction(usize, Choice),
    // IntroImplies
    // ElimImplies
    // IntroForAll
    ElimForAll(usize, Term),
    // IntroExists
    // ElimExists
    // DeMorgan
    // Contrapositive
    // Premise,
    // Induction,
    // Symmetry,
    // Transivity,
    // CarryOver,
}

impl Display for Justification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Axiom => write!(f, "Axiom"),
            Self::IntroConjection(p_line_num, q_line_num) => {
                write!(f, "Intro ∧ ({}, {})", p_line_num, q_line_num)
            }
            Self::ElimConjunction(line_num, _) => {
                write!(f, "Elim ∧ ({})", line_num)
            }
            Self::ElimForAll(line_number, term) => {
                write!(f, "Elim ∀ at {} with {}", line_number, term)
            }
        }
    }
}

pub struct Repl {
    theorems: Vec<(Formula, Justification)>,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            theorems: Vec::new(),
        }
    }

    pub fn run_examples(&mut self) {
        self.theorems
            .push((intro_axiom(1).unwrap(), Justification::Axiom));

        self.theorems
            .push((intro_axiom(2).unwrap(), Justification::Axiom));

        self.theorems
            .push((intro_axiom(3).unwrap(), Justification::Axiom));

        let new_term = new_succ(new_zero());
        let new_theorem = elim_forall(self.theorems[0].0.clone(), new_term.clone()).unwrap();
        self.theorems
            .push((new_theorem.clone(), Justification::ElimForAll(1, new_term)));
        let new_term = new_zero();
        let new_new_theorem = elim_forall(self.theorems[0].0.clone(), new_term.clone()).unwrap();
        self.theorems.push((
            new_new_theorem.clone(),
            Justification::ElimForAll(1, new_term),
        ));
        let new_theorem = intro_conjunction(new_theorem, new_new_theorem);
        self.theorems
            .push((new_theorem, Justification::IntroConjection(4, 5)));
    }

    pub fn run(&mut self) {
        let mut command = String::new();
        let mut quit = false;
        while !quit {
            print!("> ");
            io::stdout().flush().unwrap();
            command.clear();
            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");
            let command = command.trim();
            match command {
                "quit" | "q" => quit = true,
                "list" | "ls" => self.list_theorems(),
                _ => match (command, command.split_once(' ')) {
                    ("axiom", _) => {
                        println!("There are 5 axioms. (1 - 5)");
                    }
                    (_, Some(("axiom", n))) => {
                        let n: usize = n.parse().expect("Expected n to be a integer");
                        let axiom = intro_axiom(n).expect("Expected n to be in the range 1 - 5");
                        self.theorems.push((axiom, Justification::Axiom));
                        self.list_last_theorem();
                    }
                    _ => panic!(),
                },
            }
        }
    }

    fn print_help() {
        todo!();
    }

    fn list_theorems(&self) {
        let mut width = 0;

        for (i, (theorem, _)) in self.theorems.iter().enumerate() {
            let left = format!("{}: {}", i + 1, theorem);
            width = width.max(left.len());
        }

        for (i, (theorem, justification)) in self.theorems.iter().enumerate() {
            let left = format!("{}: {}", i + 1, theorem);
            println!("{:width$} | {}", left, justification, width = width);
        }

        println!();
    }

    fn list_last_theorem(&self) {
        let mut width = 0;

        for (i, (theorem, _)) in self.theorems.iter().enumerate() {
            let left = format!("{}: {}", i + 1, theorem);
            width = width.max(left.len());
        }

        let Some((i, (theorem, justification))) = self.theorems.iter().enumerate().next_back()
        else {
            return;
        };
        let left = format!("{}: {}", i + 1, theorem);
        println!("{:width$} | {}\n", left, justification, width = width);
    }
}
