use std::sync::Arc;

use auto_lsp::{
    builders::{swap_ast, BuilderParams},
    symbol::SymbolData,
};
use lsp_types::DidChangeTextDocumentParams;

use super::tree_sitter_extend::{
    tree_sitter_edit::edit_tree, tree_sitter_lexer::get_tree_sitter_errors,
};
use crate::{session::Session, CST_PARSERS};

impl Session {
    pub fn edit_document(&mut self, params: DidChangeTextDocumentParams) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;
        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let language_id = workspace.document.language_id();
        let extension = match self.extensions.get(language_id) {
            Some(extension) => extension,
            None => {
                return Err(anyhow::format_err!(
                    "Extension {} is not registered",
                    language_id
                ))
            }
        };

        let arc_uri = Arc::new(uri.clone());

        let cst_parser = CST_PARSERS
            .get(extension)
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        workspace
            .document
            .update(&params.content_changes[..], params.text_document.version);

        let cst;
        let mut errors = vec![];

        cst = edit_tree(workspace, &params)?;
        errors.extend(get_tree_sitter_errors(
            &cst.root_node(),
            workspace.document.get_content(None).as_bytes(),
        ));

        let workspace = self
            .workspaces
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        swap_ast(
            workspace.ast.as_ref(),
            &params.content_changes,
            &mut BuilderParams {
                ctx: self,
                query: &cst_parser.queries.outline,
                root_node: cst.root_node(),
                doc: &workspace.document,
                url: arc_uri.clone(),
                diagnostics: &mut errors,
                checks: &mut vec![],
            },
        );

        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.cst = cst;
        workspace.errors.extend(errors);

        self.add_comments(uri)?;

        Ok(())
    }
}
