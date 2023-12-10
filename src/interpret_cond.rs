
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

pub fn interpret_cond_expr(expr: &Cond) -> Value {
    for case in &expr.cases {
        let condition_result = interpret_bool_expr(&case.condition);
        if condition_result {
            return interpret(&case.result);
        }
    }

    panic!("No case in cond expression evaluated to true");
}