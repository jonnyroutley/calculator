mod ast;
mod rpn;
mod utils;

fn main() {
    let use_rpn = false;
    let input = utils::input::get_input();

    if use_rpn {
        match rpn::calculate(input) {
            Ok(result) => println!("{}", result),
            Err(error) => println!("Error: {}", error),
        }
    } else {
        ast::main(input);
    }
}
