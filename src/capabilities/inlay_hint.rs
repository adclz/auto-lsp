use lsp_types::{InlayHint, InlayHintParams};

use crate::session::{Session, WORKSPACES};

impl Session {
    pub fn get_inlay_hint(
        &mut self,
        params: InlayHintParams,
    ) -> anyhow::Result<Option<Vec<InlayHint>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let workspace = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.ast.iter().for_each(|ast| {
            ast.read()
                .build_inlay_hint(&workspace.document, &mut results);
        });

        Ok(Some(results))
    }
}
