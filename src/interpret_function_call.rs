use crate::{
    interpret::{interpret, Environment, Expr, Value},
    interpret_function::{interpret_function_expr, Function},
};

pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}

pub fn interpret_function_call(function_call: &FunctionCall, env: &mut Environment) -> Value {
    let argument_values: Vec<_> = function_call.arguments.iter().map(|expr| interpret(expr, env)).collect();

    // Add the vars to the environment.
    for i in 0..argument_values.len() {
        let arg = argument_values.get(i).unwrap();
        let name = &env.functions[&function_call.name].parameter_names[i];

        env.variable_map
            .entry(name.to_string())
            .or_insert(Vec::new())
            .insert(0, arg.clone());
    }

    let result = interpret_function_expr(
        &Function {
            name: function_call.name.clone(),
        },
        env,
    );

    // Remove the vars to the environment.
    for i in 0..argument_values.len() {
        let arg = argument_values.get(i).unwrap();
        let name = &env.functions[&function_call.name].parameter_names[i];

        env.variable_map.get_mut(name).expect("Expected variable not found").pop();
    }

    result
}
