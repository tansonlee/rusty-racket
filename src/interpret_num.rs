use crate::interpret::{FunctionMap, Value, VariableMap, N};
use crate::interpret_function_call::{interpret_function_call, FunctionCallExpr};
use crate::interpret_list::{interpret_car_expr, CarExpr};
use crate::interpret_variable::{interpret_variable_expr, VariableExpr};

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryNumOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

// All possible expression types that can result in a NumValue.
#[derive(Debug, Clone, PartialEq)]
pub enum NumExpr {
    LiteralNumExpr(N),
    VariableExpr(VariableExpr),
    BinaryNumExpr(Box<BinaryNumExpr>),
    FunctionCallExpr(FunctionCallExpr),
    CarExpr(CarExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryNumExpr {
    pub op: BinaryNumOp,
    pub left: NumExpr,
    pub right: NumExpr,
}

pub fn interpret_num_expr(expr: &NumExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> N {
    match expr {
        NumExpr::LiteralNumExpr(x) => *x,
        NumExpr::BinaryNumExpr(x) => interpret_binary_num_expr(x, variable_map, function_map),
        NumExpr::VariableExpr(x) => interpret_variable_num_expr(x, variable_map),
        NumExpr::FunctionCallExpr(x) => {
            if let Value::NumValue(y) = interpret_function_call(x, variable_map, function_map) {
                y
            } else {
                panic!(
                    "Expected call to function {} to return number but returned something else instead",
                    x.name
                );
            }
        }
        NumExpr::CarExpr(x) => {
            if let Value::NumValue(y) = interpret_car_expr(x, variable_map, function_map) {
                y
            } else {
                panic!()
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

fn interpret_variable_num_expr(expr: &VariableExpr, variable_map: &VariableMap) -> N {
    let res = interpret_variable_expr(expr, variable_map);

    match res {
        Value::NumValue(x) => x,
        x => panic!("Variable {} does not refer to a num value", x),
    }
}
