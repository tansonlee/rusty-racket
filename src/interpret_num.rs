use crate::interpret::{FunctionMap, Value, VariableMap, N, V};
use crate::interpret_function_call::{interpret_function_call, FunctionCall};

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryNumOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum Num {
    Literal(N),
    Variable(V),
    Binary(Box<BinaryNumExpr>),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone)]
pub struct BinaryNumExpr {
    pub op: BinaryNumOp,
    pub left: Num,
    pub right: Num,
}

pub fn interpret_num_expr(expr: &Num, mut variable_map: &mut VariableMap, function_map: &FunctionMap) -> N {
    match expr {
        Num::Literal(x) => *x,
        Num::Binary(x) => interpret_binary_num_expr(x, variable_map, function_map),
        Num::Variable(x) => interpret_variable_num_expr(x, variable_map),
        Num::FunctionCall(x) => {
            if let Value::Num(x) = interpret_function_call(x, &mut variable_map, function_map) {
                x
            } else {
                panic!(
                    "Expected call to function {} to return number but returned boolean instead",
                    x.name
                );
            }
        }
    }
}

fn interpret_binary_num_expr(expr: &BinaryNumExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> N {
    let left = interpret_num_expr(&expr.left, variable_map, function_map);
    let right = interpret_num_expr(&expr.right, variable_map, function_map);

    // Division by zero!
    if (expr.op == BinaryNumOp::Div) && (right == 0) {
        panic!("Tried to divide by zero: {} / {}", left, right);
    }

    match expr.op {
        BinaryNumOp::Add => left + right,
        BinaryNumOp::Sub => left - right,
        BinaryNumOp::Mul => left * right,
        BinaryNumOp::Div => left / right,
        BinaryNumOp::Mod => left % right,
    }
}

fn interpret_variable_num_expr(expr: &V, variable_map: &VariableMap) -> N {
    match variable_map.get(expr) {
        Some(x) => match x[..] {
            [.., Value::Num(y)] => y,
            [.., Value::Bool(_)] => panic!("Variable expected to be a number. Got a boolean."),
            _ => panic!("Couldn't find value for variable"),
        },
        None => panic!("Undefined variable"),
    }
}
