#![allow(deprecated)]
use super::ast::{Function, Module};
use auto_lsp_core::ast::{AstSymbol, BuildDocumentSymbols};
use auto_lsp_core::document::Document;
use auto_lsp_core::document_symbols_builder::DocumentSymbolsBuilder;

impl BuildDocumentSymbols for Module {
    fn build_document_symbols(
        &self,
        doc: &Document,
        builder: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        self.statements.build_document_symbols(doc, builder)
    }
}

impl BuildDocumentSymbols for Function {
    fn build_document_symbols(
        &self,
        doc: &Document,
        builder: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        let mut nested_builder = DocumentSymbolsBuilder::default();

        self.body.build_document_symbols(doc, &mut nested_builder)?;

        builder.push_symbol(lsp_types::DocumentSymbol {
            name: self
                .name
                .read()
                .get_text(doc.texter.text.as_bytes())?
                .to_string(),
            kind: lsp_types::SymbolKind::FUNCTION,
            range: self.name.read().get_lsp_range(doc).unwrap(),
            selection_range: self.name.read().get_lsp_range(doc).unwrap(),
            tags: None,
            detail: None,
            deprecated: None,
            children: Some(nested_builder.finalize()),
        });
        Ok(())
    }
}
