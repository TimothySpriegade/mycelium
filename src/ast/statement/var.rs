use crate::ast::{Expression, Identifier, Node};
use crate::token::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct VarDefinitionStatement {
    pub token: Token,
    pub name: Identifier,
    pub ty: Identifier,
    pub value: Expression,
}

impl Node for VarDefinitionStatement {
    fn token_literal(&self) -> String {
        "var".to_string()
    }
}
