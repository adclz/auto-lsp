use auto_lsp_core::salsa::db::BaseDatabase;
use auto_lsp_core::{document_symbols_builder::DocumentSymbolsBuilder, salsa::tracked::get_ast};
use lsp_types::{Location, OneOf, WorkspaceSymbol, WorkspaceSymbolParams, WorkspaceSymbolResponse};

/// Request to get root symbols
///
/// This function will return all symbols found in the root recursively
pub fn get_workspace_symbols<Db: BaseDatabase>(
    db: &Db,
    params: WorkspaceSymbolParams,
) -> anyhow::Result<Option<WorkspaceSymbolResponse>> {
    if params.query.is_empty() {
        return Ok(None);
    }

    let mut symbols = vec![];

    db.get_files().iter().try_for_each(|file| {
        let file = *file;
        let url = file.url(db);
        let document = file.document(db).read();
        let ast = get_ast(db, file).to_symbol();

        let mut builder = DocumentSymbolsBuilder::default();

        if let Some(root) = ast {
            root.read()
                .build_document_symbols(&document, &mut builder)?;
        }

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
