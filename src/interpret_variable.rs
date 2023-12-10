use crate::interpret::{Environment, Value};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
}

pub fn interpret_variable_expr(variable: &Variable, env: &mut Environment) -> Value {
    match env.variable_map.get(&variable.name) {
        Some(x) => x.clone(),
        None => panic!("Undefined variable")
    }
}
