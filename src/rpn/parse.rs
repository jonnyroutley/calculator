use crate::utils::operators::{Associativity, get_operator_info};

pub fn infix_to_postfix(input: Vec<String>) -> Result<Vec<String>, String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    for token in input.into_iter() {
        if token.parse::<f64>().is_ok() {
            output.push(token);
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
                            output.push(operators.pop().unwrap());
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

                            output.push(operators.pop().unwrap())
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
            _ => output.push(o),
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::tokens;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic_infix_conversion_addition() {
        let input = tokens(&["4", "+", "5"]);
        assert_eq!(infix_to_postfix(input), Ok(tokens(&["4", "5", "+"])))
    }

    #[test]
    fn test_basic_infix_conversion_subtraction() {
        let input = tokens(&["4", "-", "5"]);
        assert_eq!(infix_to_postfix(input), Ok(tokens(&["4", "5", "-"])))
    }

    #[test]
    fn test_basic_infix_conversion_multiplication() {
        let input = tokens(&["4", "*", "5"]);
        assert_eq!(infix_to_postfix(input), Ok(tokens(&["4", "5", "*"])))
    }

    #[test]
    fn test_basic_infix_conversion_division() {
        let input = tokens(&["4", "/", "5"]);
        assert_eq!(infix_to_postfix(input), Ok(tokens(&["4", "5", "/"])))
    }

    #[test]
    fn test_basic_infix_conversion_index() {
        let input = tokens(&["4", "^", "5"]);
        assert_eq!(infix_to_postfix(input), Ok(tokens(&["4", "5", "^"])))
    }

    #[test]
    fn test_wikipedia_example() {
        let input = tokens(&[
            "3", "+", "4", "*", "2", "/", "(", "1", "-", "5", ")", "^", "2", "^", "3",
        ]);

        assert_eq!(
            infix_to_postfix(input),
            Ok(tokens(&[
                "3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+"
            ]))
        )
    }
    #[test]
    fn test_foo() {
        let input = tokens(&["4", "+", "(", "1", "-", "5", ")"]);
        assert_eq!(
            infix_to_postfix(input),
            Ok(tokens(&["4", "1", "5", "-", "+"]))
        )
    }
    #[test]
    fn test_another_example() {
        let input = tokens(&["4", "+", "5", "-", "2", "*", "5"]);
        assert_eq!(
            infix_to_postfix(input),
            Ok(tokens(&["4", "5", "+", "2", "5", "*", "-"]))
        )
    }
}
