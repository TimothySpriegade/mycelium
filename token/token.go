// Package token is meant to be handeling the tokenization of the mycelium
package token

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"

	// Identifiers + literals
	IDENT = "IDENT"
	INT   = "INT"
	SRING = "STRING"
	TRUE  = "TRUE"
	FALSE = "FALSE"
	BOOL  = "BOOL"

	// Operators
	ASSIGN    = "="
	PLUS      = "+"
	MINUS     = "-"
	MULT      = "*"
	DIV       = "/"
	EQ        = "=="
	NOTEQ     = "!="
	LESSTHAN  = "<"
	GREATTHAN = ">"
	LESSEQ    = "<="
	GREATEQ   = ">="
	NOT       = "!"

	// Delimiters
	COMMA        = ","
	SEMICOLON    = ";"
	OBRACKET     = "("
	CBRACKET     = ")"
	OCURLBRACKET = "{"
	CCURLBRACKET = "}"
	OCORNBRACKET = "["
	CCORNBRACKE  = "]"
	COLON        = ":"

	// Keywords
	FUNCTION = "FNC"
	VAR      = "VAR"
	VAL      = "VAL"
	RETURN   = "RETURN"
	IF       = "IF"
	ELSE     = "ELSE"
	PRIVATE  = "PRV"
)
