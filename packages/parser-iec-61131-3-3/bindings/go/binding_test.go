package tree_sitter_IEC61131_rev_3_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-IEC61131_rev_3"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_IEC61131_rev_3.Language())
	if language == nil {
		t.Errorf("Error loading Iec61131Rev3 grammar")
	}
}
