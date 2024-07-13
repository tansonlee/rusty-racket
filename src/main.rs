#![allow(dead_code)]

mod interpret;
mod interpret_bool;
mod interpret_cond;
mod interpret_function;
mod interpret_function_call;
mod interpret_list;
mod interpret_num;
mod interpret_variable;
mod lexer;
mod parser;

use crate::{interpret::interpret_program, lexer::string_to_tokens};

fn main() {
    let program = "
        (define (length lst)
            (cond
                [(empty? lst) 0]
                [true (+ 1 (length (cdr lst)))]))
        
        (define (main) (length (list 1)))
        ";

    println!("{:#?}", string_to_tokens(program.to_string()));
    println!("{:?}", interpret_program(program.to_string()));
}

#[cfg(test)]
mod test;
