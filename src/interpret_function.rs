use crate::interpret::{Expr, FunctionInfo, FunctionMap, Value, VariableMap};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<Expr>,
}

pub fn interpret_function_expr(function: &Function, variable_map: &VariableMap, function_map: &mut FunctionMap) -> Value {
    function_map.insert(
        function.name.clone(),
        FunctionInfo {
            parameter_names: function.parameters.clone(),
            body: *function.body,
        },
    );

    Value::Num(0)
}
