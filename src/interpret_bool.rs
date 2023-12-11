use crate::interpret::*;
use crate::interpret_num::*;

#[derive(Debug)]
pub enum Bool {
    Literal(B),
    Variable(V),
    Binary(Box<BinaryBoolExpr>),
    Unary(Box<UnaryBoolExpr>),
    Cmp(Box<CmpBoolExpr>),
}

#[derive(Debug)]
pub enum BinaryBoolOp {
    And,
    Or,
}

#[derive(Debug)]
pub enum UnaryBoolOp {
    Not,
}

#[derive(Debug)]
pub enum CmpBoolOp {
    Lt,
    Eq,
    Gt,
}

#[derive(Debug)]
pub struct BinaryBoolExpr {
    pub op: BinaryBoolOp,
    pub left: Bool,
    pub right: Bool,
}

#[derive(Debug)]
pub struct UnaryBoolExpr {
    pub op: UnaryBoolOp,
    pub value: Bool,
}

#[derive(Debug)]
pub struct CmpBoolExpr {
    pub op: CmpBoolOp,
    pub left: Num,
    pub right: Num,
}

pub fn interpret_bool_expr(expr: &Bool, env: &mut Environment) -> B {
    match expr {
        Bool::Literal(x) => *x,
        Bool::Variable(x) => interpret_variable_bool_expr(&x, env),
        Bool::Binary(x) => interpret_binary_bool_expr(&x, env),
        Bool::Unary(x) => interpret_unary_bool_expr(&x, env),
        Bool::Cmp(x) => interpret_cmp_bool_expr(&x, env),
    }
}

fn interpret_variable_bool_expr(expr: &V, env: &mut Environment) -> B {
    match env.variable_map.get(expr) {
        Some(x) => match x {
            Value::Bool(y) => *y,
            Value::Num(_) => panic!("Variable expected to be a number. Got a boolean.")
        },
        None => panic!("Undefined variable")
    }
}

fn interpret_binary_bool_expr(expr: &BinaryBoolExpr, env: &mut Environment) -> B {
    match expr.op {
        BinaryBoolOp::And => {
            let left = interpret_bool_expr(&expr.left, env);
            // Short circuit.
            if left == false {
                return false;
            }
            let right = interpret_bool_expr(&expr.right, env);
            left && right
        }
        BinaryBoolOp::Or => {
            let left = interpret_bool_expr(&expr.left, env);
            // Short circuit.
            if left == true {
                return true;
            }
            let right = interpret_bool_expr(&expr.right, env);
            left || right
        }
    }
}

fn interpret_unary_bool_expr(expr: &UnaryBoolExpr, env: &mut Environment) -> B {
    match expr.op {
        UnaryBoolOp::Not => !interpret_bool_expr(&expr.value, env),
    }
}

fn interpret_cmp_bool_expr(expr: &CmpBoolExpr, env: &mut Environment) -> B {
    let left = interpret_num_expr(&expr.left, env);
    let right = interpret_num_expr(&expr.right, env);

    match expr.op {
        CmpBoolOp::Lt => left < right,
        CmpBoolOp::Eq => left == right,
        CmpBoolOp::Gt => left > right,
    }
}
