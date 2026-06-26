use crate::generated::FunctionDefinition;
use auto_lsp::core::ast::AstNode;
use auto_lsp::core::dispatch;
use auto_lsp::core::semantic_tokens_builder::SemanticTokensBuilder;
use auto_lsp::default::db::BaseDatabase;
use auto_lsp::default::db::file::File;
use auto_lsp::default::db::tracked::{ParsedAst, get_ast};
use auto_lsp::lsp_types::{SemanticTokensParams, SemanticTokensRangeParams, SemanticTokensResult};
use auto_lsp::{anyhow, define_semantic_token_modifiers, define_semantic_token_types};

define_semantic_token_types![
    standard {
        FUNCTION,
    }

    custom {}
];

define_semantic_token_modifiers![
    standard {
        DECLARATION,
    }

    custom {}
];

pub fn semantic_tokens_full(
    db: &impl BaseDatabase,
    params: SemanticTokensParams,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut builder = SemanticTokensBuilder::new("".into());

    let ast = get_ast(db, file);
    ast.iter().try_for_each(|node| {
        dispatch!(
            node.lower(),
            [
                FunctionDefinition => build_semantic_tokens(db, file, ast, &mut builder)
            ]
        );
        anyhow::Ok(())
    })?;
    Ok(Some(SemanticTokensResult::Tokens(builder.build())))
}

pub fn semantic_tokens_range(
    db: &impl BaseDatabase,
    params: SemanticTokensRangeParams,
) -> anyhow::Result<Option<SemanticTokensResult>> {
    let uri = params.text_document.uri;

    let file = db
        .get_file(&uri)
        .ok_or_else(|| anyhow::format_err!("File not found in workspace"))?;

    let mut builder = SemanticTokensBuilder::new("".into());

    let ast = get_ast(db, file);
    for node in ast.iter() {
        let range = node.get_lsp_range(file.document(db))?;
        if range.end <= params.range.start {
            continue;
        }
        if range.start >= params.range.end {
            break;
        }
        dispatch!(node.lower(),
            [
                FunctionDefinition => build_semantic_tokens(db, file, ast, &mut builder)
            ]
        );
    }

    Ok(Some(SemanticTokensResult::Tokens(builder.build())))
}

impl FunctionDefinition {
    fn build_semantic_tokens(
        &self,
        db: &impl BaseDatabase,
        file: File,
        ast: &ParsedAst,
        builder: &mut SemanticTokensBuilder,
    ) -> anyhow::Result<()> {
        builder.push(
            self.name.cast(ast).get_lsp_range(file.document(db))?,
            SUPPORTED_TYPES.iter().position(|x| *x == FUNCTION).unwrap() as u32,
            SUPPORTED_MODIFIERS
                .iter()
                .position(|x| *x == DECLARATION)
                .unwrap() as u32,
        );
        Ok(())
    }
}
