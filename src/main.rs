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

use interpret::parse_functions;

use crate::{interpret::interpret_program, lexer::string_to_tokens};

fn main() {
    let program = "
        (define (list-double lst)
            (cond
                [(empty? lst) empty]
                [true (cons (* 2 (car lst)) (list-double (cdr lst)))]))
        
        (define (main) (list-double (list 1)))
        ";

    println!("{:#?}", string_to_tokens(program.to_string()));
    println!("{:#?}", parse_functions(program.to_string()));
    println!("{:?}", interpret_program(program.to_string()));
}

#[cfg(test)]
mod test;
