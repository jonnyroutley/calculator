use crate::{calculate::perform_calculations, parse::infix_to_postfix};

mod calculate;
mod parse;

fn main() {
    let infix_input = parse::get_normalized_input();
    let postfix_input = infix_to_postfix(infix_input);
    let result = perform_calculations(postfix_input);
    println!("Result: {}", result);
}
