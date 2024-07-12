use crate::interpret::{interpret, Expr, FunctionMap, Value, ValueList, ValueNode, VariableMap};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub data: Box<Expr>,
    pub next: Box<List>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum List {
    Empty,
    Node(Node),
}

pub fn interpret_list_expr(list: &List, variable_map: &mut VariableMap, function_map: &FunctionMap) -> Value {
    match list {
        List::Empty => Value::List(ValueList::Empty),
        List::Node(x) => {
            let interpreted_data = interpret(&x.data, variable_map, function_map);
            let interpreted_next = interpret_list_expr(&x.next, variable_map, function_map);

            match interpreted_next {
                Value::List(x) => Value::List(ValueList::Node(ValueNode {
                    data: Box::new(interpreted_data),
                    next: Box::new(x),
                })),
                _ => panic!("Invalid second item to list"),
            }
        }
    }
}
