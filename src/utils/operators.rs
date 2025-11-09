
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorInfo {
    pub precedence: u8,
    pub associativity: Associativity,
}

pub fn get_operator(symbol: &str) -> Option<&'static OperatorInfo> {
    match symbol {
        "^" => Some(&OperatorInfo {
            precedence: 4,
            associativity: Associativity::Right,
        }),
        "*" => Some(&OperatorInfo {
            precedence: 3,
            associativity: Associativity::Left,
        }),
        "/" => Some(&OperatorInfo {
            precedence: 3,
            associativity: Associativity::Left,
        }),
        "+" => Some(&OperatorInfo {
            precedence: 2,
            associativity: Associativity::Left,
        }),
        "-" => Some(&OperatorInfo {
            precedence: 2,
            associativity: Associativity::Left,
        }),
        _ => None,
    }
}