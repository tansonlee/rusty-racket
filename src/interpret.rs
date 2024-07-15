use std::collections::HashMap;
use std::fmt;
use std::fs;

use itertools::peek_nth;
use regex::Regex;

use crate::interpret_bool::*;
use crate::interpret_cond::*;
use crate::interpret_function::*;
use crate::interpret_function_call::interpret_function_call;
use crate::interpret_function_call::FunctionCallExpr;
use crate::interpret_list::interpret_car_expr;
use crate::interpret_list::CarExpr;
use crate::interpret_list::{interpret_list_expr, ListExpr};
use crate::interpret_num::*;
use crate::interpret_variable::*;
use crate::parser::{parse, parse_expr};
use crate::tokenizer::string_to_tokens;
use crate::tokenizer::TokenIter;

// Concrete value types that represent results of expression evaluation.
pub type N = i32;
pub type B = bool;
pub type L = ValueList;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    NumValue(N),
    BoolValue(B),
    ListValue(L),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::NumValue(ref n) => write!(f, "{}", n),
            Value::BoolValue(ref b) => write!(f, "{}", b),
            Value::ListValue(ref l) => write!(f, "{}", l),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueList {
    Empty,
    Node(ValueNode),
}

impl fmt::Display for ValueList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueList::Empty => write!(f, "empty"),
            ValueList::Node(node) => write!(f, "({} -> {})", node.data, node.next),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueNode {
    pub data: Box<Value>,
    pub next: Box<ValueList>,
}

// Expressions that are produced from parsing. These are to be interpreted to create Values
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    BoolExpr(BoolExpr),
    NumExpr(NumExpr),
    CondExpr(CondExpr),
    FunctionExpr(FunctionExpr),
    VariableExpr(VariableExpr),
    FunctionCallExpr(FunctionCallExpr),
    ListExpr(ListExpr),
    EmptyExpr(ListExpr),
    CarExpr(CarExpr),
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub parameter_names: Vec<String>,
    pub body: Expr,
}

#[derive(Debug)]
pub struct Environment {
    pub variable_map: HashMap<String, Vec<Value>>,
    pub functions: HashMap<String, FunctionInfo>,
}

pub type VariableMap = HashMap<String, Vec<Value>>;
pub type FunctionMap = HashMap<String, FunctionInfo>;

pub fn parse_functions(program: String) -> FunctionMap {
    // Tokenize.
    let tokens = string_to_tokens(program);
    let mut token_iterator = peek_nth(TokenIter::new(&tokens));

    let mut function_map = HashMap::new();
    // Parse each function and add it to the function map.
    while token_iterator.peek().is_some() {
        if let Expr::FunctionExpr(function) = parse_expr(&mut token_iterator) {
            interpret_function_expr(&function, &mut function_map);
        } else {
            panic!("Invalid program: cannot parse functions");
        }
    }

    function_map
}

pub fn interpret_program(program: String) -> Value {
    // 1. Preprocessor: handle module includes. (can make this smarter in the future)
    let re = Regex::new(r"\(include ([^\)]+)\)").unwrap();
    let mut processed = program;
    // Repeat substitution while possible. Do this because one substitution may
    // substitute in code that contains potential new substitutions.
    loop {
        let new_processed = re
            .replace_all(&processed, |captures: &regex::Captures| {
                let module_name = &captures[1];
                if module_name == "stdlib::list" || module_name == "stdlib::num" {
                    get_module_content(module_name)
                } else {
                    panic!("Unknown module name {}", module_name);
                }
            })
            .to_string();
        if new_processed == processed {
            break;
        }

        processed = new_processed;
    }

    // 2. Interpret all of the functions to fill the function map.
    let function_map = parse_functions(processed);

    // Begin interpreting from the main function.
    interpret(&parse("(main)".to_string()), &mut HashMap::new(), &function_map)
}

#[allow(dead_code)] // Used in tests
pub fn interpret_program_snippet(program: String) -> Value {
    interpret(&parse(program.to_string()), &mut HashMap::new(), &HashMap::new())
}

pub fn interpret(expr: &Expr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> Value {
    match expr {
        Expr::NumExpr(x) => Value::NumValue(interpret_num_expr(&x, variable_map, function_map)),
        Expr::BoolExpr(x) => Value::BoolValue(interpret_bool_expr(&x, variable_map, function_map)),
        Expr::CondExpr(x) => interpret_cond_expr(&x, variable_map, function_map),
        Expr::VariableExpr(x) => interpret_variable_expr(&x, variable_map),
        Expr::FunctionCallExpr(x) => interpret_function_call(&x, variable_map, function_map),
        Expr::CarExpr(x) => interpret_car_expr(x, variable_map, function_map),
        Expr::ListExpr(x) | Expr::EmptyExpr(x) => Value::ListValue(interpret_list_expr(&x, variable_map, function_map)),
        // Function definitions should be interpreted in the previous pass.
        Expr::FunctionExpr(_) => panic!("Encountered function expr"),
    }
}

// module_name is something like 'stdlib::list'
fn get_module_content(module_name: &str) -> String {
    let module_prefix = "stdlib::";
    if !module_name.starts_with(module_prefix) {
        panic!("Invalid module name {}", module_name);
    }

    let module_file = format!("{}.rkt", &module_name[module_prefix.len()..]);

    let program = fs::read_to_string(format!("examples/stdlib/{}", module_file))
        .expect(&format!("Failed to read module file {}", module_name));

    program
}
