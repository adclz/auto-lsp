use super::ast::{Function, Module};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::BuildCodeActions;
use auto_lsp_core::document::Document;

impl BuildCodeActions for Module {
    fn build_code_actions(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeAction>) {
        for statement in &self.statements {
            statement.read().build_code_actions(doc, acc);
        }
    }
}

impl BuildCodeActions for Function {
    fn build_code_actions(&self, _doc: &Document, acc: &mut Vec<lsp_types::CodeAction>) {
        acc.push(lsp_types::CodeAction {
            title: "A code action".to_string(),
            kind: None,
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: None,
            disabled: None,
            data: None,
        })
    }
}
