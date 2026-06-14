pub mod return_stmt;
pub mod val;
pub mod var;

pub use return_stmt::ReturnStatement;
pub use val::ValDefinitionStatement;
pub use var::VarDefinitionStatement;

use super::Node;
use super::expression::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Var(VarDefinitionStatement),
    Val(ValDefinitionStatement),
    Return(ReturnStatement),
    Identifier(Identifier),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Var(stmt) => stmt.token_literal(),
            Statement::Val(stmt) => stmt.token_literal(),
            Statement::Return(stmt) => stmt.token_literal(),
            Statement::Identifier(ident) => ident.token_literal(),
        }
    }
}
