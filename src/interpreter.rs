use crate::{ast::{Symbol,AST,Kind,Expr}, error};
use crate::error::error;
use std::{process::Command, os::fd::IntoRawFd, io::Write,char, fs};
use std::fs::{OpenOptions,File};

#[derive(Debug)]
pub struct Value{
    sym: Symbol,
    val: String,
    flags: Vec<String>,
    args: Vec<String>
}

fn exec_cmds(cmds: Vec<Value>) -> Vec<u8> {
    let mut out = Vec::new();
    for cmd in cmds{
        let flags = cmd.flags.iter()
            .map(|flag: &String| String::from("-") + flag);
        if cmd.sym == Symbol::CMD{
            let output = Command::new(cmd.val)
                .args(flags)
                .args(cmd.args)
                .output();
            if output.is_ok(){
                out.append(output.unwrap().stdout.as_mut());
            }
        }
    }
    return out;
}
fn interpret_expression(expr: &Expr,delay_exec: bool) -> Option<Value>{
    match expr.kind {
        Kind::EXPR =>{
            match expr.symbol {
                Symbol::NONE =>{
                    let left = interpret_program(expr.left.as_ref().unwrap(),delay_exec).unwrap();
                    if delay_exec{
                        return Some(Value { 
                            sym: left[0].sym, 
                            val: left[0].val.clone(), 
                            flags: left[0].flags.clone(), 
                            args: left[0].args.clone()
                        });       
                    }
                    if left.len() > 0{
                        let flags = left[0].flags.iter()
                            .map(|flag: &String| String::from("-") + flag);
                        let output = Command::new(&left[0].val)
                            .args(flags)
                            .args(&left[0].args)
                            .output();
                        if output.is_ok() {
                            let out_string = String::from_utf8(output.unwrap().stdout).unwrap();
                            println!("{}",out_string);
                            return Some(Value { 
                                sym: Symbol::NONE, 
                                val: out_string, 
                                flags: Vec::new(), 
                                args: Vec::new() 
                            });
                        }
                        else if output.is_err(){
                            println!("{:#?}",output);
                        }   
                    } 
                    else{
                        println!("{:#?}",expr);
                    }
                }
                Symbol::REDIR_RIGHT | Symbol::DOUBLE_REDIR_RIGHT =>{
                    let left = interpret_program(expr.left.as_ref().unwrap(),true).unwrap(); 
                    let right = interpret_program(expr.right.as_ref().unwrap(),true).unwrap(); 
                    if right[0].sym != Symbol::FILE {
                        println!("{:#?}",right);
                        println!("Expected some type of file.");
                        return None;
                    }
                    let path = fs::metadata(&right[0].val);
                    if path.is_ok() {
                        let meta = path.unwrap();
                        if meta.is_file(){
                            if expr.symbol == Symbol::DOUBLE_REDIR_RIGHT{
                                let mut file: File = OpenOptions::new()
                                    .append(true)
                                    .open(&right[0].val)
                                    .unwrap();
                                let buf = &exec_cmds(left);
                                file.write_all(buf);
                            }
                            else{
                                let buf = &exec_cmds(left);
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
                            let buf = &exec_cmds(left);
                            file.unwrap().write_all(buf);
                        }
                        else {
                            println!("Error when opening file: {:#?}",file);
                        }
                    }

                }
                _=>{}
            }
        }
        Kind::VALUE =>{
            return Some(Value { 
                sym: expr.symbol, 
                val: String::from(expr.value.as_ref().unwrap().as_str()),
                flags: expr.flags.clone(), 
                args: expr.args.clone() }
            );
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
pub fn interpret_program(ast : &AST, delay_exec: bool) -> Option<Vec<Value>>{
    let mut output = Vec::new();
    ast.exprs.iter().for_each(|expr: &Expr|{
        let outCur = interpret_expression(expr,delay_exec);
        if !outCur.is_none(){
            output.push(outCur.unwrap());
        }
        //println!("{:#?}",output);
    });
    return Some(output);
}