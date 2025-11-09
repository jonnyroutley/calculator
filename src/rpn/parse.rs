use std::io;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct OperatorInfo {
    precedence: u8,
    associativity: Associativity,
}

fn get_operator(symbol: &str) -> Option<&'static OperatorInfo> {
    match symbol {
        "^" => Some(&OperatorInfo {
            precedence: 4,
            associativity: Associativity::Right,
        }),
        "*" => Some(&OperatorInfo {
            precedence: 3,
            associativity: Associativity::Left,
        }),
        "/" => Some(&OperatorInfo {
            precedence: 3,
            associativity: Associativity::Left,
        }),
        "+" => Some(&OperatorInfo {
            precedence: 2,
            associativity: Associativity::Left,
        }),
        "-" => Some(&OperatorInfo {
            precedence: 2,
            associativity: Associativity::Left,
        }),
        _ => None,
    }
}

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn get_normalized_input(input: &str) -> Result<Vec<String>, String> {
    let normalized = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .replace('÷', "/");

    let mut parts = Vec::new();
    // to hold a number that is split across multiple chars
    let mut accumulated = String::new();

    for ch in normalized.chars() {
        match ch {
            '+' | '-' | '/' | '*' | '^' | '(' | ')' => {
                if !accumulated.is_empty() {
                    parts.push(accumulated);
                    accumulated = String::new();
                }

                // + and - are also unary operators
                // if we find one that has come after an operator, then push in an extra 0 so we can pretend it is binary
                if ch == '+' || ch == '-' {
                    let last_part = parts.last();
                    if last_part.is_none()
                        || last_part.is_some_and(|x: &String| x.parse::<f64>().is_err())
                    {
                        parts.push("0".to_string());
                    }
                }

                parts.push(ch.to_string());
            }
            _ => {
                if ch.is_ascii_digit() || ch == '.' {
                    accumulated.push(ch)
                } else {
                    return Err(format!("Found unsupported token: {}", ch));
                }
            }
        }
    }

    if !accumulated.is_empty() {
        parts.push(accumulated);
    }

    Ok(parts)
}

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

                        let o1_config = get_operator(&token).unwrap();
                        let o2_config = get_operator(o2).unwrap();
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic_infix_conversion_addition() {
        let input = vec!["4".to_string(), "+".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            Ok(vec!["4".to_string(), "5".to_string(), "+".to_string()])
        )
    }

    #[test]
    fn test_basic_infix_conversion_subtraction() {
        let input = vec!["4".to_string(), "-".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            Ok(vec!["4".to_string(), "5".to_string(), "-".to_string()])
        )
    }

    #[test]
    fn test_basic_infix_conversion_multiplication() {
        let input = vec!["4".to_string(), "*".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            Ok(vec!["4".to_string(), "5".to_string(), "*".to_string()])
        )
    }

    #[test]
    fn test_basic_infix_conversion_division() {
        let input = vec!["4".to_string(), "/".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            Ok(vec!["4".to_string(), "5".to_string(), "/".to_string()])
        )
    }

    #[test]
    fn test_basic_infix_conversion_index() {
        let input = vec!["4".to_string(), "^".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            Ok(vec!["4".to_string(), "5".to_string(), "^".to_string()])
        )
    }

    #[test]
    fn test_wikipedia_example() {
        let input = vec![
            "3".to_string(),
            "+".to_string(),
            "4".to_string(),
            "*".to_string(),
            "2".to_string(),
            "/".to_string(),
            "(".to_string(),
            "1".to_string(),
            "-".to_string(),
            "5".to_string(),
            ")".to_string(),
            "^".to_string(),
            "2".to_string(),
            "^".to_string(),
            "3".to_string(),
        ];

        assert_eq!(
            infix_to_postfix(input),
            Ok(vec![
                "3".to_string(),
                "4".to_string(),
                "2".to_string(),
                "*".to_string(),
                "1".to_string(),
                "5".to_string(),
                "-".to_string(),
                "2".to_string(),
                "3".to_string(),
                "^".to_string(),
                "^".to_string(),
                "/".to_string(),
                "+".to_string()
            ])
        )
    }
    #[test]
    fn test_foo() {
        let input = vec![
            "4".to_string(),
            "+".to_string(),
            "(".to_string(),
            "1".to_string(),
            "-".to_string(),
            "5".to_string(),
            ")".to_string(),
        ];
        assert_eq!(
            infix_to_postfix(input),
            Ok(vec![
                "4".to_string(),
                "1".to_string(),
                "5".to_string(),
                "-".to_string(),
                "+".to_string(),
            ])
        )
    }
    #[test]
    fn test_another_example() {
        let input = vec![
            "4".to_string(),
            "+".to_string(),
            "5".to_string(),
            "-".to_string(),
            "2".to_string(),
            "*".to_string(),
            "5".to_string(),
        ];
        assert_eq!(
            infix_to_postfix(input),
            Ok(vec![
                "4".to_string(),
                "5".to_string(),
                "+".to_string(),
                "2".to_string(),
                "5".to_string(),
                "*".to_string(),
                "-".to_string()
            ])
        )
    }

    #[test]
    fn test_unary_operators() {
        let result = get_normalized_input("4+-5");
        let expected: Vec<String> = vec!["4", "+", "0", "-", "5"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(result, Ok(expected))
    }
}
