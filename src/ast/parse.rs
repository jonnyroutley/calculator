use crate::{
    ast::ast::{FunctionExpr, Node},
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
        .unwrap()
        .split(")")
        .nth(0)
        .unwrap()
        .split(",")
        .collect::<Vec<&str>>();

    let function_name = input
        .split("fn")
        .nth(1)
        .unwrap()
        .split("(")
        .nth(0)
        .unwrap()
        .to_string();

    let function_body = input
        .split("{")
        .nth(1)
        .unwrap()
        .split("}")
        .nth(0)
        .unwrap()
        .to_string();

    // TODO: deal with placeholder nodes
    let body = infix_to_ast(get_normalized_input(function_body.as_str()).unwrap()).unwrap();

    Ok(FunctionExpr {
        name: function_name,
        num_arguments: arg_names.len(),
        template: Box::new(body),
    })
}

pub fn infix_to_ast(input: Vec<String>) -> Result<Node, String> {
    let mut output: Vec<Node> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    let mut placeholder_count = 0;

    for token in input.into_iter() {
        if token.parse::<f64>().is_ok() {
            output.push(Node::Operand {
                value: token.parse().unwrap(),
            });
            continue;
        }

        if token.chars().all(|c| c.is_ascii_alphabetic()) {
            output.push(Node::Placeholder {
                position: placeholder_count,
            });
            placeholder_count += 1;
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

                        let o1_config = get_operator_info(&token).unwrap();
                        let o2_config = get_operator_info(o2).unwrap();
                        if o2_config.precedence > o1_config.precedence
                            || (o2_config.precedence == o1_config.precedence
                                && o1_config.associativity == Associativity::Left)
                        {
                            let operation =
                                match get_operator_info(operators.pop().unwrap().as_str()) {
                                    Some(info) => info.operation,
                                    None => {
                                        return Err(format!(
                                            "{} is not a valid operator",
                                            operators.pop().unwrap()
                                        ))?;
                                    }
                                };
                            let right = match output.pop() {
                                Some(right) => right,
                                None => return Err("Not enough values on stack".to_string()),
                            };
                            let left = match output.pop() {
                                Some(left) => left,
                                None => return Err("Not enough values on stack".to_string()),
                            };
                            output.push(Node::BinaryExpr {
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

                            let operation =
                                match get_operator_info(operators.pop().unwrap().as_str()) {
                                    Some(info) => info.operation,
                                    None => {
                                        return Err(format!(
                                            "{} is not a valid operator",
                                            operators.pop().unwrap()
                                        ));
                                    }
                                };
                            let right = match output.pop() {
                                Some(right) => right,
                                None => return Err("Not enough values on stack".to_string()),
                            };
                            let left = match output.pop() {
                                Some(left) => left,
                                None => return Err("Not enough values on stack".to_string()),
                            };
                            output.push(Node::BinaryExpr {
                                operation,
                                lhs: Box::new(left),
                                rhs: Box::new(right),
                            });
                        }
                        None => return Err("Mismatched parentheses found!".to_string()),
                    }
                }

                let o = operators.pop().unwrap();
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
                let operation = match get_operator_info(o.as_str()) {
                    Some(info) => info.operation,
                    None => {
                        return Err(format!(
                            "{} is not a valid operator",
                            operators.pop().unwrap()
                        ));
                    }
                };
                let right = match output.pop() {
                    Some(right) => right,
                    None => return Err("Not enough values on stack".to_string()),
                };
                let left = match output.pop() {
                    Some(left) => left,
                    None => return Err("Not enough values on stack".to_string()),
                };
                output.push(Node::BinaryExpr {
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
    Ok(output.pop().unwrap())
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
            Ok(Node::BinaryExpr {
                operation: BinaryOperator::Addition,
                lhs: Box::new(Node::Operand { value: 4.0 }),
                rhs: Box::new(Node::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_basic_infix_conversion_subtraction() {
        let input = tokens(&["4", "-", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(Node::BinaryExpr {
                operation: BinaryOperator::Subtraction,
                lhs: Box::new(Node::Operand { value: 4.0 }),
                rhs: Box::new(Node::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_basic_infix_conversion_multiplication() {
        let input = tokens(&["4", "*", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(Node::BinaryExpr {
                operation: BinaryOperator::Multiplication,
                lhs: Box::new(Node::Operand { value: 4.0 }),
                rhs: Box::new(Node::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_basic_infix_conversion_division() {
        let input = tokens(&["4", "/", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(Node::BinaryExpr {
                operation: BinaryOperator::Division,
                lhs: Box::new(Node::Operand { value: 4.0 }),
                rhs: Box::new(Node::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_basic_infix_conversion_index() {
        let input = tokens(&["4", "^", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(Node::BinaryExpr {
                operation: BinaryOperator::Index,
                lhs: Box::new(Node::Operand { value: 4.0 }),
                rhs: Box::new(Node::Operand { value: 5.0 }),
            })
        )
    }

    #[test]
    fn test_foo() {
        let input = tokens(&["4", "+", "(", "1", "-", "5", ")"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(Node::BinaryExpr {
                operation: BinaryOperator::Addition,
                lhs: Box::new(Node::Operand { value: 4.0 }),
                rhs: Box::new(Node::BinaryExpr {
                    operation: BinaryOperator::Subtraction,
                    lhs: Box::new(Node::Operand { value: 1.0 }),
                    rhs: Box::new(Node::Operand { value: 5.0 }),
                }),
            })
        );
    }
    #[test]
    fn test_another_example() {
        let input = tokens(&["4", "+", "5", "-", "2", "*", "5"]);
        assert_eq!(
            infix_to_ast(input),
            Ok(Node::BinaryExpr {
                operation: BinaryOperator::Subtraction,
                lhs: Box::new(Node::BinaryExpr {
                    operation: BinaryOperator::Addition,
                    lhs: Box::new(Node::Operand { value: 4.0 }),
                    rhs: Box::new(Node::Operand { value: 5.0 })
                }),
                rhs: Box::new(Node::BinaryExpr {
                    operation: BinaryOperator::Multiplication,
                    lhs: Box::new(Node::Operand { value: 2.0 }),
                    rhs: Box::new(Node::Operand { value: 5.0 }),
                }),
            })
        );
    }
}
