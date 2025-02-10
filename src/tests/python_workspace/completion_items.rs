use std::sync::LazyLock;

use super::ast::{Block, Function, Module};
use crate::{self as auto_lsp};
use auto_lsp::core::ast::BuildCompletionItems;
use auto_lsp_core::document::Document;

/// Globally available completion items
static GLOBAL_COMPLETION_ITEMS: LazyLock<Vec<lsp_types::CompletionItem>> = LazyLock::new(|| {
    vec![lsp_types::CompletionItem {
        label: "def ...".to_string(),
        kind: Some(lsp_types::CompletionItemKind::SNIPPET),
        insert_text_format: Some(lsp_types::InsertTextFormat::SNIPPET),
        insert_text: Some("def ${1:func_name}(${2:arg1}):$0".to_string()),
        ..Default::default()
    }]
});

impl BuildCompletionItems for Module {
    fn build_completion_items(&self, _doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
    }
}

impl BuildCompletionItems for Function {
    fn build_completion_items(&self, _doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
    }
}

impl BuildCompletionItems for Block {
    fn build_completion_items(&self, _doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
    }
}
