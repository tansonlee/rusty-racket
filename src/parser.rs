use itertools::peek_nth;
use itertools::PeekNth;

use crate::interpret::*;
use crate::interpret_bool::*;
use crate::interpret_cond::*;
use crate::interpret_function::FunctionExpr;
use crate::interpret_function_call::FunctionCallExpr;
use crate::interpret_list::CarExpr;
use crate::interpret_list::CdrExpr;
use crate::interpret_list::ListLiteralExpr;
use crate::interpret_list::{ListExpr, Node};
use crate::interpret_num::*;
use crate::interpret_variable::*;
use crate::lexer::*;

pub fn parse(program: String) -> Expr {
    let tokens = string_to_tokens(program);
    let mut token_iterator = peek_nth(TokenIter::new(&tokens));

    let parsed = parse_expr(&mut token_iterator);

    if token_iterator.peek().is_some() {
        panic!(
            "Malformed program, more tokens after program completion {}",
            token_iterator
                .map(|token| format!("{}", token))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    parsed
}

pub fn parse_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Expr {
    match tokens.peek().unwrap().kind {
        TokenKind::Identifier => Expr::VariableExpr(parse_variable_expr(tokens)),
        TokenKind::Minus => Expr::NumExpr(parse_num_expr(tokens)),
        TokenKind::Number => Expr::NumExpr(NumExpr::LiteralNumExpr(tokens.next().unwrap().text.parse::<i32>().unwrap())),
        TokenKind::Boolean => Expr::BoolExpr(BoolExpr::LiteralBoolExpr(
            tokens.next().unwrap().text.parse::<bool>().unwrap(),
        )),
        TokenKind::Empty => {
            tokens.next();
            Expr::EmptyExpr(ListExpr::ListLiteralExpr(ListLiteralExpr::Empty))
        }
        TokenKind::OpenParen => match tokens.peek_nth(1).unwrap().kind {
            TokenKind::Define => Expr::FunctionExpr(parse_function_expr(tokens)),
            TokenKind::Cond => Expr::CondExpr(parse_cond_expr(tokens)),
            TokenKind::Car => Expr::CarExpr(parse_car_expr(tokens)),
            TokenKind::Identifier => Expr::FunctionCallExpr(parse_function_call(tokens)),
            TokenKind::List | TokenKind::Cons | TokenKind::Cdr => Expr::ListExpr(parse_list_expr(tokens)),
            TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Star | TokenKind::Percent => {
                Expr::NumExpr(parse_num_expr(tokens))
            }
            TokenKind::Ampersand
            | TokenKind::Pipe
            | TokenKind::Bang
            | TokenKind::LessThan
            | TokenKind::Equal
            | TokenKind::GreaterThan
            | TokenKind::EmptyHuh
            | TokenKind::ListHuh => Expr::BoolExpr(parse_bool_expr(tokens)),
            _ => panic!(
                "Invalid expression starting with an open parenthesis '(': {}",
                tokens.peek_nth(1).unwrap()
            ),
        },
        _ => panic!(
            "Malformed expression, expression begins with an illegal character {}.",
            tokens.peek().unwrap()
        ),
    }
}

pub fn parse_num_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> NumExpr {
    match tokens.peek().unwrap().kind {
        TokenKind::Number => NumExpr::LiteralNumExpr(tokens.next().unwrap().text.parse::<N>().unwrap()),
        TokenKind::Identifier => NumExpr::VariableExpr(VariableExpr {
            name: tokens.next().unwrap().text.parse::<String>().unwrap(),
        }),
        // Do nth(1) because the negative sign needs to be consumed.
        TokenKind::Minus => NumExpr::LiteralNumExpr(-tokens.nth(1).unwrap().text.parse::<N>().unwrap()),
        _ => match tokens.peek_nth(1).unwrap().kind {
            TokenKind::Identifier => NumExpr::FunctionCallExpr(parse_function_call(tokens)),
            TokenKind::Car => NumExpr::CarExpr(parse_car_expr(tokens)),
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
                consume_open_paren(tokens);
                let op_token = tokens.next().unwrap();
                let left = parse_num_expr(tokens);
                let right = parse_num_expr(tokens);
                consume_close_paren(tokens);

                NumExpr::BinaryNumExpr(Box::new(BinaryNumExpr {
                    op: token_kind_to_binary_num_op(&op_token.kind),
                    left,
                    right,
                }))
            }
            _ => panic!(
                "Invalid num expr {:?}",
                tokens.map(|token| format!("{}", token)).collect::<Vec<_>>().join(", ")
            ),
        },
    }
}

fn parse_bool_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> BoolExpr {
    match tokens.peek().unwrap().kind {
        TokenKind::Boolean => BoolExpr::LiteralBoolExpr(tokens.next().unwrap().text.parse::<B>().unwrap()),
        TokenKind::Identifier => BoolExpr::VariableExpr(VariableExpr {
            name: tokens.next().unwrap().text.parse::<String>().unwrap(),
        }),
        TokenKind::OpenParen => match tokens.peek_nth(1).unwrap().kind {
            TokenKind::Ampersand | TokenKind::Pipe => parse_binary_bool_expr(tokens),
            TokenKind::Bang => parse_unary_bool_expr(tokens),
            TokenKind::LessThan | TokenKind::Equal | TokenKind::GreaterThan => parse_cmp_bool_expr(tokens),
            TokenKind::Identifier => BoolExpr::FunctionCallExpr(parse_function_call(tokens)),
            TokenKind::EmptyHuh => BoolExpr::EmptyHuhExpr(parse_empty_huh_expr(tokens)),
            TokenKind::ListHuh => BoolExpr::ListHuhExpr(parse_list_huh_expr(tokens)),
            TokenKind::Car => BoolExpr::CarExpr(parse_car_expr(tokens)),
            _ => panic!(
                "Invalid expression starting with an open parenthesis '(': {}",
                tokens.peek_nth(1).unwrap()
            ),
        },
        _ => panic!("Invalid start token to bool expr {}", tokens.peek().unwrap()),
    }
}

fn parse_binary_bool_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> BoolExpr {
    consume_open_paren(tokens);
    let op = token_kind_to_binary_bool_op(&tokens.next().unwrap().kind);
    let left = parse_bool_expr(tokens);
    let right = parse_bool_expr(tokens);
    consume_close_paren(tokens);

    BoolExpr::BinaryBoolExpr(Box::new(BinaryBoolExpr { op, left, right }))
}

fn parse_unary_bool_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> BoolExpr {
    consume_open_paren(tokens);
    let op = token_kind_to_unary_bool_op(&tokens.next().unwrap().kind);
    let value = parse_bool_expr(tokens);
    consume_close_paren(tokens);

    BoolExpr::UnaryBoolExpr(Box::new(UnaryBoolExpr { op, value }))
}

fn parse_cmp_bool_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> BoolExpr {
    consume_open_paren(tokens);
    let op = token_kind_to_cmp_bool_op(&tokens.next().unwrap().kind);
    let left = parse_num_expr(tokens);
    let right = parse_num_expr(tokens);
    consume_close_paren(tokens);

    BoolExpr::CmpExpr(Box::new(CmpBoolExpr { op, left, right }))
}

// (cond (case 1) (case 2))
fn parse_cond_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> CondExpr {
    consume_open_paren(tokens);
    tokens.next(); // 'cond'

    // Does not contain a case.
    if tokens.peek().unwrap().kind == TokenKind::CloseParen {
        panic!("No case in cond.");
    }

    let mut cases: Vec<CondCase> = Vec::new();
    while tokens.peek().unwrap().kind != TokenKind::CloseParen {
        cases.push(parse_cond_case(tokens));
    }

    consume_close_paren(tokens);

    CondExpr { cases }
}

fn parse_cond_case(tokens: &mut PeekNth<TokenIter<'_>>) -> CondCase {
    consume_open_paren(tokens);
    let condition = parse_bool_expr(tokens);
    let result = parse_expr(tokens);
    consume_close_paren(tokens);

    CondCase { condition, result }
}

fn parse_variable_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> VariableExpr {
    VariableExpr {
        name: tokens.next().unwrap().text.to_string(),
    }
}

/*
(define (add a b) (+ a b))
*/
fn parse_function_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> FunctionExpr {
    consume_open_paren(tokens);
    tokens.next(); // 'define'
    consume_open_paren(tokens);
    let function_name = &tokens.next().unwrap().text;

    let mut function_parameters = Vec::new();
    while tokens.peek().unwrap().kind == TokenKind::Identifier {
        function_parameters.push(tokens.next().unwrap().text.to_string());
    }

    consume_close_paren(tokens);

    let function_body = parse_expr(tokens);

    consume_close_paren(tokens);

    FunctionExpr {
        name: function_name.to_string(),
        parameters: function_parameters,
        body: Box::new(function_body),
    }
}

// (add 1 2) or (main)
fn parse_function_call(tokens: &mut PeekNth<TokenIter<'_>>) -> FunctionCallExpr {
    consume_open_paren(tokens);

    let name = &tokens.next().unwrap().text;

    let mut arguments: Vec<Expr> = Vec::new();
    while tokens.peek().unwrap().kind != TokenKind::CloseParen {
        arguments.push(parse_expr(tokens));
    }

    tokens.next();

    FunctionCallExpr {
        name: name.to_string(),
        arguments,
    }
}

fn parse_list_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> ListExpr {
    match tokens.peek().unwrap().kind {
        TokenKind::Identifier => ListExpr::VariableExpr(VariableExpr {
            name: tokens.next().unwrap().text.parse::<String>().unwrap(),
        }),
        TokenKind::Empty => {
            tokens.next();
            ListExpr::ListLiteralExpr(ListLiteralExpr::Empty)
        }
        TokenKind::OpenParen => match tokens.peek_nth(1).unwrap().kind {
            TokenKind::Identifier => ListExpr::FunctionCallExpr(parse_function_call(tokens)),
            TokenKind::List => ListExpr::ListLiteralExpr(parse_list_literal_expr(tokens)),
            TokenKind::Cons => ListExpr::ListLiteralExpr(parse_cons_expr(tokens)),
            TokenKind::Car => ListExpr::CarExpr(parse_car_expr(tokens)),
            TokenKind::Cdr => ListExpr::CdrExpr(parse_cdr_expr(tokens)),
            _ => panic!(
                "Invalid expression starting with an open parenthesis '(': {}",
                tokens.peek_nth(1).unwrap()
            ),
        },
        _ => panic!("Invalid start token to bool expr {}", tokens.peek().unwrap()),
    }
}

fn parse_list_literal_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> ListLiteralExpr {
    consume_open_paren(tokens);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::List);

    let mut list_items = Vec::new();

    while tokens.peek().unwrap().kind != TokenKind::CloseParen {
        list_items.push(parse_expr(tokens));
    }

    consume_close_paren(tokens);

    // Turn vec of list items to a `List`
    let mut result = ListLiteralExpr::Empty;

    for item in list_items.into_iter().rev() {
        result = ListLiteralExpr::Node(Node {
            data: Box::new(item),
            next: Box::new(ListExpr::ListLiteralExpr(result)),
        });
    }

    result
}

// (cons 1 (cons 2 empty)) or (cons 1 empty)
fn parse_cons_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> ListLiteralExpr {
    let first_token = tokens.next().unwrap();

    if let TokenKind::Empty = first_token.kind {
        ListLiteralExpr::Empty
    } else {
        let next = tokens.next().unwrap();
        assert_eq!(next.kind, TokenKind::Cons, "{:#?}", next);

        let first = parse_expr(tokens);
        let rest = parse_list_expr(tokens);

        consume_close_paren(tokens);

        ListLiteralExpr::Node(Node {
            data: Box::new(first),
            next: Box::new(rest),
        })
    }
}

// (car <some list>)
fn parse_car_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> CarExpr {
    consume_open_paren(tokens);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::Car);

    let list = parse_expr(tokens);

    consume_close_paren(tokens);

    match list {
        Expr::ListExpr(x) | Expr::EmptyExpr(x) => CarExpr { list: Box::new(x) },
        Expr::VariableExpr(x) => CarExpr {
            list: Box::new(ListExpr::VariableExpr(x)),
        },
        _ => panic!("Malformed car expr: argument is not a list"),
    }
}

// (cdr <some list>)
fn parse_cdr_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> CdrExpr {
    consume_open_paren(tokens);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::Cdr);

    let list = parse_list_expr(tokens);

    consume_close_paren(tokens);

    CdrExpr { list: Box::new(list) }
}

// (empty? <some list>)
fn parse_empty_huh_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> EmptyHuhExpr {
    consume_open_paren(tokens);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::EmptyHuh);

    let list = parse_expr(tokens);

    consume_close_paren(tokens);

    match list {
        Expr::ListExpr(x) | Expr::EmptyExpr(x) => EmptyHuhExpr { list: Box::new(x) },
        Expr::VariableExpr(x) => EmptyHuhExpr {
            list: Box::new(ListExpr::VariableExpr(x)),
        },
        x => panic!("Malformed empty? expr: argument is not a list: {:#?}", x),
    }
}

// (list? expr)
fn parse_list_huh_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> ListHuhExpr {
    consume_open_paren(tokens);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::ListHuh);

    let expr = parse_expr(tokens);

    consume_close_paren(tokens);

    ListHuhExpr { expr: Box::new(expr) }
}

fn consume_open_paren(tokens: &mut PeekNth<TokenIter<'_>>) {
    let next_token = tokens.next().unwrap();
    assert_eq!(
        next_token.kind,
        TokenKind::OpenParen,
        "Open paren not found. Got {} instead. Rest of tokens are: {}",
        next_token.text,
        tokens.map(|token| format!("{}", token)).collect::<Vec<_>>().join(", ")
    );
}

fn consume_close_paren(tokens: &mut PeekNth<TokenIter<'_>>) {
    let next_token = tokens.next().unwrap();
    assert_eq!(
        next_token.kind,
        TokenKind::CloseParen,
        "Close paren not found. Got {} instead. Rest of tokens are: {}",
        next_token.text,
        tokens.map(|token| format!("{}", token)).collect::<Vec<_>>().join(", ")
    );
}
