use std::collections::HashMap;

use crate::interpret_bool::*;
use crate::interpret_cond::*;
use crate::interpret_function::*;
use crate::interpret_function_call::interpret_function_call;
use crate::interpret_function_call::FunctionCall;
use crate::interpret_num::*;
use crate::interpret_variable::*;
use crate::parser::parse;

pub type N = i32;
pub type B = bool;
pub type V = String;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Num(N),
    Bool(B),
}

#[derive(Debug, Clone)]
pub enum Expr {
    BoolExpr(Bool),
    NumExpr(Num),
    CondExpr(Cond),
    FunctionExpr(Function),
    VariableExpr(Variable),
    FunctionCallExpr(FunctionCall),
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

fn parse_functions(program: &String) -> FunctionMap {
    // 1. Separate each function.
    // let function_strings = Vec::new();
    let expr = parse(program.to_string());
    if let Expr::FunctionExpr(function) = expr {
        let mut function_map = HashMap::new();
        interpret_function_expr(&function, &mut function_map);
        function_map
    } else {
        panic!("Malformed program");
    }
}

pub fn interpret_program(program: String) -> Value {
    // First interpret all of the functions to fill the function map.
    let function_map = parse_functions(&program);

    // Begin interpreting from the main function.
    interpret(&parse("(main)".to_string()), &mut HashMap::new(), &function_map)
}

pub fn interpret_program_snippet(program: String) -> Value {
    interpret(&parse(program.to_string()), &mut HashMap::new(), &HashMap::new())
}

pub fn interpret(expr: &Expr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> Value {
    match expr {
        Expr::NumExpr(x) => Value::Num(interpret_num_expr(&x, variable_map)),
        Expr::BoolExpr(x) => Value::Bool(interpret_bool_expr(&x, variable_map)),
        Expr::CondExpr(x) => interpret_cond_expr(&x, variable_map, function_map),
        Expr::VariableExpr(x) => interpret_variable_expr(&x, variable_map),
        Expr::FunctionCallExpr(x) => interpret_function_call(&x, variable_map, function_map),
        // Function definitions should be interpreted in the previous pass.
        Expr::FunctionExpr(_) => panic!("Encountered function expr"),
    }
}
