#![allow(deprecated)]
use crate::generated::Module;
use auto_lsp::anyhow;
use auto_lsp::core::dispatch;
use auto_lsp::core::document_symbols_builder::DocumentSymbolsBuilder;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::{
    Location, OneOf, WorkspaceSymbol, WorkspaceSymbolParams, WorkspaceSymbolResponse,
};

pub fn workspace_symbols(
    db: &impl BaseDatabase,
    params: WorkspaceSymbolParams,
) -> anyhow::Result<Option<WorkspaceSymbolResponse>> {
    if params.query.is_empty() {
        return Ok(None);
    }

    let mut symbols = vec![];

    db.get_files().iter().try_for_each(|file| {
        let file = *file;
        let url = file.url(db);
        let doc = file.document(db);

        let mut builder = DocumentSymbolsBuilder::default();

        let ast = get_ast(db, file);
        if let Some(node) = ast.get_root() {
            dispatch!(node.lower(),
                [
                    Module => build_document_symbols(&doc, ast, &mut builder)
                ]
            );
        };

        symbols.extend(
            builder
                .finalize()
                .into_iter()
                .map(|p| WorkspaceSymbol {
                    name: p.name,
                    kind: p.kind,
                    tags: None,
                    container_name: None,
                    location: OneOf::Left(Location {
                        uri: url.to_owned(),
                        range: p.range,
                    }),
                    data: None,
                })
                .collect::<Vec<_>>(),
        );
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(Some(WorkspaceSymbolResponse::Nested(symbols)))
}
