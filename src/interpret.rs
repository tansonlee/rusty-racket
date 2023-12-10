use crate::interpret_bool::*;
use crate::interpret_function::*;
use crate::interpret_num::*;
use crate::interpret_cond::*;
use crate::interpret_variable::*;

pub type N = i32;
pub type B = bool;
pub type Variable = String;

#[derive(PartialEq, Debug)]
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

pub fn interpret(expr: &Expr) -> Value {
    match expr {
        Expr::NumExpr(x) => Value::Num(interpret_num_expr(&x)),
        Expr::BoolExpr(x) => Value::Bool(interpret_bool_expr(&x)),
        Expr::CondExpr(x) => interpret_cond_expr(&x),
        Expr::FunctionExpr(x) => interpret_function_expr(&x),
        Expr::VariableExpr(x) => interpret_variable_expr(&x),
    }
}
