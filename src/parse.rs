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

pub fn get_normalized_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Input: {}", input);
    remove_whitespace(&mut input);
    let mut parts: Vec<String> = vec![];

    let mut accumulated: Vec<char> = vec![];
    for thing in input.chars() {
        if thing.to_digit(10).is_some() {
            accumulated.push(thing)
        } else {
            parts.push(accumulated.iter().collect());
            parts.push(thing.to_string());
            accumulated.clear();
        }
    }
    parts.push(accumulated.iter().collect());
    // let parts: Vec<String> = input.split('o').map(|s| s.to_string()).collect();
    println!("Parts: {:?}", parts);
    // parts.reverse();
    parts
}

// shunting yard https://en.wikipedia.org/wiki/Shunting_yard_algorithm
pub fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    let mut reversed_input = input.clone();
    reversed_input.reverse();

    while reversed_input.len() > 0 {
        println!("Output: {:?}, operators: {:?}", output, operators);
        let token = reversed_input.pop().unwrap();
        if token.parse::<f64>().is_ok() {
            output.push(token);
            continue;
        }
        // if function, push onto operator stack

        match token.as_str() {
            "+" | "-" | "/" | "^" | "*" => {
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
                            output.push(o2.to_string());
                        }
                    } else {
                        break;
                    }
                }
                operators.push(token.clone())
            }
            // here is where you would handle comma
            "(" => operators.push(token.clone()),
            ")" => {
                loop {
                    let o = operators.last();
                    match o {
                        Some(o) => output.push(o.clone()),
                        None => panic!("Mismatched parentheses found!"),
                    }
                    // assert left parenthesis at the top of the operator stack
                    let o = operators.last().unwrap();
                    match o.as_str() {
                        "(" => {
                            operators.pop();
                        }
                        _ => panic!("Something wrong, found {}", o),
                    }
                }
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
    println!("Postfix: {:?}", output);
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
}
