#![allow(dead_code)]

mod interpret;
mod interpret_bool;
mod interpret_cond;
mod interpret_function;
mod interpret_function_call;
mod interpret_num;
mod interpret_variable;
mod lexer;
mod parser;

use crate::interpret::interpret_program;

fn main() {
    let program = "
    (define (main) 
        (cond 
            ((< 5 6) (+ 5 10))
            ((= 1 1) (+ 50 100))))
    ";
    println!("FINAL RESULT: {:?}", interpret_program(program.to_string()));
}

#[cfg(test)]
mod test;
