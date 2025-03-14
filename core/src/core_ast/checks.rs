use crate::ast::{CheckStatus, DynSymbol, WeakSymbol};
use crate::document::Document;
use crate::root::Root;
use crate::workspace::Workspace;

impl Workspace {
    pub fn resolve_checks(&mut self) {
        #[cfg(not(feature = "rayon"))]
        self.roots.iter_mut().for_each(|(_url, (root, document))| {
            root.resolve_checks(document);
        });

        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            self.roots
                .par_iter_mut()
                .for_each(|(_url, (root, document))| {
                    root.resolve_checks(document);
                });
        }
    }
}

impl Root {
    pub(crate) fn add_unsolved_check(&mut self, symbol: &DynSymbol) -> &mut Self {
        self.unsolved_checks.push(symbol.to_weak());
        self
    }

    pub fn get_unsolved_checks(&self) -> &Vec<WeakSymbol> {
        &self.unsolved_checks
    }

    pub fn resolve_checks(&mut self, document: &Document) {
        #[cfg(not(feature = "rayon"))]
        {
            self.unsolved_checks.retain(|item| {
                let item = match item.to_dyn() {
                    Some(read) => read,
                    None => return false,
                };
                let check_result = item.read().check(document, &mut self.ast_diagnostics);
                match check_result {
                    CheckStatus::Ok => {
                        item.write().update_check_pending(false);
                        false
                    }
                    CheckStatus::Fail => {
                        item.write().update_check_pending(true);
                        true
                    }
                }
            });
        }
        #[cfg(feature = "rayon")]
        {
            use crate::ast::WeakSymbol;
            use parking_lot::RwLock;
            use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

            let diagnostics = RwLock::new(vec![]);
            self.unsolved_checks = self
                .unsolved_checks
                .par_iter()
                .cloned()
                .filter(|item| {
                    let item = match item.to_dyn() {
                        Some(read) => read,
                        None => return false,
                    };
                    let check_result = item.read().check(document, &mut diagnostics.write());
                    match check_result {
                        CheckStatus::Ok => {
                            item.write().update_check_pending(false);
                            false
                        }
                        CheckStatus::Fail => {
                            item.write().update_check_pending(true);
                            true
                        }
                    }
                })
                .collect::<Vec<WeakSymbol>>();
            self.ast_diagnostics.extend(diagnostics.into_inner());
        }
    }
}
