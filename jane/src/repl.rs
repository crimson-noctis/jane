use std::fmt::Display;
use std::io;
use std::io::Write;

use jane::ast::Choice;
use jane::ast::Formula;
use jane::ast::Term;
use jane::ast::elim_conjunction;
use jane::ast::elim_forall;
use jane::ast::elim_succ;
use jane::ast::intro_axiom;
use jane::ast::intro_conjunction;
use jane::ast::intro_induction;
use jane::ast::intro_succ;
use jane::ast::intro_symmetry;
use jane::ast::intro_transitivity;
use jane::ast::list_axioms;
use jane::ast::new_succ;
use jane::ast::new_zero;
use jane::lexer::Lexer;
use jane::parser::Parser;

// TODO: Undo
// TODO: Help
// TODO: Tactics
// TODO: Readline support

#[derive(Debug, Clone)]
enum Justification {
    Axiom,
    Premise,
    IntroConjunction(usize, usize),
    ElimConjunction(usize, Choice),
    IntroImplies(usize, usize),
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
            Self::IntroForAll => todo!(),
            Self::ElimForAll(line_number, term) => {
                write!(f, "Elim ∀ at {} with {}", line_number, term)
            }
            Self::IntroExists => todo!(),
            Self::ElimExists => todo!(),
            Self::IntroSucc(l) => write!(f, "Add S to line {}", l),
            Self::ElimSucc(l) => write!(f, "Drop S from line {}", l),
            Self::Symmetry(_) => write!(f, "Symmetry"),
            Self::Transivity(_, _) => write!(f, "Transivity"),
            Self::IntroImplies(_, _) => todo!(),
            Self::ElimImplies(_, _) => todo!(),
            Self::DeMorgan(_) => todo!(),
            Self::Contrapositive(_) => todo!(),
            Self::Interchange(_) => todo!(),
            Self::Induction(p, q) => write!(f, "Induction lines ({}, {})", p, q),
            Self::CarryOver(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
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

    parser.parse_formula()
}

fn parse_str_to_term(f: &str) -> Term {
    let mut lexer = Lexer::new(f.to_string());
    lexer.tokenize().unwrap();
    let tokens = lexer.tokens();

    let mut parser = Parser::new(tokens.clone());

    parser.parse_term()
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
        let new_theorem = elim_forall(self.theorems[1].body.clone(), Term::Zero).unwrap();
        self.push_theorem(Theorem::new(
            new_theorem,
            Justification::ElimForAll(2, Term::Zero),
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
            ["undo"] => todo!("undo"),
            ["axiom" | "axioms"] => println!("{}\n", list_axioms().join("\n")),
            ["push"] => {
                println!("[");
                self.indentation_level += 1;
            }
            ["pop"] => {
                if self.indentation_level > 0 {
                    println!("]");
                    self.indentation_level -= 1;
                }
            }
            // Tactics
            ["premise", rest @ ..] => {
                if rest.is_empty() {
                    return Err("Usage: premise <formula>".to_string());
                }
                let formula_str = rest.join(" ");
                let formula = parse_str_to_formula(&formula_str);
                self.push_theorem(Theorem {
                    body: formula,
                    reason: Justification::Premise,
                });
                self.list_last_theorem();
            }
            ["intro", "and" | "conjunction", p, q] | ["joining", p, q] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let q_num = q.parse::<usize>().unwrap();
                let theorem_q = self.theorems[q_num - 1].clone();
                let formula = intro_conjunction(theorem_p.body, theorem_q.body);
                self.push_theorem(Theorem::new(
                    formula,
                    Justification::IntroConjunction(p_num, q_num),
                ));
                self.list_last_theorem();
            }
            ["elim", "and" | "conjunction", p, c] | ["seperation", p, c] => {
                let choice = match *c {
                    "l" | "left" | "1" => Choice::Left,
                    "r" | "right" | "2" => Choice::Right,
                    _ => return Err("Not a valid choice".to_string()),
                };
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let formula = elim_conjunction(theorem_p.body, choice).unwrap();
                self.push_theorem(Theorem::new(
                    formula,
                    Justification::ElimConjunction(p_num, choice),
                ));
                self.list_last_theorem();
            }
            ["intro" | "add", "succ", p] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let formula = intro_succ(theorem_p.body).unwrap();
                self.push_theorem(Theorem::new(formula, Justification::IntroSucc(p_num)));
                self.list_last_theorem();
            }
            ["elim" | "drop", "succ", p] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let formula = elim_succ(theorem_p.body).unwrap();
                self.push_theorem(Theorem::new(formula, Justification::ElimSucc(p_num)));
                self.list_last_theorem();
            }
            ["symmetry", p] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let formula = intro_symmetry(theorem_p.body.clone()).unwrap();
                self.push_theorem(Theorem::new(formula, Justification::Symmetry(p_num)));
                self.list_last_theorem();
            }
            ["transivity", p, q] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let q_num = q.parse::<usize>().unwrap();
                let theorem_q = self.theorems[q_num - 1].clone();
                let formula = intro_transitivity(theorem_p.body, theorem_q.body).unwrap();
                self.push_theorem(Theorem::new(
                    formula,
                    Justification::Transivity(p_num, q_num),
                ));
                self.list_last_theorem();
            }
            ["induction", p, q] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let q_num = q.parse::<usize>().unwrap();
                let theorem_q = self.theorems[q_num - 1].clone();
                let formula = intro_induction(theorem_p.body, theorem_q.body).unwrap();
                self.push_theorem(Theorem::new(
                    formula,
                    Justification::Induction(p_num, q_num),
                ));
                self.list_last_theorem();
            }
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
            _ => return Err("Unknown command".to_string()),
        }
        Ok(())
    }

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
        println!("help");
        println!("axiom <n>");
        println!("intro and <line number> <line number>");
        println!("elim and <line number> <l or r>");
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
