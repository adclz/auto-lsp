use crate::globals::{Session, Workspace};
use auto_lsp::traits::ast_item::AstItem;
use lsp_server::{RequestId, Response};
use lsp_types::DocumentSymbolResponse;

pub fn get_document_symbols(id: RequestId, workspace: &Workspace) -> Response {
    let source = &workspace.document;

    let symbols = workspace
        .ast
        .iter()
        .filter_map(|p| p.get_document_symbols(source))
        .collect::<Vec<_>>();

    let result = Some(DocumentSymbolResponse::Nested(symbols));
    let result = serde_json::to_value(&result).unwrap();
    Response {
        id,
        result: Some(result),
        error: None,
    }
}
