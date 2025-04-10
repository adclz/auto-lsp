use super::ast::{Function, Module};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::{AstSymbol, BuildInlayHints, GetSymbolData};
use auto_lsp_core::document::Document;

impl BuildInlayHints for Module {
    fn build_inlay_hints(
        &self,
        doc: &Document,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        self.statements.build_inlay_hints(doc, acc)
    }
}

impl BuildInlayHints for Function {
    fn build_inlay_hints(
        &self,
        doc: &Document,
        acc: &mut Vec<auto_lsp::lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        let range = self.get_range();
        let read = self.name.read();
        let name = format!(
            "[{} {}] - {}",
            range.start,
            range.end,
            self.name.read().get_text(doc.texter.text.as_bytes())?
        );
        acc.push(auto_lsp::lsp_types::InlayHint {
            kind: Some(auto_lsp::lsp_types::InlayHintKind::TYPE),
            label: auto_lsp::lsp_types::InlayHintLabel::String(name),
            position: read.get_start_position(doc).unwrap(),
            tooltip: None,
            text_edits: None,
            padding_left: None,
            padding_right: None,
            data: None,
        });
        Ok(())
    }
}
