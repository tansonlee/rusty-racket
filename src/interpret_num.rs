use crate::interpret::{Environment, Value, N, V};

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

pub fn interpret_num_expr(expr: &Num, env: &mut Environment) -> N {
    match expr {
        Num::Literal(x) => *x,
        Num::Binary(x) => interpret_binary_num_expr(x, env),
        Num::Variable(x) => interpret_variable_num_expr(x, env),
    }
}

fn interpret_binary_num_expr(expr: &BinaryNumExpr, env: &mut Environment) -> N {
    let left = interpret_num_expr(&expr.left, env);
    let right = interpret_num_expr(&expr.right, env);

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

fn interpret_variable_num_expr(expr: &V, env: &mut Environment) -> N {
    match env.variable_map.get(expr) {
        Some(x) => match x[..] {
            [Value::Num(y), ..] => y,
            [Value::Bool(_), ..] => panic!("Variable expected to be a number. Got a boolean."),
            _ => panic!("Couldn't find value for variable"),
        },
        None => panic!("Undefined variable"),
    }
}
