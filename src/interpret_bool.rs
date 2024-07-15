use crate::interpret::{interpret, Expr, FunctionMap, Value, ValueList, VariableMap, B};
use crate::interpret_function_call::{interpret_function_call, FunctionCallExpr};
use crate::interpret_list::{interpret_car_expr, interpret_list_expr, CarExpr, ListExpr};
use crate::interpret_num::*;
use crate::interpret_variable::{interpret_variable_expr, VariableExpr};

// All possible expression types that can result in a BoolValue.
#[derive(Debug, Clone, PartialEq)]
pub enum BoolExpr {
    LiteralBoolExpr(B),
    VariableExpr(VariableExpr),
    BinaryBoolExpr(Box<BinaryBoolExpr>),
    UnaryBoolExpr(Box<UnaryBoolExpr>),
    CmpExpr(Box<CmpBoolExpr>),
    FunctionCallExpr(FunctionCallExpr),
    EmptyHuhExpr(EmptyHuhExpr),
    ListHuhExpr(ListHuhExpr),
    CarExpr(CarExpr),
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
    pub left: BoolExpr,
    pub right: BoolExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryBoolExpr {
    pub op: UnaryBoolOp,
    pub value: BoolExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CmpBoolExpr {
    pub op: CmpBoolOp,
    pub left: NumExpr,
    pub right: NumExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyHuhExpr {
    pub list: Box<ListExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListHuhExpr {
    pub expr: Box<Expr>,
}

pub fn interpret_bool_expr(expr: &BoolExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    match expr {
        BoolExpr::LiteralBoolExpr(x) => *x,
        BoolExpr::VariableExpr(x) => interpret_variable_bool_expr(&x, variable_map),
        BoolExpr::BinaryBoolExpr(x) => interpret_binary_bool_expr(&x, variable_map, function_map),
        BoolExpr::UnaryBoolExpr(x) => interpret_unary_bool_expr(&x, variable_map, function_map),
        BoolExpr::CmpExpr(x) => interpret_cmp_bool_expr(&x, variable_map, function_map),
        BoolExpr::EmptyHuhExpr(x) => interpret_empty_huh_expr(x, variable_map, function_map),
        BoolExpr::ListHuhExpr(x) => interpret_list_huh_expr(x, variable_map, function_map),
        BoolExpr::FunctionCallExpr(x) => {
            if let Value::BoolValue(x) = interpret_function_call(&x, variable_map, function_map) {
                x
            } else {
                panic!(
                    "Expected call to function {} to return boolean but returned something else instead",
                    x.name
                );
            }
        }
        BoolExpr::CarExpr(x) => {
            if let Value::BoolValue(y) = interpret_car_expr(x, variable_map, function_map) {
                y
            } else {
                panic!()
            }
        }
    }
}

fn interpret_variable_bool_expr(expr: &VariableExpr, variable_map: &VariableMap) -> B {
    let res = interpret_variable_expr(expr, variable_map);

    match res {
        Value::BoolValue(x) => x,
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

pub fn interpret_list_huh_expr(expr: &ListHuhExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> B {
    let res = interpret(&expr.expr, variable_map, function_map);

    match res {
        Value::ListValue(_) => true,
        _ => false,
    }
}
