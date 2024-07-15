use crate::interpret::{Value, VariableMap};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableExpr {
    pub name: String,
}

pub fn interpret_variable_expr(variable: &VariableExpr, variable_map: &VariableMap) -> Value {
    match variable_map.get(&variable.name) {
        Some(x) => match &x[..] {
            [.., val] => val.clone(),
            _ => panic!("Val not found"),
        },
        None => panic!("Undefined variable"),
    }
}
