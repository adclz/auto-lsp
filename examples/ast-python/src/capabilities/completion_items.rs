use crate::generated::{FunctionDefinition, Identifier, Module};
use auto_lsp::core::dispatch;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::{
    CompletionContext, CompletionItem, CompletionParams, CompletionResponse,
};
use auto_lsp::{anyhow, lsp_types};
use std::sync::LazyLock;

pub fn completion_items(
    db: &impl BaseDatabase,
    params: CompletionParams,
) -> anyhow::Result<Option<CompletionResponse>> {
    let uri = &params.text_document_position.text_document.uri;

    let file = db
        .get_file(uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut acc = vec![];

    if let Some(node) = get_ast(db, file).get_root() {
        dispatch!(node.lower(), [
                Module => build_completion_items(db, file, &params.context, &mut acc),
                FunctionDefinition => build_completion_items(db, file, &params.context, &mut acc),
                Identifier => build_completion_items(db, file, &params.context, &mut acc)
        ]);
    }
    Ok(Some(acc.into()))
}

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

impl Module {
    fn build_completion_items(
        &self,
        _db: &impl BaseDatabase,
        _file: File,
        _params: &Option<CompletionContext>,
        acc: &mut Vec<CompletionItem>,
    ) -> auto_lsp::anyhow::Result<()> {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
        Ok(())
    }
}

impl FunctionDefinition {
    fn build_completion_items(
        &self,
        _db: &impl BaseDatabase,
        _file: File,
        _params: &Option<CompletionContext>,
        acc: &mut Vec<CompletionItem>,
    ) -> auto_lsp::anyhow::Result<()> {
        acc.extend(GLOBAL_COMPLETION_ITEMS.iter().cloned());
        Ok(())
    }
}

impl Identifier {
    fn build_completion_items(
        &self,
        _db: &impl BaseDatabase,
        _file: File,
        params: &Option<CompletionContext>,
        acc: &mut Vec<CompletionItem>,
    ) -> auto_lsp::anyhow::Result<()> {
        if params
            .as_ref()
            .unwrap()
            .trigger_character
            .as_deref()
            .unwrap()
            == "."
        {
            acc.push(CompletionItem {
                label: "triggered! ...".to_string(),
                kind: Some(lsp_types::CompletionItemKind::SNIPPET),
                insert_text_format: Some(lsp_types::InsertTextFormat::SNIPPET),
                insert_text: Some("def ${1:func_name}(${2:arg1}):$0".to_string()),
                ..Default::default()
            });
        };
        Ok(())
    }
}
