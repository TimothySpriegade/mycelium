package ast

import (
	"mycelium/token"
)

type Node interface {
	TokenLiteral() string
}

type Statement interface {
	Node
	statementNode()
}

type Expression interface {
	Node
	expressionNode()
}

type Program struct {
	Statements []Statement
}

type Identifier struct {
	Token token.Token
	Value string
}

func (i *Identifier) statementNode()       {}
func (i *Identifier) TokenLiteral() string { return i.Token.Literal }

type VarDefinitionStatement struct {
	Token token.Token
	Name  *Identifier
	Type  *Identifier
	Value Expression
}

type ValDefinitionStatement struct {
	Token token.Token
	Name  *Identifier
	Type  *Identifier
	Value Expression
}

func (varS *VarDefinitionStatement) statementNode()       {}
func (varS *VarDefinitionStatement) TokenLiteral() string { return varS.Token.Literal }

func (valS *ValDefinitionStatement) statementNode()       {}
func (valS *ValDefinitionStatement) TokenLiteral() string { return valS.Token.Literal }

type ReturnStatement struct {
	Token       token.Token
	ReturnValue Expression
}

func (rs *ReturnStatement) statementNode()       {}
func (rs *ReturnStatement) TokenLiteral() string { return rs.Token.Literal }

func (p *Program) TokenLiteral() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].TokenLiteral()
	} else {
		return ""
	}
}
