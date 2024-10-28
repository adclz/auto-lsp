use std::str::FromStr;

use crate::globals::{Session, Workspace};
use auto_lsp::traits::ast_item::AstItem;
use lsp_server::{RequestId, Response};
use lsp_types::{
    Location, OneOf, Uri, WorkspaceLocation, WorkspaceSymbol, WorkspaceSymbolParams,
    WorkspaceSymbolResponse,
};

pub fn get_workspace_symbols(
    id: RequestId,
    params: &WorkspaceSymbolParams,
    session: &Session,
) -> Response {
    let query = &params.query;
    if params.query.is_empty() {
        return Response {
            id,
            result: Some(serde_json::to_value::<Option<WorkspaceSymbolResponse>>(None).unwrap()),
            error: None,
        };
    }

    let mut symbols = vec![];

    session.workspaces.iter().for_each(|(uri, v)| {
        let ast = &v.ast;

        symbols.extend(
            ast.iter()
                .filter_map(|p| p.get_document_symbols(&v.document))
                .map(|p| WorkspaceSymbol {
                    name: p.name,
                    kind: p.kind,
                    tags: None,
                    container_name: None,
                    location: OneOf::Left(Location {
                        uri: Uri::from_str(&uri).unwrap(),
                        range: p.range,
                    }),
                    data: None,
                })
                .collect::<Vec<_>>(),
        );
    });

    let result = Some(WorkspaceSymbolResponse::Nested(symbols));
    let result = serde_json::to_value(&result).unwrap();
    Response {
        id,
        result: Some(result),
        error: None,
    }
}
