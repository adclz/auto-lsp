#![allow(deprecated)]
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
use crate::generated::{
    CompoundStatement, CompoundStatement_SimpleStatement, FunctionDefinition, Module,
};
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch;
use auto_lsp::core::document::Document;
use auto_lsp::core::document_symbols_builder::DocumentSymbolsBuilder;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::default::db::{BaseDatabase, File};
use auto_lsp::lsp_types::{DocumentSymbolParams, DocumentSymbolResponse};
use auto_lsp::{anyhow, lsp_types};

pub fn document_symbols(
    db: &impl BaseDatabase,
    params: DocumentSymbolParams,
) -> anyhow::Result<Option<DocumentSymbolResponse>> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let doc = file.document(db);
    let mut builder = DocumentSymbolsBuilder::default();

    if let Some(node) = get_ast(db, file).get_root() {
        dispatch!(node.lower(),
            [
                Module => build_document_symbols(&doc, &mut builder)
            ]
        );
    }
    Ok(Some(DocumentSymbolResponse::Nested(builder.finalize())))
}

impl Module {
    pub(crate) fn build_document_symbols(
        &self,
        doc: &Document,
        builder: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        self.children
            .iter()
            .try_for_each(|f| f.build_document_symbols(doc, builder))
    }
}

impl CompoundStatement_SimpleStatement {
    fn build_document_symbols(
        &self,
        doc: &Document,
        acc: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        match self {
            CompoundStatement_SimpleStatement::CompoundStatement(
                CompoundStatement::FunctionDefinition(f),
            ) => f.build_document_symbols(doc, acc),
            _ => Ok(()),
        }
    }
}

impl FunctionDefinition {
    fn build_document_symbols(
        &self,
        doc: &Document,
        builder: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        let mut nested_builder = DocumentSymbolsBuilder::default();

        self.body
            .children
            .iter()
            .try_for_each(|f| f.build_document_symbols(doc, &mut nested_builder))?;

        builder.push_symbol(lsp_types::DocumentSymbol {
            name: self.name.get_text(doc.texter.text.as_bytes())?.to_string(),
            kind: lsp_types::SymbolKind::FUNCTION,
            range: self.name.get_lsp_range(),
            selection_range: self.name.get_lsp_range(),
            tags: None,
            detail: None,
            deprecated: None,
            children: Some(nested_builder.finalize()),
        });
        Ok(())
    }
}
