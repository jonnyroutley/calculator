mod ast;
mod rpn;
mod utils;

fn main() {
    let use_rpn = false;

    if use_rpn {
        if let Ok(input) = utils::input::get_input() {
            match rpn::calculate(input) {
                Ok(result) => println!("{}", result),
                Err(error) => println!("Error: {}", error),
            }
        }
    } else {
        ast::main(None);
    }
}
