use crate::interpret_bool::*;
use crate::interpret_num::*;
use crate::lexer::*;
use crate::parser::parse;

fn num_literal_0() -> Num {
    Num::Literal(0)
}
fn num_literal_4() -> Num {
    Num::Literal(4)
}
fn num_literal_6() -> Num {
    Num::Literal(6)
}

fn binary_num_expr_div_0() -> Num {
    Num::Binary(Box::new(BinaryNumExpr {
        op: BinaryNumOp::Div,
        left: num_literal_4(),
        right: num_literal_0(),
    }))
}

fn leaf_binary_num_expr_10() -> Num {
    Num::Binary(Box::new(BinaryNumExpr {
        op: BinaryNumOp::Add,
        left: num_literal_4(),
        right: num_literal_6(),
    }))
}

fn complex_binary_num_expr_6() -> Num {
    Num::Binary(Box::new(BinaryNumExpr {
        op: BinaryNumOp::Sub,
        left: leaf_binary_num_expr_10(),
        right: num_literal_4(),
    }))
}

fn complex_binary_num_expr_40() -> Num {
    Num::Binary(Box::new(BinaryNumExpr {
        op: BinaryNumOp::Mul,
        left: leaf_binary_num_expr_10(),
        right: num_literal_4(),
    }))
}

fn complex_binary_num_expr_100() -> Num {
    Num::Binary(Box::new(BinaryNumExpr {
        op: BinaryNumOp::Mul,
        left: leaf_binary_num_expr_10(),
        right: leaf_binary_num_expr_10(),
    }))
}

#[test]
fn num_expr() {
    assert_eq!(interpret_num_expr(leaf_binary_num_expr_10()), 10);
    assert_eq!(interpret_num_expr(complex_binary_num_expr_6()), 6);
    assert_eq!(interpret_num_expr(complex_binary_num_expr_40()), 40);
    assert_eq!(interpret_num_expr(complex_binary_num_expr_100()), 100);
}

#[test]
#[should_panic]
fn num_expr_div_0() {
    interpret_num_expr(binary_num_expr_div_0());
}

fn bool_literal_true() -> Bool {
    Bool::Literal(true)
}
fn bool_literal_false() -> Bool {
    Bool::Literal(false)
}

fn binary_bool_expr_true() -> Bool {
    Bool::Binary(Box::new(BinaryBoolExpr {
        op: BinaryBoolOp::Or,
        left: bool_literal_true(),
        right: bool_literal_false(),
    }))
}

fn binary_bool_expr_false() -> Bool {
    Bool::Binary(Box::new(BinaryBoolExpr {
        op: BinaryBoolOp::And,
        left: bool_literal_true(),
        right: bool_literal_false(),
    }))
}

fn complex_binary_bool_expr_true() -> Bool {
    Bool::Binary(Box::new(BinaryBoolExpr {
        op: BinaryBoolOp::Or,
        left: binary_bool_expr_false(),
        right: binary_bool_expr_true(),
    }))
}

fn unary_bool_expr_true() -> Bool {
    Bool::Unary(Box::new(UnaryBoolExpr {
        op: UnaryBoolOp::Not,
        value: bool_literal_false(),
    }))
}

#[test]
fn bool_expr() {
    assert_eq!(interpret_bool_expr(bool_literal_true()), true);
    assert_eq!(interpret_bool_expr(bool_literal_false()), false);
    assert_eq!(interpret_bool_expr(complex_binary_bool_expr_true()), true);
    assert_eq!(interpret_bool_expr(unary_bool_expr_true()), true);
}

#[test]
fn lexer() {
    assert!(
        string_to_tokens("(+ 1 2)".to_string())
            == vec![
                Token {
                    kind: TokenKind::OpenParen,
                    text: "(".to_string()
                },
                Token {
                    kind: TokenKind::Plus,
                    text: "+".to_string()
                },
                Token {
                    kind: TokenKind::Number,
                    text: "1".to_string()
                },
                Token {
                    kind: TokenKind::Number,
                    text: "2".to_string()
                },
                Token {
                    kind: TokenKind::CloseParen,
                    text: ")".to_string()
                },
            ]
    )
}

#[test]
fn interpreter() {
    assert_eq!(interpret(parse("(+ 1 2)".to_string())), 3);
}