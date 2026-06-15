use crate::ast::Node;
use crate::token::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.value.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}
