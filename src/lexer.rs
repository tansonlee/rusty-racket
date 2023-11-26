use crate::interpret_bool::*;
use crate::interpret_num::*;

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Number,
    Boolean,

    // Keywords

    // Special characters
    OpenParen,
    CloseParen,

    // Operators
    Plus,
    Minus,
    Slash,
    Star,

    Ampersand,
    Pipe,
    Bang,

    LessThan,
    Equal,
    GreaterThan,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

#[derive(Debug)]
pub struct TokenIter<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
}

impl<'a> TokenIter<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        TokenIter { tokens, index: 0 }
    }

    pub fn peek_n(&self, n: i32) -> Option<(&Token, &Token)> {
        if self.index + (n as usize) < self.tokens.len() {
            Some((&self.tokens[self.index], &self.tokens[self.index + n as usize]))
        } else {
            None
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            let token = &self.tokens[self.index];
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }
}

pub fn token_kind_to_binary_num_op(kind: &TokenKind) -> BinaryNumOp {
    match kind {
        TokenKind::Plus => BinaryNumOp::Add,
        TokenKind::Minus => BinaryNumOp::Sub,
        TokenKind::Slash => BinaryNumOp::Div,
        TokenKind::Star => BinaryNumOp::Mul,
        _ => panic!("Could not parse token kind to binary num op"),
    }
}

pub fn token_kind_to_binary_bool_op(kind: &TokenKind) -> BinaryBoolOp {
    match kind {
        TokenKind::Ampersand => BinaryBoolOp::And,
        TokenKind::Pipe => BinaryBoolOp::Or,
        _ => {
            println!("{:?}", kind);
            panic!("Could not parse token kind to binary bool op");
        }
    }
}

pub fn token_kind_to_unary_bool_op(kind: &TokenKind) -> UnaryBoolOp {
    match kind {
        TokenKind::Bang => UnaryBoolOp::Not,
        _ => panic!("Could not parse token kind to unary bool op"),
    }
}

pub fn token_kind_to_cmp_bool_op(kind: &TokenKind) -> CmpBoolOp {
    match kind {
        TokenKind::LessThan => CmpBoolOp::Lt,
        TokenKind::Equal => CmpBoolOp::Eq,
        TokenKind::GreaterThan => CmpBoolOp::Gt,
        _ => panic!("Could not parse token kind to cmp bool op"),
    }
}

fn token_from_position(s: &mut std::iter::Peekable<std::str::Chars>) -> Token {
    if s.peek().is_none() {
        panic!("Index out of range while parsing tokens.")
    }

    if s.peek().unwrap().is_numeric() {
        let mut number_buff = String::new();
        while s.peek().is_some() && s.peek().unwrap().is_numeric() {
            number_buff.push(s.next().unwrap());
        }

        return Token {
            kind: TokenKind::Number,
            text: number_buff,
        };
    }

    if s.peek().unwrap().is_ascii_alphabetic() {
        let mut buff = String::new();
        while s.peek().is_some() && s.peek().unwrap().is_ascii_alphabetic() {
            buff.push(s.next().unwrap());
        }

        if buff == "true" || buff == "false" {
            return Token {
                kind: TokenKind::Boolean,
                text: buff,
            };
        }

        println!("{}", buff);
        panic!("Unknown keyword");
    }

    match s.next().unwrap() {
        '(' => Token {
            kind: TokenKind::OpenParen,
            text: "(".to_string(),
        },
        ')' => Token {
            kind: TokenKind::CloseParen,
            text: ")".to_string(),
        },
        '+' => Token {
            kind: TokenKind::Plus,
            text: "+".to_string(),
        },
        '-' => Token {
            kind: TokenKind::Minus,
            text: "-".to_string(),
        },
        '/' => Token {
            kind: TokenKind::Slash,
            text: "/".to_string(),
        },
        '*' => Token {
            kind: TokenKind::Star,
            text: "*".to_string(),
        },
        '&' => Token {
            kind: TokenKind::Ampersand,
            text: "&".to_string(),
        },
        '|' => Token {
            kind: TokenKind::Pipe,
            text: "|".to_string(),
        },
        '!' => Token {
            kind: TokenKind::Bang,
            text: "!".to_string(),
        },
        '<' => Token {
            kind: TokenKind::LessThan,
            text: "<".to_string(),
        },
        '=' => Token {
            kind: TokenKind::Equal,
            text: "=".to_string(),
        },
        '>' => Token {
            kind: TokenKind::GreaterThan,
            text: ">".to_string(),
        },
        _ => todo!("Unhandled case in token_from_position"),
    }
}

pub fn string_to_tokens(s: String) -> Vec<Token> {
    let mut result = Vec::new();
    let mut s_iterator = s.chars().into_iter().peekable();

    while s_iterator.peek().is_some() {
        while s_iterator.peek().is_some_and(|x| x.is_whitespace()) {
            s_iterator.next();
        }
        let token = token_from_position(&mut s_iterator);
        result.push(token);
    }

    result
}
