use std::fs;

use jane::{lexer::Lexer, parser};

fn main() -> Result<(), String> {
    let content = fs::read_to_string("../example/juliet.jane").expect("Failed to Read File");
    let mut lexer = Lexer::new(content);
    lexer.tokenize().unwrap();

    let mut parser = parser::Parser::new(lexer.tokens().clone());

    // while !parser.is_end() {
    //     let formula = parser.parse_formula();

    //     println!("{:#?}", formula);
    // }

    let formula = parser.parse_formula();

    println!("{:#?}", formula);

    let formula = parser.parse_formula();

    println!("{:#?}", formula);

    Ok(())
}
