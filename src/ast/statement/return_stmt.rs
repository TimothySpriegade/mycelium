use crate::ast::{Expression, Node};
use crate::token::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        "return".to_string()
    }
}
