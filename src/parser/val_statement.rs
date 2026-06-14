use crate::ast::{Expression, Identifier, ValDefinitionStatement};
use crate::token::token::Token;

use super::Parser;

impl Parser {
    pub(super) fn parse_val_statement(&mut self) -> Option<ValDefinitionStatement> {
        if !self.expect_peek_ident() {
            return None;
        };

        let name = Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone()?.literal(),
        };

        if !self.expect_peek(Token::COLON) {
            return None;
        };

        if !self.expect_peek_ident() {
            return None;
        };

        let ty = Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone()?.literal(),
        };

        if !self.expect_peek(Token::ASSIGN) {
            return None;
        };

        let temp_expression = Expression::Identifier(name.clone());

        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(ValDefinitionStatement {
            token: Token::VAR,
            name,
            ty,
            value: temp_expression,
        })
    }
}
