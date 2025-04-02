use crate::core::ast::BuildDocumentSymbols;
use auto_lsp_core::salsa::db::BaseDatabase;
use auto_lsp_core::{document_symbols_builder::DocumentSymbolsBuilder, salsa::tracked::get_ast};
use lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};

/// Request to get document symbols for a file
///
/// This function will recursively traverse the ast and return all symbols found.
pub fn get_document_symbols<Db: BaseDatabase>(
    db: &Db,
    params: DocumentSymbolParams,
) -> anyhow::Result<Option<DocumentSymbolResponse>> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let document = file.document(db).read();
    let root = get_ast(db, file).to_symbol();

    let mut builder = DocumentSymbolsBuilder::default();

    if let Some(p) = root { p.build_document_symbols(&document, &mut builder) }

    Ok(Some(DocumentSymbolResponse::Nested(builder.finalize())))
}
