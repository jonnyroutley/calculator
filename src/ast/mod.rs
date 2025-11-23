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
    ast.calculate()
}

pub fn main(inputs: Vec<String>) {
    let mut functions: HashMap<String, FunctionExpr> = HashMap::new();
    for input in inputs {
        let input = input.replace(" ", "");
        let _ = match utils::input::is_function_assignment(&input) {
            true => {
                let function = parse::parse_function_assignment(input).unwrap();
                functions.insert(function.name.clone(), function.clone());
                println!("Function inserted: {}", function.name.clone());
            }
            false => match utils::input::is_function_call(&input) {
                true => {
                    let function_name = input.split("(").nth(0).unwrap();
                    let function = functions.get(function_name).unwrap();

                    let mut arguments = HashMap::<String, &str>::new();
                    input
                        .split("(")
                        .nth(1)
                        .unwrap()
                        .split(")")
                        .nth(0)
                        .unwrap()
                        .split(",")
                        .enumerate()
                        .for_each(|(index, token)| {
                            arguments.insert(
                                function
                                    .arguments
                                    .iter()
                                    .find(|x| x.position == index)
                                    .unwrap()
                                    .name
                                    .clone(),
                                token,
                            );
                        });
                    // here we have 1,2 at positions 0,1 => lookup names from positions and then put them into map
                    // .enumerate().for_each(|idx. arg_name| arguments_map.insert(k, v));
                    // .collect::<Vec<&str>>();

                    // foo(a,b) => foo(1,2) => replace_placeholders({a:1, b:2})

                    // get the function template, replace all placeholders with actual arguments, then call calculate
                    let node = function.template.replace_placeholders(&arguments).unwrap();
                    let result = node.calculate();
                    println!("Result: {:?}", result);
                }
                false => {
                    let result = calculate(input);
                    println!("Result: {:?}", result);
                }
            },
        };
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
        main(inputs)
    }

    #[test]
    fn test_function_assignment_double_variable() {
        let inputs = vec!["fn foo (a) { a * a }".to_string(), "foo(2)".to_string()];
        main(inputs)
    }

    proptest! {
        #[test]
        fn doesnt_crash_random_strings(s in "\\PC*") {
            let _ = calculate(s);
        }
    }
}
