use super::ast::{Identifier, PassStatement};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::{AstSymbol, GetHover, GetSymbolData};
use auto_lsp_core::document::Document;

impl GetHover for PassStatement {
    fn get_hover(&self, _doc: &Document) -> Option<lsp_types::Hover> {
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: r#"This is a pass statement

[See python doc](https://docs.python.org/3/reference/simple_stmts.html#the-pass-statement)"#
                    .into(),
            }),
            range: None,
        })
    }
}

impl GetHover for Identifier {
    fn get_hover(&self, doc: &Document) -> Option<lsp_types::Hover> {
        let parent = self.get_parent().unwrap().to_dyn().unwrap();
        let comment = parent.read().get_comment(doc.texter.text.as_bytes());
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::PlainText,
                value: format!(
                    "{}hover {}",
                    if let Some(comment) = comment {
                        format!("{}\n", comment)
                    } else {
                        "".to_string()
                    },
                    self.get_text(doc.texter.text.as_bytes()).unwrap()
                ),
            }),
            range: None,
        })
    }
}
