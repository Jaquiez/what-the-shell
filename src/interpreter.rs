use crate::{ast::{Symbol,AST,Kind,Expr}, error};
use crate::error::error;
use std::{process::{Command,Stdio}, io::Write,io::stdin, fs};
use std::fs::{OpenOptions,File};

#[derive(Debug)]
pub struct Value{
    sym: Symbol,
    val: String,
    flags: Vec<String>,
    args: Vec<String>
}

#[derive(PartialEq,Eq,Clone, Copy, Debug)]
pub enum ExecType{
    NORMAL,
    DELAY_EXEC,
    QUIET
}

fn exec_cmds(cmds: Vec<Value>) -> Vec<u8> {
    let mut out = Vec::new();
    for cmd in cmds{
        if cmd.sym == Symbol::CMD{
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
        if cmd.sym == Symbol::CMD{
            let proc= Command::new(&cmd.val)
                .args(cmd.flags)
                .args(cmd.args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn();
            if proc.is_ok(){
                let mut child = proc.unwrap();
                child.stdin.as_mut().unwrap().write_all(&input);
                out.append(&mut child.wait_with_output().expect("Failed to read stdout").stdout);
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
        Kind::EXPR =>{
            match expr.symbol {
                Symbol::NONE =>{
                    if expr.left.is_none() {
                        return None;
                    }
                    let astleft = interpret_program(expr.left.as_ref().unwrap(),exec_type);
                    if astleft.is_none(){
                        //println!("{:#?}",astleft);
                        return None;
                    }
                    let left = astleft.unwrap();
                    if exec_type == ExecType::DELAY_EXEC{
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
                        if exec_type != ExecType::QUIET{
                            print!("{}",out_string);
                        }
                        return Some(Value { 
                            sym: Symbol::NONE, 
                            val: out_string, 
                            flags: Vec::new(), 
                            args: Vec::new()
                        });     
                    } 
                    else{
                        //println!("ERROR! {:#?}",left);
                    }
                }
                Symbol::REDIR_RIGHT | Symbol::DOUBLE_REDIR_RIGHT =>{
                    let left = interpret_program(expr.left.as_ref().unwrap(),ExecType::QUIET).unwrap(); 
                    let right = interpret_program(expr.right.as_ref().unwrap(),ExecType::DELAY_EXEC).unwrap(); 
                    if right[0].sym != Symbol::FILE {
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
                            if expr.symbol == Symbol::DOUBLE_REDIR_RIGHT{
                                let mut file: File = OpenOptions::new()
                                    .append(true)
                                    .open(&right[0].val)
                                    .unwrap();
                                file.write_all(buf.as_bytes());
                            }
                            else{
                                fs::write(&right[0].val, buf);
                            }

                        }
                        else if meta.is_dir(){
                            println!("Expected some type of file, got directory.");
                            return None;
                        }
                    }
                    else{
                        let mut file = File::create(&right[0].val);
                        if file.is_ok(){
                            file.unwrap().write_all(buf.as_bytes());
                        }
                        else {
                            println!("Error when opening file: {:#?}",file);
                        }
                    }

                },
                Symbol::PIPE=>{
                    let left = interpret_program(expr.left.as_ref().unwrap(),ExecType::QUIET).unwrap(); 
                    let right = interpret_program(expr.right.as_ref().unwrap(),ExecType::DELAY_EXEC).unwrap(); 
                    //println!("LEFT -> {:#?}",left);
                    //println!("RIGHT -> {:#?}",right);
                    let input = exec_cmds(left);

                    let out = pipe_cmds(right, input);
                    let out_string = String::from_utf8(out).unwrap();
                    if exec_type != ExecType::QUIET{
                        print!("{}",out_string);
                    }
                    return Some(Value { 
                        sym: Symbol::NONE, 
                        val: out_string, 
                        flags: Vec::new(), 
                        args: Vec::new() 
                    });
                },
                _=>{}
            }
        }
        Kind::VALUE =>{
            //println!("VALUE -> {:#?}",expr);
            //println!("Exec_Type -> {:#?}",exec_type);
            if exec_type == ExecType::DELAY_EXEC || expr.symbol != Symbol::CMD{
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
                        sym: Symbol::NONE, 
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
        Kind::NONE =>{
            println!("Got none\n{:#?}",expr);
        }
        _=>{
            error(0,String::from("Unexpected Kind!"));
        }
    }
    return None;
}
pub fn interpret_program(ast : &AST, exec_type: ExecType) -> Option<Vec<Value>>{
    let mut output = Vec::new();
    ast.exprs.iter().for_each(|expr: &Expr|{
        let outCur = interpret_expression(expr,exec_type);
        if !outCur.is_none(){
            output.push(outCur.unwrap());
        }
        //println!("{:#?}",output);
    });
    return Some(output);
}