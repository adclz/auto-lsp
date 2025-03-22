use std::{hash::Hash, sync::Arc};

use dashmap::{DashMap, Entry};
use lsp_types::Url;
use parking_lot::Mutex;
use salsa::{Database, Storage};
use ustr::UstrMap;

use crate::{ast::WeakSymbol, document::Document, root::Root};

#[salsa::input]
pub struct File {
    #[return_ref]
    data: (Root, Document),
}

#[salsa::db]
#[derive(Clone, Default)]
struct Workspace {
    storage: Storage<Self>,
    logs: Arc<Mutex<Vec<String>>>,
    pub files: DashMap<Url, File>,
    pub symbol_table: UstrMap<WeakSymbol>,
}

impl Workspace {
    pub fn take_logs(&self) -> Vec<String> {
        let mut logs = self.logs.lock();
        std::mem::take(&mut *logs)
    }
}

#[salsa::db]
pub trait WorkspaceDatabase: Database {
    fn input(&self, file: Url, document: Document, root: Root) -> File;
}

#[salsa::db]
impl salsa::Database for Workspace {
    fn salsa_event(&self, _event: &dyn Fn() -> salsa::Event) {
        #[cfg(feature = "log")]
        {
            let event = _event();
            if let salsa::EventKind::WillExecute { .. } = event.kind {
                self.logs.lock().push(format!("{:?}", event));
            }
        }
    }
}

#[salsa::db]
impl WorkspaceDatabase for Workspace {
    fn input(&self, url: Url, document: Document, root: Root) -> File {
        match self.files.entry(url.clone()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(File::new(self, (root, document))),
        }
    }
}
