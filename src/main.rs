mod ast;
mod error;
mod interpreter;
mod parser;
mod scanner;
mod token;
use scanner::Scanner;
use std::env::current_dir;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process;
use std::thread;

use crate::{interpreter::ExecContext, parser::parse_program};
fn main() {
    run_prompt();
}

fn run(source: &str, exec: ExecContext) {
    let mut lexer = Scanner::new(source.to_string());
    lexer.scan_tokens();
    let ast = parse_program(lexer).unwrap();
    exec.interpret_program(&ast, interpreter::ExecType::Normal);
}

fn eval_path_str(path_str: &str) -> String {
    let mut strs: Vec<&str> = path_str.split("/").collect();
    let mut i = 0;
    while i < strs.len() {
        if strs[i] == ".." {
            if i > 0 {
                strs.remove(i - 1);
                strs.remove(i - 1);
                i -= 1;
            } else {
                strs.remove(i);
            }
        } else if strs[i].is_empty() {
            strs.remove(i);
        } else {
            i += 1;
        }
    }
    let mut path = strs
        .iter()
        .fold(String::new(), |cur, next| cur.to_owned() + "/" + next);
    if path.is_empty() {
        path = String::from("/");
    }
    return path;
}
fn check_special(mut line: String, exec: &mut ExecContext) -> bool {
    line.pop();
    let lines: Vec<&str> = line.split(" ").collect();
    match lines[0] {
        "exit" => {
            process::exit(0);
        }
        "cd" => {
            let mut path_str = String::new();
            if !lines[1].starts_with("/") {
                path_str = exec.cur_dir.to_str().unwrap().to_owned() + "/";
            }
            path_str += lines[1];
            path_str = eval_path_str(&path_str);
            let path = Path::new(path_str.as_str());
            if path.is_dir() {
                exec.cur_dir = path.into();
            } else {
                println!("{} is not a valid directory.", lines[1]);
            }
            return true;
        }
        _ => {}
    }
    return false;
}
fn run_prompt() {
    let current_dir = current_dir().unwrap_or_else(|e| panic!("failed to get current dir: {}", e));
    let mut current_dir_str = current_dir.to_str().unwrap().to_owned();
    let mut exec = ExecContext::new(current_dir_str.clone());
    let mut line = String::new();
    loop {
        print!("λ {} ", current_dir_str);
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut line).unwrap();

        if check_special(line.clone(), &mut exec) {
            current_dir_str = exec.cur_dir.to_str().unwrap().to_owned();
            line.clear();
            continue;
        }
        let run_str = line.clone();
        let exec_clone = exec.clone();
        let out = thread::spawn(move || {
            run(&run_str, exec_clone);
        })
        .join();
        match out {
            _ => {}
        }
        line.clear();
    }
}
