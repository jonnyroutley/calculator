use crate::utils;

pub mod ast;
pub mod parse;

pub fn calculate(input: String) -> Result<f64, String> {
    let infix_input = match utils::input::get_normalized_input(&input) {
        Ok(input) => input,
        Err(e) => return Err(e),
    };
    let ast = match parse::infix_to_ast(infix_input) {
        Ok(ast) => ast,
        Err(e) => return Err(e),
    };
    ast.calculate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proptest::prelude::*;

    #[test]
    fn test_wikipedia_example() {
        let input = String::from("3 + 4 * 2 ÷ ( 1 - 5 ) ^ 2 ^ 3");
        let expected =
            3.0_f64 + 4.0_f64 * 2.0_f64 / (1.0_f64 - 5.0_f64).powf(2.0_f64.powf(3.0_f64));
        assert_eq!(calculate(input), Ok(expected));
    }
    #[test]
    fn test_another_example() {
        let input = String::from("4+5-2*5");
        let expected = 4.0_f64 + 5.0_f64 - 2.0_f64 * 5.0_f64;
        assert_eq!(calculate(input), Ok(expected));
    }

    proptest! {
        #[test]
        fn doesnt_crash_random_strings(s in "\\PC*") {
            let _ = calculate(s);
        }
    }
}
