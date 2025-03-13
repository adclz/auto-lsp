use lsp_types::Diagnostic;

use crate::ast::{DynSymbol, WeakSymbol};
use crate::core_ast::data::ReferrersTrait;
use crate::{document::Document, root::Root, workspace::Workspace};

impl Workspace {
    pub fn resolve_references(&mut self) {
        #[cfg(not(feature = "rayon"))]
        let mut result: Vec<_> = self
            .roots
            .iter()
            .map(|(_url, (root, document))| root.collect_references(document, self))
            .collect();

        #[cfg(feature = "rayon")]
        let mut result: Vec<_> = {
            use rayon::prelude::*;

            self.roots
                .par_iter()
                .map(|(_url, (root, document))| root.collect_references(document, self))
                .collect()
        };

        self.roots.iter_mut().for_each(|(_url, (root, _document))| {
            let result = result.pop().unwrap();
            root.unsolved_references = result.0;
            root.diagnostics = result.1;
        });
    }
}

impl Root {
    pub(crate) fn add_unsolved_reference(&mut self, symbol: &DynSymbol) -> &mut Self {
        self.unsolved_references.push(symbol.to_weak());
        self
    }

    pub fn get_unsolved_references(&self) -> &Vec<WeakSymbol> {
        &self.unsolved_references
    }

    pub fn collect_references(
        &self,
        document: &Document,
        workspace: &Workspace,
    ) -> (Vec<WeakSymbol>, Vec<Diagnostic>) {
        #[cfg(not(feature = "rayon"))]
        {
            let mut diagnostics = vec![];

            (
                self.unsolved_references
                    .iter()
                    .filter_map(|item| {
                        Self::validate_reference(&item, &mut diagnostics, document, workspace)
                    })
                    .collect(),
                diagnostics,
            )
        }

        #[cfg(feature = "rayon")]
        {
            use parking_lot::RwLock;
            use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

            let diagnostics = RwLock::new(vec![]);

            (
                self.unsolved_references
                    .par_iter()
                    .filter_map(|item| {
                        Self::validate_reference(
                            &item,
                            &mut diagnostics.write(),
                            document,
                            workspace,
                        )
                    })
                    .collect(),
                diagnostics.into_inner(),
            )
        }
    }

    fn validate_reference(
        item: &WeakSymbol,
        diagnostics: &mut Vec<Diagnostic>,
        document: &Document,
        workspace: &Workspace,
    ) -> Option<WeakSymbol> {
        let dyn_item = match item.to_dyn() {
            Some(read) => read,
            None => return None,
        };
        let read = dyn_item.read();
        match read.find_reference(document, workspace, diagnostics) {
            Some(target) => {
                target.write().add_referrer(dyn_item.to_weak());
                drop(read);
                dyn_item.write().set_target_reference(target.to_weak());
                None
            }
            None => Some(item.clone()),
        }
    }
}
