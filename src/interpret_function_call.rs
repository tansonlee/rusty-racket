use crate::interpret::{interpret, Expr, FunctionMap, Value, VariableMap};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallExpr {
    pub name: String,
    pub arguments: Vec<Expr>,
}

pub fn interpret_function_call(
    function_call: &FunctionCallExpr,
    variable_map: &mut VariableMap,
    function_map: &FunctionMap,
) -> Value {
    // 1. Interpret all of the arguments into the function.
    let argument_values: Vec<Value> = function_call
        .arguments
        .iter()
        .map(|expr| interpret(expr, variable_map, function_map))
        .collect();

    // 2. Add the parameters to the variable map so the function can access its parameter values.
    let mut i: usize = 0;
    for arg in argument_values {
        let name = &function_map
            .get(&function_call.name)
            .expect(&format!("Undefined function: '{}'", function_call.name))
            .parameter_names[i];
        variable_map.entry(name.to_string()).or_insert(Vec::new()).push(arg);
        i += 1;
    }

    // 3. Get the body of the function.
    let function_body = &function_map
        .get(&function_call.name)
        .expect(&format!("Undefined function '{}'", &function_call.name))
        .body;

    // 4. Execute the function itself and get the result.
    let result = interpret(&function_body, variable_map, function_map);

    // 5. Clean up by removing the parameters from the environment.
    for j in 0..i {
        let name = &function_map[&function_call.name].parameter_names[j];
        variable_map.get_mut(name).expect("Expected variable not found").pop();
    }

    result
}
