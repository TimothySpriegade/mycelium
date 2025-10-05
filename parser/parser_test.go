package parser

import (
	"testing"

	"mycelium/ast"
	"mycelium/lexer"
	"mycelium/types"
)

func TestVarStatements(t *testing.T) {
	input := `
	var testvar: int = 12;
	var testvartwo: string = "test";
	`
	lex := lexer.New(input)
	pars := New(lex)

	program := pars.ParseProgram()
	if program == nil {
		t.Fatalf("ParseProgram() returned nil")
	}

	if len(program.Statements) != 2 {
		t.Fatalf("program.Statements does not contain 2 statements got %d", len(program.Statements))
	}

	tests := []struct {
		expectedIdentifier string
	}{
		{"testvar"},
		{"testvartwo"},
	}

	for i, tt := range tests {
		stmt := program.Statements[i]
		if !testVarAndVarStatement(t, stmt, tt.expectedIdentifier) {
			return
		}
	}
}

func TestValStatements(t *testing.T) {
	input := `
	val testval: int = 12;
	val testvaltwo: string =  "testval";
	`
	lex := lexer.New(input)
	pars := New(lex)

	program := pars.ParseProgram()
	if program == nil {
		t.Fatalf("ParseProgram() returned nil")
	}

	if len(program.Statements) != 2 {
		t.Fatalf("program.Statements does not contain 2 statements got %d", len(program.Statements))
	}

	tests := []struct {
		expectedIdentifier string
	}{
		{"testval"},
		{"testvaltwo"},
	}

	for i, tt := range tests {
		stmt := program.Statements[i]
		if !testVarAndValStatement(t, stmt, tt.expectedIdentifier) {
			return
		}
	}
}

func testVarAndValStatement(t *testing.T, statement ast.Statement, name string) bool {
	if statement.TokenLiteral() != "val" {
		t.Errorf("statement.TokenLiteral not 'val' got=%q", statement.TokenLiteral())
		return false
	}

	valStmt, ok := statement.(*ast.ValDefinitionStatement)
	
	if !ok {
		t.Errorf("statement is not *ast.ValDefinitionStatement got %d", statement)
		return false
	}

	if valStmt.Name.Value != name {
		t.Errorf("valStmt.Name.Value not '%s'. got=%s", name, valStmt.Name.Value)
		return false
	}

	if !types.IsValidType(valStmt.Type.Value) {
		t.Errorf("valStmt.Type.Value is not a valid type, got %s", valStmt.Type.Value)
		return false
	}

	return true
}


func testVarAndVarStatement(t *testing.T, statement ast.Statement, name string) bool {
	if statement.TokenLiteral() != "var" {
		t.Errorf("statement.TokenLiteral not 'var' got=%q", statement.TokenLiteral())
		return false
	}

	varStmt, ok := statement.(*ast.VarDefinitionStatement)
	
	if !ok {
		t.Errorf("statement is not *ast.VarDefinitionStatement got %d", statement)
		return false
	}

	if varStmt.Name.Value != name {
		t.Errorf("varStmt.Name.Value not '%s'. got=%s", name, varStmt.Name.Value)
		return false
	}

	if !types.IsValidType(varStmt.Type.Value) {
		t.Errorf("varStmt.Type.Value is not a valid type, got %s", varStmt.Type.Value)
		return false
	}

	return true
}
