use auto_lsp::builders::ast_item::builder;
use lsp_textdocument::FullTextDocument;
use lsp_types::Url;

use super::{tree_sitter_extend::tree_sitter_lexer::get_tree_sitter_errors, Workspace};
use crate::{session::Session, AST_BUILDERS, CST_PARSERS};

impl<'a> Session<'a> {
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

        let cst_parser = CST_PARSERS
            .get(extension)
            .ok_or(anyhow::format_err!("No parser available for {}", extension))?;

        let ast_builder = AST_BUILDERS.get(extension).ok_or(anyhow::format_err!(
            "No AST builder available for {}",
            extension
        ))?;

        let cst;
        let ast;
        let mut errors = vec![];

        let source_code = source_code.as_bytes();

        cst = cst_parser.try_parse(&source_code, None).unwrap().clone();
        errors.extend(get_tree_sitter_errors(&cst.root_node(), source_code));

        ast = builder(
            &cst_parser.queries.outline,
            ast_builder,
            cst.root_node(),
            source_code,
        )
        .into_iter()
        .filter_map(|f| match f {
            Ok(ast) => Some(ast),
            Err(e) => {
                errors.push(e);
                None
            }
        })
        .collect();

        self.workspaces.insert(
            uri.to_owned(),
            Workspace {
                ast_builder,
                cst_parser,
                document,
                errors,
                cst,
                ast,
            },
        );
        Ok(())
    }
}
