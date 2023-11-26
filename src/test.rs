use crate::interpret::*;
use crate::parser::parse;

#[test]
fn number_literal() {
    assert_eq!(interpret(parse("1010101".to_string())), Result::Num(1010101));
    assert_eq!(interpret(parse("0".to_string())), Result::Num(0));
    assert_eq!(interpret(parse("0010".to_string())), Result::Num(10));
    assert_eq!(interpret(parse("-10".to_string())), Result::Num(-10));
    assert_eq!(interpret(parse("-0010".to_string())), Result::Num(-10));
    assert_eq!(interpret(parse("-0".to_string())), Result::Num(0));
}

#[test]
fn boolean_literal() {
    assert_eq!(interpret(parse("false".to_string())), Result::Bool(false));
    assert_eq!(interpret(parse("true".to_string())), Result::Bool(true));
}

#[test]
fn interpret_number_expr() {
    assert_eq!(interpret(parse("(+ 1 2)".to_string())), Result::Num(3));
    assert_eq!(interpret(parse("(+ (- 5 1) 2)".to_string())), Result::Num(6));
    assert_eq!(interpret(parse("(+ (- 5 1) (/ 10 5))".to_string())), Result::Num(6));
    assert_eq!(interpret(parse("(+ 1 (+ 1 (+ 1 (+ 1 (+ 1 (+ 1 0))))))".to_string())), Result::Num(6));
}

#[test]
fn interpret_boolean_expr() {
    assert_eq!(interpret(parse("(& true false)".to_string())), Result::Bool(false));
    assert_eq!(interpret(parse("(& (| false true) (| false true))".to_string())), Result::Bool(true));
    assert_eq!(interpret(parse("(& (| false true) (| (! true) true))".to_string())), Result::Bool(true));
}

