use crate::core::ast::BuildDocumentSymbols;
use crate::server::session::Session;
use auto_lsp_core::document_symbols_builder::DocumentSymbolsBuilder;
use auto_lsp_core::salsa::db::WorkspaceDatabase;
use lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};
use std::ops::Deref;

impl<Db: WorkspaceDatabase> Session<Db> {
    /// Request to get document symbols for a file
    ///
    /// This function will recursively traverse the ast and return all symbols found.
    pub fn get_document_symbols(
        &mut self,
        params: DocumentSymbolParams,
    ) -> anyhow::Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        let file = self
            .db
            .get_file(&uri)
            .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

        let document = file.document(&self.db).read();
        let root = file.get_ast(&self.db).clone().into_inner();

        let mut builder = DocumentSymbolsBuilder::default();

        root.ast
            .iter()
            .for_each(|p| p.build_document_symbols(&document, &mut builder));

        Ok(Some(DocumentSymbolResponse::Nested(builder.finalize())))
    }
}
