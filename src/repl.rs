use std::fmt::Display;
use std::io;
use std::io::Write;

use crate::{
    ast::{
        Choice, Formula, Term, elim_conjunction, elim_forall, elim_succ, intro_axiom,
        intro_conjunction, intro_induction, intro_succ, intro_symmetry, intro_transitivity,
        list_axioms, new_succ, new_zero,
    },
    lexer::Lexer,
    parser::Parser,
};

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

    fn undo(&mut self) {
        self.theorems.pop();
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

    pub fn process_command(&mut self, cmd: &str) -> Result<String, String> {
        let tokens: Vec<&str> = cmd.split_whitespace().collect();

        if tokens.is_empty() {
            return Ok("".to_string());
        }

        return match tokens.as_slice() {
            // CLI commands
            ["quit" | "q"] => {
                self.should_quit = true;
                Ok(String::new())
            }
            ["list" | "ls"] => Ok(self.list_theorems()),
            ["help" | "commands"] => Ok(self.print_help()),
            ["undo"] => {
                self.undo();
                Ok(String::new())
            }
            ["axiom" | "axioms"] => Ok(format!("{}\n", list_axioms().join("\n"))),
            ["push"] => {
                self.indentation_level += 1;
                Ok(format!("["))
            }
            ["pop"] => {
                if self.indentation_level > 0 {
                    self.indentation_level -= 1;
                    Ok(format!("]"))
                } else {
                    Err("Nothing to pop from".to_string())
                }
            }
            // Tactics
            ["premise", rest @ ..] => {
                // TODO: Add restrictions
                if rest.is_empty() {
                    Err("Usage: premise <formula>".to_string())
                } else {
                    let formula_str = rest.join(" ");
                    let formula = parse_str_to_formula(&formula_str);
                    self.push_theorem(Theorem {
                        body: formula,
                        reason: Justification::Premise,
                    });
                    Ok(self.list_last_theorem())
                }
            }
            ["carryover"] => todo!("carryover"),
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
                Ok(self.list_last_theorem())
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
                Ok(self.list_last_theorem())
            }
            ["intro" | "add", "succ", p] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let formula = intro_succ(theorem_p.body).unwrap();
                self.push_theorem(Theorem::new(formula, Justification::IntroSucc(p_num)));
                Ok(self.list_last_theorem())
            }
            ["elim" | "drop", "succ", p] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let formula = elim_succ(theorem_p.body).unwrap();
                self.push_theorem(Theorem::new(formula, Justification::ElimSucc(p_num)));
                Ok(self.list_last_theorem())
            }
            ["intro", "not" | "negation"] => todo!("intro negation"),
            ["elim", "not" | "negation"] => todo!("elim negation"),
            ["intro", "implies"] => todo!("intro implies"),
            ["elim", "implies"] => todo!("elim implies"),
            ["demorgan"] => todo!("demorgan"),
            ["symmetry", p] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let formula = intro_symmetry(theorem_p.body.clone()).unwrap();
                self.push_theorem(Theorem::new(formula, Justification::Symmetry(p_num)));
                Ok(self.list_last_theorem())
            }
            ["transitivity", p, q] => {
                let p_num = p.parse::<usize>().unwrap();
                let theorem_p = self.theorems[p_num - 1].clone();
                let q_num = q.parse::<usize>().unwrap();
                let theorem_q = self.theorems[q_num - 1].clone();
                let formula = intro_transitivity(theorem_p.body, theorem_q.body).unwrap();
                self.push_theorem(Theorem::new(
                    formula,
                    Justification::Transivity(p_num, q_num),
                ));
                Ok(self.list_last_theorem())
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
                Ok(self.list_last_theorem())
            }
            ["axiom", n_str] => {
                let n: usize = n_str.parse().map_err(|_| "Expected n to be a integer")?;
                match intro_axiom(n) {
                    Ok(axiom) => {
                        self.push_theorem(Theorem::new(axiom, Justification::Axiom));
                        Ok(self.list_last_theorem())
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
            _ => return Err("Unknown command".to_string()),
        };
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
            // if let Err(err) = self.process_command(&command) {
            // println!("ERROR: {}", err);
            // }
            match self.process_command(&command) {
                Ok(s) => println!("{}", s),
                Err(err) => println!("ERROR: {}", err),
            }
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn print_help(&self) -> String {
        let mut help = String::new();
        help.push_str("Available commands:\n");
        help.push_str("  help, commands       Show this help message\n");
        help.push_str("  list, ls             List all theorems in the current proof\n");
        help.push_str("  undo                 Remove the last theorem added\n");
        help.push_str("  axiom, axioms        List all available axioms\n");
        help.push_str("  axiom <n>            Introduce axiom <n> into the proof\n");
        help.push_str("  push                 Start a subproof (increases indentation)\n");
        help.push_str("  pop                  End a subproof (decreases indentation)\n");
        help.push_str("  quit, q              Exit the REPL\n");
        help.push_str("\nProof tactics:\n");
        help.push_str("  premise <formula>    Introduce a premise\n");
        help.push_str("  intro and <l1> <l2>  Introduction of conjunction (joining)\n");
        help.push_str("  elim and <l> <c>     Elimination of conjunction (separation). choice is 'l' or 'r'\n");
        help.push_str("  intro succ <l>       Introduction of successor (add succ)\n");
        help.push_str("  elim succ <l>        Elimination of successor (drop succ)\n");
        help.push_str("  symmetry <l>         Symmetry of equality\n");
        help.push_str("  transitivity <l1> <l2> Transitivity of equality\n");
        help.push_str("  induction <l1> <l2>  Mathematical induction\n");
        help
    }

    fn list_theorems(&self) -> String {
        let mut str = String::new();
        for (i, Theorem { body, reason }) in self.theorems.iter().enumerate() {
            let left = format!("{}: {}", i + 1, body);
            str.push_str(&format!(
                "{:width$} | {}\n",
                left,
                reason,
                width = self.max_width
            ));
        }

        str.push('\n');
        str
    }

    fn list_last_theorem(&self) -> String {
        let Some((i, Theorem { body, reason })) = self.theorems.iter().enumerate().next_back()
        else {
            return String::new();
        };
        let left = format!("{}: {}", i + 1, body);
        format!("{:width$} | {}\n", left, reason, width = self.max_width)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn always_passes() {
        assert!(true)
    }
}
