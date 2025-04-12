/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

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
