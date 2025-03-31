use std::ops::Deref;
use crate::server::session::{Session};
use auto_lsp_core::document_symbols_builder::DocumentSymbolsBuilder;
use lsp_types::{Location, OneOf, WorkspaceSymbol, WorkspaceSymbolParams, WorkspaceSymbolResponse};
use auto_lsp_core::salsa::db::WorkspaceDatabase;

impl<Db: WorkspaceDatabase> Session<Db> {
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

        let db = &*self.db.lock();

        db.get_files().iter().for_each(|file| {
            let file = *file;
            let url = file.url(db.deref());
            let document = file.document(db.deref()).read();
            let ast = file.get_ast(db.deref()).clone().into_inner();

            let mut builder = DocumentSymbolsBuilder::default();

            ast.ast.iter()
                .for_each(|p| p.read().build_document_symbols(&document, &mut builder));

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
        });

        Ok(Some(WorkspaceSymbolResponse::Nested(symbols)))
    }
}
