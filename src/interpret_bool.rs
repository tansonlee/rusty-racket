use crate::interpret::{FunctionMap, Value, ValueList, VariableMap, B};
use crate::interpret_function_call::{interpret_function_call, FunctionCall};
use crate::interpret_list::{interpret_car_expr, interpret_list_expr, Car, List};
use crate::interpret_num::*;
use crate::interpret_variable::{interpret_variable_expr, Variable};

#[derive(Debug, Clone, PartialEq)]
pub enum Bool {
    Literal(B),
    Variable(Variable),
    Binary(Box<BinaryBoolExpr>),
    Unary(Box<UnaryBoolExpr>),
    Cmp(Box<CmpBoolExpr>),
    FunctionCall(FunctionCall),
    EmptyHuh(EmptyHuhExpr),
    Car(Car),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryBoolOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryBoolOp {
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CmpBoolOp {
    Lt,
    Eq,
    Gt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryBoolExpr {
    pub op: BinaryBoolOp,
    pub left: Bool,
    pub right: Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryBoolExpr {
    pub op: UnaryBoolOp,
    pub value: Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CmpBoolExpr {
    pub op: CmpBoolOp,
    pub left: Num,
    pub right: Num,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyHuhExpr {
    pub list: Box<List>,
}

pub fn interpret_bool_expr(expr: &Bool, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    match expr {
        Bool::Literal(x) => *x,
        Bool::Variable(x) => interpret_variable_bool_expr(&x, variable_map),
        Bool::Binary(x) => interpret_binary_bool_expr(&x, variable_map, function_map),
        Bool::Unary(x) => interpret_unary_bool_expr(&x, variable_map, function_map),
        Bool::Cmp(x) => interpret_cmp_bool_expr(&x, variable_map, function_map),
        Bool::EmptyHuh(x) => interpret_empty_huh_expr(x, variable_map, function_map),
        Bool::FunctionCall(x) => {
            if let Value::Bool(x) = interpret_function_call(&x, variable_map, function_map) {
                x
            } else {
                panic!(
                    "Expected call to function {} to return boolean but returned something else instead",
                    x.name
                );
            }
        }
        Bool::Car(x) => {
            if let Value::Bool(y) = interpret_car_expr(x, variable_map, function_map) {
                y
            } else {
                panic!()
            }
        }
    }
}

fn interpret_variable_bool_expr(expr: &Variable, variable_map: &VariableMap) -> B {
    let res = interpret_variable_expr(expr, variable_map);

    match res {
        Value::Bool(x) => x,
        x => panic!("Variable {} does not refer to a bool value.", x),
    }
}

fn interpret_binary_bool_expr(expr: &BinaryBoolExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    match expr.op {
        BinaryBoolOp::And => {
            let left = interpret_bool_expr(&expr.left, variable_map, function_map);
            // Short circuit.
            if left == false {
                return false;
            }
            let right = interpret_bool_expr(&expr.right, variable_map, function_map);
            left && right
        }
        BinaryBoolOp::Or => {
            let left = interpret_bool_expr(&expr.left, variable_map, function_map);
            // Short circuit.
            if left == true {
                return true;
            }
            let right = interpret_bool_expr(&expr.right, variable_map, function_map);
            left || right
        }
    }
}

fn interpret_unary_bool_expr(expr: &UnaryBoolExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    match expr.op {
        UnaryBoolOp::Not => !interpret_bool_expr(&expr.value, variable_map, function_map),
    }
}

fn interpret_cmp_bool_expr(expr: &CmpBoolExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    let left = interpret_num_expr(&expr.left, variable_map, function_map);
    let right = interpret_num_expr(&expr.right, variable_map, function_map);

    match expr.op {
        CmpBoolOp::Lt => left < right,
        CmpBoolOp::Eq => left == right,
        CmpBoolOp::Gt => left > right,
    }
}

pub fn interpret_empty_huh_expr(list: &EmptyHuhExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    let res = interpret_list_expr(&list.list, variable_map, function_map);

    match res {
        ValueList::Empty => true,
        ValueList::Node(_) => false,
    }
}
