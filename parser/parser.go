package parser

import (
	"mycelium/ast"
	"mycelium/lexer"
	"mycelium/token"
)

type Parser struct {
	lex *lexer.Lexer

	currentToken token.Token
	peekToken    token.Token
}

func New(lex *lexer.Lexer) *Parser {
	parser := &Parser{lex: lex}

	parser.nextToken()
	parser.nextToken()

	return parser
}

func (parser *Parser) nextToken() {
	parser.currentToken = parser.peekToken
	parser.peekToken = parser.lex.NextToken()
}

func (parser *Parser) ParseProgram() *ast.Program {
	program := &ast.Program{}
	program.Statements = []ast.Statement{}

	for parser.currentToken.Type != token.EOF {
		stmt := parser.parseStatement()
		if stmt != nil {
			program.Statements = append(program.Statements, stmt)
		}
		parser.nextToken()
	}
	return program
}

func (parser *Parser) parseStatement() ast.Statement {
	switch parser.currentToken.Type {
	case token.VAR:
		return parser.parseVarStatement()
	case token.VAL:
		return parser.parseValStatement()
	default:
		return nil
	}
}

func (parser *Parser) parseVarStatement() *ast.VarDefinitionStatement {
	stmt := &ast.VarDefinitionStatement{Token: parser.currentToken}

	if !parser.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Name = &ast.Identifier{Token: parser.currentToken, Value: parser.currentToken.Literal}

	if !parser.expectPeek(token.COLON) {
		return nil
	}

	if !parser.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Type = &ast.Identifier{Token: parser.currentToken, Value: parser.currentToken.Literal}

	if !parser.expectPeek(token.ASSIGN) {
		return nil
	}
	// TODO we skip expression for now and search straight for semicolon

	for !parser.currentTokenIs(token.SEMICOLON) {
		parser.nextToken()
	}
	return stmt
}

func (parser *Parser) parseValStatement() *ast.ValDefinitionStatement {
	stmt := &ast.ValDefinitionStatement{Token: parser.currentToken}

	if !parser.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Name = &ast.Identifier{Token: parser.currentToken, Value: parser.currentToken.Literal}

	if !parser.expectPeek(token.COLON) {
		return nil
	}

	if !parser.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Type = &ast.Identifier{Token: parser.currentToken, Value: parser.currentToken.Literal}

	if !parser.expectPeek(token.ASSIGN) {
		return nil
	}
	// TODO we skip expression for now and search straight for semicolon

	for !parser.currentTokenIs(token.SEMICOLON) {
		parser.nextToken()
	}
	return stmt
}

func (parser *Parser) currentTokenIs(token token.TokenType) bool {
	return parser.currentToken.Type == token
}

func (parser *Parser) peekTokenIs(token token.TokenType) bool {
	return parser.peekToken.Type == token
}

func (parser *Parser) expectPeek(token token.TokenType) bool {
	if parser.peekTokenIs(token) {
		parser.nextToken()
		return true
	} else {
		return false
	}
}
