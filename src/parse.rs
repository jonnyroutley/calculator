use std::io;

#[derive(PartialEq, Eq)]
enum Associativity {
    Left,
    Right,
}

const OPERATOR_MAP: &[(&str, i32, Associativity)] = &[
    ("^", 4, Associativity::Right),
    ("*", 3, Associativity::Left),
    ("/", 3, Associativity::Left),
    ("+", 2, Associativity::Left),
    ("-", 2, Associativity::Left),
];

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn get_normalized_input(mut input: &mut String) -> Vec<String> {
    remove_whitespace(&mut input);
    // Normalize alternative operator symbols
    *input = input.replace('÷', "/");
    let mut parts: Vec<String> = vec![];

    let mut accumulated: Vec<char> = vec![];
    for thing in input.chars() {
        if thing.to_digit(10).is_some() {
            accumulated.push(thing)
        } else {
            if !accumulated.is_empty() {
                parts.push(accumulated.iter().collect());
            }
            parts.push(thing.to_string());
            accumulated.clear();
        }
    }
    if !accumulated.is_empty() {
        parts.push(accumulated.iter().collect());
    }
    parts
}

// shunting yard https://en.wikipedia.org/wiki/Shunting_yard_algorithm
pub fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    let mut reversed_input = input.clone();
    reversed_input.reverse();

    // while there are tokens to be read
    while reversed_input.len() > 0 {
        let token = reversed_input.pop().unwrap();

        // if the token is a number, push it into the output queue
        if token.parse::<f64>().is_ok() {
            output.push(token);
            continue;
        }
        // TODO: implement functions here

        // if the token is an operator,
        match token.as_str() {
            "+" | "-" | "/" | "^" | "*" => {
                // there is an operator o2 at the top of the operator stack which is not a left parenthesis,
                // and (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1 is left-associative))
                loop {
                    let o2 = operators.last();
                    if let Some(o2) = o2 {
                        if o2 == "(" {
                            break;
                        }

                        let o1_config = OPERATOR_MAP.iter().find(|x| x.0 == token).unwrap();
                        let o2_config = OPERATOR_MAP.iter().find(|x| x.0 == o2).unwrap();
                        if o2_config.1 > o1_config.1
                            || (o2_config.1 == o1_config.1 && o1_config.2 == Associativity::Left)
                        {
                            // pop o2 from the operator stack into the output queue
                            output.push(operators.pop().unwrap());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                // push o1 onto the operator stack
                operators.push(token.clone())
            }
            // TODO: implement commas here
            // if the token is a left parenthesis, push it onto the operator stack
            "(" => operators.push(token.clone()),
            // if the token is a right parenthesis,
            ")" => {
                // while the operator at the top of the operator stack is not a left parenthesis:
                loop {
                    let o = operators.last();
                    match o {
                        Some(o) => {
                            if o == "(" {
                                break;
                            }
                            // pop the operator from the operator stack into the output queue
                            output.push(operators.pop().unwrap().clone())
                        }
                        None => panic!("Mismatched parentheses found!"),
                    }
                }
                // pop the left parenthesis from the operator stack and discard it
                let o = operators.pop().unwrap();
                if o != "(" {
                    panic!("Unexpected!");
                }
                // handle more functions here
            }
            _ => panic!("Something isn't supported here"),
        }
    }

    while operators.len() > 0 {
        // assert operator on top of the stack is not a left parenthesis
        let o = operators.pop().unwrap();
        match o.as_str() {
            "(" => panic!("mismatched parentheses found!"),
            _ => output.push(o),
        }
    }
    output
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
            vec!["4".to_string(), "5".to_string(), "+".to_string()]
        )
    }

    #[test]
    fn test_basic_infix_conversion_subtraction() {
        let input = vec!["4".to_string(), "-".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            vec!["4".to_string(), "5".to_string(), "-".to_string()]
        )
    }

    #[test]
    fn test_basic_infix_conversion_multiplication() {
        let input = vec!["4".to_string(), "*".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            vec!["4".to_string(), "5".to_string(), "*".to_string()]
        )
    }

    #[test]
    fn test_basic_infix_conversion_division() {
        let input = vec!["4".to_string(), "/".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            vec!["4".to_string(), "5".to_string(), "/".to_string()]
        )
    }

    #[test]
    fn test_basic_infix_conversion_index() {
        let input = vec!["4".to_string(), "^".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            vec!["4".to_string(), "5".to_string(), "^".to_string()]
        )
    }

    #[test]
    fn test_wikipedia_example() {
        // 3 + 4 × 2 ÷ ( 1 − 5 ) ^ 2 ^ 3
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
        // output: 3 4 2 × 1 5 − 2 3 ^ ^ ÷ +
        assert_eq!(
            infix_to_postfix(input),
            vec![
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
            ]
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
            vec![
                "4".to_string(),
                "1".to_string(),
                "5".to_string(),
                "-".to_string(),
                "+".to_string(),
            ]
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
            vec![
                "4".to_string(),
                "5".to_string(),
                "+".to_string(),
                "2".to_string(),
                "5".to_string(),
                "*".to_string(),
                "-".to_string()
            ]
        )
    }
}
