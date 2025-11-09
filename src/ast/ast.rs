enum UnaryOperator {
    Addition,
    Subtraction,
}
enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Index,
}

pub enum Node {
    Operand {
        value: f64,
    },
    UnaryExpression {
        operation: UnaryOperator,
        child: Box<Node>,
    },
    BinaryExpr {
        operation: BinaryOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Node {
    fn calculate(&self) -> Result<f64, String> {
        match self {
            Node::Operand { value } => Ok(*value),
            Node::UnaryExpression { operation, child } => {
                let value = match child.calculate() {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };
                match operation {
                    UnaryOperator::Addition => Ok(value),
                    UnaryOperator::Subtraction => Ok(-value),
                }
            }
            Node::BinaryExpr {
                operation,
                lhs,
                rhs,
            } => {
                let lhs = match lhs.calculate() {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };
                let rhs = match rhs.calculate() {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };
                match operation {
                    BinaryOperator::Addition => Ok(lhs + rhs),
                    BinaryOperator::Subtraction => Ok(lhs - rhs),
                    BinaryOperator::Multiplication => Ok(lhs * rhs),
                    BinaryOperator::Division => Ok(lhs / rhs),
                    BinaryOperator::Index => Ok(lhs.powf(rhs)),
                }
            }
        }
    }
}

// struct Ast {
//     root: Node,
// }

pub fn test_ast() -> Result<f64, String> {
    let ast = Node::BinaryExpr {
        operation: BinaryOperator::Addition,
        lhs: Box::new(Node::UnaryExpression {
            operation: UnaryOperator::Subtraction,
            child: Box::new(Node::Operand { value: 2.0 }),
        }),
        rhs: Box::new(Node::Operand { value: 3.0 }),
    };
    ast.calculate()
}
