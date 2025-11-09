mod ast;
mod rpn;

fn main() {
    let use_rpn = false;
    if use_rpn {
        let input = rpn::parse::get_input();
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
