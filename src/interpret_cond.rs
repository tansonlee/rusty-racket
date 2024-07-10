use crate::interpret::*;
use crate::interpret_bool::*;

#[derive(Debug, Clone)]
pub struct Cond {
    pub cases: Vec<CondCase>,
}

#[derive(Debug, Clone)]
pub struct CondCase {
    pub condition: Bool,
    pub result: Expr,
}

pub fn interpret_cond_expr(expr: &Cond, variable_map: &mut VariableMap, function_map: &FunctionMap) -> Value {
    for case in &expr.cases {
        let condition_result = interpret_bool_expr(&case.condition, variable_map, function_map);
        if condition_result {
            return interpret(&case.result, variable_map, function_map);
        }
    }

    panic!("No case in cond expression evaluated to true");
}
