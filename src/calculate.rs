enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

struct Calculation {
    left: f64,
    operation: Operation,
    right: f64,
}

fn perform_calculation(calculation: &Calculation) -> f64 {
    match calculation.operation {
        Operation::Add => calculation.left + calculation.right,
        Operation::Subtract => calculation.left - calculation.right,
        Operation::Multiply => calculation.left * calculation.right,
        Operation::Divide => calculation.left / calculation.right,
        Operation::Power => calculation.left.powf(calculation.right),
    }
}

pub fn perform_calculations(input: Vec<String>) -> Result<f64, String> {
    if input.is_empty() {
        return Err("Input is empty".to_string());
    }

    let mut stack: Vec<f64> = vec![];
    for item in input {
        match item.as_str() {
            "+" | "-" | "/" | "^" | "*" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let calculation = Calculation {
                    left,
                    operation: match item.as_str() {
                        "+" => Operation::Add,
                        "-" => Operation::Subtract,
                        "*" => Operation::Multiply,
                        "/" => Operation::Divide,
                        "^" => Operation::Power,
                        _ => panic!("Unknown operation"),
                    },
                    right,
                };
                let result = perform_calculation(&calculation);
                stack.push(result)
            }
            _ => stack.push(item.parse().unwrap()),
        }
    }
    if stack.len() != 1 {
        return Err(format!("Invalid expression: {} values remain", stack.len()));
    }
    Ok(stack.pop().unwrap().try_into().unwrap())
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
