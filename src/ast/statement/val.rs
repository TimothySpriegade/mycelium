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
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.string());
        out.push_str(": ");
        out.push_str(&self.ty.string());
        out.push_str(" = ");
        out.push_str(&self.value.string());
        out.push_str(";");
        out
    }
}
