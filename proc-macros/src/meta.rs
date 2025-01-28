#![allow(unused)]
use syn::{Ident, Type};

use crate::features::*;
use darling::{ast, util, FromDeriveInput, FromField, FromMeta};

/// Struct input when `seq` macro is used
#[derive(Debug, FromDeriveInput)]
pub struct StructInput {
    pub data: ast::Data<util::Ignored, StructHelpers>,
}

/// Mandatory `query_name` field and `kind` field (reference or symbol)
#[derive(Debug, FromMeta)]
pub struct UserFeatures {
    pub query_name: String,
    pub kind: AstStructKind,
}

#[derive(FromField, Debug)]
#[darling(attributes(ast))]
pub struct StructHelpers {
    pub ident: Option<Ident>,
    pub ty: Type,
}

/// Reference or Symbol kind
#[derive(Debug, FromMeta)]
pub enum AstStructKind {
    Reference(ReferenceFeatures),
    Symbol(SymbolFeatures),
}

/// Reference or Symbol features
#[derive(Debug, FromMeta)]
pub enum Feature<T>
where
    T: Sized + FromMeta,
{
    User,
    CodeGen(T),
}

#[derive(Debug, FromMeta)]
pub struct SymbolFeatures {
    pub scope: Option<Feature<ScopeFeature>>,
    pub check: Option<Feature<CheckFeature>>,
    pub comment: Option<Feature<CommentFeature>>,
    // LSP
    pub lsp_document_symbols: Option<Feature<DocumentSymbolFeature>>,
    pub lsp_hover_info: Option<Feature<HoverFeature>>,
    pub lsp_semantic_tokens: Option<Feature<SemanticTokenFeature>>,
    pub lsp_inlay_hints: Option<Feature<InlayHintFeature>>,
    pub lsp_code_lens: Option<Feature<CodeLensFeature>>,
    pub lsp_completion_items: Option<Feature<CompletionItemFeature>>,
    pub lsp_invoked_completion_items: Option<Feature<InvokedCompletionItemFeature>>,
    pub lsp_go_to_definition: Option<Feature<GotoDefinitionFeature>>,
    pub lsp_go_to_declaration: Option<Feature<GoToDeclarationFeature>>,
}

#[derive(Debug, FromMeta)]
pub enum ReferenceFeature {
    User,
    Reference,
    Disable,
}

#[derive(Debug, FromMeta)]
pub struct ReferenceFeatures {
    pub check: Option<ReferenceFeature>,
    pub comment: Option<ReferenceFeature>,
    pub lsp_document_symbols: Option<ReferenceFeature>,
    pub lsp_hover_info: Option<ReferenceFeature>,
    pub lsp_semantic_tokens: Option<ReferenceFeature>,
    pub lsp_inlay_hints: Option<ReferenceFeature>,
    pub lsp_code_lens: Option<ReferenceFeature>,
    pub lsp_completion_items: Option<ReferenceFeature>,
    pub lsp_invoked_completion_items: Option<ReferenceFeature>,
    pub lsp_go_to_definition: Option<ReferenceFeature>,
    pub lsp_go_to_declaration: Option<ReferenceFeature>,
}

pub enum ReferenceOrSymbolFeatures<'a> {
    Reference(&'a ReferenceFeatures),
    Symbol(&'a SymbolFeatures),
}
