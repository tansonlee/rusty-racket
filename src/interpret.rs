use std::collections::HashMap;
use std::fmt;

use itertools::peek_nth;

use crate::interpret_bool::*;
use crate::interpret_cond::*;
use crate::interpret_function::*;
use crate::interpret_function_call::interpret_function_call;
use crate::interpret_function_call::FunctionCall;
use crate::interpret_list::Car;
use crate::interpret_list::{interpret_car_expr, interpret_list_expr, List};
use crate::interpret_num::*;
use crate::interpret_variable::*;
use crate::lexer::string_to_tokens;
use crate::lexer::TokenIter;
use crate::parser::{parse, parse_expr};

#[derive(Debug, Clone, PartialEq)]
pub struct ValueNode {
    pub data: Box<Value>,
    pub next: Box<ValueList>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueList {
    Empty,
    Node(ValueNode),
}

pub type N = i32;
pub type B = bool;
pub type L = ValueList;
pub type V = String;

impl fmt::Display for ValueList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueList::Empty => write!(f, "empty"),
            ValueList::Node(node) => write!(f, "({:#?} -> {:#?})", node.data, node.next),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Num(N),
    Bool(B),
    List(L),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Num(ref n) => write!(f, "{}", n),
            Value::Bool(ref b) => write!(f, "{}", b),
            Value::List(ref l) => write!(f, "{}", l),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    BoolExpr(Bool),
    NumExpr(Num),
    CondExpr(Cond),
    FunctionExpr(Function),
    VariableExpr(Variable),
    FunctionCallExpr(FunctionCall),
    ListExpr(List),
    ConsExpr(List),
    EmptyExpr(List),
    CarExpr(Car),
    CdrExpr(List),
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

fn parse_functions(program: String) -> FunctionMap {
    let tokens = string_to_tokens(program);
    let mut token_iterator = peek_nth(TokenIter::new(&tokens));

    let mut function_map = HashMap::new();
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
    // First interpret all of the functions to fill the function map.
    let function_map = parse_functions(program);

    // Begin interpreting from the main function.
    interpret(&parse("(main)".to_string()), &mut HashMap::new(), &function_map)
}

pub fn interpret_program_snippet(program: String) -> Value {
    interpret(&parse(program.to_string()), &mut HashMap::new(), &HashMap::new())
}

pub fn interpret(expr: &Expr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> Value {
    match expr {
        Expr::NumExpr(x) => Value::Num(interpret_num_expr(&x, variable_map, function_map)),
        Expr::BoolExpr(x) => Value::Bool(interpret_bool_expr(&x, variable_map, function_map)),
        Expr::CondExpr(x) => interpret_cond_expr(&x, variable_map, function_map),
        Expr::VariableExpr(x) => interpret_variable_expr(&x, variable_map),
        Expr::FunctionCallExpr(x) => interpret_function_call(&x, variable_map, function_map),
        Expr::ListExpr(x) | Expr::ConsExpr(x) | Expr::EmptyExpr(x) | Expr::CdrExpr(x) => {
            Value::List(interpret_list_expr(&x, variable_map, function_map))
        }
        Expr::CarExpr(x) => interpret_car_expr(&x, variable_map, function_map),
        // Function definitions should be interpreted in the previous pass.
        Expr::FunctionExpr(_) => panic!("Encountered function expr"),
    }
}
