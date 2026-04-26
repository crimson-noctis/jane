use std::fmt::Display;
use std::io;
use std::io::Write;

use jane::ast::Choice;
use jane::ast::Formula;
use jane::ast::Term;
use jane::ast::elim_forall;
use jane::ast::intro_axiom;
use jane::ast::intro_conjunction;
use jane::ast::list_axioms;
use jane::ast::new_succ;
use jane::ast::new_zero;
use jane::lexer::Lexer;
use jane::parser::Parser;

// TODO: Undo
// TODO: Help
// TODO: Tactics
// TODO: Readline support

#[derive(Debug)]
enum Justification {
    Axiom,
    IntroConjunction(usize, usize),
    ElimConjunction(usize, Choice),
    IntroImplies,
    ElimImplies(usize, usize),
    IntroForAll,
    ElimForAll(usize, Term),
    IntroExists,
    ElimExists,
    IntroSucc(usize),
    ElimSucc(usize),
    DeMorgan(usize),
    Contrapositive(usize),
    Interchange(usize),
    Premise,
    Induction(usize, usize),
    Symmetry(usize),
    Transivity(usize, usize),
    CarryOver(usize),
}

impl Display for Justification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Axiom => write!(f, "Axiom"),
            Self::Premise => write!(f, "Premise"),
            Self::IntroConjunction(p_line_num, q_line_num) => {
                write!(f, "Intro ∧ ({}, {})", p_line_num, q_line_num)
            }
            Self::ElimConjunction(line_num, _) => {
                write!(f, "Elim ∧ ({})", line_num)
            }
            Self::ElimForAll(line_number, term) => {
                write!(f, "Elim ∀ at {} with {}", line_number, term)
            }
            _ => todo!(),
        }
    }
}

pub struct Theorem {
    body: Formula,
    reason: Justification,
}

impl Theorem {
    fn new(body: Formula, reason: Justification) -> Self {
        Self { body, reason }
    }
}

fn parse_str_to_formula(f: &str) -> Formula {
    let mut lexer = Lexer::new(f.to_string());
    lexer.tokenize().unwrap();
    let tokens = lexer.tokens();

    let mut parser = Parser::new(tokens.clone());

    let formula = parser.parse_formula();
    formula
}

pub struct Repl {
    theorems: Vec<Theorem>,
    indentation_level: usize,
    should_quit: bool,
    max_width: usize,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            theorems: Vec::new(),
            indentation_level: 0,
            should_quit: false,
            max_width: 0,
        }
    }

    fn push_theorem(&mut self, t: Theorem) {
        let theorem_count = self.theorems.len();
        let i = theorem_count + 1;

        let left = format!("{}: {}", i, t.body);
        self.max_width = self.max_width.max(left.len());
        self.theorems.push(t);
    }

    pub fn run_examples(&mut self) {
        self.push_theorem(Theorem::new(intro_axiom(1).unwrap(), Justification::Axiom));

        self.push_theorem(Theorem::new(intro_axiom(2).unwrap(), Justification::Axiom));

        self.push_theorem(Theorem::new(intro_axiom(3).unwrap(), Justification::Axiom));

        let new_term = new_succ(new_zero());
        let new_theorem = elim_forall(self.theorems[0].body.clone(), new_term.clone()).unwrap();
        self.push_theorem(Theorem::new(
            new_theorem.clone(),
            Justification::ElimForAll(1, new_term),
        ));
        let new_term = new_zero();
        let new_new_theorem = elim_forall(self.theorems[0].body.clone(), new_term.clone()).unwrap();
        self.push_theorem(Theorem::new(
            new_new_theorem.clone(),
            Justification::ElimForAll(1, new_term),
        ));
        let new_theorem = intro_conjunction(new_theorem, new_new_theorem);
        self.push_theorem(Theorem::new(
            new_theorem,
            Justification::IntroConjunction(4, 5),
        ));
    }

    fn process_command(&mut self, cmd: &str) -> Result<(), String> {
        let tokens: Vec<&str> = cmd.split_whitespace().collect();

        if tokens.is_empty() {
            return Ok(());
        }

        match tokens.as_slice() {
            // CLI commands
            ["quit" | "q"] => self.should_quit = true,
            ["list" | "ls"] => self.list_theorems(),
            ["help" | "commands"] => self.print_help(),
            ["axioms"] => println!("{}\n", list_axioms().join("\n")),
            // Tactics
            ["premise", formula_str] => {
                println!("{}", formula_str);
                let formula = parse_str_to_formula(formula_str);
                self.push_theorem(Theorem {
                    body: formula,
                    reason: Justification::Premise,
                });
                self.list_last_theorem();
            }
            ["intro", "and" | "conjunction"] | ["joining"] => todo!("intro and"),
            ["elim", "and" | "conjunction"] | ["seperation"] => todo!("elim and"),
            ["axiom", n_str] => {
                let n: usize = n_str.parse().map_err(|_| "Expected n to be a integer")?;
                match intro_axiom(n) {
                    Ok(axiom) => {
                        self.push_theorem(Theorem::new(axiom, Justification::Axiom));
                        self.list_last_theorem();
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
            _ => return Err("Unknown or malformed command".to_string()),
        }
        Ok(())
    }

    // TODO: Split run into actual repl loop and run line with String
    pub fn run(&mut self) {
        let mut command = String::new();
        while !self.should_quit {
            print!("> ");
            io::stdout().flush().unwrap();
            command.clear();
            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");
            if let Err(err) = self.process_command(&command) {
                println!("ERROR: {}", err);
            }
        }
    }

    fn print_help(&self) {
        todo!();
    }

    fn list_theorems(&self) {
        for (i, Theorem { body, reason }) in self.theorems.iter().enumerate() {
            let left = format!("{}: {}", i + 1, body);
            println!("{:width$} | {}", left, reason, width = self.max_width);
        }

        println!();
    }

    fn list_last_theorem(&self) {
        let Some((i, Theorem { body, reason })) = self.theorems.iter().enumerate().next_back()
        else {
            return;
        };
        let left = format!("{}: {}", i + 1, body);
        println!("{:width$} | {}\n", left, reason, width = self.max_width);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn always_passes() {
        assert!(true)
    }
}
