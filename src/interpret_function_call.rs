use crate::interpret::{interpret, Expr, FunctionMap, Value, VariableMap};

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}

pub fn interpret_function_call(
    function_call: &FunctionCall,
    variable_map: &mut VariableMap,
    function_map: &FunctionMap,
) -> Value {
    let argument_values: Vec<_> = function_call
        .arguments
        .iter()
        .map(|expr| interpret(expr, variable_map, function_map))
        .collect();

    // Add the vars to the environment.
    for i in 0..argument_values.len() {
        let arg = argument_values.get(i).unwrap();
        let name = &function_map[&function_call.name].parameter_names[i];

        variable_map
            .entry(name.to_string())
            .or_insert(Vec::new())
            .insert(0, arg.clone());
    }

    let function_body = &function_map.get(&function_call.name).clone().unwrap().body;

    let result = interpret(&function_body, variable_map, function_map);

    // Remove the vars to the environment.
    for i in 0..argument_values.len() {
        let name = &function_map[&function_call.name].parameter_names[i];
        variable_map.get_mut(name).expect("Expected variable not found").pop();
    }

    result
}
