use super::ast::{Function, Module};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::BuildCodeLenses;
use auto_lsp_core::ast::AstSymbol;
use auto_lsp_core::document::Document;

impl BuildCodeLenses for Module {
    fn build_code_lens(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) {
        for statement in &self.statements {
            statement.read().build_code_lens(doc, acc);
        }
    }
}

impl BuildCodeLenses for Function {
    fn build_code_lens(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) {
        let read = self.name.read();
        acc.push(lsp_types::CodeLens {
            range: read.get_lsp_range(doc),
            command: None,
            data: None,
        })
    }
}
