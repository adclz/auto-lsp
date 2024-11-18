use auto_lsp::traits::ast_item::DocumentSymbols;
use lsp_types::{Location, OneOf, WorkspaceSymbol, WorkspaceSymbolParams, WorkspaceSymbolResponse};

use crate::session::Session;

impl Session {
    pub fn get_workspace_symbols(
        &mut self,
        params: WorkspaceSymbolParams,
    ) -> anyhow::Result<Option<WorkspaceSymbolResponse>> {
        if params.query.is_empty() {
            return Ok(None);
        }

        let mut symbols = vec![];

        self.workspaces.iter().for_each(|(uri, v)| {
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
