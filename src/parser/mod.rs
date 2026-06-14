// 1. Declare your submodules so Rust compiles them
mod return_statement;
mod test;
mod val_statement;
mod var_statement;
// mod return_statement; // Add this when you create the file!

use crate::ast::{Program, Statement};
use crate::lexer::lexer::Lexer;
use crate::token::token::Token;

pub struct Parser {
    pub lexer: Lexer,
    pub errors: Vec<String>,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let errors = vec![];

        let mut parser = Parser {
            lexer,
            errors,
            current_token: None,
            peek_token: None,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };

        while self.current_token != Some(Token::EOF("EOF".to_string())) {
            let statement = self.parse_statement();
            if let Some(statement) = statement { program.statements.push(statement) }
            self.next_token()
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Some(Token::VAR) => self.parse_var_statement().map(Statement::Var),
            Some(Token::VAL) => self.parse_val_statement().map(Statement::Val),
            Some(Token::RETURN) => self.parse_return_statement().map(Statement::Return),
            _ => None,
        }
    }

    pub(super) fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Option::from(self.lexer.next_token());
    }

    pub(super) fn current_token_is(&mut self, token: Token) -> bool {
        self.current_token == Some(token)
    }

    pub(super) fn peek_token_is(&mut self, token: Token) -> bool {
        self.peek_token == Some(token)
    }

    pub(super) fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token_is(token.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    pub(super) fn expect_peek_ident(&mut self) -> bool {
        match self.peek_token {
            Some(Token::IDENT(_)) => {
                self.next_token();
                true
            }
            _ => false,
        }
    }

    fn peek_error(&mut self, token: Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            token, self.peek_token
        );
        self.errors.push(msg);
    }
}
