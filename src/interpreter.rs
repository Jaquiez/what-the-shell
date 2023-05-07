use crate::{ast::{Symbol,AST,Kind,Expr}, error};
use crate::error::error;
use std::{process::Command, os::fd::IntoRawFd, io::Write,char};
use std::fs::File;

#[derive(Debug)]
pub struct Value{
    sym: Symbol,
    val: String,
    flags: Vec<String>,
    args: Vec<String>
}
fn interpret_expression(expr: &Expr) -> Option<Value>{
    match expr.kind {
        Kind::EXPR =>{
            match expr.symbol {
                Symbol::NONE =>{
                    let left = interpret_program(expr.left.as_ref().unwrap()).unwrap();
                    if left.len() > 0{
                        let flags = left[0].flags.iter()
                            .map(|flag: &String| String::from("-") + flag);
                        let output = Command::new(&left[0].val)
                            .args(flags)
                            .args(&left[0].args)
                            .output();
                        if output.is_ok() {
                            println!("{:#?}",output);
                            println!("{}",String::from_utf8(output.unwrap().stdout).unwrap());
                        }
                        else if output.is_err(){
                            println!("{:#?}",output);
                        }   
                    } 
                    else{
                        println!("{:#?}",left);
                    }
                }
                Symbol::REDIR_RIGHT =>{
                    let left = interpret_program(expr.left.as_ref().unwrap()).unwrap(); 
                    let right = interpret_program(expr.right.as_ref().unwrap()).unwrap(); 
                    let mut file = File::open(&right[0].val).unwrap();
                }
                _=>{}
            }
        }
        Kind::VALUE =>{
            return Some(Value { 
                sym: Symbol::CMD, 
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
pub fn interpret_program(ast : &AST) -> Option<Vec<Value>>{
    let mut output = Vec::new();
    ast.exprs.iter().for_each(|expr: &Expr|{
        let outCur = interpret_expression(expr);
        if !outCur.is_none(){
            output.push(outCur.unwrap());
            
        }
    });
    return Some(output);
}