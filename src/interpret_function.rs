use crate::interpret::{Expr, FunctionInfo, FunctionMap};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<Expr>,
}

pub fn interpret_function_expr(function: &Function, function_map: &mut FunctionMap) {
    function_map.insert(
        function.name.clone(),
        FunctionInfo {
            parameter_names: function.parameters.clone(),
            body: *function.body.clone(),
        },
    );
}
