use auto_lsp::traits::ast_item::AstItem;
use lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_document_symbols(
        &mut self,
        params: DocumentSymbolParams,
    ) -> anyhow::Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;
        let workspace = self.workspaces.get(uri).unwrap();
        let source = &workspace.document;

        let symbols = workspace
            .ast
            .iter()
            .filter_map(|p| p.get_document_symbols(source))
            .collect::<Vec<_>>();

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }
}
