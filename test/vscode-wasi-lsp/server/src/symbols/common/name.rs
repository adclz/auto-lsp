use auto_lsp_core::builders::BuilderParams;
use auto_lsp_core::pending_symbol::AstBuilder;
use auto_lsp_core::symbol::*;
use auto_lsp_core::symbol::{AstSymbol, HoverInfo};
use auto_lsp_macros::seq;
use lsp_types::Diagnostic;
use std::fmt::Debug;

use crate::symbols::pous::function::Function;
use crate::symbols::symbols::SourceFile;

#[seq(
    query_name = "name",
    kind(symbol(lsp_inlay_hints(code_gen(query = true)), lsp_hover_info(user)))
)]
pub struct Name {}

impl HoverInfo for Name {
    fn get_hover(&self, doc: &lsp_textdocument::FullTextDocument) -> Option<lsp_types::Hover> {
        let comment = if let Some(parent) = self.get_parent() {
            if let Some(parent) = parent.to_dyn() {
                if let Some(comment) = parent.read().get_comment(doc.get_content(None).as_bytes()) {
                    Some(comment)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        Some(lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: format!(
                    r#"
```typescript
{}
var {}
```"#,
                    comment.unwrap_or(""),
                    self.get_text(doc.get_content(None).as_bytes())?
                ),
            }),
            range: Some(self.get_lsp_range(doc)),
        })
    }
}
