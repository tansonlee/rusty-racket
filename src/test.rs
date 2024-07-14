use std::collections::HashMap;

use itertools::peek_nth;

use crate::{
    interpret::*,
    lexer::{string_to_tokens, TokenIter},
    parser::{parse, parse_num_expr},
};

fn empty_variable_map() -> VariableMap {
    HashMap::new()
}

fn empty_function_map() -> FunctionMap {
    HashMap::new()
}

#[test]
fn number_literal() {
    assert_eq!(interpret_program_snippet("1010101".to_string()), Value::Num(1010101));
    assert_eq!(interpret_program_snippet("0".to_string()), Value::Num(0));
    assert_eq!(interpret_program_snippet("0010".to_string()), Value::Num(10));
    assert_eq!(interpret_program_snippet("-10".to_string()), Value::Num(-10));
    assert_eq!(interpret_program_snippet("-0010".to_string()), Value::Num(-10));
    assert_eq!(interpret_program_snippet("-0".to_string()), Value::Num(0));
}

#[test]
fn boolean_literal() {
    assert_eq!(interpret_program_snippet("false".to_string()), Value::Bool(false));
    assert_eq!(interpret_program_snippet("true".to_string()), Value::Bool(true));
}

#[test]
fn interpret_number_expr() {
    assert_eq!(interpret_program_snippet("(+ 1 2)".to_string(),), Value::Num(3));
    assert_eq!(interpret_program_snippet("(+ (- 5 1) 2)".to_string(),), Value::Num(6));
    assert_eq!(interpret_program_snippet("(+ (- 5 1) (/ 10 5))".to_string(),), Value::Num(6));
    assert_eq!(
        interpret_program_snippet("(+ 1 (+ 1 (+ 1 (+ 1 (+ 1 (+ 1 0))))))".to_string(),),
        Value::Num(6)
    );
    assert_eq!(interpret_program_snippet("(% 11 10)".to_string(),), Value::Num(1));
}

#[test]
fn interpret_binary_boolean_expr() {
    assert_eq!(interpret_program_snippet("(& true false)".to_string(),), Value::Bool(false));
    assert_eq!(
        interpret_program_snippet("(& (| false true) (| false true))".to_string()),
        Value::Bool(true)
    );
}

#[test]
fn interpret_unary_boolean_expr() {
    assert_eq!(interpret_program_snippet("(! false)".to_string()), Value::Bool(true));
    assert_eq!(interpret_program_snippet("(! true)".to_string()), Value::Bool(false));
}

#[test]
fn interpret_cmp_boolean_expr() {
    assert_eq!(
        interpret_program_snippet("(= (* 5 5) (/ 100 4))".to_string()),
        Value::Bool(true)
    );
    assert_eq!(interpret_program_snippet("(< 3 10)".to_string()), Value::Bool(true));
    assert_eq!(interpret_program_snippet("(> 3 10)".to_string()), Value::Bool(false));
    assert_eq!(
        interpret_program_snippet("(| (< 5 10) (< 10 5))".to_string()),
        Value::Bool(true)
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
    interpret_program_snippet(cond.to_string());
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

    let program3 = "
    (define (add a b) (+ a b))
    (define (sub a b) (- a b))
    (define (main) 
        (cond 
            ((< (sub 6 1) 6) (add 5 10))
            ((= 1 1) (+ 50 100))))
    ";
    assert_eq!(interpret_program(program3.to_string()), Value::Num(15));

    let tokens = string_to_tokens("(a 1)".to_string());
    let mut token_iterator = peek_nth(TokenIter::new(&tokens));
    parse_num_expr(&mut token_iterator);

    let program4 = "
    (define (identity a) a)
    (define (main) (+ (identity 5) (identity 5)))
    ";
    assert_eq!(interpret_program(program4.to_string()), Value::Num(10));
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

#[test]
fn interpret_list() {
    assert_eq!(
        interpret_program_snippet("(list 1)".to_string()),
        Value::List(ValueList::Node(ValueNode {
            data: Box::new(Value::Num(1)),
            next: Box::new(ValueList::Empty),
        }))
    );

    assert_eq!(
        interpret_program_snippet("(list 1 2)".to_string()),
        Value::List(ValueList::Node(ValueNode {
            data: Box::new(Value::Num(1)),
            next: Box::new(ValueList::Node(ValueNode {
                data: Box::new(Value::Num(2)),
                next: Box::new(ValueList::Empty),
            })),
        }))
    );

    assert_eq!(interpret_program_snippet("empty".to_string()), Value::List(ValueList::Empty));

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
    assert_eq!(interpret_program_snippet("(empty? empty)".to_string()), Value::Bool(true));
    assert_eq!(interpret_program_snippet("(empty? (list 1))".to_string()), Value::Bool(false));

    assert_eq!(
        interpret_program_snippet("(empty? (cdr (list 1)))".to_string()),
        Value::Bool(true)
    );

    assert_eq!(
        interpret_program_snippet("(empty? (cdr (list 1 2)))".to_string()),
        Value::Bool(false)
    );
}

#[test]
fn interpret_list_huh() {
    assert_eq!(interpret_program_snippet("(list? empty)".to_string()), Value::Bool(true));
    assert_eq!(interpret_program_snippet("(list? (list 1))".to_string()), Value::Bool(true));

    assert_eq!(
        interpret_program_snippet("(list? (cdr (list 1)))".to_string()),
        Value::Bool(true)
    );

    assert_eq!(
        interpret_program_snippet("(list? (cdr (list 1 2)))".to_string()),
        Value::Bool(true)
    );

    assert_eq!(interpret_program_snippet("(list? 5)".to_string()), Value::Bool(false));

    assert_eq!(interpret_program_snippet("(list? true)".to_string()), Value::Bool(false));
    assert_eq!(interpret_program_snippet("(list? true)".to_string()), Value::Bool(false));
}

#[test]
fn test_median() {
    let program = "
    (define (median a b c)
        (cond
            [(| (& (< a b) (< b c)) (& (< c b) (< b a))) b]
            [(| (& (< b a) (< a c)) (& (< c a) (< a b))) a]
            [(| (& (< a c) (< c b)) (& (< b c) (< c a))) c]))
    (define (main) (median 3 1 2))
    ";
    assert_eq!(interpret_program(program.to_string()), Value::Num(2));
}

#[test]
fn test_factorial() {
    fn factorial_program_factory(n: i32) -> String {
        format!(
            "
        (define (factorial n)
            (cond
                [(= n 1) 1]
                [true (* n (factorial (- n 1)))]))

        (define (main) (factorial {}))
        ",
            n
        )
    }

    assert_eq!(interpret_program(factorial_program_factory(1)), Value::Num(1));
    assert_eq!(interpret_program(factorial_program_factory(2)), Value::Num(2));
    assert_eq!(interpret_program(factorial_program_factory(10)), Value::Num(3628800));
}

#[test]
fn test_fibonacci() {
    fn fibonacci_program_factory(n: i32) -> String {
        format!(
            "
        (define (fibonacci n)
            (cond
                [(= n 0) 0]
                [(= n 1) 1]
                [true (+ (fibonacci (- n 1)) (fibonacci (- n 2)))]))

        (define (main) (fibonacci {}))
        ",
            n
        )
    }

    assert_eq!(interpret_program(fibonacci_program_factory(1)), Value::Num(1));
    assert_eq!(interpret_program(fibonacci_program_factory(2)), Value::Num(1));
    assert_eq!(interpret_program(fibonacci_program_factory(5)), Value::Num(5));
    assert_eq!(interpret_program(fibonacci_program_factory(10)), Value::Num(55));
}

#[test]
fn test_list_length() {
    fn list_length_program_factory(s: &str) -> String {
        format!(
            "
        (define (length lst)
            (cond
                [(empty? lst) 0]
                [true (+ 1 (length (cdr lst)))]))
        
        (define (main) (length {}))
        ",
            s
        )
    }

    assert_eq!(interpret_program(list_length_program_factory("empty")), Value::Num(0));
    assert_eq!(interpret_program(list_length_program_factory("(list 1)")), Value::Num(1));
    assert_eq!(
        interpret_program(list_length_program_factory("(list 1 2 3 4 5)")),
        Value::Num(5)
    )
}

#[test]
fn test_list_sum() {
    fn list_length_program_factory(s: &str) -> String {
        format!(
            "
        (define (list-sum lst)
            (cond
                [(empty? lst) 0]
                [true (+ (car lst) (list-sum (cdr lst)))]))
        
        (define (main) (list-sum {}))
        ",
            s
        )
    }

    assert_eq!(interpret_program(list_length_program_factory("empty")), Value::Num(0));
    assert_eq!(interpret_program(list_length_program_factory("(list 1)")), Value::Num(1));
    assert_eq!(
        interpret_program(list_length_program_factory("(list 1 2 3 4 5)")),
        Value::Num(15)
    )
}

#[test]
fn test_list_sum_acc() {
    fn list_length_program_factory(s: &str) -> String {
        format!(
            "
        (define (list-sum-helper lst acc)
            (cond
                [(empty? lst) acc]
                [true (list-sum-helper (cdr lst) (+ acc (car lst)))]))
        
        (define (list-sum lst) (list-sum-helper lst 0))
        
        (define (main) (list-sum {}))
        ",
            s
        )
    }

    assert_eq!(interpret_program(list_length_program_factory("empty")), Value::Num(0));
    assert_eq!(interpret_program(list_length_program_factory("(list 1)")), Value::Num(1));
    assert_eq!(
        interpret_program(list_length_program_factory("(list 1 2 3 4 5)")),
        Value::Num(15)
    )
}

#[test]
fn test_list_double() {
    fn list_double_program_factory(s: &str) -> String {
        format!(
            "
        (define (list-double lst)
            (cond
                [(empty? lst) empty]
                [true (cons (* 2 (car lst)) (list-double (cdr lst)))]))
        
        (define (main) (list-double {}))
        ",
            s
        )
    }

    assert_eq!(
        interpret_program(list_double_program_factory("empty")),
        interpret_program_snippet("empty".to_string())
    );

    assert_eq!(
        interpret_program(list_double_program_factory("(list 1)")),
        interpret_program_snippet("(list 2)".to_string())
    );
    assert_eq!(
        interpret_program(list_double_program_factory("(list 1 2 3 4 5)")),
        interpret_program_snippet("(list 2 4 6 8 10)".to_string())
    );
}

#[test]
fn test_list_reverse() {
    fn list_reverse_program_factory(s: &str) -> String {
        format!(
            "
        (define (list-reverse-helper lst acc)
            (cond
                [(empty? lst) acc]
                [true (list-reverse-helper (cdr lst) (cons (car lst) acc))]))

        (define (list-reverse lst) (list-reverse-helper lst empty))

        (define (main) (list-reverse {}))
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
    fn list_filter_even_program_factory(s: &str) -> String {
        format!(
            "
        (define (list-filter-even lst)
            (cond
                [(empty? lst) empty]
                [true (cond
                        [(= (% (car lst) 2) 0) (list-filter-even (cdr lst))]
                        [true (cons (car lst) (list-filter-even (cdr lst)))])])) 
        
        (define (main) (list-filter-even {}))
            ",
            s
        )
    }

    assert_eq!(
        interpret_program(list_filter_even_program_factory("empty")),
        interpret_program_snippet("empty".to_string())
    );
    assert_eq!(
        interpret_program(list_filter_even_program_factory("(list 1 3 5)")),
        interpret_program_snippet("(list 1 3 5)".to_string())
    );
    assert_eq!(
        interpret_program(list_filter_even_program_factory("(list 2 4 6)")),
        interpret_program_snippet("empty".to_string())
    );
    assert_eq!(
        interpret_program(list_filter_even_program_factory("(list 1 2 3 4 5 6 7 8)")),
        interpret_program_snippet("(list 1 3 5 7)".to_string())
    );
}

#[test]
fn test_list_contains() {
    fn list_contains_program_factory(list: &str, val: &str) -> String {
        format!(
            "
        (define (list-contains lst val)
            (cond
                [(empty? lst) false]
                [true (cond
                        [(= val (car lst)) true]
                        [true (list-contains (cdr lst))])])) 
        
        (define (main) (list-contains {} {}))
            ",
            list, val
        )
    }

    assert_eq!(
        interpret_program(list_contains_program_factory("empty", "5")),
        Value::Bool(false)
    );
    assert_eq!(
        interpret_program(list_contains_program_factory("(list 1 3 5)", "10")),
        Value::Bool(false)
    );
    assert_eq!(
        interpret_program(list_contains_program_factory("(list 1 3 5)", "1")),
        Value::Bool(true)
    );
}

#[test]
fn test_list_append() {
    fn list_append_program_factory(list1: &str, list2: &str) -> String {
        format!(
            "
        (define (list-append lst1 lst2)
            (cond
                [(empty? lst1) lst2]
                [true (cons (car lst1) (list-append (cdr lst1) lst2))])) 

        (define (main) (list-append {} {}))
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
    fn list_flatten_program_factory(list: &str) -> String {
        format!(
            "
        (define (list-append lst1 lst2)
            (cond
                [(empty? lst1) lst2]
                [true (cons (car lst1) (list-append (cdr lst1) lst2))])) 

        (define (list-flatten lst)
            (cond
                [(empty? lst) empty]
                [(list? (car lst)) (list-append (list-flatten (car lst)) (list-flatten (cdr lst)))]
                [true (cons (car lst) (list-flatten (cdr lst)))])) 
        
        (define (main) (list-flatten {}))
            ",
            list
        )
    }

    assert_eq!(
        interpret_program(list_flatten_program_factory("empty")),
        interpret_program_snippet("empty".to_string())
    );

    assert_eq!(
        interpret_program(list_flatten_program_factory("(list 1 2 3)")),
        interpret_program_snippet("(list 1 2 3)".to_string())
    );

    assert_eq!(
        interpret_program(list_flatten_program_factory("(list (list 1 2 3))")),
        interpret_program_snippet("(list 1 2 3)".to_string())
    );

    assert_eq!(
        interpret_program(list_flatten_program_factory("(list (list 1 2 (list 3 (list 4 5) 6 7)) 8 9)")),
        interpret_program_snippet("(list 1 2 3 4 5 6 7 8 9)".to_string())
    );
}
