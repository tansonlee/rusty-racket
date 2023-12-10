use crate::interpret::{Value, Expr};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
}

pub fn interpret_variable_expr(variable: &Variable) -> Value {
    Value::Num(0)
}
