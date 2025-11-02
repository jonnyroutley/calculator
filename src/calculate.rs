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

pub fn perform_calculations(mut stack: Vec<String>) -> String {
    while stack.len() > 1 {
        // get first three
        let left = stack.pop().unwrap();
        let right = stack.pop().unwrap();
        let operation = stack.pop().unwrap();
        println!("left: {}, right: {}, operation: {}", left, right, operation);

        let calculation = Calculation {
            left: left.parse().unwrap(),
            operation: match operation.as_str() {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                "^" => Operation::Power,
                _ => panic!("Unknown operation"),
            },
            right: right.parse().unwrap(),
        };
        let result = perform_calculation(&calculation);
        stack.push(result.to_string())
    }
    stack.pop().unwrap().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic_addition() {
        assert_eq!(
            perform_calculations(vec!["+".to_string(), "2".to_string(), "3".to_string()]),
            "5"
        );
    }

    #[test]
    fn test_basic_subtraction() {
        assert_eq!(
            perform_calculations(vec!["-".to_string(), "3".to_string(), "5".to_string()]),
            "2"
        );
    }

    #[test]
    fn test_basic_multiplication() {
        assert_eq!(
            perform_calculations(vec!["*".to_string(), "4".to_string(), "6".to_string()]),
            "24"
        );
    }

    #[test]
    fn test_basic_division() {
        assert_eq!(
            perform_calculations(vec!["/".to_string(), "3".to_string(), "15".to_string()]),
            "5"
        );
    }

    #[test]
    fn test_chained_operations() {
        assert_eq!(
            perform_calculations(vec![
                "*".to_string(),
                "4".to_string(),
                "+".to_string(),
                "2".to_string(),
                "3".to_string(),
            ]),
            "20"
        );
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(
            perform_calculations(vec![
                "-".to_string(),
                "15".to_string(),
                "/".to_string(),
                "7".to_string(),
                "+".to_string(),
                "1".to_string(),
                "1".to_string()
            ]),
            "-14.714285714285714"
        );
    }

    #[test]
    fn test_single_number() {
        assert_eq!(perform_calculations(vec!["42".to_string()]), "42");
    }

    #[test]
    fn test_negative_result() {
        assert_eq!(
            perform_calculations(vec!["-".to_string(), "5".to_string(), "0".to_string()]),
            "-5"
        );
    }

    #[test]
    fn test_multiple_additions() {
        assert_eq!(
            perform_calculations(vec![
                "+".to_string(),
                "1".to_string(),
                "+".to_string(),
                "2".to_string(),
                "3".to_string()
            ]),
            "6"
        );
    }
}
