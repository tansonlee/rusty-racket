use crate::{
    interpret::{interpret, Expr, FunctionMap, Value, ValueList, ValueNode, VariableMap, L},
    interpret_function_call::{interpret_function_call, FunctionCallExpr},
    interpret_variable::{interpret_variable_expr, VariableExpr},
};

// All possible expression types that can result in a ListValue.
#[derive(Debug, Clone, PartialEq)]
pub enum ListExpr {
    ListLiteralExpr(ListLiteralExpr),
    CdrExpr(CdrExpr),
    VariableExpr(VariableExpr),
    CarExpr(CarExpr),
    FunctionCallExpr(FunctionCallExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListLiteralExpr {
    Empty,
    Node(Node),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub data: Box<Expr>,
    pub next: Box<ListExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CarExpr {
    pub list: Box<ListExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CdrExpr {
    pub list: Box<ListExpr>,
}

pub fn interpret_list_expr(list: &ListExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> L {
    match list {
        ListExpr::ListLiteralExpr(x) => interpret_list_literal_expr(x, variable_map, function_map),
        ListExpr::CdrExpr(x) => interpret_cdr_expr(&x, variable_map, function_map),
        ListExpr::VariableExpr(x) => interpret_variable_list_expr(x, variable_map),
        ListExpr::FunctionCallExpr(x) => {
            if let Value::ListValue(y) = interpret_function_call(x, variable_map, function_map) {
                y
            } else {
                panic!(
                    "Expected call to function {} to return list but returned something else instead",
                    x.name
                );
            }
        }
        ListExpr::CarExpr(x) => {
            if let Value::ListValue(y) = interpret_car_expr(x, variable_map, function_map) {
                y
            } else {
                panic!("Malformed car expression {:#?}", x);
            }
        }
    }
}

pub fn interpret_list_literal_expr(list: &ListLiteralExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> L {
    match list {
        ListLiteralExpr::Empty => ValueList::Empty,
        ListLiteralExpr::Node(y) => {
            let interpreted_data = interpret(&y.data, variable_map, function_map);
            let interpreted_next = interpret_list_expr(&y.next, variable_map, function_map);

            ValueList::Node(ValueNode {
                data: Box::new(interpreted_data),
                next: Box::new(interpreted_next),
            })
        }
    }
}

pub fn interpret_car_expr(list: &CarExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> Value {
    let result = interpret_list_expr(&list.list, variable_map, function_map);
    match result {
        ValueList::Empty => panic!("Argument to car cannot be empty"),
        ValueList::Node(result) => *result.data,
    }
}

pub fn interpret_cdr_expr(list: &CdrExpr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> L {
    let res = interpret_list_expr(&list.list, variable_map, function_map);

    match res {
        ValueList::Empty => panic!("Cannot take cdr of empty list"),
        ValueList::Node(x) => *x.next,
    }
}

pub fn interpret_variable_list_expr(expr: &VariableExpr, variable_map: &VariableMap) -> L {
    let res = interpret_variable_expr(expr, variable_map);

    match res {
        Value::ListValue(x) => x,
        x => panic!("Variable {} does not refer to a num value", x),
    }
}
