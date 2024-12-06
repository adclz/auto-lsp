use lsp_textdocument::FullTextDocument;

use crate::symbol::{AstSymbol, DynSymbol, WeakSymbol};

pub trait Finder {
    fn find_in_file(&self, doc: &FullTextDocument) -> Option<WeakSymbol>;
}

impl<T: AstSymbol> Finder for T {
    fn find_in_file(&self, doc: &FullTextDocument) -> Option<WeakSymbol> {
        let source_code = doc.get_content(None).as_bytes();
        let pattern = self.get_text(doc.get_content(None).as_bytes());

        let mut curr = self.get_parent_scope();
        while let Some(scope) = curr {
            let scope = scope.read();
            let ranges = scope.get_scope_range();

            for range in ranges {
                let area = doc
                    .get_content(None)
                    .get(range[0] as usize..range[1] as usize)
                    .unwrap();

                for (index, _) in area.match_indices(pattern) {
                    if let Some(elem) = scope.find_at_offset(range[0] + index) {
                        if elem.read().get_range() != self.get_range() {
                            if elem.read().get_text(source_code) == pattern {
                                return Some(elem.to_weak());
                            }
                        }
                    }
                }
            }
            curr = scope.get_parent_scope();
        }
        None
    }
}
