use crate::interpret::{Value, Expr};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<Expr>,
}
pub fn interpret_function_expr(function: &Function) -> Value {
    Value::Num(0)
}
