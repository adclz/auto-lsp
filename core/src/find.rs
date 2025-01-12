use crate::{
    symbol::{AstSymbol, DynSymbol},
    workspace::Document,
};

pub trait Finder {
    fn find_in_file(&self, doc: &Document) -> Option<DynSymbol>;
}

impl<T: AstSymbol> Finder for T {
    fn find_in_file(&self, doc: &Document) -> Option<DynSymbol> {
        let source_code = &doc.document.text;
        let pattern = match self.get_text(source_code.as_bytes()) {
            Some(a) => a,
            None => return None,
        };

        let mut curr = self.get_parent_scope();
        while let Some(scope) = curr {
            let scope = scope.read();
            let ranges = scope.get_scope_range();

            for range in ranges {
                let area = match source_code.as_str().get(range[0]..range[1]) {
                    Some(a) => a,
                    None => {
                        log::warn!("Invalid document range: {:?}", range);
                        continue;
                    }
                };

                for (index, _) in area.match_indices(pattern) {
                    if let Some(elem) = scope.find_at_offset(range[0] + index) {
                        if elem.read().get_range() != self.get_range() {
                            match elem.read().get_text(source_code.as_bytes()) {
                                Some(a) => {
                                    if a == pattern {
                                        return Some(elem.clone());
                                    }
                                }
                                None => {}
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
