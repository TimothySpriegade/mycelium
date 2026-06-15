use crate::ast::{Expression, Identifier, ReturnStatement};
use crate::token::token::Token;

use super::Parser;

impl Parser {
    pub(super) fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        self.next_token();

        // TODO: still skipping expression for now

        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(ReturnStatement {
            token: Token::RETURN,
            return_value: Some(Expression::Identifier(Identifier {
                token: Token::RETURN,
                value: "TODO".to_string(),
            })),
        })
    }
}
