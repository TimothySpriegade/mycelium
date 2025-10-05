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

func (varS *ValDefinitionStatement) statementNode()       {}
func (varS *ValDefinitionStatement) TokenLiteral() string { return varS.Token.Literal }

func (p *Program) TokenLiteral() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].TokenLiteral()
	} else {
		return ""
	}
}
