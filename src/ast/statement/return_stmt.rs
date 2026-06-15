use crate::ast::{Expression, Node};
use crate::token::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        if self.return_value.is_some() {
            out.push_str(&self.return_value.as_ref().unwrap().string());
        }
        out.push_str(";");
        out
    }
}
