use crate::interpret::{Environment, Expr, Value};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<Expr>,
}
pub fn interpret_function_expr(function: &Function, env: &mut Environment) -> Value {
    Value::Num(0)
}
