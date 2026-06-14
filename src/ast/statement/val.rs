use crate::ast::{Expression, Identifier, Node};
use crate::token::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct ValDefinitionStatement {
    pub token: Token,
    pub name: Identifier,
    pub ty: Identifier,
    pub value: Expression,
}

impl Node for ValDefinitionStatement {
    fn token_literal(&self) -> String {
        "val".to_string()
    }
}
