pub type N = i32;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum BinaryNumOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Num {
    Literal(N),
    Binary(Box<BinaryNumExpr>),
}

#[derive(Debug)]
pub struct BinaryNumExpr {
    pub op: BinaryNumOp,
    pub left: Num,
    pub right: Num,
}

pub fn interpret_num_expr(expr: Num) -> N {
    match expr {
        Num::Literal(x) => x,
        Num::Binary(x) => interpret_binary_num_expr(*x),
    }
}

fn interpret_binary_num_expr(expr: BinaryNumExpr) -> N {
    let left = {
        match expr.left {
            Num::Literal(x) => x,
            Num::Binary(x) => interpret_binary_num_expr(*x),
        }
    };

    let right = {
        match expr.right {
            Num::Literal(x) => x,
            Num::Binary(x) => interpret_binary_num_expr(*x),
        }
    };

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
