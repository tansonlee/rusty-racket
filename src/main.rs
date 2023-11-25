
type T = i32;

#[allow(dead_code)]
enum BinaryNumOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[allow(dead_code)]
enum BinaryNum {
    NumLiteral(T),
    NumBinary(Box<BinaryNumExpr>),
}

struct BinaryNumExpr {
    op: BinaryNumOp,
    left: BinaryNum,
    right: BinaryNum,
}



fn interpret_binary_num_expr(expr: BinaryNumExpr) -> T {
    let left = {
        match expr.left {
            BinaryNum::NumLiteral(x) => x,
            BinaryNum::NumBinary(x) => interpret_binary_num_expr(*x),
        }
    };

    let right = {
        match expr.right {
            BinaryNum::NumLiteral(x) => x,
            BinaryNum::NumBinary(x) => interpret_binary_num_expr(*x),
        }
    };

    match expr.op {
        BinaryNumOp::Add => left + right,
        BinaryNumOp::Sub => left - right,
        BinaryNumOp::Mul => left * right,
        BinaryNumOp::Div => left / right,
       
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn leaf_binary_num_expr_10() -> BinaryNumExpr {
        BinaryNumExpr {
            op: BinaryNumOp::Add,
            left: BinaryNum::NumLiteral(4),
            right: BinaryNum::NumLiteral(6),
        }
    }

    fn complex_binary_num_expr_30() -> BinaryNumExpr {
        BinaryNumExpr {
            op: BinaryNumOp::Mul,
            left: BinaryNum::NumBinary(Box::new(leaf_binary_num_expr_10())),
            right: BinaryNum::NumLiteral(3),
        }
    }

    fn complex_binary_num_expr_100() -> BinaryNumExpr {
        BinaryNumExpr {
            op: BinaryNumOp::Mul,
            left: BinaryNum::NumBinary(Box::new(leaf_binary_num_expr_10())),
            right: BinaryNum::NumBinary(Box::new(leaf_binary_num_expr_10())),
        }
    }

    #[test]
    fn binary_expr() {
        assert_eq!(interpret_binary_num_expr(leaf_binary_num_expr_10()), 10);
        assert_eq!(interpret_binary_num_expr(complex_binary_num_expr_30()), 30);
        assert_eq!(interpret_binary_num_expr(complex_binary_num_expr_100()), 100);
    }
}