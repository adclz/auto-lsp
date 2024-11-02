use lsp_types::{CodeLens, CodeLensParams};

use crate::session::Session;

impl<'a> Session<'a> {
    pub fn get_code_lens(
        &mut self,
        params: CodeLensParams,
    ) -> anyhow::Result<Option<Vec<CodeLens>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.ast.iter().for_each(|ast| {
            ast.read().unwrap().build_code_lens(&mut results);
        });

        Ok(Some(results))
    }
}
