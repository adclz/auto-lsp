use crate::server::session::{Session, WORKSPACE};
use auto_lsp_core::document_symbols_builder::DocumentSymbolsBuilder;
use lsp_types::{Location, OneOf, WorkspaceSymbol, WorkspaceSymbolParams, WorkspaceSymbolResponse};

impl Session {
    /// Request to get root symbols
    ///
    /// This function will return all symbols found in the root recursively
    pub fn get_workspace_symbols(
        &mut self,
        params: WorkspaceSymbolParams,
    ) -> anyhow::Result<Option<WorkspaceSymbolResponse>> {
        if params.query.is_empty() {
            return Ok(None);
        }

        let mut symbols = vec![];

        let lock = WORKSPACE.lock();

        lock.roots.iter().for_each(|(uri, (root, document))| {
            let ast = &root.ast;

            let mut builder = DocumentSymbolsBuilder::default();

            ast.iter()
                .for_each(|p| p.read().build_document_symbols(document, &mut builder));

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
                            uri: uri.to_owned(),
                            range: p.range,
                        }),
                        data: None,
                    })
                    .collect::<Vec<_>>(),
            );
        });

        Ok(Some(WorkspaceSymbolResponse::Nested(symbols)))
    }
}
