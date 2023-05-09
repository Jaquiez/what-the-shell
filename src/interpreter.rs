use crate::{ast::{Symbol,AST,Kind,Expr}};
use std::{process::{Command,Stdio}, io::Write,io::{stdin,stdout}, fs};
use std::fs::{OpenOptions,File};

#[derive(Debug,Clone)]
pub struct Value{
    sym: Symbol,
    val: String,
    flags: Vec<String>,
    args: Vec<String>
}

#[derive(PartialEq,Eq,Clone, Copy, Debug)]
pub enum ExecType{
    Normal,
    DelayExec,
    Quiet
}

fn exec_cmds(cmds: Vec<Value>) -> Vec<u8> {
    let mut out = Vec::new();
    for cmd in cmds{
        if cmd.sym == Symbol::Cmd{
            let output = Command::new(&cmd.val)
                .args(cmd.flags)
                .args(cmd.args)
                .output();
            if output.is_ok(){
                out.append(output.unwrap().stdout.as_mut());
            }
            else{
                println!("Error when executing command \"{}\"\n {:#?}"
                    ,cmd.val.clone().to_string(),output);
            }
        }
        else{
            out.append(&mut cmd.val.as_bytes().to_vec());
        }
    }
    return out;
}
fn pipe_cmds(cmds: Vec<Value>,input: Vec<u8>) -> Vec<u8>{
    let mut out = Vec::new();
    for cmd in cmds{
        if cmd.sym == Symbol::Cmd{
            let proc= Command::new(&cmd.val)
                .args(cmd.flags)
                .args(cmd.args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn();
            if proc.is_ok(){
                let mut child = proc.unwrap();
                child.stdin.as_mut().unwrap().write_all(&input)
                    .expect("Error when writing to file.");
                out.append(&mut child.wait_with_output()
                    .expect("Failed to read stdout.").stdout);
            }
            else{
                println!("Error when executing command \"{}\"\n {:#?}"
                    ,cmd.val.clone().to_string(),proc);
            }
        }
    }
    return out;
}

fn interpret_expression(expr: &Expr,exec_type: ExecType) -> Option<Value>{
    match expr.kind {
        Kind::Expr =>{
            match expr.symbol {
                Symbol::None =>{
                    if expr.left.is_none() {
                        return None;
                    }
                    let astleft;
                    if exec_type == ExecType::DelayExec{
                        astleft = interpret_program(expr.left.as_ref().unwrap(),exec_type);
                    }
                    else{
                        astleft = interpret_program(expr.left.as_ref().unwrap(),ExecType::Quiet);
                    }
                    if astleft.is_none(){
                        //println!("{:#?}",astleft);
                        return None;
                    }
                    //println!("{:#?}",astleft);
                    let left = astleft.unwrap();
                    if exec_type == ExecType::DelayExec{
                        if left.len() > 0 {
                            return Some(Value { 
                                sym: left[0].sym, 
                                val: left[0].val.clone(), 
                                flags: left[0].flags.clone(), 
                                args: left[0].args.clone()
                            });     
                        }
                    }
                    else if left.len() > 0 {
                        let out_string = left.iter()
                            .fold(String::from(""), |cur,next| cur + &next.val);
                        if exec_type != ExecType::Quiet{
                            print!("{}",out_string);
                        }
                        return Some(Value { 
                            sym: Symbol::String, 
                            val: out_string, 
                            flags: Vec::new(), 
                            args: Vec::new()
                        });     
                    } 
                    else{
                        //println!("ERROR! {:#?}",left);
                    }
                }
                Symbol::RedirRight | Symbol::DoubleRedirRight =>{
                    let left = interpret_program(expr.left.as_ref().unwrap(),ExecType::Quiet).unwrap(); 
                    let right = interpret_program(expr.right.as_ref().unwrap(),ExecType::DelayExec).unwrap(); 
                    if right[0].sym != Symbol::File {
                        println!("{:#?}",right);
                        println!("Expected some type of file.");
                        return None;
                    }
                    let path = fs::metadata(&right[0].val);
                    let buf = left.iter()
                        .fold(String::from(""),|cur,next| cur+ &next.val);
                    if path.is_ok() {
                        let meta = path.unwrap();
                        if meta.is_file(){
                            if expr.symbol == Symbol::DoubleRedirRight{
                                let mut file: File = OpenOptions::new()
                                    .append(true)
                                    .open(&right[0].val)
                                    .unwrap();
                                file.write_all(buf.as_bytes())
                                    .expect("Error when writing to file.");
                            }
                            else{
                                fs::write(&right[0].val, buf)
                                    .expect("Error when writing to file");
                            }

                        }
                        else if meta.is_dir(){
                            println!("Expected some type of file, got directory.");
                            return None;
                        }
                    }
                    else{
                        let file = File::create(&right[0].val);
                        if file.is_ok(){
                            file.unwrap().write_all(buf.as_bytes())
                                .expect("Error when writing to file");
                        }
                        else {
                            println!("Error when opening file: {:#?}",file);
                        }
                    }

                },
                Symbol::Pipe=>{
                    let left = interpret_program(expr.left.as_ref().unwrap(),ExecType::Quiet).unwrap(); 
                    let right = interpret_program(expr.right.as_ref().unwrap(),ExecType::DelayExec).unwrap(); 
                    //println!("LEFT -> {:#?}",left);
                    //println!("RIGHT -> {:#?}",right);
                    let input = exec_cmds(left);

                    let out = pipe_cmds(right, input);
                    let out_string = String::from_utf8(out).unwrap();
                    if exec_type != ExecType::Quiet{
                        print!("{}",out_string);
                    }
                    return Some(Value { 
                        sym: Symbol::String, 
                        val: out_string, 
                        flags: Vec::new(), 
                        args: Vec::new() 
                    });
                },
                Symbol::RedirLeft =>{
                    let left = interpret_program(expr.left.as_ref().unwrap(),ExecType::DelayExec).unwrap(); 
                    let right = interpret_program(expr.right.as_ref().unwrap(),ExecType::DelayExec).unwrap(); 
                    if right[0].sym != Symbol::File {
                        println!("{:#?}",right);
                        println!("Expected some type of file.");
                        return None;
                    }
                    let path = fs::metadata(&right[0].val);
                    if path.is_ok() {
                        let meta = path.unwrap();
                        if meta.is_file(){
                            let buf = fs::read_to_string(&right[0].val)
                                .expect("Failed to read file");
                            let output = pipe_cmds(left, buf.into_bytes().to_vec());
                            let out_string = String::from_utf8(output).unwrap();

                            if exec_type == ExecType::Normal{
                                print!("{}",out_string);
                            }
                            return Some(Value { 
                                sym: Symbol::String, 
                                val: out_string, 
                                flags: Vec::new(), 
                                args: Vec::new() 
                            });
                        }
                        else if meta.is_dir(){
                            println!("Expected some type of file, got directory.");
                            return None;
                        }
                    }
                    else{
                        println!("Error: No such file \"{}\"",&right[0].val);
                    }
                }
                Symbol::DoubleRedirLeft=>{
                    let left = interpret_program(expr.left.as_ref()
                        .unwrap(),ExecType::DelayExec)
                        .expect("Error when executing left"); 
                    let right = interpret_program(expr.right.as_ref()
                        .unwrap(),ExecType::DelayExec)
                        .expect("Error when executing right");
                    let end_str = &right.clone()[0].val;
                    let mut input = String::new();
                    let mut line = String::new();
                    loop {
                        print!("heredoc > ");
                        stdout().flush().expect("Failed to flush stdout");
                        stdin().read_line(&mut line).unwrap();
                        input += &line;
                        if line.strip_suffix("\n").unwrap() == end_str.as_str(){
                            break;
                        }
                        line.clear();
                    }
                    let output = pipe_cmds(left, input.as_bytes().to_vec());
                    let out_string = String::from_utf8(output)
                        .expect("Error converting output to utf-8.");
                    if exec_type == ExecType::Normal{
                        print!("{}",out_string);
                    }
                    return Some(Value { 
                        sym: Symbol::String, 
                        val: out_string, 
                        flags: Vec::new(), 
                        args: Vec::new() 
                    });
                }
                _=>{}
            }
        }
        Kind::Value =>{
            if exec_type == ExecType::DelayExec || expr.symbol != Symbol::Cmd{
                return Some(Value { 
                    sym: expr.symbol, 
                    val: String::from(expr.value.as_ref().unwrap().as_str()),
                    flags: expr.flags.clone(), 
                    args: expr.args.clone() 
                });
            }
            else{
                let output = Command::new(expr.value.as_ref().unwrap())
                    .args(expr.flags.clone())
                    .args(expr.args.clone())
                    .output();
                if output.is_ok(){
                    return Some(Value { 
                        sym: Symbol::String, 
                        val:String::from_utf8(output.unwrap().stdout).unwrap(), 
                        flags: Vec::new(), 
                        args: Vec::new() 
                    })
                }
                else {
                    println!("Error when executing command \"{}\"\n {:#?}",expr.value.as_ref().unwrap(),output);
                }

            }

        }
    }
    return None;
}
pub fn interpret_program(ast : &AST, exec_type: ExecType) -> Option<Vec<Value>>{
    let mut output = Vec::new();
    ast.exprs.iter().for_each(|expr: &Expr|{
        let out_cur = interpret_expression(expr,exec_type);
        if !out_cur.is_none(){
            output.push(out_cur.unwrap());
        }
        //println!("{:#?}",output);
    });
    return Some(output);
}