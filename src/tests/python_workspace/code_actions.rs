use super::ast::{Function, Module};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::BuildCodeActions;
use auto_lsp_core::document::Document;
use lsp_types::CodeAction;

impl BuildCodeActions for Module {
    fn build_code_actions(
        &self,
        doc: &Document,
        acc: &mut Vec<lsp_types::CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        for statement in &self.statements {
            statement.read().build_code_actions(doc, acc)?;
        }
        Ok(())
    }
}

impl BuildCodeActions for Function {
    fn build_code_actions(
        &self,
        _doc: &Document,
        acc: &mut Vec<lsp_types::CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        acc.push(lsp_types::CodeActionOrCommand::CodeAction(CodeAction {
            title: "A code action".to_string(),
            kind: None,
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: None,
            disabled: None,
            data: None,
        }));
        Ok(())
    }
}
