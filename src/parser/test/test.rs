#[cfg(test)]
mod tests {
    use crate::ast::{Node, Statement};
    use crate::lexer::lexer::Lexer;
    use crate::parser::Parser;
    use crate::types::types::is_valid_type;

    fn check_parser_errors(parser: &Parser) {
        let errors = &parser.errors;
        if errors.is_empty() {
            return;
        }

        eprintln!("Parser has {} errors:", errors.len());
        for msg in errors {
            eprintln!("Parser error: {}", msg);
        }
        panic!("Parser encountered errors during test.");
    }

    #[test]
    fn test_var_statements() {
        let input = r#"
        var testvar: int = 12;
        var testvartwo: string = "test";
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            2,
            "program.statements does not contain 2 statements, got {}",
            program.statements.len()
        );

        let expected_identifiers = vec!["testvar", "testvartwo"];

        for (i, expected_name) in expected_identifiers.into_iter().enumerate() {
            let stmt = &program.statements[i];
            test_var_statement(stmt, expected_name);
        }
    }

    #[test]
    fn test_val_statements() {
        let input = r#"
        val testval: int = 12;
        val testvaltwo: string = "testval";
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            2,
            "program.statements does not contain 2 statements, got {}",
            program.statements.len()
        );

        let expected_identifiers = vec!["testval", "testvaltwo"];

        for (i, expected_name) in expected_identifiers.into_iter().enumerate() {
            let stmt = &program.statements[i];
            test_val_statement(stmt, expected_name);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return 12345;
        return "alskjd";
        "#;

        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got: {}",
            program.statements.len()
        );

        for stmt in program.statements {
            if let Statement::Return(return_stmt) = stmt {
                assert_eq!(
                    return_stmt.token_literal(),
                    "return",
                    "return_stmt.token_literal not 'return', got: '{}'",
                    return_stmt.token_literal()
                );
            } else {
                panic!("stmt not a ReturnStatement, got: {:?}", stmt);
            }
        }
    }

    fn test_var_statement(statement: &Statement, expected_name: &str) {
        assert_eq!(
            statement.token_literal(),
            "var",
            "statement.token_literal not 'var', got='{}'",
            statement.token_literal()
        );

        if let Statement::Var(var_stmt) = statement {
            assert_eq!(
                var_stmt.name.value, expected_name,
                "var_stmt.name.value not '{}'. got={}",
                expected_name, var_stmt.name.value
            );

            assert!(
                is_valid_type(&var_stmt.ty.value),
                "var_stmt.ty.value is not a valid type, got {}",
                var_stmt.ty.value
            );

        } else {
            panic!("statement is not Statement::Var, got {:?}", statement);
        }
    }

    fn test_val_statement(statement: &Statement, expected_name: &str) {
        assert_eq!(
            statement.token_literal(),
            "val",
            "statement.token_literal not 'val', got='{}'",
            statement.token_literal()
        );

        if let Statement::Val(val_stmt) = statement {
            assert_eq!(
                val_stmt.name.value, expected_name,
                "val_stmt.name.value not '{}'. got={}",
                expected_name, val_stmt.name.value
            );

            assert!(
                is_valid_type(&val_stmt.ty.value),
                "val_stmt.ty.value is not a valid type, got {}",
                val_stmt.ty.value
            );

        } else {
            panic!("statement is not Statement::Val, got {:?}", statement);
        }
    }
}
