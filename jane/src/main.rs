use std::fs;

use jane::lexer::Lexer;

fn main() {
    let content = fs::read_to_string("../example/juliet.jane").expect("Failed to Read File");
    let mut scanner = Lexer::new(content);
    scanner.scan_tokens();

    for token in scanner.tokens {
        println!("{:?}", token);
    }
}
