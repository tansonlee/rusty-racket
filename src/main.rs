// #![allow(dead_code)]

mod interpret;
mod interpret_bool;
mod interpret_cond;
mod interpret_function;
mod interpret_function_call;
mod interpret_list;
mod interpret_num;
mod interpret_variable;
mod parser;
mod tokenizer;

use std::{env, fs};

use interpret::interpret_program;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let program = fs::read_to_string(file_path).expect(&format!("Unable to read file {}", file_path));

    println!("{}", interpret_program(program.to_string()));
}

#[cfg(test)]
mod test;
