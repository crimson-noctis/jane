mod repl;

use jane::{lexer::Lexer, parser};
use repl::Repl;
use std::fs;

fn main_repl() {
    let mut repl = Repl::new();
    repl.run_examples();
    repl.run();
}

fn main() -> Result<(), String> {
    main_repl();

    // let content = fs::read_to_string("../example/juliet.jane").expect("Failed to Read File");
    // let mut lexer = Lexer::new(content);
    // lexer.tokenize().unwrap();
    //
    // let mut parser = parser::Parser::new(lexer.tokens().clone());
    //
    // while !parser.is_end() {
    //     let formula = parser.parse_formula();
    //     println!("{:#?}", formula);
    // }

    Ok(())
}
