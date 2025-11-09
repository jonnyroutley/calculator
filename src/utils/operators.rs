#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Index,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorInfo {
    pub operation: BinaryOperator,
    pub precedence: u8,
    pub associativity: Associativity,
}

pub fn get_operator_info(symbol: &str) -> Option<&'static OperatorInfo> {
    match symbol {
        "^" => Some(&OperatorInfo {
            operation: BinaryOperator::Index,
            precedence: 4,
            associativity: Associativity::Right,
        }),
        "*" => Some(&OperatorInfo {
            operation: BinaryOperator::Multiplication,
            precedence: 3,
            associativity: Associativity::Left,
        }),
        "/" => Some(&OperatorInfo {
            operation: BinaryOperator::Division,
            precedence: 3,
            associativity: Associativity::Left,
        }),
        "+" => Some(&OperatorInfo {
            operation: BinaryOperator::Addition,
            precedence: 2,
            associativity: Associativity::Left,
        }),
        "-" => Some(&OperatorInfo {
            operation: BinaryOperator::Subtraction,
            precedence: 2,
            associativity: Associativity::Left,
        }),
        _ => None,
    }
}
