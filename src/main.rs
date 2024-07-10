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
        (define (fibonacci n)
            (cond
                [(= n 0) 0]
                [(= n 1) 1]
                [true (+ (fibonacci (- n 2)) (fibonacci (- n 1)))]))

        (define (main) (fibonacci 10))
        ";

    println!("{:?}", interpret_program(program.to_string()));
}

#[cfg(test)]
mod test;
