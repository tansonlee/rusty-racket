use std::collections::HashMap;

use crate::interpret_bool::*;
use crate::interpret_cond::*;
use crate::interpret_function::*;
use crate::interpret_num::*;
use crate::interpret_variable::*;

pub type N = i32;
pub type B = bool;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Num(N),
    Bool(B),
}

#[derive(Debug)]
pub enum Expr {
    BoolExpr(Bool),
    NumExpr(Num),
    CondExpr(Cond),
    FunctionExpr(Function),
    VariableExpr(Variable),
}

#[derive(Debug)]
pub struct Environment {
    pub variable_map: HashMap<String, Value>,
}

pub fn interpret(expr: &Expr, env: &mut Environment) -> Value {
    match expr {
        Expr::NumExpr(x) => Value::Num(interpret_num_expr(&x, env)),
        Expr::BoolExpr(x) => Value::Bool(interpret_bool_expr(&x, env)),
        Expr::CondExpr(x) => interpret_cond_expr(&x, env),
        Expr::FunctionExpr(x) => interpret_function_expr(&x, env),
        Expr::VariableExpr(x) => interpret_variable_expr(&x, env),
    }
}
