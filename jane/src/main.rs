use std::fs;

use jane::scanner::Scanner;

fn main() {
    let content = fs::read_to_string("../example/juliet.jane").expect("Failed to Read File");
    let mut scanner = Scanner::new(content);
    scanner.scan_tokens();

    for token in scanner.tokens {
        println!("{:?}", token);
    }
}
