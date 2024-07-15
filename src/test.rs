use std::{collections::HashMap, env, fs, path::PathBuf};

use itertools::peek_nth;

use crate::{
    interpret::*,
    parser::{parse, parse_num_expr},
    tokenizer::{string_to_tokens, TokenIter},
};

fn get_example_program(file: &str) -> String {
    let curr_dir = env::current_dir().expect("Failed to get current directory");
    let mut path = PathBuf::from(&curr_dir);
    path.push(format!("examples/{}", file));

    let program = fs::read_to_string(&path).expect(format!("Failed to read file at {}", path.to_str().unwrap()).as_str());

    program
}

#[test]
fn number_literal() {
    assert_eq!(interpret_program_snippet("1010101".to_string()), Value::NumValue(1010101));
    assert_eq!(interpret_program_snippet("0".to_string()), Value::NumValue(0));
    assert_eq!(interpret_program_snippet("0010".to_string()), Value::NumValue(10));
    assert_eq!(interpret_program_snippet("-10".to_string()), Value::NumValue(-10));
    assert_eq!(interpret_program_snippet("-0010".to_string()), Value::NumValue(-10));
    assert_eq!(interpret_program_snippet("-0".to_string()), Value::NumValue(0));
}

#[test]
fn boolean_literal() {
    assert_eq!(interpret_program_snippet("false".to_string()), Value::BoolValue(false));
    assert_eq!(interpret_program_snippet("true".to_string()), Value::BoolValue(true));
}

#[test]
fn interpret_number_expr() {
    assert_eq!(interpret_program_snippet("(+ 1 2)".to_string(),), Value::NumValue(3));
    assert_eq!(interpret_program_snippet("(+ (- 5 1) 2)".to_string(),), Value::NumValue(6));
    assert_eq!(
        interpret_program_snippet("(+ (- 5 1) (/ 10 5))".to_string(),),
        Value::NumValue(6)
    );
    assert_eq!(
        interpret_program_snippet("(+ 1 (+ 1 (+ 1 (+ 1 (+ 1 (+ 1 0))))))".to_string(),),
        Value::NumValue(6)
    );
    assert_eq!(interpret_program_snippet("(% 11 10)".to_string(),), Value::NumValue(1));
}

#[test]
fn interpret_binary_boolean_expr() {
    assert_eq!(
        interpret_program_snippet("(& true false)".to_string(),),
        Value::BoolValue(false)
    );
    assert_eq!(
        interpret_program_snippet("(& (| false true) (| false true))".to_string()),
        Value::BoolValue(true)
    );
}

#[test]
fn interpret_unary_boolean_expr() {
    assert_eq!(interpret_program_snippet("(! false)".to_string()), Value::BoolValue(true));
    assert_eq!(interpret_program_snippet("(! true)".to_string()), Value::BoolValue(false));
}

#[test]
fn interpret_cmp_boolean_expr() {
    assert_eq!(
        interpret_program_snippet("(= (* 5 5) (/ 100 4))".to_string()),
        Value::BoolValue(true)
    );
    assert_eq!(interpret_program_snippet("(< 3 10)".to_string()), Value::BoolValue(true));
    assert_eq!(interpret_program_snippet("(> 3 10)".to_string()), Value::BoolValue(false));
    assert_eq!(
        interpret_program_snippet("(| (< 5 10) (< 10 5))".to_string()),
        Value::BoolValue(true)
    );
}

#[test]
fn interpret_cond_expr() {
    let cond1 = "
    (cond 
      ((= 1 2) 1)
      ((= 1 3) 2)
      ((= 1 1) 3))
    ";
    assert_eq!(interpret_program_snippet(cond1.to_string()), Value::NumValue(3));

    let cond2 = "
    (cond 
      ((< 5 (- 20 2)) 1)
      ((= 5 18) 2)
      ((> 5 18) 3))
    ";
    assert_eq!(interpret_program_snippet(cond2.to_string()), Value::NumValue(1));
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
    interpret_program_snippet(cond.to_string());
}

#[test]
fn interpret_function() {
    let program = "
    (define (main) 0)
    ";
    assert_eq!(interpret_program(program.to_string()), Value::NumValue(0));

    let program2 = "
    (define (main) 
        (cond 
            ((< 5 6) (+ 5 10))
            ((= 1 1) (+ 50 100))))
    ";
    assert_eq!(interpret_program(program2.to_string()), Value::NumValue(15));

    let program3 = "
    (define (add a b) (+ a b))
    (define (sub a b) (- a b))
    (define (main) 
        (cond 
            ((< (sub 6 1) 6) (add 5 10))
            ((= 1 1) (+ 50 100))))
    ";
    assert_eq!(interpret_program(program3.to_string()), Value::NumValue(15));

    let tokens = string_to_tokens("(a 1)".to_string());
    let mut token_iterator = peek_nth(TokenIter::new(&tokens));
    parse_num_expr(&mut token_iterator);

    let program4 = "
    (define (identity a) a)
    (define (main) (+ (identity 5) (identity 5)))
    ";
    assert_eq!(interpret_program(program4.to_string()), Value::NumValue(10));
}

#[test]
fn interpret_variable() {
    let mut variable_map = HashMap::from([
        ("a".to_string(), Vec::from([Value::NumValue(10)])),
        ("b".to_string(), Vec::from([Value::NumValue(5)])),
    ]);
    let function_map = HashMap::new();

    assert_eq!(
        interpret(&parse("a".to_string()), &mut variable_map, &function_map),
        Value::NumValue(10)
    );
    assert_eq!(
        interpret(&parse("(+ a b)".to_string()), &mut variable_map, &function_map),
        Value::NumValue(15)
    );
    let cond = "
    (cond
        ((= a 5) 1)
        ((= b 5) 2)
    )";
    assert_eq!(
        interpret(&parse(cond.to_string()), &mut variable_map, &function_map),
        Value::NumValue(2)
    );
}

#[test]
fn interpret_list() {
    assert_eq!(
        interpret_program_snippet("(list 1)".to_string()),
        Value::ListValue(ValueList::Node(ValueNode {
            data: Box::new(Value::NumValue(1)),
            next: Box::new(ValueList::Empty),
        }))
    );

    assert_eq!(
        interpret_program_snippet("(list 1 2)".to_string()),
        Value::ListValue(ValueList::Node(ValueNode {
            data: Box::new(Value::NumValue(1)),
            next: Box::new(ValueList::Node(ValueNode {
                data: Box::new(Value::NumValue(2)),
                next: Box::new(ValueList::Empty),
            })),
        }))
    );

    assert_eq!(
        interpret_program_snippet("empty".to_string()),
        Value::ListValue(ValueList::Empty)
    );

    assert_eq!(
        interpret_program_snippet("(cons 1 empty)".to_string()),
        interpret_program_snippet("(list 1)".to_string())
    );

    assert_eq!(
        interpret_program_snippet("(cons 1 (cons 2 empty))".to_string()),
        interpret_program_snippet("(list 1 2)".to_string())
    );
}

#[test]
fn interpret_car_cdr() {
    assert_eq!(
        interpret_program_snippet("(car (list 1))".to_string()),
        interpret_program_snippet("1".to_string())
    );
    assert_eq!(
        interpret_program_snippet("(car (list 1 2 3 4 5))".to_string()),
        interpret_program_snippet("1".to_string())
    );
    assert_eq!(
        interpret_program_snippet("(cdr (list 1 2 3 4 5))".to_string()),
        interpret_program_snippet("(list 2 3 4 5)".to_string())
    );
    assert_eq!(
        interpret_program_snippet("(cdr (list 1))".to_string()),
        interpret_program_snippet("empty".to_string())
    );

    assert_eq!(
        interpret_program_snippet("(car (cons 1 (cons 2 (cons 3 (cons 4 (cons 5 empty))))))".to_string()),
        interpret_program_snippet("1".to_string())
    );
    assert_eq!(
        interpret_program_snippet("(cdr (cons 1 (cons 2 (cons 3 (cons 4 (cons 5 empty))))))".to_string()),
        interpret_program_snippet("(list 2 3 4 5)".to_string())
    );
}

#[test]
fn interpret_empty_huh() {
    assert_eq!(
        interpret_program_snippet("(empty? empty)".to_string()),
        Value::BoolValue(true)
    );
    assert_eq!(
        interpret_program_snippet("(empty? (list 1))".to_string()),
        Value::BoolValue(false)
    );

    assert_eq!(
        interpret_program_snippet("(empty? (cdr (list 1)))".to_string()),
        Value::BoolValue(true)
    );

    assert_eq!(
        interpret_program_snippet("(empty? (cdr (list 1 2)))".to_string()),
        Value::BoolValue(false)
    );
}

#[test]
fn interpret_list_huh() {
    assert_eq!(interpret_program_snippet("(list? empty)".to_string()), Value::BoolValue(true));
    assert_eq!(
        interpret_program_snippet("(list? (list 1))".to_string()),
        Value::BoolValue(true)
    );

    assert_eq!(
        interpret_program_snippet("(list? (cdr (list 1)))".to_string()),
        Value::BoolValue(true)
    );

    assert_eq!(
        interpret_program_snippet("(list? (cdr (list 1 2)))".to_string()),
        Value::BoolValue(true)
    );

    assert_eq!(interpret_program_snippet("(list? 5)".to_string()), Value::BoolValue(false));

    assert_eq!(interpret_program_snippet("(list? true)".to_string()), Value::BoolValue(false));
    assert_eq!(interpret_program_snippet("(list? true)".to_string()), Value::BoolValue(false));
}

#[test]
fn test_median() {
    let program = get_example_program("median.rkt");
    assert_eq!(interpret_program(program.to_string()), Value::NumValue(2));
}

#[test]
fn test_factorial() {
    let program = get_example_program("factorial.rkt");
    assert_eq!(interpret_program(program), Value::NumValue(1 + 2 + 3628800));
}

#[test]
fn test_fibonacci() {
    let program = get_example_program("fibonacci.rkt");
    assert_eq!(interpret_program(program), Value::NumValue(0 + 1 + 5 + 55));
}

#[test]
fn test_list_length() {
    fn list_length_program_factory(s: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::length {}))
        ",
            s
        )
    }

    assert_eq!(interpret_program(list_length_program_factory("empty")), Value::NumValue(0));
    assert_eq!(interpret_program(list_length_program_factory("(list 1)")), Value::NumValue(1));
    assert_eq!(
        interpret_program(list_length_program_factory("(list 1 2 3 4 5)")),
        Value::NumValue(5)
    )
}

#[test]
fn test_list_sum() {
    assert_eq!(interpret_program(get_example_program("list_sum.rkt")), Value::NumValue(15))
}

#[test]
fn test_list_reverse() {
    fn list_reverse_program_factory(s: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::reverse {}))
        ",
            s
        )
    }

    assert_eq!(
        interpret_program(list_reverse_program_factory("empty")),
        interpret_program_snippet("empty".to_string())
    );
    assert_eq!(
        interpret_program(list_reverse_program_factory("(list 1)")),
        interpret_program_snippet("(list 1)".to_string())
    );
    assert_eq!(
        interpret_program(list_reverse_program_factory("(list 1 2 3 4 5 6)")),
        interpret_program_snippet("(list 6 5 4 3 2 1)".to_string())
    );
}

#[test]
fn test_list_filter_even() {
    assert_eq!(
        interpret_program(get_example_program("list_filter_even.rkt")),
        interpret_program_snippet("(list 1 3 5 7)".to_string())
    );
}

#[test]
fn test_list_contains() {
    fn list_contains_program_factory(list: &str, val: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::contains {} {}))
            ",
            list, val
        )
    }

    assert_eq!(
        interpret_program(list_contains_program_factory("empty", "5")),
        Value::BoolValue(false)
    );
    assert_eq!(
        interpret_program(list_contains_program_factory("(list 1 3 5)", "10")),
        Value::BoolValue(false)
    );
    assert_eq!(
        interpret_program(list_contains_program_factory("(list 1 3 5)", "1")),
        Value::BoolValue(true)
    );
}

#[test]
fn test_list_append() {
    fn list_append_program_factory(list1: &str, list2: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::append {} {}))
            ",
            list1, list2
        )
    }

    assert_eq!(
        interpret_program(list_append_program_factory("empty", "empty")),
        interpret_program_snippet("empty".to_string())
    );
    assert_eq!(
        interpret_program(list_append_program_factory("empty", "(list 1 2 3)")),
        interpret_program_snippet("(list 1 2 3)".to_string())
    );
    assert_eq!(
        interpret_program(list_append_program_factory("(list 1 2 3)", "empty")),
        interpret_program_snippet("(list 1 2 3)".to_string())
    );
    assert_eq!(
        interpret_program(list_append_program_factory("(list 1 2 3)", "(list 4 5 6)")),
        interpret_program_snippet("(list 1 2 3 4 5 6)".to_string())
    );
}

#[test]
fn test_list_flatten() {
    assert_eq!(
        interpret_program(get_example_program("list_flatten.rkt")),
        interpret_program_snippet("(list 1 2 3 4 5 6 7 8 9)".to_string())
    );
}

#[test]
fn test_list_take() {
    fn list_take_program_factory(list: &str, val: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::take {} {}))
            ",
            list, val
        )
    }

    assert_eq!(
        interpret_program(list_take_program_factory("empty", "0")),
        interpret_program_snippet("empty".to_string())
    );
    assert_eq!(
        interpret_program(list_take_program_factory("(list 1 2 3)", "3")),
        interpret_program_snippet("(list 1 2 3)".to_string())
    );
    assert_eq!(
        interpret_program(list_take_program_factory("(list 1 2 3 4 5 6)", "3")),
        interpret_program_snippet("(list 1 2 3)".to_string())
    );
}

#[test]
fn test_list_drop() {
    fn list_drop_program_factory(list: &str, val: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::drop {} {}))
            ",
            list, val
        )
    }

    assert_eq!(
        interpret_program(list_drop_program_factory("empty", "0")),
        interpret_program_snippet("empty".to_string())
    );
    assert_eq!(
        interpret_program(list_drop_program_factory("(list 1 2 3)", "3")),
        interpret_program_snippet("empty".to_string())
    );
    assert_eq!(
        interpret_program(list_drop_program_factory("(list 1 2 3 4 5 6)", "3")),
        interpret_program_snippet("(list 4 5 6)".to_string())
    );
}

#[test]
fn test_list_min() {
    fn list_min_program_factory(list: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::min {}))
            ",
            list
        )
    }

    assert_eq!(
        interpret_program(list_min_program_factory("(list 1 2 3)")),
        interpret_program_snippet("1".to_string())
    );
    assert_eq!(
        interpret_program(list_min_program_factory("(list 3 2 1)")),
        interpret_program_snippet("1".to_string())
    );
}

#[test]
fn test_list_max() {
    fn list_min_program_factory(list: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::max {}))
            ",
            list
        )
    }

    assert_eq!(
        interpret_program(list_min_program_factory("(list 1 2 3)")),
        interpret_program_snippet("3".to_string())
    );
    assert_eq!(
        interpret_program(list_min_program_factory("(list 3 2 1)")),
        interpret_program_snippet("3".to_string())
    );
}

#[test]
fn test_list_sort() {
    fn list_sort_program_factory(list: &str) -> String {
        format!(
            "
        (include stdlib::list)
        (define (main) (list::sort {}))
            ",
            list
        )
    }

    assert_eq!(
        interpret_program(list_sort_program_factory("empty")),
        interpret_program_snippet("empty".to_string())
    );
    assert_eq!(
        interpret_program(list_sort_program_factory("(list 1 2 3)")),
        interpret_program_snippet("(list 1 2 3)".to_string())
    );
    assert_eq!(
        interpret_program(list_sort_program_factory("(list 9 1 8 4 3 2 7 6 5)")),
        interpret_program_snippet("(list 1 2 3 4 5 6 7 8 9)".to_string())
    );
    assert_eq!(
        interpret_program(list_sort_program_factory("(list 9 1 8 4 3 2 7 6 5)")),
        interpret_program_snippet("(list 1 2 3 4 5 6 7 8 9)".to_string())
    );
    assert_eq!(
        interpret_program(list_sort_program_factory(
            "(list 9 9 17 3 15 20 13 3 14 5 20 17 13 13 17 9 20 3 2 19)"
        )),
        interpret_program_snippet("(list 2 3 3 3 5 9 9 9 13 13 13 14 15 17 17 17 19 20 20 20)".to_string())
    );
}
