use crate::{
    interpret::{interpret, Expr, FunctionMap, Value, ValueList, ValueNode, VariableMap, L},
    interpret_variable::{interpret_variable_expr, Variable},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub data: Box<Expr>,
    pub next: Box<ListLiteral>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum List {
    ListLiteral(ListLiteral),
    Cdr(Cdr),
    Variable(Variable),
    Car(Car),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ListLiteral {
    Empty,
    Node(Node),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Car {
    pub list: Box<List>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cdr {
    pub list: Box<List>,
}

pub fn interpret_list_expr(list: &List, variable_map: &mut VariableMap, function_map: &FunctionMap) -> L {
    match list {
        List::ListLiteral(x) => interpret_list_literal_expr(x, variable_map, function_map),
        List::Cdr(x) => interpret_cdr_expr(&x, variable_map, function_map),
        List::Variable(x) => interpret_variable_list_expr(x, variable_map),
        List::Car(x) => {
            if let Value::List(y) = interpret_car_expr(x, variable_map, function_map) {
                y
            } else {
                panic!("Malformed car expression {:#?}", x);
            }
        }
    }
}

pub fn interpret_list_literal_expr(list: &ListLiteral, variable_map: &mut VariableMap, function_map: &FunctionMap) -> L {
    match list {
        ListLiteral::Empty => ValueList::Empty,
        ListLiteral::Node(y) => {
            let interpreted_data = interpret(&y.data, variable_map, function_map);
            let interpreted_next = interpret_list_literal_expr(&y.next, variable_map, function_map);

            ValueList::Node(ValueNode {
                data: Box::new(interpreted_data),
                next: Box::new(interpreted_next),
            })
        }
    }
}

pub fn interpret_car_expr(list: &Car, variable_map: &mut VariableMap, function_map: &FunctionMap) -> Value {
    let result = interpret_list_expr(&list.list, variable_map, function_map);
    match result {
        ValueList::Empty => panic!("Argument to car cannot be empty"),
        ValueList::Node(result) => *result.data,
    }
}

pub fn interpret_cdr_expr(list: &Cdr, variable_map: &mut VariableMap, function_map: &FunctionMap) -> L {
    let res = interpret_list_expr(&list.list, variable_map, function_map);

    match res {
        ValueList::Empty => panic!("Cannot take cdr of empty list"),
        ValueList::Node(x) => *x.next,
    }
}

pub fn interpret_variable_list_expr(expr: &Variable, variable_map: &VariableMap) -> L {
    let res = interpret_variable_expr(expr, variable_map);

    match res {
        Value::List(x) => x,
        x => panic!("Variable {} does not refer to a num value", x),
    }
}
