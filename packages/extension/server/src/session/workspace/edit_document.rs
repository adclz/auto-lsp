use std::sync::Arc;

use lsp_types::DidChangeTextDocumentParams;

use super::tree_sitter_extend::{
    tree_sitter_edit::edit_tree, tree_sitter_lexer::get_tree_sitter_errors,
};
use crate::{session::Session, AST_BUILDERS, CST_PARSERS};

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

        let cst_parser = CST_PARSERS
            .get(extension)
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        let ast_builder = AST_BUILDERS.get(extension).ok_or(anyhow::format_err!(
            "No AST builder available for {}",
            extension
        ))?;

        workspace
            .document
            .update(&params.content_changes[..], params.text_document.version);

        let cst;
        let ast;
        let mut errors = vec![];

        cst = edit_tree(workspace, &params)?;

        let workspace = self
            .workspaces
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let source_code = workspace.document.get_content(None).as_bytes();

        errors.extend(get_tree_sitter_errors(&cst.root_node(), source_code));

        let arc_uri = Arc::new(uri.clone());

        let ast_build = ast_builder(
            &cst_parser.queries.outline,
            cst.root_node(),
            source_code,
            arc_uri,
        );

        ast = match ast_build.item {
            Ok(item) => Some(item),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        errors.extend(ast_build.errors);

        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.cst = cst;
        workspace.ast = ast;
        workspace.errors = errors;

        Ok(())
    }
}
