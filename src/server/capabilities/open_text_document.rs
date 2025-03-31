use auto_lsp_core::root::Root;
use lsp_types::DidOpenTextDocumentParams;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use crate::server::session::{Session};

impl<Db: WorkspaceDatabase> Session<Db> {
    pub fn open_text_document(&mut self, params: DidOpenTextDocumentParams) -> anyhow::Result<()> {
        let url = &params.text_document.uri;
        let mut db = &mut *self.db.lock();

        if db.get_file(url).is_some() {
            // The file is already in db
            // We can ignore this change
            return Ok(());
        };

        let extension = params.text_document.language_id;

        let extension = match self.extensions.get(&extension) {
            Some(extension) => extension,
            None => {
                return Err(anyhow::format_err!(
                    "Extension {} is not registered",
                    extension
                ))
            }
        };

        let text = (self.text_fn)(params.text_document.text.clone());

        let parsers = self
            .init_options
            .parsers
            .get(extension.as_str())
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        db.add_file_from_texter(parsers, url,text)
    }
}