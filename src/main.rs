#![allow(dead_code)]

mod interpret_bool;
mod interpret_num;
mod lexer;
mod parser;

use crate::parser::*;

fn main() {
    println!("{:?}", parse("(& true false)".to_string()));
    println!("Hello, world!");
}

#[cfg(test)]
mod test;
