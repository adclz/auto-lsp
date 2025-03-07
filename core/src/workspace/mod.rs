use super::document::Document;
use super::root::Root;
use lsp_types::Url;
use std::collections::HashMap;

#[derive(Default)]
pub struct Workspace {
    pub roots: HashMap<Url, (Root, Document)>,
}

impl Workspace {
    pub fn resolve_references(&mut self) {
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            self.roots
                .par_iter_mut()
                .for_each(|(_url, (root, document))| {
                    root.resolve_references(document);
                });
        }

        #[cfg(not(feature = "rayon"))]
        self.roots.iter_mut().for_each(|(_url, (root, document))| {
            root.resolve_references(document);
        });
    }

    pub fn resolve_checks(&mut self) {
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            self.roots
                .par_iter_mut()
                .for_each(|(_url, (root, document))| {
                    root.resolve_checks(document);
                });
        }

        #[cfg(not(feature = "rayon"))]
        self.roots.iter_mut().for_each(|(_url, (root, document))| {
            root.resolve_checks(document);
        });
    }
}
