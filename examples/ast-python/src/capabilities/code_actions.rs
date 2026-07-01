use crate::generated::FunctionDefinition;
use auto_lsp::core::dispatch;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::tracked::get_ast;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::lsp_types::{CodeActionOrCommand, CodeActionParams};
use auto_lsp::{anyhow, lsp_types};

pub fn code_actions(
    db: &impl BaseDatabase,
    params: CodeActionParams,
) -> anyhow::Result<Option<Vec<CodeActionOrCommand>>> {
    let mut acc = vec![];

    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    get_ast(db, file).iter().try_for_each(|node| {
        dispatch!(
            node.lower(),
            [
                FunctionDefinition => build_code_actions(db, file, &mut acc)
            ]
        );
        anyhow::Ok(())
    })?;
    Ok(Some(acc))
}

impl FunctionDefinition {
    fn build_code_actions(
        &self,
        _db: &impl BaseDatabase,
        _file: File,
        acc: &mut Vec<CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        acc.push(lsp_types::CodeActionOrCommand::CodeAction(
            lsp_types::CodeAction {
                title: "A code action".to_string(),
                kind: None,
                diagnostics: None,
                edit: None,
                command: None,
                is_preferred: None,
                disabled: None,
                data: None,
            },
        ));
        Ok(())
    }
}
