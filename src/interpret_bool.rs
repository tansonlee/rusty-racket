use crate::interpret::*;
use crate::interpret_function_call::{interpret_function_call, FunctionCall};
use crate::interpret_num::*;

#[derive(Debug, Clone)]
pub enum Bool {
    Literal(B),
    Variable(V),
    Binary(Box<BinaryBoolExpr>),
    Unary(Box<UnaryBoolExpr>),
    Cmp(Box<CmpBoolExpr>),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone)]
pub enum BinaryBoolOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryBoolOp {
    Not,
}

#[derive(Debug, Clone)]
pub enum CmpBoolOp {
    Lt,
    Eq,
    Gt,
}

#[derive(Debug, Clone)]
pub struct BinaryBoolExpr {
    pub op: BinaryBoolOp,
    pub left: Bool,
    pub right: Bool,
}

#[derive(Debug, Clone)]
pub struct UnaryBoolExpr {
    pub op: UnaryBoolOp,
    pub value: Bool,
}

#[derive(Debug, Clone)]
pub struct CmpBoolExpr {
    pub op: CmpBoolOp,
    pub left: Num,
    pub right: Num,
}

pub fn interpret_bool_expr(expr: &Bool, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    match expr {
        Bool::Literal(x) => *x,
        Bool::Variable(x) => interpret_variable_bool_expr(&x, variable_map),
        Bool::Binary(x) => interpret_binary_bool_expr(&x, variable_map, function_map),
        Bool::Unary(x) => interpret_unary_bool_expr(&x, variable_map, function_map),
        Bool::Cmp(x) => interpret_cmp_bool_expr(&x, variable_map, function_map),
        Bool::FunctionCall(x) => {
            if let Value::Bool(x) = interpret_function_call(&x, variable_map, function_map) {
                x
            } else {
                panic!(
                    "Expected call to function {} to return boolean but returned number instead",
                    x.name
                );
            }
        }
    }
}

fn interpret_variable_bool_expr(expr: &V, variable_map: &VariableMap) -> B {
    match variable_map.get(expr) {
        Some(x) => match x[..] {
            [Value::Bool(y), ..] => y,
            [Value::Num(_), ..] => panic!("Variable expected to be a number. Got a boolean."),
            _ => panic!("Couldn't find value for variable"),
        },
        None => panic!("Undefined variable"),
    }
}

fn interpret_binary_bool_expr(expr: &BinaryBoolExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    match expr.op {
        BinaryBoolOp::And => {
            let left = interpret_bool_expr(&expr.left, variable_map, function_map);
            // Short circuit.
            if left == false {
                return false;
            }
            let right = interpret_bool_expr(&expr.right, variable_map, function_map);
            left && right
        }
        BinaryBoolOp::Or => {
            let left = interpret_bool_expr(&expr.left, variable_map, function_map);
            // Short circuit.
            if left == true {
                return true;
            }
            let right = interpret_bool_expr(&expr.right, variable_map, function_map);
            left || right
        }
    }
}

fn interpret_unary_bool_expr(expr: &UnaryBoolExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    match expr.op {
        UnaryBoolOp::Not => !interpret_bool_expr(&expr.value, variable_map, function_map),
    }
}

fn interpret_cmp_bool_expr(expr: &CmpBoolExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    let left = interpret_num_expr(&expr.left, variable_map, function_map);
    let right = interpret_num_expr(&expr.right, variable_map, function_map);

    match expr.op {
        CmpBoolOp::Lt => left < right,
        CmpBoolOp::Eq => left == right,
        CmpBoolOp::Gt => left > right,
    }
}
