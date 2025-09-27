package lexer

import (
	"testing"

	"mycelium/token"
)

func TestNextToken(t *testing.T) {
	input := `var varname: string = "test";
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
`
	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.VAR, "var"},
		{token.IDENT, "varname"},
		{token.COLON, ":"},
		{token.IDENT, "string"},
		{token.ASSIGN, "="},
		{token.STRING, "test"},
		{token.SEMICOLON, ";"},
		{token.VAL, "val"},
		{token.IDENT, "valname"},
		{token.COLON, ":"},
		{token.IDENT, "int"},
		{token.ASSIGN, "="},
		{token.INT, "5"},
		{token.SEMICOLON, ";"},
		{token.PRIVATE, "prv"},
		{token.FUNCTION, "fnc"},
		{token.IDENT, "fib"},
		{token.OBRACKET, "("},
		{token.IDENT, "a"},
		{token.COLON, ":"},
		{token.IDENT, "int"},
		{token.CBRACKET, ")"},
		{token.COLON, ":"},
		{token.IDENT, "int"},
		{token.OCURLBRACKET, "{"},
		{token.IF, "if"},
		{token.OBRACKET, "("},
		{token.IDENT, "a"},
		{token.EQ, "=="},
		{token.INT, "0"},
		{token.CBRACKET, ")"},
		{token.OCURLBRACKET, "{"},
		{token.RETURN, "return"},
		{token.INT, "0"},
		{token.SEMICOLON, ";"},
		{token.CCURLBRACKET, "}"},
		{token.RETURN, "return"},
		{token.IDENT, "fib"},
		{token.OBRACKET, "("},
		{token.IDENT, "a"},
		{token.MINUS, "-"},
		{token.INT, "1"},
		{token.CBRACKET, ")"},
		{token.PLUS, "+"},
		{token.IDENT, "fib"},
		{token.OBRACKET, "("},
		{token.IDENT, "a"},
		{token.MINUS, "-"},
		{token.INT, "2"},
		{token.CBRACKET, ")"},
		{token.SEMICOLON, ";"},
		{token.CCURLBRACKET, "}"},
		{token.VAR, "var"},
		{token.IDENT, "result"},
		{token.COLON, ":"},
		{token.IDENT, "int"},
		{token.ASSIGN, "="},
		{token.IDENT, "fib"},
		{token.OBRACKET, "("},
		{token.INT, "5"},
		{token.CBRACKET, ")"},
		{token.SEMICOLON, ";"},
		{token.VAL, "val"},
		{token.IDENT, "testrue"},
		{token.COLON, ":"},
		{token.IDENT, "bool"},
		{token.ASSIGN, "="},
		{token.TRUE, "true"},
		{token.SEMICOLON, ";"},
		{token.VAR, "var"},
		{token.IDENT, "testfalse"},
		{token.COLON, ":"},
		{token.IDENT, "bool"},
		{token.ASSIGN, "="},
		{token.FALSE, "false"},
		{token.COLON, ":"},
		{token.EOF, ""},
	}
	lex := New(input)
	for i, tokentype := range tests {
		tok := lex.NextToken()
		if tok.Type != tokentype.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong. expected=%q, got=%q",
				i, tokentype.expectedType, tok.Type)
		}
		if tok.Literal != tokentype.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q, got=%q",
				i, tokentype.expectedLiteral, tok.Literal)
		}
	}
}
