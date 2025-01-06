use std::sync::Arc;

use auto_lsp_core::{builders::BuilderParams, symbol::SymbolData};
use lsp_types::{DidChangeTextDocumentParams, TextDocumentContentChangeEvent};

use super::tree_sitter_extend::{
    tree_sitter_edit::edit_tree, tree_sitter_lexer::get_tree_sitter_errors,
};
use crate::{Session, CST_PARSERS};

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

        let edits: Vec<(&TextDocumentContentChangeEvent, bool)> = params
            .content_changes
            .iter()
            .map(|edit| {
                (
                    edit,
                    match edit.text.trim().is_empty() {
                        true => workspace.document.get_content(edit.range).trim().is_empty(),
                        false => false,
                    },
                )
            })
            .collect();

        workspace
            .document
            .update(&params.content_changes[..], params.text_document.version);

        workspace.errors.clear();

        let cst;
        let mut errors = vec![];

        let new_tree = edit_tree(workspace, &params)?;

        cst = new_tree;
        errors.extend(get_tree_sitter_errors(
            &cst.root_node(),
            workspace.document.get_content(None).as_bytes(),
        ));

        let mut unsolved_checks = vec![];
        unsolved_checks.extend(workspace.unsolved_checks.clone());
        let mut unsolved_references = vec![];
        unsolved_references.extend(workspace.unsolved_references.clone());

        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        if let Some(ref mut ast) = workspace.ast {
            if params.content_changes.iter().any(|edit| {
                let edit_range = edit.range.unwrap();

                let range_offset = workspace.document.offset_at(edit_range.start) as usize;
                let start_byte = range_offset;
                let old_end_byte = range_offset + edit.range_length.unwrap() as usize;
                let new_end_byte = range_offset + edit.text.len();

                let range = ast.read().get_range();

                eprintln!("range.start {} range.end {}", range.start, range.end);
                eprintln!("start_byte {} new_end_byte {}", start_byte, new_end_byte);
                if start_byte <= range.start && (new_end_byte - old_end_byte) >= range.end {
                    eprintln!("ROOT NODE GOT DELETED");
                    true
                } else {
                    false
                }
            }) {
                workspace.ast = None;
                workspace.unsolved_checks.clear();
                workspace.unsolved_references.clear();
                workspace.errors.clear();
            }
        }

        let workspace = self
            .workspaces
            .get(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        let mut builder_params = BuilderParams {
            ctx: self,
            query: &cst_parser.queries.outline,
            root_node: cst.root_node(),
            doc: &workspace.document,
            url: arc_uri.clone(),
            diagnostics: &mut errors,
            unsolved_checks: &mut unsolved_checks,
            unsolved_references: &mut unsolved_references,
        };
        if let Some(ast) = &workspace.ast {
            builder_params
                .swap_ast(&ast, &edits)
                .resolve_references()
                .resolve_checks();

            eprintln!("checks remaining {:?}", unsolved_checks.len());
            eprintln!("references remaining {:?}", unsolved_references.len());
        } else {
            let b = workspace.ast_builder;
            let ast_build = b(&mut builder_params, None);

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

        let workspace = self
            .workspaces
            .get_mut(&uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        workspace.cst = cst;
        workspace.unsolved_checks = unsolved_checks;
        workspace.unsolved_references = unsolved_references;
        workspace.errors.extend(errors);

        self.add_comments(uri)?;

        Ok(())
    }
}
