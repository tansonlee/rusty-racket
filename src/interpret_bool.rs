use crate::interpret::*;
use crate::interpret_num::*;

#[derive(Debug)]
pub enum Bool {
    Literal(B),
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

pub fn interpret_bool_expr(expr: Bool) -> B {
    match expr {
        Bool::Literal(x) => x,
        Bool::Binary(x) => interpret_binary_bool_expr(*x),
        Bool::Unary(x) => interpret_unary_bool_expr(*x),
        Bool::Cmp(x) => interpret_cmp_bool_expr(*x),
    }
}

fn interpret_binary_bool_expr(expr: BinaryBoolExpr) -> B {
    match expr.op {
        BinaryBoolOp::And => {
            let left = interpret_bool_expr(expr.left);
            // Short circuit.
            if left == false {
                return false;
            }
            let right = interpret_bool_expr(expr.right);
            left && right
        }
        BinaryBoolOp::Or => {
            let left = interpret_bool_expr(expr.left);
            // Short circuit.
            if left == true {
                return true;
            }
            let right = interpret_bool_expr(expr.right);
            left || right
        }
    }
}

fn interpret_unary_bool_expr(expr: UnaryBoolExpr) -> B {
    match expr.op {
        UnaryBoolOp::Not => !interpret_bool_expr(expr.value),
    }
}

fn interpret_cmp_bool_expr(expr: CmpBoolExpr) -> B {
    let left = interpret_num_expr(expr.left);
    let right = interpret_num_expr(expr.right);

    match expr.op {
        CmpBoolOp::Lt => left < right,
        CmpBoolOp::Eq => left == right,
        CmpBoolOp::Gt => left > right,
    }
}
