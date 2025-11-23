use crate::{
    ast::ast::{AbstractNode, ArgumentDefinition, FunctionExpr},
    utils::{
        input::get_normalized_input,
        operators::{Associativity, get_operator_info},
    },
};

pub fn parse_function_assignment(input: String) -> Result<FunctionExpr, String> {
    //remove whitespace
    let input = input.replace(" ", "");
    let arg_names = input
        .split("(")
        .nth(1)
        .ok_or_else(|| "Invalid function syntax: missing opening parenthesis".to_string())?
        .split(")")
        .nth(0)
        .ok_or_else(|| "Invalid function syntax: missing closing parenthesis".to_string())?
        .split(",")
        .collect::<Vec<&str>>();

    let function_name = input
        .split("fn")
        .nth(1)
        .ok_or_else(|| "Invalid function syntax: missing 'fn' keyword".to_string())?
        .split("(")
        .nth(0)
        .ok_or_else(|| {
            "Invalid function syntax: missing opening parenthesis after 'fn'".to_string()
        })?
        .to_string();

    let function_body = input
        .split("{")
        .nth(1)
        .ok_or_else(|| "Invalid function syntax: missing opening brace".to_string())?
        .split("}")
        .nth(0)
        .ok_or_else(|| "Invalid function syntax: missing closing brace".to_string())?
        .to_string();

    // TODO: deal with placeholder nodes
    let normalized = get_normalized_input(function_body.as_str())
        .map_err(|e| format!("Failed to normalize function body: {}", e))?;
    let body = infix_to_ast(normalized)?;

    Ok(FunctionExpr {
        name: function_name,
        arguments: arg_names
            .iter()
            .enumerate()
            .map(|(position, name)| ArgumentDefinition {
                name: name.to_string(),
                position,
            })
            .collect(),
        template: Box::new(body),
    })
}

pub fn infix_to_ast(input: Vec<String>) -> Result<AbstractNode, String> {
    let mut output: Vec<AbstractNode> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    for token in input.into_iter() {
        if let Ok(value) = token.parse::<f64>() {
            output.push(AbstractNode::Operand { value });
            continue;
        }

        if token.chars().all(|c| c.is_ascii_alphabetic()) {
            output.push(AbstractNode::Placeholder { arg_name: token });
            continue;
        }

        match token.as_str() {
            "+" | "-" | "/" | "^" | "*" => {
                loop {
                    let o2 = operators.last();
                    if let Some(o2) = o2 {
                        if o2 == "(" {
                            break;
                        }

                        let o1_config = get_operator_info(&token)
                            .ok_or_else(|| format!("Unknown operator: {}", token))?;
                        let o2_config = get_operator_info(o2)
                            .ok_or_else(|| format!("Unknown operator: {}", o2))?;
                        if o2_config.precedence > o1_config.precedence
                            || (o2_config.precedence == o1_config.precedence
                                && o1_config.associativity == Associativity::Left)
                        {
                            let operator_str = operators.pop().ok_or_else(|| {
                                "Internal error: operator stack is empty".to_string()
                            })?;
                            let operation = get_operator_info(operator_str.as_str())
                                .ok_or_else(|| format!("{} is not a valid operator", operator_str))?
                                .operation;
                            let right = match output.pop() {
                                Some(right) => right,
                                None => return Err("Not enough values on stack".to_string()),
                            };
                            let left = match output.pop() {
                                Some(left) => left,
                                None => return Err("Not enough values on stack".to_string()),
                            };
                            output.push(AbstractNode::BinaryExpr {
                                operation,
                                lhs: Box::new(left),
                                rhs: Box::new(right),
                            });
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                operators.push(token.clone())
            }

            "(" => operators.push(token.clone()),

            ")" => {
                loop {
                    let o = operators.last();
                    match o {
                        Some(o) => {
                            if o == "(" {
                                break;
                            }

                            let operator_str = operators.pop().ok_or_else(|| {
                                "Internal error: operator stack is empty".to_string()
                            })?;
                            let operation = get_operator_info(operator_str.as_str())
                                .ok_or_else(|| format!("{} is not a valid operator", operator_str))?
                                .operation;
                            let right = match output.pop() {
                                Some(right) => right,
                                None => return Err("Not enough values on stack".to_string()),
                            };
                            let left = match output.pop() {
                                Some(left) => left,
                                None => return Err("Not enough values on stack".to_string()),
                            };
                            output.push(AbstractNode::BinaryExpr {
                                operation,
                                lhs: Box::new(left),
                                rhs: Box::new(right),
                            });
                        }
                        None => return Err("Mismatched parentheses found!".to_string()),
                    }
                }

                let o = operators.pop()
                    .ok_or_else(|| "Internal error: operator stack is empty when expecting '('. Mismatched parentheses found!".to_string())?;
                if o != "(" {
                    return Err("Expected left parenthesis".to_string());
                }
            }
            _ => return Err(format!("Found unsupported token: {}", token)),
        }
    }

    while let Some(o) = operators.pop() {
        match o.as_str() {
            "(" => return Err("Mismatched parentheses found!".to_string()),
            _ => {
                let operation = get_operator_info(o.as_str())
                    .ok_or_else(|| format!("{} is not a valid operator", o))?
                    .operation;
                let right = match output.pop() {
                    Some(right) => right,
                    None => return Err("Not enough values on stack".to_string()),
                };
                let left = match output.pop() {
                    Some(left) => left,
                    None => return Err("Not enough values on stack".to_string()),
                };
                output.push(AbstractNode::BinaryExpr {
                    operation,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                });
            }
        }
    }
    if output.len() != 1 {
        return Err(format!(
            "Invalid expression: {} values remain",
            output.len()
        ));
    }
    Ok(output
        .pop()
        .ok_or_else(|| "Internal error: output stack is empty".to_string())?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::operators::BinaryOperator;
    use crate::utils::test::tokens;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic_infix_conversion_addition() {
        let input = tokens(&["4", "+", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(AbstractNode::BinaryExpr {
                operation: BinaryOperator::Addition,
                lhs: Box::new(AbstractNode::Operand { value: 4.0 }),
                rhs: Box::new(AbstractNode::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_basic_infix_conversion_subtraction() {
        let input = tokens(&["4", "-", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(AbstractNode::BinaryExpr {
                operation: BinaryOperator::Subtraction,
                lhs: Box::new(AbstractNode::Operand { value: 4.0 }),
                rhs: Box::new(AbstractNode::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_basic_infix_conversion_multiplication() {
        let input = tokens(&["4", "*", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(AbstractNode::BinaryExpr {
                operation: BinaryOperator::Multiplication,
                lhs: Box::new(AbstractNode::Operand { value: 4.0 }),
                rhs: Box::new(AbstractNode::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_basic_infix_conversion_division() {
        let input = tokens(&["4", "/", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(AbstractNode::BinaryExpr {
                operation: BinaryOperator::Division,
                lhs: Box::new(AbstractNode::Operand { value: 4.0 }),
                rhs: Box::new(AbstractNode::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_basic_infix_conversion_index() {
        let input = tokens(&["4", "^", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(AbstractNode::BinaryExpr {
                operation: BinaryOperator::Index,
                lhs: Box::new(AbstractNode::Operand { value: 4.0 }),
                rhs: Box::new(AbstractNode::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_foo() {
        let input = tokens(&["4", "+", "(", "1", "-", "5", ")"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(AbstractNode::BinaryExpr {
                operation: BinaryOperator::Addition,
                lhs: Box::new(AbstractNode::Operand { value: 4.0 }),
                rhs: Box::new(AbstractNode::BinaryExpr {
                    operation: BinaryOperator::Subtraction,
                    lhs: Box::new(AbstractNode::Operand { value: 1.0 }),
                    rhs: Box::new(AbstractNode::Operand { value: 5.0 }),
                }),
            })
        );
    }
    #[test]
    fn test_another_example() {
        let input = tokens(&["4", "+", "5", "-", "2", "*", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(AbstractNode::BinaryExpr {
                operation: BinaryOperator::Subtraction,
                lhs: Box::new(AbstractNode::BinaryExpr {
                    operation: BinaryOperator::Addition,
                    lhs: Box::new(AbstractNode::Operand { value: 4.0 }),
                    rhs: Box::new(AbstractNode::Operand { value: 5.0 })
                }),
                rhs: Box::new(AbstractNode::BinaryExpr {
                    operation: BinaryOperator::Multiplication,
                    lhs: Box::new(AbstractNode::Operand { value: 2.0 }),
                    rhs: Box::new(AbstractNode::Operand { value: 5.0 }),
                }),
            })
        );
    }

    #[test]
    fn test_function_assignment() {
        let input = String::from("fn foo(a, b) { a + b }");
        assert_eq!(
            parse_function_assignment(input),
            Ok(FunctionExpr {
                name: String::from("foo"),
                arguments: vec![
                    ArgumentDefinition {
                        position: 0,
                        name: "a".to_string(),
                    },
                    ArgumentDefinition {
                        position: 1,
                        name: "b".to_string(),
                    }
                ],
                template: Box::new(AbstractNode::BinaryExpr {
                    operation: BinaryOperator::Addition,
                    lhs: Box::new(AbstractNode::Placeholder {
                        arg_name: "a".to_string(),
                    }),
                    rhs: Box::new(AbstractNode::Placeholder {
                        arg_name: "b".to_string(),
                    }),
                }),
            })
        );
    }
    // #[test]
    // fn test_placeholder_assignment() {
    //     let input = tokens(&[
    //         "fn", "foo", "(", "a", ",", "b", ")", "{", "a", "+", "b", "}",
    //     ]);
    //     assert_eq!(
    //         infix_to_ast(input),
    //         Ok(AbstractNode::FunctionExpr {
    //             name: "foo".to_string(),
    //             num_arguments: 2,
    //             template: Box::new(AbstractNode::BinaryExpr {
    //                 operation: BinaryOperator::Addition,
    //                 lhs: Box::new(AbstractNode::Placeholder { position: 0 }),
    //                 rhs: Box::new(AbstractNode::Placeholder { position: 1 }),
    //             }),
    //         })
    //     );
    // }
}
