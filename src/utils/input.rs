use std::io;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn is_function_assignment(input: &str) -> bool {
    input.starts_with("fn")
}

pub fn is_function_call(input: &str) -> bool {
    input
        .split("(")
        .nth(0)
        .is_some_and(|x| x.chars().all(|c| c.is_ascii_alphabetic()))
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
                // FIXME: add in brackets too around the 0
                if ch == '+' || ch == '-' {
                    let last_part = parts.last();
                    if last_part.is_none()
                        || last_part.is_some_and(|x: &String| x.parse::<f64>().is_err())
                    {
                        parts.push("0".to_string());
                    }
                }

                parts.push(ch.to_string());
            },
            'a'..='z' => {
                accumulated.push(ch);
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
