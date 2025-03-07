use crate::core::ast::BuildDocumentSymbols;
use auto_lsp_core::document_symbols_builder::DocumentSymbolsBuilder;
use lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};

use crate::server::session::{Session, WORKSPACE};

impl Session {
    /// Request to get document symbols for a file
    ///
    /// This function will recursively traverse the ast and return all symbols found.
    pub fn get_document_symbols(
        &mut self,
        params: DocumentSymbolParams,
    ) -> anyhow::Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;

        let workspace = WORKSPACE.lock();

        let (root, document) = workspace
            .roots
            .get(uri)
            .ok_or(anyhow::anyhow!("Root not found"))?;

        let mut builder = DocumentSymbolsBuilder::default();

        root.ast
            .iter()
            .for_each(|p| p.build_document_symbols(document, &mut builder));

        Ok(Some(DocumentSymbolResponse::Nested(builder.finalize())))
    }
}
