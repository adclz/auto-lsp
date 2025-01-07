from unittest import TestCase

import tree_sitter, tree_sitter_iec61131_3_2


class TestLanguage(TestCase):
    def test_can_load_grammar(self):
        try:
            tree_sitter.Language(tree_sitter_iec61131_3_2.language())
        except Exception:
            self.fail("Error loading Iec6113132 grammar")
