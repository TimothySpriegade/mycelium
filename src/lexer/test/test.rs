#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::token::token::Token; // Adjust this path if your token module is named differently

    #[test]
    fn test_next_token_2() {
        // Using a raw string literal (r#"..."#) so we don't have to escape standard characters
        let input = r#"
        val mult: int = 5 * 5;
        var div: int = 5 / 5

        if (mult !>= div) [
            return mult,
        ] else {
            return div.
        }
        \\
        <=
        !=
    "#
        .to_string();

        let expected_tokens = vec![
            Token::VAL,
            Token::IDENT("mult".to_string()),
            Token::COLON,
            Token::IDENT("int".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::MULTIPLY,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::VAR,
            Token::IDENT("div".to_string()),
            Token::COLON,
            Token::IDENT("int".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::DIVIDE,
            Token::INT("5".to_string()),
            Token::IF,
            Token::OPENBRACKET,
            Token::IDENT("mult".to_string()),
            Token::BANG,
            Token::GREATERTHANEQ, // Based on the '!>=' in the string (lexed as BANG, GREATERTHANEQ)
            Token::IDENT("div".to_string()),
            Token::CLOSEBRACKET,
            Token::OPENCORNERBRACKET,
            Token::RETURN,
            Token::IDENT("mult".to_string()),
            Token::COMMA,
            Token::CLOSEDCORNERBRACKET,
            Token::ELSE,
            Token::OPENCURLBRACKET,
            Token::RETURN,
            Token::IDENT("div".to_string()),
            Token::ILLEGAL(".".to_string()),
            Token::CLOSEDCURLBRACKET,
            Token::BACKSLASH,
            Token::BACKSLASH,
            Token::LESSERTHENEQ,
            Token::NOTEQUAL,
            Token::EOF("EOF".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        for (i, expected_token) in expected_tokens.into_iter().enumerate() {
            let actual_token = lexer.next_token();
            assert_eq!(
                actual_token, expected_token,
                "Test case {} failed. Expected {:?}, got {:?}",
                i, expected_token, actual_token
            );
        }
    }

    #[test]
    fn test_next_token() {
        let input = r#"
        var varname: string = "test";
        val valname: int = 5;

        prv fnc fib(a: int): int {
        if (a == 0) {
            return 0;
        }
        return fib(a-1) + fib(a-2);
    }

        var result: int = fib(5);

        val testrue: bool = true;
        var testfalse: bool = false:
    "#
        .to_string();

        let expected_tokens = vec![
            Token::VAR,
            Token::IDENT("varname".to_string()),
            Token::COLON,
            Token::IDENT("string".to_string()),
            Token::ASSIGN,
            Token::STRING("test".to_string()),
            Token::SEMICOLON,
            Token::VAL,
            Token::IDENT("valname".to_string()),
            Token::COLON,
            Token::IDENT("int".to_string()),
            Token::ASSIGN,
            Token::INT("5".to_string()),
            Token::SEMICOLON,
            Token::PRIVATE,
            Token::FUNCTION,
            Token::IDENT("fib".to_string()),
            Token::OPENBRACKET,
            Token::IDENT("a".to_string()),
            Token::COLON,
            Token::IDENT("int".to_string()),
            Token::CLOSEBRACKET,
            Token::COLON,
            Token::IDENT("int".to_string()),
            Token::OPENCURLBRACKET,
            Token::IF,
            Token::OPENBRACKET,
            Token::IDENT("a".to_string()),
            Token::EQ,
            Token::INT("0".to_string()),
            Token::CLOSEBRACKET,
            Token::OPENCURLBRACKET,
            Token::RETURN,
            Token::INT("0".to_string()),
            Token::SEMICOLON,
            Token::CLOSEDCURLBRACKET,
            Token::RETURN,
            Token::IDENT("fib".to_string()),
            Token::OPENBRACKET,
            Token::IDENT("a".to_string()),
            Token::MINUS,
            Token::INT("1".to_string()),
            Token::CLOSEBRACKET,
            Token::PLUS,
            Token::IDENT("fib".to_string()),
            Token::OPENBRACKET,
            Token::IDENT("a".to_string()),
            Token::MINUS,
            Token::INT("2".to_string()),
            Token::CLOSEBRACKET,
            Token::SEMICOLON,
            Token::CLOSEDCURLBRACKET,
            Token::VAR,
            Token::IDENT("result".to_string()),
            Token::COLON,
            Token::IDENT("int".to_string()),
            Token::ASSIGN,
            Token::IDENT("fib".to_string()),
            Token::OPENBRACKET,
            Token::INT("5".to_string()),
            Token::CLOSEBRACKET,
            Token::SEMICOLON,
            Token::VAL,
            Token::IDENT("testrue".to_string()),
            Token::COLON,
            Token::IDENT("bool".to_string()),
            Token::ASSIGN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::VAR,
            Token::IDENT("testfalse".to_string()),
            Token::COLON,
            Token::IDENT("bool".to_string()),
            Token::ASSIGN,
            Token::FALSE,
            Token::COLON,
            Token::EOF("EOF".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        for (i, expected_token) in expected_tokens.into_iter().enumerate() {
            let actual_token = lexer.next_token();
            assert_eq!(
                actual_token, expected_token,
                "Test case {} failed. Expected {:?}, got {:?}",
                i, expected_token, actual_token
            );
        }
    }
}
