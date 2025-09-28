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
	IDENT  = "IDENT"
	INT    = "INT"
	STRING = "STRING"
	TRUE   = "TRUE"
	FALSE  = "FALSE"

	// Operators
	ASSIGN    = "="
	PLUS      = "+"
	MINUS     = "-"
	MULT      = "*"
	DIV       = "/"
	BACKSLASH = "\\"
	EQ        = "=="
	LESSTHAN  = "<"
	GREATTHAN = ">"
	LESSEQ    = "<="
	GREATEQ   = ">="
	BANG      = "!"

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

var keywords = map[string]TokenType{
	"fnc":    FUNCTION,
	"var":    VAR,
	"val":    VAL,
	"return": RETURN,
	"if":     IF,
	"else":   ELSE,
	"prv":    PRIVATE,
	"true":   TRUE,
	"false":  FALSE,
}

func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}

var comparators = map[string]TokenType{
	"==": EQ,
	"<=": LESSEQ,
	">=": GREATEQ,
}

func LookUpComparator(comp string) TokenType {
	if tok, ok := comparators[comp]; ok {
		return tok
	}
	return ILLEGAL
}
