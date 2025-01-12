use std::sync::Arc;

use auto_lsp_core::builders::BuilderParams;
use lsp_types::DidChangeTextDocumentParams;

use crate::{
    session::lexer::get_tree_sitter_errors,
    texter_impl::{change::NewChange, updateable::NewTree},
    Session,
};

impl Session {
    pub fn edit_document(&mut self, params: DidChangeTextDocumentParams) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;
        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let extension = uri.to_file_path().unwrap();
        let language_id = extension.extension().unwrap().to_str().unwrap();

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

        let parsers = self
            .init_options
            .parsers
            .get(extension.as_str())
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        let cst_parser = &parsers.cst_parser;

        let mut new_tree = NewTree::from(&mut workspace.document.cst);
        for ch in params.content_changes {
            workspace
                .document
                .document
                .update(NewChange::from(&ch).change, &mut new_tree)?;
        }
        let edits = new_tree.get_edits();

        workspace.errors.clear();

        let cst;
        let mut errors = vec![];

        let new_tree = workspace
            .parsers
            .cst_parser
            .parser
            .write()
            .map_err(|_| {
                anyhow::format_err!(
                    "Parser lock is poisoned while editing cst of document {}",
                    uri
                )
            })?
            .parse(
                workspace.document.document.text.as_bytes(),
                Some(&workspace.document.cst),
            )
            .ok_or(anyhow::format_err!(
                "Tree sitter failed to edit cst of document {}",
                uri
            ))?;

        cst = new_tree;
        errors.extend(get_tree_sitter_errors(
            &cst.root_node(),
            workspace.document.document.text.as_bytes(),
        ));

        let mut unsolved_checks = vec![];
        unsolved_checks.extend(workspace.unsolved_checks.clone());
        let mut unsolved_references = vec![];
        unsolved_references.extend(workspace.unsolved_references.clone());

        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let mut builder_params = BuilderParams {
            document: &workspace.document,
            query: &cst_parser.queries.outline,
            url: arc_uri.clone(),
            diagnostics: &mut errors,
            unsolved_checks: &mut unsolved_checks,
            unsolved_references: &mut unsolved_references,
        };
        if let Some(ast) = &mut workspace.ast {
            builder_params
                .swap_ast(ast, &edits, &parsers.ast_parser)
                .resolve_references()
                .resolve_checks();
        } else {
            let ast_parser = &workspace.parsers.ast_parser;
            let ast_build = ast_parser(&mut builder_params, None);

            let workspace = self
                .workspaces
                .get_mut(&uri)
                .ok_or(anyhow::anyhow!("Workspace not found"))?;

            workspace.ast = match ast_build {
                Ok(item) => Some(item),
                Err(e) => {
                    errors.push(e);
                    None
                }
            };
        }

        if !unsolved_checks.is_empty() {
            log::info!("");
            log::warn!("Unsolved checks: {:?}", unsolved_checks.len());
        }

        if !unsolved_references.is_empty() {
            log::info!("");
            log::warn!("Unsolved references: {:?}", unsolved_references.len());
        }

        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.unsolved_checks = unsolved_checks;
        workspace.unsolved_references = unsolved_references;
        workspace.errors.extend(errors);

        self.add_comments(uri)?;

        Ok(())
    }
}
