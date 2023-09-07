use crate::ast::{Expr, Kind, Symbol, AST};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::{
    fs,
    io::Write,
    io::{stdin, stdout, Error},
    process::{Child, Command, Stdio},
};

#[derive(Debug, Clone)]
pub struct Value {
    sym: Symbol,
    val: String,
    flags: Vec<String>,
    args: Vec<String>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ExecType {
    Normal,
    DelayExec,
    Quiet,
}

#[derive(Debug, Clone)]
pub struct ExecContext {
    pub cur_dir: Box<Path>,
}
impl ExecContext {
    pub fn new(path: String) -> Self {
        Self {
            cur_dir: Path::new(&path).into(),
        }
    }
    fn exec_cmds(&self, cmds: Vec<Value>) -> Vec<u8> {
        let mut out = Vec::new();
        for cmd in cmds {
            if cmd.sym == Symbol::Cmd {
                let proc = Command::new(&cmd.val)
                    .args(cmd.flags)
                    .args(cmd.args)
                    .current_dir(self.cur_dir.clone())
                    .spawn();
                if proc.is_ok() {
                    let child = proc.unwrap();
                    let mut output = child
                        .wait_with_output()
                        .expect("Error occured when parsing output")
                        .stdout;
                    out.append(&mut output);
                } else {
                    println!(
                        "Error when executing command \"{}\"\n {:#?}",
                        cmd.val.clone().to_string(),
                        proc
                    );
                }
            } else {
                out.append(&mut cmd.val.as_bytes().to_vec());
            }
        }
        return out;
    }
    fn pipe_cmds(&self, cmds: Vec<Value>, input: Vec<u8>) -> Vec<u8> {
        let mut out = Vec::new();
        for cmd in cmds {
            if cmd.sym == Symbol::Cmd {
                let proc = Command::new(&cmd.val)
                    .args(cmd.flags)
                    .args(cmd.args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .current_dir(self.cur_dir.clone())
                    .spawn();
                if proc.is_ok() {
                    let mut child = proc.unwrap();
                    child
                        .stdin
                        .as_mut()
                        .unwrap()
                        .write_all(&input)
                        .expect("Error when writing to file.");
                    out.append(
                        &mut child
                            .wait_with_output()
                            .expect("Failed to read stdout.")
                            .stdout,
                    );
                } else {
                    println!(
                        "Error when executing command \"{}\"\n {:#?}",
                        cmd.val.clone().to_string(),
                        proc
                    );
                }
            }
        }
        return out;
    }

    fn redir_right(&self, expr: &Expr) -> Option<Value> {
        let left = self
            .interpret_program(expr.left.as_ref().unwrap(), ExecType::Quiet)
            .unwrap();
        let right = self
            .interpret_program(expr.right.as_ref().unwrap(), ExecType::DelayExec)
            .unwrap();
        if right[0].sym != Symbol::File {
            println!("{:#?}", right);
            println!("Expected some type of file.");
            return None;
        }
        let path = fs::metadata(&right[0].val);
        let buf = left
            .iter()
            .fold(String::from(""), |cur, next| cur + &next.val);
        if path.is_ok() {
            let meta = path.unwrap();
            if meta.is_file() {
                if expr.symbol == Symbol::DoubleRedirRight {
                    let mut file: File =
                        OpenOptions::new().append(true).open(&right[0].val).unwrap();
                    file.write_all(buf.as_bytes())
                        .expect("Error when writing to file.");
                } else {
                    fs::write(&right[0].val, buf).expect("Error when writing to file");
                }
            } else if meta.is_dir() {
                println!("Expected some type of file, got directory.");
                return None;
            }
        } else {
            let file = File::create(&right[0].val);
            if file.is_ok() {
                file.unwrap()
                    .write_all(buf.as_bytes())
                    .expect("Error when writing to file");
            } else {
                println!("Error when opening file: {:#?}", file);
            }
        }
        return None;
    }

    fn pipe(&self, expr: &Expr, exec_type: ExecType) -> Option<Value> {
        let left = self
            .interpret_program(expr.left.as_ref().unwrap(), ExecType::Quiet)
            .unwrap();
        let right = self
            .interpret_program(expr.right.as_ref().unwrap(), ExecType::DelayExec)
            .unwrap();
        let input = self.exec_cmds(left);
        let out = self.pipe_cmds(right, input);
        let out_string = String::from_utf8(out).unwrap();
        if exec_type != ExecType::Quiet {
            print!("{}", out_string);
        }
        return Some(Value {
            sym: Symbol::String,
            val: out_string,
            flags: Vec::new(),
            args: Vec::new(),
        });
    }
    fn redir_left(&self, expr: &Expr, exec_type: ExecType) -> Option<Value> {
        let left = self
            .interpret_program(expr.left.as_ref().unwrap(), ExecType::DelayExec)
            .unwrap();
        let right = self
            .interpret_program(expr.right.as_ref().unwrap(), ExecType::DelayExec)
            .unwrap();
        if right[0].sym != Symbol::File {
            println!("{:#?}", right);
            println!("Expected some type of file.");
            return None;
        }
        let path = fs::metadata(&right[0].val);
        if path.is_ok() {
            let meta = path.unwrap();
            if meta.is_file() {
                let buf = fs::read_to_string(&right[0].val).expect("Failed to read file");
                let output = self.pipe_cmds(left, buf.into_bytes().to_vec());
                let out_string = String::from_utf8(output).unwrap();

                if exec_type == ExecType::Normal {
                    print!("{}", out_string);
                }
                return Some(Value {
                    sym: Symbol::String,
                    val: out_string,
                    flags: Vec::new(),
                    args: Vec::new(),
                });
            } else if meta.is_dir() {
                println!("Expected some type of file, got directory.");
                return None;
            }
        } else {
            println!("Error: No such file \"{}\"", &right[0].val);
        }
        return None;
    }

    fn double_redir_left(&self, expr: &Expr, exec_type: ExecType) -> Option<Value> {
        let left = self
            .interpret_program(expr.left.as_ref().unwrap(), ExecType::DelayExec)
            .expect("Error when executing left");
        let right = self
            .interpret_program(expr.right.as_ref().unwrap(), ExecType::DelayExec)
            .expect("Error when executing right");
        let end_str = &right.clone()[0].val;
        let mut input = String::new();
        let mut line = String::new();
        loop {
            print!("heredoc >");
            stdout().flush().expect("Failed to flush stdout");
            stdin().read_line(&mut line).unwrap();
            if line.strip_suffix("\n").unwrap() == end_str.as_str() {
                break;
            }
            input += &line;
            line.clear();
        }
        let output = self.pipe_cmds(left, input.as_bytes().to_vec());
        let out_string = String::from_utf8(output).expect("Error converting output to utf-8.");
        if exec_type == ExecType::Normal {
            print!("{}", out_string);
        }
        return Some(Value {
            sym: Symbol::String,
            val: out_string,
            flags: Vec::new(),
            args: Vec::new(),
        });
    }
    fn interpret_expression(&self, expr: &Expr, exec_type: ExecType) -> Option<Value> {
        match expr.kind {
            Kind::Expr => {
                match expr.symbol {
                    Symbol::None => {
                        if expr.left.is_none() {
                            return None;
                        }
                        let astleft;
                        if exec_type == ExecType::DelayExec {
                            astleft =
                                self.interpret_program(expr.left.as_ref().unwrap(), exec_type);
                        } else {
                            astleft =
                                self.interpret_program(expr.left.as_ref().unwrap(), exec_type);
                        }
                        if astleft.is_none() {
                            return None;
                        }
                        let left = astleft.unwrap();
                        if exec_type == ExecType::DelayExec {
                            if left.len() > 0 {
                                return Some(Value {
                                    sym: left[0].sym,
                                    val: left[0].val.clone(),
                                    flags: left[0].flags.clone(),
                                    args: left[0].args.clone(),
                                });
                            }
                        } else if left.len() > 0 {
                            let out_string = left
                                .iter()
                                .fold(String::from(""), |cur, next| cur + &next.val);
                            return Some(Value {
                                sym: Symbol::String,
                                val: out_string,
                                flags: Vec::new(),
                                args: Vec::new(),
                            });
                        } else {
                            //println!("ERROR! {:#?}",left);
                        }
                    }
                    Symbol::RedirRight | Symbol::DoubleRedirRight => {
                        return self.redir_right(expr);
                    }
                    Symbol::Pipe => {
                        return self.pipe(expr, exec_type);
                    }
                    Symbol::RedirLeft => {
                        return self.redir_left(expr, exec_type);
                    }
                    Symbol::DoubleRedirLeft => {
                        return self.double_redir_left(expr, exec_type);
                    }
                    _ => {}
                }
            }
            Kind::Value => {
                if exec_type == ExecType::DelayExec || expr.symbol != Symbol::Cmd {
                    return Some(Value {
                        sym: expr.symbol,
                        val: String::from(expr.value.as_ref().unwrap().as_str()),
                        flags: expr.flags.clone(),
                        args: expr.args.clone(),
                    });
                } else {
                    //This should be a default pipe that
                    //is the same as a normal terminal
                    let mut proc: Result<Child, Error>;
                    //pipe it and read the output so our program can handle it
                    if exec_type == ExecType::Quiet {
                        proc = Command::new(expr.value.as_ref().unwrap())
                            .args(expr.flags.clone())
                            .args(expr.args.clone())
                            .current_dir(self.cur_dir.clone())
                            .stdout(Stdio::piped())
                            .spawn();
                    } else {

                        proc = Command::new(expr.value.as_ref().unwrap())
                            .args(expr.flags.clone())
                            .args(expr.args.clone())
                            .current_dir(self.cur_dir.clone())
                            .spawn();
                    }

                    if proc.is_ok() {
                        let child = proc.unwrap();
                        let output = child
                            .wait_with_output()
                            .expect("Error occured when parsing output")
                            .stdout;
                        return Some(Value {
                            sym: Symbol::String,
                            val: String::from_utf8(output).unwrap(),
                            flags: Vec::new(),
                            args: Vec::new(),
                        });
                    } else {
                        println!(
                            "Error when executing command \"{}\"\n {:#?}",
                            expr.value.as_ref().unwrap(),
                            proc
                        );
                    }
                }
            }
        }
        return None;
    }
    pub fn interpret_program(&self, ast: &AST, exec_type: ExecType) -> Option<Vec<Value>> {
        let mut output = Vec::new();
        ast.exprs.iter().for_each(|expr: &Expr| {
            let out_cur = self.interpret_expression(expr, exec_type);
            if !out_cur.is_none() {
                output.push(out_cur.unwrap());
            }
        });
        return Some(output);
    }
}
