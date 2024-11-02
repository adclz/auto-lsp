use lsp_types::{InlayHint, InlayHintParams};

use auto_lsp::traits::ast_item::AstItem;

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_inlay_hint(
        &mut self,
        params: InlayHintParams,
    ) -> anyhow::Result<Option<Vec<InlayHint>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.ast.iter().for_each(|ast| {
            ast.read().unwrap().build_inlay_hint(&mut results);
        });

        Ok(Some(results))
    }
}
