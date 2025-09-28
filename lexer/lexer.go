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
	lexer := &Lexer{input: input}
	lexer.readChar()
	return lexer
}

func (lex *Lexer) NextToken() token.Token {
	var tok token.Token
	lex.skipWhitespace()
	switch lex.ch {
	case '=':
		if isComparator(lex.ch) {
			tok.Literal = lex.readComparator()
			if len(tok.Literal) == 2 {
				tok.Type = token.LookUpComparator(tok.Literal)
				return tok
			}
			tok.Type = token.ASSIGN
		}
		return tok
	case '+':
		tok = newToken(token.PLUS, lex.ch)
	case '-':
		tok = newToken(token.MINUS, lex.ch)
	case '*':
		tok = newToken(token.MULT, lex.ch)
	case '/':
		tok = newToken(token.DIV, lex.ch)
	case '\\':
		tok = newToken(token.BACKSLASH, lex.ch)
	case '<':
		if isComparator(lex.ch) {
			tok.Literal = lex.readComparator()
			if len(tok.Literal) == 2 {
				tok.Type = token.LookUpComparator(tok.Literal)
				return tok
			}
			tok.Type = token.ASSIGN
		}
		return tok
	case '>':
		if isComparator(lex.ch) {
			tok.Literal = lex.readComparator()
			if len(tok.Literal) == 2 {
				tok.Type = token.LookUpComparator(tok.Literal)
				return tok
			}
			tok.Type = token.ASSIGN
		}
		return tok
	case '!':
		tok = newToken(token.BANG, lex.ch)
	case ',':
		tok = newToken(token.COMMA, lex.ch)
	case ';':
		tok = newToken(token.SEMICOLON, lex.ch)
	case '(':
		tok = newToken(token.OBRACKET, lex.ch)
	case ')':
		tok = newToken(token.CBRACKET, lex.ch)
	case '{':
		tok = newToken(token.OCURLBRACKET, lex.ch)
	case '}':
		tok = newToken(token.CCURLBRACKET, lex.ch)
	case '[':
		tok = newToken(token.OCORNBRACKET, lex.ch)
	case ']':
		tok = newToken(token.CCORNBRACKE, lex.ch)
	case ':':
		tok = newToken(token.COLON, lex.ch)
	case '"':
		tok.Literal = lex.readString()
		tok.Type = token.STRING
	case 0:
		tok.Literal = ""
		tok.Type = token.EOF
	default:
		if isLetter(lex.ch) {
			tok.Literal = lex.readIdentifier()
			tok.Type = token.LookupIdent(tok.Literal)
			return tok
		} else if isDigit(lex.ch) {
			tok.Type = token.INT
			tok.Literal = lex.readNumber()
			return tok
		} else {
			tok = newToken(token.ILLEGAL, lex.ch)
		}
	}

	lex.readChar()
	return tok
}

func newToken(tokenType token.TokenType, ch byte) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}

func (lex *Lexer) skipWhitespace() {
	for lex.ch == ' ' || lex.ch == '\t' || lex.ch == '\n' || lex.ch == '\r' {
		lex.readChar()
	}
}

func (lex *Lexer) readString() string {
	lex.readChar()
	startposition := lex.position
	var endposition int
	for {
		if lex.ch == '"' || lex.ch == 0 {
			endposition = lex.position
			break
		}
		lex.readChar()
	}
	return lex.input[startposition:endposition]
}

func (lex *Lexer) readIdentifier() string {
	position := lex.position
	for isLetter(lex.ch) {
		lex.readChar()
	}
	return lex.input[position:lex.position]
}

func (lex *Lexer) readComparator() string {
	position := lex.position
	for isComparator(lex.ch) {
		lex.readChar()
	}
	return lex.input[position:lex.position]
}

func (lex *Lexer) readChar() {
	if lex.readPosition >= len(lex.input) {
		lex.ch = 0
	} else {
		lex.ch = lex.input[lex.readPosition]
	}
	lex.position = lex.readPosition
	lex.readPosition += 1
}

func (lex *Lexer) readNumber() string {
	position := lex.position
	for isDigit(lex.ch) {
		lex.readChar()
	}
	return lex.input[position:lex.position]
}

func isDigit(ch byte) bool {
	return '0' <= ch && ch <= '9'
}

func isComparator(ch byte) bool {
	return '<' <= ch && ch <= '>'
}

func isLetter(ch byte) bool {
	return 'A' <= ch && ch <= 'Z' || 'a' <= ch && ch <= 'z' || ch == '_'
}
