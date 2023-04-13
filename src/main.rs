mod scanner;
mod parser;
mod token;
mod error;
mod symbol;
use scanner::Scanner;


use std::{ env, fs, io::{ stdin, stdout, Write } };

use crate::parser::parse_program;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [path_to_script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run(source: &String) {
    let mut lexer = Scanner::new(source.to_string());
    println!("{:#?}", lexer);
    lexer.scan_tokens();
    println!("{:#?}", lexer);
    parse_program(lexer);
}
fn run_file(path: &String) {
    let contents = fs::read_to_string(path).expect(format!("{path} is not a valid path").as_str());
    run(&contents);
}

fn run_prompt() {
    let mut line = String::new();
    loop {
        print!("> ");
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut line).unwrap();
        run(&line);
        line.clear();
    }
}