use crate::core::ast::BuildInlayHints;
use lsp_types::{InlayHint, InlayHintParams};

use crate::server::session::{Session, WORKSPACES};

impl Session {
    /// Get inlay hints for a document.
    pub fn get_inlay_hints(
        &mut self,
        params: InlayHintParams,
    ) -> anyhow::Result<Option<Vec<InlayHint>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let (workspace, document) = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.ast.iter().for_each(|ast| {
            ast.build_inlay_hints(document, &mut results);
        });

        Ok(Some(results))
    }
}
