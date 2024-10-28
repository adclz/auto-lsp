use crate::{
    globals::{Session, Workspace},
    symbols::symbols::Symbol,
};
use lsp_server::{RequestId, Response, ResponseError};
use lsp_types::{
    request::FoldingRangeRequest, FoldingRange, FoldingRangeKind, Hover, HoverParams, MarkupContent,
};

use auto_lsp::traits::ast_item::AstItem;

pub fn get_hover_info(id: RequestId, params: &HoverParams, workspace: &Workspace) -> Response {
    let position = params.text_document_position_params.position;
    let doc = &workspace.document;

    let offset = doc.offset_at(position) as usize;
    let item = workspace
        .ast
        .iter()
        .find_map(|symbol| symbol.find_at_offset(&offset));

    match item {
        Some(item) => {
            let hover = item.read().unwrap().get_hover(doc);
            let result = serde_json::to_value(&hover).unwrap();
            Response {
                id,
                result: Some(result),
                error: None,
            }
        }
        None => Response {
            id,
            result: Some(serde_json::to_value::<Option<Hover>>(None).unwrap()),
            error: None,
        },
    }
}
