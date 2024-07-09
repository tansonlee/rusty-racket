use std::collections::HashMap;

use crate::{interpret::*, parser::parse};

fn empty_variable_map() -> VariableMap {
    HashMap::new()
}

fn empty_function_map() -> FunctionMap {
    HashMap::new()
}

#[test]
fn number_literal() {
    assert_eq!(interpret_program("1010101".to_string()), Value::Num(1010101));
    assert_eq!(interpret_program("0".to_string()), Value::Num(0));
    assert_eq!(interpret_program("0010".to_string()), Value::Num(10));
    assert_eq!(interpret_program("-10".to_string()), Value::Num(-10));
    assert_eq!(interpret_program("-0010".to_string()), Value::Num(-10));
    assert_eq!(interpret_program("-0".to_string()), Value::Num(0));
}

#[test]
fn boolean_literal() {
    assert_eq!(interpret_program("false".to_string()), Value::Bool(false));
    assert_eq!(interpret_program("true".to_string()), Value::Bool(true));
}

#[test]
fn interpret_number_expr() {
    assert_eq!(interpret_program("(+ 1 2)".to_string(),), Value::Num(3));
    assert_eq!(interpret_program("(+ (- 5 1) 2)".to_string(),), Value::Num(6));
    assert_eq!(interpret_program("(+ (- 5 1) (/ 10 5))".to_string(),), Value::Num(6));
    assert_eq!(
        interpret_program("(+ 1 (+ 1 (+ 1 (+ 1 (+ 1 (+ 1 0))))))".to_string(),),
        Value::Num(6)
    );
}

#[test]
fn interpret_binary_boolean_expr() {
    assert_eq!(interpret_program("(& true false)".to_string(),), Value::Bool(false));
    assert_eq!(
        interpret_program("(& (| false true) (| false true))".to_string()),
        Value::Bool(true)
    );
}

#[test]
fn interpret_unary_boolean_expr() {
    assert_eq!(interpret_program("(! false)".to_string()), Value::Bool(true));
    assert_eq!(interpret_program("(! true)".to_string()), Value::Bool(false));
}

#[test]
fn interpret_cmp_boolean_expr() {
    assert_eq!(interpret_program("(= (* 5 5) (/ 100 4))".to_string()), Value::Bool(true));
    assert_eq!(interpret_program("(< 3 10)".to_string()), Value::Bool(true));
    assert_eq!(interpret_program("(> 3 10)".to_string()), Value::Bool(false));
    assert_eq!(interpret_program("(| (< 5 10) (< 10 5))".to_string()), Value::Bool(true));
}

#[test]
fn interpret_cond_expr() {
    let cond1 = "
    (cond 
      ((= 1 2) 1)
      ((= 1 3) 2)
      ((= 1 1) 3))
    ";
    assert_eq!(interpret_program_snippet(cond1.to_string()), Value::Num(3));

    let cond2 = "
    (cond 
      ((< 5 (- 20 2)) 1)
      ((= 5 18) 2)
      ((> 5 18) 3))
    ";
    assert_eq!(interpret_program_snippet(cond2.to_string()), Value::Num(1));
}

#[test]
#[should_panic]
fn interpret_cond_expr_panic() {
    let cond = "
    (cond 
      ((= 1 2) 1)
      ((= 1 3) 2)
      ((= 1 4) 3))
    ";
    interpret_program(cond.to_string());
}

#[test]
fn interpret_function() {
    let program = "
    (define (main) 0)
    ";
    assert_eq!(interpret_program(program.to_string()), Value::Num(0));

    let program2 = "
    (define (main) 
        (cond 
            ((< 5 6) (+ 5 10))
            ((= 1 1) (+ 50 100))))
    ";

    assert_eq!(interpret_program(program2.to_string()), Value::Num(15));
}

#[test]
fn interpret_variable() {
    let mut variable_map = HashMap::from([
        ("a".to_string(), Vec::from([Value::Num(10)])),
        ("b".to_string(), Vec::from([Value::Num(5)])),
    ]);
    let function_map = HashMap::new();

    assert_eq!(
        interpret(&parse("a".to_string()), &mut variable_map, &function_map),
        Value::Num(10)
    );
    assert_eq!(
        interpret(&parse("(+ a b)".to_string()), &mut variable_map, &function_map),
        Value::Num(15)
    );
    let cond = "
    (cond
        ((= a 5) 1)
        ((= b 5) 2)
    )";
    assert_eq!(
        interpret(&parse(cond.to_string()), &mut variable_map, &function_map),
        Value::Num(2)
    );
}
