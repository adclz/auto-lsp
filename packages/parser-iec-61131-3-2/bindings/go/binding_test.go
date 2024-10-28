package tree_sitter_iec61131_3_2_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_iec61131_3_2 "github.com/tree-sitter/tree-sitter-IEC61131_3_2/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_iec61131_3_2.Language())
	if language == nil {
		t.Errorf("Error loading Iec6113132 grammar")
	}
}
