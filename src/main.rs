mod scanner;

use std::{ env, fs, io::{ stdin, stdout, Write } };
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
    let mut lexer = scanner::Scanner::new(source.to_string());
    println!("{:#?}", lexer);
    lexer.scan_tokens();
    println!("{:#?}", lexer);
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