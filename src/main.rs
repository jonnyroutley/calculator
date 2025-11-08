use crate::{calculate::perform_calculations, parse::infix_to_postfix};

mod calculate;
mod parse;

fn main() {
    let mut input = parse::get_input();
    let infix_input = parse::get_normalized_input(&mut input);
    let postfix_input = infix_to_postfix(infix_input);
    let result = perform_calculations(postfix_input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::get_normalized_input;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_wikipedia_example() {
        let mut input = String::from("3 + 4 * 2 ÷ ( 1 - 5 ) ^ 2 ^ 3");
        let normalized_input = get_normalized_input(&mut input);
        let postfix_input = infix_to_postfix(normalized_input);
        let result = perform_calculations(postfix_input);
        let expected = 3 + 4 * 2 / (1 - 5) ^ 2 ^ 3;
        assert_eq!(result, expected.to_string());
    }
}
