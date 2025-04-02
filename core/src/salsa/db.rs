use crate::document::Document;
use crate::parsers::Parsers;
use dashmap::{DashMap, Entry};
use lsp_types::Url;
use parking_lot::RwLock;
use salsa::Setter;
use salsa::{Database, Storage};
use std::{hash::Hash, sync::Arc};
use texter::core::text::Text;

#[salsa::input]
pub struct File {
    #[id]
    pub url: Url,
    pub parsers: &'static Parsers,
    #[return_ref]
    pub document: Arc<RwLock<Document>>,
}

#[salsa::db]
#[derive(Default, Clone)]
pub struct BaseDb {
    storage: Storage<Self>,
    #[cfg(feature = "log")]
    logs: Arc<parking_lot::Mutex<Vec<String>>>,
    pub(crate) files: DashMap<Url, File>,
}

#[salsa::db]
pub trait BaseDatabase: Database {
    fn add_file_from_texter(
        &mut self,
        parsers: &'static Parsers,
        url: &Url,
        text: Text,
    ) -> anyhow::Result<()>;
    fn update(
        &mut self,
        url: &Url,
        edits: &Vec<lsp_types::TextDocumentContentChangeEvent>,
    ) -> anyhow::Result<()>;

    fn remove_file(&mut self, url: &Url) -> anyhow::Result<()>;
    fn get_file(&self, file: &Url) -> Option<File>;
    fn get_files(&self) -> &DashMap<Url, File>;
    #[cfg(feature = "log")]
    fn take_logs(&self) -> Vec<String>;
}

#[salsa::db]
impl salsa::Database for BaseDb {
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
impl BaseDatabase for BaseDb {
    fn add_file_from_texter(
        &mut self,
        parsers: &'static Parsers,
        url: &Url,
        texter: Text,
    ) -> anyhow::Result<()> {
        let tree = parsers
            .tree_sitter
            .parser
            .write()
            .parse(texter.text.as_bytes(), None)
            .ok_or_else(|| anyhow::format_err!("Tree-sitter failed to parse source code"))?;

        // Initialize the document with the source code and syntax tree.
        let document = Document { texter, tree };

        let file = File::new(self, url.clone(), parsers, Arc::new(RwLock::new(document)));
        match self.files.entry(url.clone()) {
            Entry::Occupied(_) => Err(anyhow::anyhow!("File {:?} not found", url)),
            Entry::Vacant(entry) => {
                entry.insert(file);
                Ok(())
            }
        }
    }

    fn update(
        &mut self,
        url: &Url,
        changes: &Vec<lsp_types::TextDocumentContentChangeEvent>,
    ) -> anyhow::Result<()> {
        let file = *self
            .files
            .get_mut(url)
            .ok_or(anyhow::anyhow!("File {:?} not found", url))?;

        let data_lock = file.document(self);
        let ptr = data_lock.clone();

        let mut doc = data_lock.write();

        // Apply updates
        doc.update(&mut file.parsers(self).tree_sitter.parser.write(), changes)?;

        // Update Salsa data
        drop(doc);
        file.set_document(self).to(ptr);
        Ok(())
    }

    fn remove_file(&mut self, url: &Url) -> anyhow::Result<()> {
        match self.files.remove(url) {
            None => Err(anyhow::format_err!("File {:?} not found", url)),
            Some(_) => Ok(()),
        }
    }

    fn get_file(&self, url: &Url) -> Option<File> {
        self.files.get(url).map(|file| *file)
    }

    fn get_files(&self) -> &DashMap<Url, File> {
        &self.files
    }
    #[cfg(feature = "log")]
    fn take_logs(&self) -> Vec<String> {
        std::mem::take(&mut self.logs.lock())
    }
}
