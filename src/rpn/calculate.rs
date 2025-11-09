pub fn perform_calculations(input: Vec<String>) -> Result<f64, String> {
    if input.is_empty() {
        return Err("Input is empty".to_string());
    }

    let mut stack: Vec<f64> = vec![];
    for token in input {
        match token.as_str() {
            "+" | "-" | "/" | "^" | "*" => {
                if stack.len() < 2 {
                    return Err(format!("Not enough values on stack: {}", stack.len()));
                }

                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                let result = match token.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    "^" => left.powf(right),
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => {
                let value = token
                    .parse()
                    .map_err(|e| format!("Invalid number: {}", e))
                    .unwrap();
                stack.push(value);
            }
        }
    }
    if stack.len() != 1 {
        return Err(format!("Invalid expression: {} values remain", stack.len()));
    }
    Ok(stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic_addition() {
        assert_eq!(
            perform_calculations(vec!["2".to_string(), "3".to_string(), "+".to_string()]),
            Ok(5.0)
        );
    }

    #[test]
    fn test_basic_subtraction() {
        assert_eq!(
            perform_calculations(vec!["5".to_string(), "3".to_string(), "-".to_string()]),
            Ok(2.0)
        );
    }

    #[test]
    fn test_basic_multiplication() {
        assert_eq!(
            perform_calculations(vec!["4".to_string(), "6".to_string(), "*".to_string()]),
            Ok(24.0)
        );
    }

    #[test]
    fn test_basic_division() {
        assert_eq!(
            perform_calculations(vec!["15".to_string(), "3".to_string(), "/".to_string()]),
            Ok(5.0)
        );
    }

    #[test]
    fn test_chained_operations() {
        assert_eq!(
            perform_calculations(vec![
                "4".to_string(),
                "2".to_string(),
                "3".to_string(),
                "+".to_string(),
                "*".to_string(),
            ]),
            Ok(20.0)
        );
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(
            perform_calculations(vec![
                "1".to_string(),
                "1".to_string(),
                "+".to_string(),
                "7".to_string(),
                "/".to_string(),
                "15".to_string(),
                "-".to_string(),
            ]),
            Ok(-14.714285714285714)
        );
    }

    #[test]
    fn test_single_number() {
        assert_eq!(perform_calculations(vec!["42".to_string()]), Ok(42.0));
    }

    #[test]
    fn test_negative_result() {
        assert_eq!(
            perform_calculations(vec!["0".to_string(), "5".to_string(), "-".to_string()]),
            Ok(-5.0)
        );
    }

    #[test]
    fn test_foo() {
        assert_eq!(
            perform_calculations(vec![
                "4".to_string(),
                "1".to_string(),
                "5".to_string(),
                "-".to_string(),
                "+".to_string(),
            ]),
            Ok(0.0)
        );
    }
}
