use std::collections::HashMap;

use crate::{ast::ast::FunctionExpr, utils};

pub mod ast;
pub mod parse;

pub fn calculate(input: String) -> Result<f64, String> {
    let infix_input = match utils::input::get_normalized_input(&input) {
        Ok(input) => input,
        Err(e) => return Err(e),
    };
    let ast = match parse::infix_to_ast(infix_input) {
        Ok(ast) => ast,
        Err(e) => return Err(e),
    };
    // FIXME: this sucks?
    let empty_args = HashMap::<String, &str>::new();
    let replaced = ast.replace_placeholders(&empty_args)?;
    replaced.calculate()
}

pub fn handle_input(input: String, functions: &mut HashMap<String, FunctionExpr>) {
    let _ = match utils::input::is_function_assignment(&input) {
        true => match parse::parse_function_assignment(input) {
            Ok(function) => {
                functions.insert(function.name.clone(), function.clone());
                println!("Function inserted: {}", function.name.clone());
            }
            Err(e) => {
                println!("Error parsing function assignment: {}", e);
            }
        },
        false => match utils::input::is_function_call(&input) {
            true => {
                if let Some(function_name) = input.split("(").nth(0) {
                    if let Some(function) = functions.get(function_name) {
                        let mut arguments = HashMap::<String, &str>::new();
                        let args_str = input
                            .split("(")
                            .nth(1)
                            .and_then(|s| s.split(")").nth(0))
                            .unwrap_or("");

                        args_str.split(",").enumerate().for_each(|(index, token)| {
                            if let Some(arg_def) =
                                function.arguments.iter().find(|x| x.position == index)
                            {
                                arguments.insert(arg_def.name.clone(), token);
                            }
                        });

                        match function.template.replace_placeholders(&arguments) {
                            Ok(node) => {
                                let result = node.calculate();
                                println!("Result: {:?}", result);
                            }
                            Err(e) => {
                                println!("Error replacing placeholders: {}", e);
                            }
                        }
                    } else {
                        println!("Error: Function '{}' not found", function_name);
                    }
                } else {
                    println!("Error: Invalid function call syntax: missing opening parenthesis");
                }
            }
            false => {
                let result = calculate(input);
                match result {
                    Ok(result) => {
                        println!("Result: {:?}", result);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        },
    };
}

pub fn main(inputs: Option<Vec<String>>) {
    let mut functions: HashMap<String, FunctionExpr> = HashMap::new();

    match inputs {
        Some(inputs) => {
            for input in inputs {
                handle_input(input.replace(" ", ""), &mut functions);
            }
        }
        None => loop {
            match utils::input::get_input() {
                Ok(input) => {
                    if !input.is_empty() {
                        handle_input(input.replace(" ", ""), &mut functions);
                    }
                }
                Err(_) => break,
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proptest::prelude::*;

    #[test]
    fn test_wikipedia_example() {
        let input = String::from("3 + 4 * 2 ÷ ( 1 - 5 ) ^ 2 ^ 3");
        let expected =
            3.0_f64 + 4.0_f64 * 2.0_f64 / (1.0_f64 - 5.0_f64).powf(2.0_f64.powf(3.0_f64));
        assert_eq!(calculate(input), Ok(expected));
    }
    #[test]
    fn test_another_example() {
        let input = String::from("4+5-2*5");
        let expected = 4.0_f64 + 5.0_f64 - 2.0_f64 * 5.0_f64;
        assert_eq!(calculate(input), Ok(expected));
    }

    #[test]
    fn test_function_assignment() {
        let inputs = vec![
            "fn foo(a, b) { a + b }".to_string(),
            "foo(1, 2)".to_string(),
        ];
        main(Some(inputs))
    }

    #[test]
    fn test_function_assignment_double_variable() {
        let inputs = vec!["fn foo (a) { a * a }".to_string(), "foo(2)".to_string()];
        main(Some(inputs))
    }

    proptest! {
        #[test]
        fn doesnt_crash_random_strings(s in "\\PC*") {
            let _ = calculate(s);
        }
    }
}
