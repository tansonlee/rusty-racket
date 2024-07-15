use crate::interpret::{Expr, FunctionInfo, FunctionMap};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Box<Expr>,
}

pub fn interpret_function_expr(function: &FunctionExpr, function_map: &mut FunctionMap) {
    function_map.insert(
        function.name.clone(),
        FunctionInfo {
            parameter_names: function.parameters.clone(),
            // Clone here may be slightly expensive however, it is done only once when
            // bringing the expression from the parser into the interpreter.
            body: *function.body.clone(),
        },
    );
}
