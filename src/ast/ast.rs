use crate::utils::operators::BinaryOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Placeholder {
        position: usize,
    },
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
    pub fn replace_placeholders(&self, arguments: &Vec<&str>) -> Result<Node, String> {
        match self {
            Node::Placeholder { position } => {
                println!("Arguments: {:?}", arguments);
                println!("Position: {:?}", position);
                println!("Argument: {:?}", arguments[*position]);
                Ok(Node::Operand {
                    value: arguments[*position].parse().unwrap(),
                })
            }
            Node::Operand { value } => Ok(Node::Operand {
                value: value.clone(),
            }),
            Node::BinaryExpr {
                operation,
                lhs,
                rhs,
            } => Ok(Node::BinaryExpr {
                operation: operation.clone(),
                lhs: Box::new(lhs.replace_placeholders(arguments).unwrap()),
                rhs: Box::new(rhs.replace_placeholders(arguments).unwrap()),
            }),
            // Node::FunctionExpr {

            // }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    pub name: String,
    pub num_arguments: usize,
    pub template: Box<Node>,
}

// fn
impl Node {
    pub fn calculate(&self) -> Result<f64, String> {
        match self {
            Node::Placeholder {
                position: _position,
            } => Err("Placeholder cannot be calculated".to_string()),
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
