use crate::ast::ReferrersTrait;
use crate::ast::{DynSymbol, WeakSymbol};
use crate::document::Document;

use super::Workspace;

impl Workspace {
    pub(crate) fn add_unsolved_check(&mut self, symbol: &DynSymbol) -> &mut Self {
        self.unsolved_checks.push(symbol.to_weak());
        self
    }

    pub fn get_unsolved_checks(&self) -> &Vec<WeakSymbol> {
        &self.unsolved_checks
    }

    pub(crate) fn add_unsolved_reference(&mut self, symbol: &DynSymbol) -> &mut Self {
        self.unsolved_references.push(symbol.to_weak());
        self
    }

    pub fn get_unsolved_references(&self) -> &Vec<WeakSymbol> {
        &self.unsolved_references
    }

    #[cfg(not(feature = "rayon"))]
    pub(crate) fn resolve_references(&mut self, document: &Document) -> &mut Self {
        self.unsolved_references.retain(|item| {
            let item = match item.to_dyn() {
                Some(read) => read,
                None => return false,
            };
            let read = item.read();
            match read.find(&document) {
                Ok(Some(target)) => {
                    target.write().add_referrer(item.to_weak());
                    drop(read);
                    item.write().set_target_reference(target.to_weak());
                    false
                }
                Ok(None) => true,
                Err(err) => {
                    self.diagnostics.push(err);
                    true
                }
            }
        });
        self
    }

    #[cfg(feature = "rayon")]
    pub(crate) fn resolve_references(&mut self, document: &Document) -> &mut Self {
        use parking_lot::RwLock;
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let diagnostics = RwLock::new(vec![]);
        self.unsolved_references = self
            .unsolved_references
            .par_iter()
            .cloned()
            .filter(|item| {
                let item = match item.to_dyn() {
                    Some(read) => read,
                    None => return false,
                };
                let read = item.read();
                match read.find(&document) {
                    Ok(Some(target)) => {
                        target.write().add_referrer(item.to_weak());
                        drop(read);
                        item.write().set_target_reference(target.to_weak());
                        false
                    }
                    Ok(None) => true,
                    Err(err) => {
                        diagnostics.write().push(err);
                        true
                    }
                }
            })
            .collect::<Vec<WeakSymbol>>();
        self.diagnostics.extend(diagnostics.into_inner());
        self
    }

    #[cfg(not(feature = "rayon"))]
    pub(crate) fn resolve_checks(&mut self, document: &Document) -> &mut Self {
        self.unsolved_checks.retain(|item| {
            let item = match item.to_dyn() {
                Some(read) => read,
                None => return false,
            };
            let read = item.read();
            match read.check(&document, &mut self.diagnostics) {
                Ok(()) => false,
                Err(()) => true,
            }
        });
        self
    }

    #[cfg(feature = "rayon")]
    pub(crate) fn resolve_checks(&mut self, document: &Document) -> &mut Self {
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
                let read = item.read();
                match read.check(&document, &mut diagnostics.write()) {
                    Ok(()) => false,
                    Err(()) => true,
                }
            })
            .collect::<Vec<WeakSymbol>>();
        self.diagnostics.extend(diagnostics.into_inner());
        self
    }
}
