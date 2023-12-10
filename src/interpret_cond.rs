use crate::interpret::*;
use crate::interpret_bool::*;

#[derive(Debug)]
pub struct Cond {
    pub cases: Vec<CondCase>,
}

#[derive(Debug)]
pub struct CondCase {
    pub condition: Bool,
    pub result: Expr,
}

pub fn interpret_cond_expr(expr: &Cond, env: &mut Environment) -> Value {
    for case in &expr.cases {
        let condition_result = interpret_bool_expr(&case.condition, env);
        if condition_result {
            return interpret(&case.result, env);
        }
    }

    panic!("No case in cond expression evaluated to true");
}
