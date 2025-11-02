use std::io;

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
pub fn infix_to_postfix(mut input: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    while input.len() > 0 {
        let token = input.pop().unwrap();
        println!("Parsing token {}", token);
        if token.parse::<f64>().is_ok() {
            output.push(token);
            continue;
        }
        // if function, push onto operator stack

        match token.as_str() {
            "+" | "-" | "/" | "^" | "*" => {
                while operators.pop().is_some_and(|x| x == "(") {
                    // possible and condition?
                    output.push(operators.last().unwrap().clone());
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
                        _ => panic!("Something wrong"),
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
    output.reverse();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_basic_infix_conversion() {
        let input = vec!["4".to_string(), "+".to_string(), "5".to_string()];
        assert_eq!(
            infix_to_postfix(input),
            vec!["5".to_string(), "4".to_string(), "+".to_string()]
        )
    }
}
