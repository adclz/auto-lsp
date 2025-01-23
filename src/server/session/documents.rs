use std::sync::Arc;

use auto_lsp_core::document::Document;
use auto_lsp_core::workspace::Workspace;
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
        let ast_parser = &parsers.ast_parser;
        let source_code = source_code.as_bytes();
        let mut errors = vec![];

        let cst = tree_sitter
            .parser
            .write()
            .parse(&source_code, None)
            .unwrap();

        get_tree_sitter_errors(&cst.root_node(), source_code, &mut errors);

        let document = Document {
            texter: text,
            tree: cst,
        };

        let mut workspace = Workspace {
            url: Arc::new(uri.clone()),
            parsers,
            diagnostics: errors,
            unsolved_checks: vec![],
            unsolved_references: vec![],
            ast: None,
        };

        workspace.ast = match ast_parser(&mut workspace, &document, None) {
            Ok(item) => {
                workspace.resolve_references(&document);
                workspace.resolve_checks(&document);
                Some(item)
            }
            Err(e) => {
                workspace.diagnostics.push(e);
                None
            }
        };

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

        workspace.set_comments(&document)?;

        WORKSPACES
            .lock()
            .insert(uri.to_owned(), (workspace, document));

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
        let (workspace, document) = workspaces
            .get_mut(uri)
            .ok_or(anyhow::anyhow!("Workspace not found"))?;

        // Update document and tree sitter
        let mut new_tree = WrapTree::from(&mut document.tree);
        for ch in params.content_changes {
            document
                .texter
                .update(WrapChange::from(&ch).change, &mut new_tree)?;
        }
        let edits = new_tree.get_edits();

        document.tree = workspace
            .parsers
            .tree_sitter
            .parser
            .write()
            .parse(document.texter.text.as_bytes(), Some(&document.tree))
            .ok_or(anyhow::format_err!(
                "Tree sitter failed to edit tree of document {}",
                uri
            ))?;

        // Clear diagnostics
        workspace.diagnostics.clear();

        // Get new diagnostics from tree sitter
        get_tree_sitter_errors(
            &document.tree.root_node(),
            document.texter.text.as_bytes(),
            &mut workspace.diagnostics,
        );

        // Update AST
        workspace
            .swap_ast(&edits, &document)
            .resolve_references(&document)
            .resolve_checks(&document);

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

        workspace.set_comments(&document)?;

        Ok(())
    }
}
