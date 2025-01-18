use auto_lsp_core::ast::VecOrSymbol;
use lsp_types::{Location, OneOf, WorkspaceSymbol, WorkspaceSymbolParams, WorkspaceSymbolResponse};

use crate::server::session::{Session, WORKSPACES};

impl Session {
    /// Request to get workspace symbols
    ///
    /// This function will return all symbols found in the workspace recursively by calling the inner [`Session::get_document_symbols`]
    /// of every documents.
    pub fn get_workspace_symbols(
        &mut self,
        params: WorkspaceSymbolParams,
    ) -> anyhow::Result<Option<WorkspaceSymbolResponse>> {
        if params.query.is_empty() {
            return Ok(None);
        }

        let mut symbols = vec![];

        let workspaces = WORKSPACES.lock();

        workspaces.iter().for_each(|(uri, v)| {
            let ast = &v.ast;

            symbols.extend(
                ast.iter()
                    .filter_map(|p| p.read().get_document_symbols(&v.document))
                    .flat_map(|p| match p {
                        VecOrSymbol::Symbol(s) => vec![WorkspaceSymbol {
                            name: s.name,
                            kind: s.kind,
                            tags: None,
                            container_name: None,
                            location: OneOf::Left(Location {
                                uri: uri.to_owned(),
                                range: s.range,
                            }),
                            data: None,
                        }],
                        VecOrSymbol::Vec(v) => v
                            .into_iter()
                            .map(|s| WorkspaceSymbol {
                                name: s.name,
                                kind: s.kind,
                                tags: None,
                                container_name: None,
                                location: OneOf::Left(Location {
                                    uri: uri.to_owned(),
                                    range: s.range,
                                }),
                                data: None,
                            })
                            .collect::<Vec<_>>(),
                    })
                    .collect::<Vec<_>>(),
            );
        });

        Ok(Some(WorkspaceSymbolResponse::Nested(symbols)))
    }
}
