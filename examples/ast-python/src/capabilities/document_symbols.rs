#![allow(deprecated)]
use crate::generated::{
    CompoundStatement, CompoundStatement_SimpleStatement, FunctionDefinition, Module,
};
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch;
use auto_lsp::core::document::Document;
use auto_lsp::core::document_symbols_builder::DocumentSymbolsBuilder;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::default::db::tracked::{ParsedAst, get_ast};
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

    let ast = get_ast(db, file);
    if let Some(node) = ast.get_root() {
        dispatch!(node.lower(),
            [
                Module => build_document_symbols(&doc, ast, &mut builder)
            ]
        );
    }
    Ok(Some(DocumentSymbolResponse::Nested(builder.finalize())))
}

impl Module {
    pub(crate) fn build_document_symbols(
        &self,
        doc: &Document,
        ast: &ParsedAst,
        builder: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        self.children
            .iter()
            .try_for_each(|f| f.cast(ast).build_document_symbols(doc, ast, builder))
    }
}

impl CompoundStatement_SimpleStatement {
    fn build_document_symbols(
        &self,
        doc: &Document,
        ast: &ParsedAst,
        acc: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        match self {
            CompoundStatement_SimpleStatement::CompoundStatement(
                CompoundStatement::FunctionDefinition(f),
            ) => f.build_document_symbols(doc, ast, acc),
            _ => Ok(()),
        }
    }
}

impl FunctionDefinition {
    fn build_document_symbols(
        &self,
        doc: &Document,
        ast: &ParsedAst,
        builder: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        let mut nested_builder = DocumentSymbolsBuilder::default();

        self.body.cast(ast).children.iter().try_for_each(|f| {
            f.cast(ast)
                .build_document_symbols(doc, ast, &mut nested_builder)
        })?;

        builder.push_symbol(lsp_types::DocumentSymbol {
            name: self.name.cast(ast).get_text(doc.as_bytes())?.to_string(),
            kind: lsp_types::SymbolKind::FUNCTION,
            range: self.name.cast(ast).get_lsp_range(doc)?,
            selection_range: self.name.cast(ast).get_lsp_range(doc)?,
            tags: None,
            detail: None,
            deprecated: None,
            children: Some(nested_builder.finalize()),
        });
        Ok(())
    }
}
