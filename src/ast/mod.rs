pub mod expression;
pub mod statement;

pub use expression::*;
pub use statement::*;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .first()
            .map_or_else(String::new, |stmt| stmt.token_literal())
    }

    fn string(&self) -> String {
        self.statements.iter().map(|stmt| stmt.string()).collect::<String>()
    }
}
