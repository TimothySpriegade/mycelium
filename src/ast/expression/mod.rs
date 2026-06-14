pub mod identifier;

pub use identifier::Identifier;

use super::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(ident) => ident.token_literal(),
        }
    }
}
