// Package lexer: contains the lexer
package lexer

import (
	"mycelium/token"
)

type Lexer struct {
	input        string
	position     int  // current position in input
	readPosition int  // points to the h after position
	ch           byte // current char
}

func New(input string) *Lexer {
	l := &Lexer{input: input}
	return l
}

func (lexer *Lexer) readChar() {
	if lexer.readPosition >= len(lexer.input) {
		lexer.ch = 0
	} else {
		lexer.ch = lexer.input[lexer.readPosition]
	}
	lexer.position = lexer.readPosition
	lexer.readPosition += 1
}

func (lexer *Lexer) NextToken() token.Token {
	var tok token.Token
	switch lexer.ch {
	case '=':
		tok = newToken(token.ASSIGN, lexer.ch)
	case '+':
		tok = newToken(token.PLUS, lexer.ch)
	case '-':
		tok = newToken(token.MINUS, lexer.ch)
	case '*':
		tok = newToken(token.MULT, lexer.ch)
	case '/':
		tok = newToken(token.DIV, lexer.ch)
	case '<':
		tok = newToken(token.LESSTHAN, lexer.ch)
	case '>':
		tok = newToken(token.GREATTHAN, lexer.ch)
	case '!':
		tok = newToken(token.NOT, lexer.ch)
	case ',':
		tok = newToken(token.COMMA, lexer.ch)
	case ';':
		tok = newToken(token.SEMICOLON, lexer.ch)
	case '(':
		tok = newToken(token.OBRACKET, lexer.ch)
	case ')':
		tok = newToken(token.CBRACKET, lexer.ch)
	case '{':
		tok = newToken(token.OCURLBRACKET, lexer.ch)
	case '}':
		tok = newToken(token.CCURLBRACKET, lexer.ch)
	case '[':
		tok = newToken(token.OCORNBRACKET, lexer.ch)
	case ']':
		tok = newToken(token.CCORNBRACKE, lexer.ch)
	case ':':
		tok = newToken(token.COLON, lexer.ch)
	case 0:
		tok.Literal = ""
		tok.Type = token.EOF
	default:
		tok = newToken(token.ILLEGAL, lexer.ch)
	}

	lexer.readChar()
	return tok
}

func newToken(tokenType token.TokenType, ch byte) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}
