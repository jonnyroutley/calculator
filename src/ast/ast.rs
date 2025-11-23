use std::collections::HashMap;

use crate::utils::operators::BinaryOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractNode {
    Placeholder {
        arg_name: String,
    },
    Operand {
        value: f64,
    },
    BinaryExpr {
        operation: BinaryOperator,
        lhs: Box<AbstractNode>,
        rhs: Box<AbstractNode>,
    },
}

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

impl AbstractNode {
    pub fn replace_placeholders(&self, arguments: &HashMap<String, &str>) -> Result<Node, String> {
        match self {
            AbstractNode::Placeholder { arg_name } => {
                let arg_value = arguments.get(arg_name)
                    .ok_or_else(|| format!("Missing argument: {}", arg_name))?;
                let value = arg_value.parse::<f64>()
                    .map_err(|e| format!("Failed to parse argument '{}' as number: {}", arg_name, e))?;
                Ok(Node::Operand { value })
            },
            AbstractNode::Operand { value } => Ok(Node::Operand {
                value: value.clone(),
            }),
            AbstractNode::BinaryExpr {
                operation,
                lhs,
                rhs,
            } => Ok(Node::BinaryExpr {
                operation: operation.clone(),
                lhs: Box::new(lhs.replace_placeholders(arguments)?),
                rhs: Box::new(rhs.replace_placeholders(arguments)?),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentDefinition {
    pub name: String,
    pub position: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpr {
    pub name: String,
    pub arguments: Vec<ArgumentDefinition>,
    pub template: Box<AbstractNode>,
}

// fn
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
