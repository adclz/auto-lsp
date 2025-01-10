use std::sync::Arc;

use auto_lsp_core::builders::BuilderParams;
use lsp_textdocument::FullTextDocument;
use lsp_types::Url;

use super::{tree_sitter_extend::tree_sitter_lexer::get_tree_sitter_errors, Workspace};
use crate::session::Session;

impl Session {
    /// Add a new document to session workspaces
    pub fn add_document(
        &mut self,
        uri: &Url,
        language_id: &str,
        source_code: &str,
    ) -> anyhow::Result<()> {
        let document = FullTextDocument::new(language_id.to_owned(), 0, source_code.to_string());
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

        let cst_parser = &parsers.cst_parser;
        let ast_parser = parsers.ast_parser;

        let cst;
        let ast;
        let mut errors = vec![];

        let source_code = source_code.as_bytes();

        cst = cst_parser.try_parse(&source_code, None).unwrap();
        errors.extend(get_tree_sitter_errors(&cst.root_node(), source_code));

        let arc_uri = Arc::new(uri.clone());

        let mut unsolved_checks = vec![];
        let mut unsolved_references = vec![];

        let params = &mut BuilderParams {
            doc: &document,
            diagnostics: &mut errors,
            ctx: self,
            query: &cst_parser.queries.outline,
            root_node: cst.root_node(),
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

        self.workspaces.insert(
            uri.to_owned(),
            Workspace {
                parsers,
                document,
                errors,
                unsolved_checks,
                unsolved_references,
                cst,
                ast,
            },
        );

        self.add_comments(uri)?;

        Ok(())
    }
}
