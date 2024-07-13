use crate::interpret::{FunctionMap, Value, VariableMap, N};
use crate::interpret_function_call::{interpret_function_call, FunctionCall};
use crate::interpret_list::{interpret_car_expr, Car};
use crate::interpret_variable::{interpret_variable_expr, Variable};

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryNumOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Num {
    Literal(N),
    Variable(Variable),
    Binary(Box<BinaryNumExpr>),
    FunctionCall(FunctionCall),
    Car(Car),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryNumExpr {
    pub op: BinaryNumOp,
    pub left: Num,
    pub right: Num,
}

pub fn interpret_num_expr(expr: &Num, variable_map: &mut VariableMap, function_map: &FunctionMap) -> N {
    match expr {
        Num::Literal(x) => *x,
        Num::Binary(x) => interpret_binary_num_expr(x, variable_map, function_map),
        Num::Variable(x) => interpret_variable_num_expr(x, variable_map),
        Num::FunctionCall(x) => {
            if let Value::Num(y) = interpret_function_call(x, variable_map, function_map) {
                y
            } else {
                panic!(
                    "Expected call to function {} to return number but returned boolean instead",
                    x.name
                );
            }
        }
        Num::Car(x) => {
            if let Value::Num(y) = interpret_car_expr(x, variable_map, function_map) {
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

fn interpret_variable_num_expr(expr: &Variable, variable_map: &VariableMap) -> N {
    let res = interpret_variable_expr(expr, variable_map);

    match res {
        Value::Num(x) => x,
        x => panic!("Variable {} does not refer to a num value", x),
    }
}
