use crate::ast::{DynSymbol, WeakSymbol};
use crate::core_ast::data::ReferrersTrait;
use crate::{document::Document, root::Root, workspace::Workspace};

impl Workspace {
    pub fn resolve_references(&mut self) {
        #[cfg(not(feature = "rayon"))]
        self.roots.iter_mut().for_each(|(_url, (root, document))| {
            root.resolve_references(document);
        });

        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            self.roots
                .par_iter_mut()
                .for_each(|(_url, (root, document))| {
                    root.resolve_references(document);
                });
        }
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

    pub fn resolve_references(&mut self, document: &Document) {
        #[cfg(not(feature = "rayon"))]
        {
            self.unsolved_references.retain(|item| {
                let item = match item.to_dyn() {
                    Some(read) => read,
                    None => return false,
                };
                let read = item.read();
                match read.find(document) {
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
        }

        #[cfg(feature = "rayon")]
        {
            use crate::ast::WeakSymbol;
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
                    match read.find(document) {
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
        }
    }
}
