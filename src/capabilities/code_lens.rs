use lsp_types::{CodeLens, CodeLensParams};

use crate::session::{Session, WORKSPACES};

impl Session {
    pub fn get_code_lens(
        &mut self,
        params: CodeLensParams,
    ) -> anyhow::Result<Option<Vec<CodeLens>>> {
        let mut results = vec![];

        let uri = params.text_document.uri;
        let workspace = WORKSPACES.lock();

        let workspace = workspace
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.ast.iter().for_each(|ast| {
            ast.read().build_code_lens(&mut results);
        });

        Ok(Some(results))
    }
}
