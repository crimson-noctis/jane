use std::fs;

use jane::{lexer::Lexer, parser};

fn main() -> Result<(), String> {
    let content = fs::read_to_string("../example/juliet.jane").expect("Failed to Read File");
    let mut scanner = Lexer::new(content);
    scanner.tokenize().unwrap();

    for token in scanner.tokens() {
        println!("{:?}", token);
    }

    let mut parser = parser::Parser::new(scanner.tokens().clone());

    let term = parser.parse_term();

    println!("{:?}", term);

    Ok(())
}
