use crate::utils::operators::BinaryOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Operand {
        value: f64,
    },
    BinaryExpr {
        operation: BinaryOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Node {
    pub fn calculate(&self) -> Result<f64, String> {
        match self {
            Node::Operand { value } => Ok(*value),
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
