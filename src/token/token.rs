#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL(String),
    EOF(String),

    IDENT(String),
    INT(String),
    STRING(String),
    TRUE,
    FALSE,

    ASSIGN,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    BACKSLASH,
    EQ,
    LESSTHAN,
    GREATERTHAN,
    LESSERTHENEQ,
    GREATERTHANEQ,
    BANG,
    NOTEQUAL,

    COMMA,
    SEMICOLON,
    OPENBRACKET,
    CLOSEBRACKET,
    OPENCURLBRACKET,
    CLOSEDCURLBRACKET,
    OPENCORNERBRACKET,
    CLOSEDCORNERBRACKET,
    COLON,

    FUNCTION,
    VAR,
    VAL,
    RETURN,
    IF,
    ELSE,
    PRIVATE,
}

impl Token {
    pub fn lookup_identifier(identifier: &str) -> Token {
        match identifier {
            "fnc" => Token::FUNCTION,
            "var" => Token::VAR,
            "val" => Token::VAL,
            "return" => Token::RETURN,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "prv" => Token::PRIVATE,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            _ => Token::IDENT(identifier.to_string()),
        }
    }

    pub fn lookup_comparator(comparator: &str) -> Token {
        match comparator {
            "==" => Token::EQ,
            "<" => Token::LESSTHAN,
            ">" => Token::GREATERTHAN,
            ">=" => Token::GREATERTHANEQ,
            "<=" => Token::LESSERTHENEQ,
            _ => Token::ILLEGAL(comparator.to_string()),
        }
    }

    pub fn literal(&self) -> String {
        match self {
            // Variants that carry their own string data
            Token::ILLEGAL(s)
            | Token::EOF(s)
            | Token::IDENT(s)
            | Token::INT(s)
            | Token::STRING(s) => s.clone(),

            // Booleans
            Token::TRUE => "true".to_string(),
            Token::FALSE => "false".to_string(),

            // Operators
            Token::ASSIGN => "=".to_string(),
            Token::PLUS => "+".to_string(),
            Token::MINUS => "-".to_string(),
            Token::MULTIPLY => "*".to_string(),
            Token::DIVIDE => "/".to_string(),
            Token::BACKSLASH => "\\".to_string(),
            Token::EQ => "==".to_string(),
            Token::LESSTHAN => "<".to_string(),
            Token::GREATERTHAN => ">".to_string(),
            Token::LESSERTHENEQ => "<=".to_string(),
            Token::GREATERTHANEQ => ">=".to_string(),
            Token::BANG => "!".to_string(),
            Token::NOTEQUAL => "!=".to_string(),

            // Delimiters
            Token::COMMA => ",".to_string(),
            Token::SEMICOLON => ";".to_string(),
            Token::OPENBRACKET => "(".to_string(),
            Token::CLOSEBRACKET => ")".to_string(),
            Token::OPENCURLBRACKET => "{".to_string(),
            Token::CLOSEDCURLBRACKET => "}".to_string(),
            Token::OPENCORNERBRACKET => "[".to_string(),
            Token::CLOSEDCORNERBRACKET => "]".to_string(),
            Token::COLON => ":".to_string(),

            // Keywords
            Token::FUNCTION => "fnc".to_string(),
            Token::VAR => "var".to_string(),
            Token::VAL => "val".to_string(),
            Token::RETURN => "return".to_string(),
            Token::IF => "if".to_string(),
            Token::ELSE => "else".to_string(),
            Token::PRIVATE => "prv".to_string(),
        }
    }
}
