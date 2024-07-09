use crate::interpret::{Value, VariableMap, N, V};

#[derive(PartialEq, Debug)]
pub enum BinaryNumOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Num {
    Literal(N),
    Variable(V),
    Binary(Box<BinaryNumExpr>),
}

#[derive(Debug)]
pub struct BinaryNumExpr {
    pub op: BinaryNumOp,
    pub left: Num,
    pub right: Num,
}

pub fn interpret_num_expr(expr: &Num, variable_map: &VariableMap) -> N {
    match expr {
        Num::Literal(x) => *x,
        Num::Binary(x) => interpret_binary_num_expr(x, variable_map),
        Num::Variable(x) => interpret_variable_num_expr(x, variable_map),
    }
}

fn interpret_binary_num_expr(expr: &BinaryNumExpr, variable_map: &VariableMap) -> N {
    let left = interpret_num_expr(&expr.left, variable_map);
    let right = interpret_num_expr(&expr.right, variable_map);

    // Division by zero!
    if (expr.op == BinaryNumOp::Div) && (right == 0) {
        panic!("Tried to divide by zero: {} / {}", left, right);
    }

    match expr.op {
        BinaryNumOp::Add => left + right,
        BinaryNumOp::Sub => left - right,
        BinaryNumOp::Mul => left * right,
        BinaryNumOp::Div => left / right,
    }
}

fn interpret_variable_num_expr(expr: &V, variable_map: &VariableMap) -> N {
    match variable_map.get(expr) {
        Some(x) => match x[..] {
            [Value::Num(y), ..] => y,
            [Value::Bool(_), ..] => panic!("Variable expected to be a number. Got a boolean."),
            _ => panic!("Couldn't find value for variable"),
        },
        None => panic!("Undefined variable"),
    }
}
