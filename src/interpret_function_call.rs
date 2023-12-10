use crate::interpret::{Value, Expr};

pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}

pub fn interpret_function_call(function_call: FunctionCall) -> Value {
    Value::Num(0)
}
