use itertools::peek_nth;
use itertools::PeekNth;

use crate::interpret::*;
use crate::interpret_bool::*;
use crate::interpret_cond::*;
use crate::interpret_function::Function;
use crate::interpret_num::*;
use crate::interpret_variable::*;
use crate::lexer::*;

pub fn parse(program: String) -> Expr {
    let tokens = string_to_tokens(program);
    let mut token_iterator = peek_nth(TokenIter::new(&tokens));

    let parsed = parse_expr(&mut token_iterator);

    if token_iterator.peek().is_some() {
        panic!("Malformed program, more tokens after program completion");
    }

    parsed
}

fn parse_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Expr {
    match tokens.peek().unwrap().kind {
        TokenKind::Identifier => Expr::VariableExpr(parse_variable_expr(tokens)),
        TokenKind::Minus => Expr::NumExpr(parse_num_expr(tokens)),
        TokenKind::Number => Expr::NumExpr(Num::Literal(tokens.next().unwrap().text.parse::<i32>().unwrap())),
        TokenKind::Boolean => Expr::BoolExpr(Bool::Literal(tokens.next().unwrap().text.parse::<bool>().unwrap())),
        TokenKind::OpenParen => match tokens.peek_nth(1).unwrap().kind {
            TokenKind::Define => Expr::FunctionExpr(parse_function_expr(tokens)),
            TokenKind::Cond => Expr::CondExpr(parse_cond_expr(tokens)),
            TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Star => Expr::NumExpr(parse_num_expr(tokens)),
            TokenKind::Ampersand
            | TokenKind::Pipe
            | TokenKind::Bang
            | TokenKind::LessThan
            | TokenKind::Equal
            | TokenKind::GreaterThan => Expr::BoolExpr(parse_bool_expr(tokens)),
            _ => panic!("Invalid expression starting with an open parenthesis '('"),
        },
        _ => panic!("Malformed expression, expression begins with an illegal character."),
    }
}

fn parse_num_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Num {
    match tokens.peek().unwrap().kind {
        TokenKind::Number => Num::Literal(tokens.next().unwrap().text.parse::<N>().unwrap()),
        TokenKind::Identifier => Num::Variable(tokens.next().unwrap().text.parse::<V>().unwrap()),
        // Do nth(1) because the negative sign needs to be consumed.
        TokenKind::Minus => Num::Literal(-tokens.nth(1).unwrap().text.parse::<N>().unwrap()),
        _ => {
            tokens.next(); // Consume open paren
            let op = token_kind_to_binary_num_op(&tokens.next().unwrap().kind);
            let left = parse_num_expr(tokens);
            let right = parse_num_expr(tokens);
            tokens.next();

            Num::Binary(Box::new(BinaryNumExpr { op, left, right }))
        }
    }
}

fn parse_bool_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Bool {
    match tokens.peek().unwrap().kind {
        TokenKind::Boolean => Bool::Literal(tokens.next().unwrap().text.parse::<B>().unwrap()),
        TokenKind::Identifier => Bool::Variable(tokens.next().unwrap().text.parse::<V>().unwrap()),
        _ => match tokens.peek_nth(1).unwrap().kind {
            TokenKind::Ampersand | TokenKind::Pipe => parse_binary_bool_expr(tokens),
            TokenKind::Bang => parse_unary_bool_expr(tokens),
            TokenKind::LessThan | TokenKind::Equal | TokenKind::GreaterThan => parse_cmp_bool_expr(tokens),
            _ => panic!("Invalid expression starting with an open parenthesis '('"),
        },
    }
}

fn parse_binary_bool_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Bool {
    tokens.next();
    let op = token_kind_to_binary_bool_op(&tokens.next().unwrap().kind);
    let left = parse_bool_expr(tokens);
    let right = parse_bool_expr(tokens);
    tokens.next();

    Bool::Binary(Box::new(BinaryBoolExpr { op, left, right }))
}

fn parse_unary_bool_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Bool {
    tokens.next();
    let op = token_kind_to_unary_bool_op(&tokens.next().unwrap().kind);
    let value = parse_bool_expr(tokens);
    tokens.next();

    Bool::Unary(Box::new(UnaryBoolExpr { op, value }))
}

fn parse_cmp_bool_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Bool {
    tokens.next();
    let op = token_kind_to_cmp_bool_op(&tokens.next().unwrap().kind);
    let left = parse_num_expr(tokens);
    let right = parse_num_expr(tokens);
    tokens.next();

    Bool::Cmp(Box::new(CmpBoolExpr { op, left, right }))
}

fn parse_cond_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Cond {
    tokens.next(); // '('
    tokens.next(); // 'cond'

    // Does not contain a case.
    if tokens.peek().unwrap().kind == TokenKind::CloseParen {
        panic!("No case in cond.");
    }

    let mut cases: Vec<CondCase> = Vec::new();
    while tokens.peek().unwrap().kind != TokenKind::CloseParen {
        cases.push(parse_cond_case(tokens));
    }

    tokens.next();

    Cond { cases }
}

fn parse_cond_case(tokens: &mut PeekNth<TokenIter<'_>>) -> CondCase {
    tokens.next();
    let condition = parse_bool_expr(tokens);
    let result = parse_expr(tokens);
    tokens.next();

    CondCase { condition, result }
}

fn parse_variable_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Variable {
    Variable {
        name: tokens.next().unwrap().text.to_string(),
    }
}

/*
(define (add a b) (+ a b))
*/
fn parse_function_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Function {
    println!("PARSE FUNCTION");
    tokens.next(); // '('
    tokens.next(); // 'define'
    tokens.next(); // '('
    let function_name = &tokens.next().unwrap().text;

    println!("fn name is {}", function_name);

    let mut function_parameters = Vec::new();
    while tokens.peek().unwrap().kind == TokenKind::Identifier {
        function_parameters.push(tokens.next().unwrap().text.to_string());
    }
    println!("params are {:?}", function_parameters);

    tokens.next(); // ')'
    println!("token rest is {:?}", tokens);

    let function_body = parse_expr(tokens);
    println!("body are {:?}", function_body);
    tokens.next(); // ')'

    Function {
        name: function_name.to_string(),
    }
}
