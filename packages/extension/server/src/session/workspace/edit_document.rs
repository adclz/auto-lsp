use std::{ops::Range, sync::Arc};

use auto_lsp::builders::get_ast_edit;
use lsp_types::DidChangeTextDocumentParams;
use rayon::vec;

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

        let arc_uri = Arc::new(uri.clone());

        let cst_parser = CST_PARSERS
            .get(extension)
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        let ast_builder = AST_BUILDERS.get(extension).ok_or(anyhow::format_err!(
            "No AST builder available for {}",
            extension
        ))?;

        let workspace = self
            .workspaces
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let nodes_to_edit = get_ast_edit(
            workspace.ast.as_ref(),
            &workspace.document,
            &params.content_changes,
        );

        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

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

        match nodes_to_edit.ast {
            Some(nodes) => {
                eprintln!("Editing nodes");
                nodes.iter().for_each(|node| {
                    let arc_uri = arc_uri.clone();
                    let mut as_mut = node.node.borrow_mut();
                    as_mut.swap(
                        self,
                        &cst_parser.queries.outline,
                        cst.root_node(),
                        Some(Range {
                            start: node.start_byte,
                            end: node.end_byte,
                        }),
                        &workspace.document,
                        arc_uri,
                        &mut errors,
                    );
                });
            }
            None => eprintln!("No nodes to edit"),
        }

        // todo: remove when finished
        let ast_build = ast_builder(
            self,
            &cst_parser.queries.outline,
            cst.root_node(),
            None,
            &workspace.document,
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
        //workspace.ast = ast;
        workspace.errors.extend(errors);

        Ok(())
    }
}
