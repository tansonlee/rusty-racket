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
    (define (list-append lst1 lst2)
    (cond
        [(empty? lst1) lst2]
        [true (cons (car lst1) (list-append (cdr lst1) lst2))])) 

(define (list-flatten lst)
    (cond
        [(empty? lst) empty]
        [(list? lst) (list-append (list-flatten (car lst)) (list-flatten (cdr lst)))]
        [true (cons (car lst) (list-flatten (cdr lst)))])) 

(define (main) (list-flatten empty))
        ";

    println!("{:#?}", string_to_tokens(program.to_string()));
    println!("{:#?}", parse_functions(program.to_string()));
    println!("{:?}", interpret_program(program.to_string()));
}

#[cfg(test)]
mod test;
