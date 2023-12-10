#![allow(dead_code)]

mod interpret;
mod interpret_bool;
mod interpret_num;
mod interpret_cond;
mod interpret_function;
mod interpret_function_call;
mod interpret_variable;
mod lexer;
mod parser;

use crate::interpret::interpret;
use crate::parser::*;

fn main() {
    let program = "
    (define (add a b) (+ a b))
    ";
    println!("{:?}", parse(program.to_string()));
}

#[cfg(test)]
mod test;
