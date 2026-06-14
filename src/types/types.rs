#[derive(Debug, Clone, PartialEq)]
pub enum ValidType {
    Integer,
    String,
}

pub fn is_valid_type(name: &str) -> bool {
    matches!(name, "int" | "string")
}

impl ValidType {
    pub fn lookup(name: &str) -> Option<ValidType> {
        match name {
            "int" => Some(ValidType::Integer),
            "string" => Some(ValidType::String),
            _ => None,
        }
    }
}