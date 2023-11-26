use itertools::peek_nth;
use itertools::PeekNth;

use crate::interpret_bool::*;
use crate::interpret_num::*;
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
        TokenKind::Number => Expr::NumExpr(Num::Literal(tokens.next().unwrap().text.parse::<i32>().unwrap())),
        TokenKind::Boolean => Expr::BoolExpr(Bool::Literal(tokens.next().unwrap().text.parse::<bool>().unwrap())),
        TokenKind::OpenParen => match tokens.peek_nth(1).unwrap().kind {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Star => Expr::NumExpr(parse_num_expr(tokens)),
            TokenKind::Ampersand | TokenKind::Pipe | TokenKind::Bang | TokenKind::LessThan | TokenKind::Equal | TokenKind::GreaterThan => {
                Expr::BoolExpr(parse_bool_expr(tokens))
            }
            _ => panic!("Invalid expression starting with an open parenthesis '('"),
        },
        _ => panic!("Malformed expression, expression begins with an illegal character."),
    }
}

fn parse_num_expr(tokens: &mut PeekNth<TokenIter<'_>>) -> Num {
    match tokens.peek().unwrap().kind {
        TokenKind::Number => Num::Literal(tokens.next().unwrap().text.parse::<N>().unwrap()),
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
