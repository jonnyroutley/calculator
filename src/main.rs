mod ast;
mod rpn;
mod utils;

fn main() {
    let use_rpn = false;
    if use_rpn {
        let input = utils::input::get_input();
        match rpn::calculate(input) {
            Ok(result) => println!("{}", result),
            Err(error) => println!("Error: {}", error),
        }
    } else {
        match ast::ast::test_ast() {
            Ok(result) => println!("{}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
}
