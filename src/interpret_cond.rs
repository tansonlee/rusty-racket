use crate::interpret::*;
use crate::interpret_bool::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CondExpr {
    pub cases: Vec<CondCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CondCase {
    pub condition: BoolExpr,
    pub result: Expr,
}

pub fn interpret_cond_expr(expr: &CondExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> Value {
    for case in &expr.cases {
        let condition_result = interpret_bool_expr(&case.condition, variable_map, function_map);
        // Return on the first condition that is true.
        if condition_result {
            // Evaluate the body.
            return interpret(&case.result, variable_map, function_map);
        }
    }

    panic!("No case in cond expression evaluated to true");
}
