#![allow(dead_code)]

mod interpret;
mod interpret_bool;
mod interpret_num;
mod lexer;
mod parser;

use crate::parser::*;
use crate::interpret::interpret;

fn main() {
    println!("{:?}", interpret(parse("(+ 1 false)".to_string())));
}

#[cfg(test)]
mod test;
