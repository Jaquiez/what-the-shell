mod scanner;
mod parser;
mod token;
mod error;
mod ast;
mod interpreter;
use scanner::Scanner;
use std::thread;
use std::env::current_dir;
use std::{io::{ stdin, stdout, Write } };

use crate::{parser::parse_program, interpreter::interpret_program};
fn main() {
    run_prompt();
}

fn run(source: &str) {
    let mut lexer = Scanner::new(source.to_string());
    //println!("Initial Source: {}",source.to_string());
    lexer.scan_tokens();
    //println!("{:#?}", lexer);
    let ast = parse_program(lexer).unwrap();
    //println!("{:#?}", ast);
    interpret_program(&ast,interpreter::ExecType::Normal);
}

fn run_prompt() {
    let cur_dir = current_dir().unwrap();
    let mut line = String::new();
    loop {
        print!("λ {} ",cur_dir.to_str().unwrap());
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut line).unwrap();
        if line == "exit\n" {
            break;
        }
        let run_str = line.clone();
        let out = thread::spawn(move ||{
            run(&run_str);
        }).join();
        match out {
            _=>{}
        }
        line.clear();
    }
}