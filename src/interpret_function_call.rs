use crate::interpret::{Environment, Expr, Value};

pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}

pub fn interpret_function_call(function_call: FunctionCall, env: &mut Environment) -> Value {
    Value::Num(0)
}
