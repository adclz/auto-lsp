use std::sync::Arc;

use auto_lsp_core::{
    build::MainBuilder,
    workspace::{Document, Workspace},
};
use lsp_types::{DidChangeTextDocumentParams, Url};

use crate::server::session::{lexer::get_tree_sitter_errors, Session};
use crate::server::texter_impl::change::WrapChange;
use crate::server::texter_impl::updateable::WrapTree;

use super::WORKSPACES;

impl Session {
    /// Add a new document to workspaces
    ///
    /// This will first try to find the correct parser for the language id,
    /// then parse the source code with the tree sitter parser,
    /// and finally build the AST with the core [`tree_sitter::Query`] and root symbol.
    pub(crate) fn add_document(
        &mut self,
        uri: &Url,
        language_id: &str,
        source_code: &str,
    ) -> anyhow::Result<()> {
        let text = (self.text_fn)(source_code.to_string());
        let extension = match self.extensions.get(language_id) {
            Some(extension) => extension,
            None => {
                return Err(anyhow::format_err!(
                    "Extension {} is not registered",
                    language_id
                ))
            }
        };

        let parsers = self
            .init_options
            .parsers
            .get(extension.as_str())
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        let tree_sitter = &parsers.tree_sitter;
        let ast_parser = parsers.ast_parser;

        let cst;
        let ast;
        let mut errors = vec![];

        let source_code = source_code.as_bytes();

        cst = tree_sitter
            .parser
            .write()
            .parse(&source_code, None)
            .unwrap();

        get_tree_sitter_errors(&cst.root_node(), source_code, &mut errors);

        let document = Document {
            document: text,
            cst,
        };

        let arc_uri = Arc::new(uri.clone());

        let mut unsolved_checks = vec![];
        let mut unsolved_references = vec![];

        let params = &mut MainBuilder {
            document: &document,
            diagnostics: &mut errors,
            query: &tree_sitter.queries.core,
            url: arc_uri.clone(),
            unsolved_checks: &mut unsolved_checks,
            unsolved_references: &mut unsolved_references,
        };
        let ast_build = ast_parser(params, None);

        ast = match ast_build {
            Ok(item) => {
                params.resolve_references();
                params.resolve_checks();
                Some(item)
            }
            Err(e) => {
                errors.push(e);
                None
            }
        };

        if !unsolved_checks.is_empty() {
            log::info!("");
            log::warn!("Unsolved checks: {:?}", unsolved_checks.len());
        }

        if !unsolved_references.is_empty() {
            log::info!("");
            log::warn!("Unsolved references: {:?}", unsolved_references.len());
        }

        let mut workspaces = WORKSPACES.lock();
        let workspace = Workspace {
            parsers,
            document,
            errors,
            unsolved_checks,
            unsolved_references,
            ast,
        };

        Self::add_comments(&workspace)?;

        workspaces.insert(uri.to_owned(), workspace);

        Ok(())
    }

    /// Edit a document in workspaces
    ///
    /// Edits are incremental, meaning that the entire document is not re-parsed.
    /// Instead, the changes are applied to the existing CST (using [`tree-sitter`] and [`texter`]).
    ///
    /// The AST is not updated if the node is either:
    ///  - an extra (comment)
    ///  - an errored node
    ///  - a whitespace
    pub(crate) fn edit_document(
        &mut self,
        params: DidChangeTextDocumentParams,
    ) -> anyhow::Result<()> {
        let uri = &params.text_document.uri;

        let mut workspaces = WORKSPACES.lock();

        let workspace = workspaces
            .get_mut(uri)
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

        let tree_sitter = &parsers.tree_sitter;

        let mut new_tree = WrapTree::from(&mut workspace.document.cst);
        for ch in params.content_changes {
            workspace
                .document
                .document
                .update(WrapChange::from(&ch).change, &mut new_tree)?;
        }
        let edits = new_tree.get_edits();

        let new_tree = workspace
            .parsers
            .tree_sitter
            .parser
            .write()
            .parse(
                workspace.document.document.text.as_bytes(),
                Some(&workspace.document.cst),
            )
            .ok_or(anyhow::format_err!(
                "Tree sitter failed to edit cst of document {}",
                uri
            ))?;

        workspace.document.cst = new_tree;

        workspace.errors.clear();
        get_tree_sitter_errors(
            &workspace.document.cst.root_node(),
            workspace.document.document.text.as_bytes(),
            &mut workspace.errors,
        );

        let mut builder_params = MainBuilder {
            document: &workspace.document,
            query: &tree_sitter.queries.core,
            url: arc_uri.clone(),
            diagnostics: &mut workspace.errors,
            unsolved_checks: &mut workspace.unsolved_checks,
            unsolved_references: &mut workspace.unsolved_references,
        };
        if let Some(ast) = &mut workspace.ast {
            builder_params
                .swap_ast(ast, &edits, &parsers.ast_parser)
                .resolve_references()
                .resolve_checks();
        } else {
            let ast_parser = &workspace.parsers.ast_parser;
            let ast_build = ast_parser(&mut builder_params, None);

            workspace.ast = match ast_build {
                Ok(item) => Some(item),
                Err(e) => {
                    workspace.errors.push(e);
                    None
                }
            };
        }

        if !workspace.unsolved_checks.is_empty() {
            log::info!("");
            log::warn!("Unsolved checks: {:?}", workspace.unsolved_checks.len());
        }

        if !workspace.unsolved_references.is_empty() {
            log::info!("");
            log::warn!(
                "Unsolved references: {:?}",
                workspace.unsolved_references.len()
            );
        }

        Self::add_comments(&workspace)?;

        Ok(())
    }
}
