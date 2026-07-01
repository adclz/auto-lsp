use crate::generated::{Identifier, PassStatement};
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch_once;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::lsp_types::{Hover, HoverParams};
use auto_lsp::{anyhow, lsp_types};

pub fn hover(db: &impl BaseDatabase, params: HoverParams) -> anyhow::Result<Option<Hover>> {
    let uri = &params.text_document_position_params.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let position = params.text_document_position_params.position;

    if let Some(node) = get_ast(db, file).descendant_for_position(file.document(db), &position) {
        dispatch_once!(node.lower(), [
            PassStatement => get_hover(db, file),
            Identifier => get_hover(db, file)
        ]);
    }
    Ok(None)
}

impl PassStatement {
    fn get_hover(
        &self,
        _db: &impl BaseDatabase,
        _file: File,
    ) -> anyhow::Result<Option<lsp_types::Hover>> {
        Ok(Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: r#"This is a pass statement

[See python doc](https://docs.python.org/3/reference/simple_stmts.html#the-pass-statement)"#
                    .into(),
            }),
            range: None,
        }))
    }
}

impl Identifier {
    fn get_hover(
        &self,
        db: &impl BaseDatabase,
        file: File,
    ) -> anyhow::Result<Option<lsp_types::Hover>> {
        let doc = file.document(db);
        Ok(Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!("hover {}", self.get_text(doc.as_bytes())?),
            }),
            range: None,
        }))
    }
}
