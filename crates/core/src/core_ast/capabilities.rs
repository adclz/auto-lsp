#![allow(unused)]

use cfg_if::cfg_if;
use lsp_types::{
    request::GotoDeclarationResponse, CompletionItem, Diagnostic, DocumentSymbol,
    GotoDefinitionResponse,
};
use std::ops::Deref;

use super::core::AstSymbol;
use super::data::*;
use super::symbol::*;
use crate::document_symbols_builder::DocumentSymbolsBuilder;
use crate::{document::Document, semantic_tokens_builder::SemanticTokensBuilder};
use aho_corasick::AhoCorasick;

/// [LSP DocumentSymbol specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_documentSymbol)
pub trait BuildDocumentSymbols {
    /// ```rust
    /// # struct MySymbol{}
    ///  use auto_lsp_core::document::Document;
    ///  use auto_lsp_core::ast::BuildDocumentSymbols;
    ///  use auto_lsp_core::document_symbols_builder::DocumentSymbolsBuilder;
    ///
    /// impl BuildDocumentSymbols for MySymbol {
    ///    fn build_document_symbols(&self, doc: &Document, acc: &mut DocumentSymbolsBuilder) {
    ///       acc.push_symbol(lsp_types::DocumentSymbol {
    ///         name: "Function Name".to_string(),
    ///         kind: lsp_types::SymbolKind::FUNCTION,
    ///         tags: None,
    ///         detail: None,
    ///         deprecated: None,
    ///         range: lsp_types::Range::default(),
    ///         selection_range: lsp_types::Range::default(),
    ///         children: None,
    ///       });
    ///    }
    /// }
    fn build_document_symbols(&self, doc: &Document, acc: &mut DocumentSymbolsBuilder) {}
}

/// [LSP Hover specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#hover)
pub trait GetHover {
    /// Returns a hover information or `None`
    ///
    /// ```rust
    /// # struct MySymbol {}
    /// use auto_lsp_core::document::Document;
    /// use auto_lsp_core::ast::GetHover;
    ///
    /// impl GetHover for MySymbol {
    ///     fn get_hover(&self, doc: &Document) -> Option<lsp_types::Hover> {
    ///        Some(lsp_types::Hover {
    ///             contents: lsp_types::HoverContents::Markup(
    ///                 lsp_types::MarkupContent {
    ///                     kind: lsp_types::MarkupKind::Markdown,
    ///                     value: "Hello, World!".to_string(),
    ///                 }
    ///             ),
    ///             range: None
    ///        })
    ///     }
    /// }
    /// ```
    fn get_hover(&self, doc: &Document) -> Option<lsp_types::Hover> {
        None
    }
}

/// [LSP GotoDefinitionResponse specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_definition)
pub trait GetGoToDefinition {
    /// Return a goto definition information or `None`
    ///
    /// ```rust
    /// # struct MySymbol{}
    /// use lsp_types::Url;
    /// use auto_lsp_core::document::Document;
    /// use auto_lsp_core::ast::GetGoToDefinition;
    ///
    /// impl GetGoToDefinition for MySymbol {
    ///    fn go_to_definition(&self, doc: &Document) -> Option<lsp_types::GotoDefinitionResponse> {
    ///       Some(lsp_types::GotoDefinitionResponse::Scalar(lsp_types::Location::new(
    ///          Url::parse("file:///path/to/file").unwrap(),
    ///         lsp_types::Range::default()
    ///      )))
    ///   }
    /// }
    fn go_to_definition(&self, doc: &Document) -> Option<GotoDefinitionResponse> {
        None
    }
}

/// [LSP GotoDeclarationResponse specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_declaration)
pub trait GetGoToDeclaration {
    /// Return a goto declaration information or `None`
    ///
    /// ```rust
    /// # struct MySymbol{}
    /// use lsp_types::Url;
    /// use auto_lsp_core::document::Document;
    /// use auto_lsp_core::ast::GetGoToDeclaration;
    ///
    /// impl GetGoToDeclaration for MySymbol {
    ///    fn go_to_declaration(&self, doc: &Document) -> Option<lsp_types::request::GotoDeclarationResponse> {
    ///       Some(lsp_types::request::GotoDeclarationResponse::Scalar(lsp_types::Location::new(
    ///          Url::parse("file:///path/to/file").unwrap(),
    ///          lsp_types::Range::default()
    ///      )))
    ///   }
    /// }
    fn go_to_declaration(&self, doc: &Document) -> Option<GotoDeclarationResponse> {
        None
    }
}

/// [LSP SemanticTokens specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_semanticTokens)
pub trait BuildSemanticTokens {
    /// Semantic tokens builder
    ///
    /// ```rust
    /// # struct MySymbol{}
    /// use lsp_types::Url;
    /// use auto_lsp_core::document::Document;
    /// use auto_lsp_core::ast::BuildSemanticTokens;
    /// use auto_lsp_core::semantic_tokens_builder::SemanticTokensBuilder;
    ///
    /// impl BuildSemanticTokens for MySymbol {
    ///   fn build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder) {
    ///      builder.push(
    ///         lsp_types::Range::default(),
    ///         10,
    ///         0
    ///     );
    ///   }
    /// }
    ///
    ///
    fn build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder) {}
}

/// [LSP InlayHints specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_inlayHints)
pub trait BuildInlayHints {
    /// Inlay hints builder
    ///
    /// ```rust
    /// # struct MySymbol{}
    /// use auto_lsp_core::document::Document;
    /// use auto_lsp_core::ast::BuildInlayHints;
    ///
    /// impl BuildInlayHints for MySymbol {
    ///   fn build_inlay_hints(&self, doc: &Document, acc: &mut Vec<lsp_types::InlayHint>) {
    ///     acc.push(lsp_types::InlayHint {
    ///         kind: Some(lsp_types::InlayHintKind::PARAMETER),
    ///         label: lsp_types::InlayHintLabel::String("Hint".to_string()),
    ///         position: lsp_types::Position::default(),
    ///         tooltip: None,
    ///         text_edits: None,
    ///         padding_left: None,
    ///         padding_right: None,
    ///         data: None,
    ///     });
    ///     }
    /// }
    /// ```
    fn build_inlay_hints(&self, doc: &Document, acc: &mut Vec<lsp_types::InlayHint>) {}
}

/// [LSP CodeLens specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeLens)
pub trait BuildCodeLenses {
    /// Code lens builder
    ///
    /// ```rust
    /// # struct MySymbol{}
    /// use auto_lsp_core::document::Document;
    /// use auto_lsp_core::ast::BuildCodeLenses;
    ///
    /// impl BuildCodeLenses for MySymbol {
    ///   fn build_code_lenses(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) {
    ///     acc.push(lsp_types::CodeLens {
    ///         range: lsp_types::Range::default(),
    ///         command: None,
    ///         data: None,
    ///     });
    ///   }
    /// }
    /// ```
    fn build_code_lenses(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) {}
}

/// [LSP CompletionItem specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionItem)
pub trait BuildCompletionItems {
    /// Completion items builder
    ///
    ///```rust
    /// # struct MySymbol{}
    /// use auto_lsp_core::document::Document;
    /// use auto_lsp_core::ast::BuildCompletionItems;
    ///
    /// impl BuildCompletionItems for MySymbol {
    ///   fn build_completion_items(&self, doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
    ///     acc.push(lsp_types::CompletionItem {
    ///         label: "Completion Item".to_string(),
    ///         kind: Some(lsp_types::CompletionItemKind::FIELD),
    ///         ..Default::default()
    ///     });
    ///   }
    ///}
    /// ```
    fn build_completion_items(&self, doc: &Document, acc: &mut Vec<CompletionItem>) {}
}

/// [LSP CompletionItem specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionItem)
pub trait BuildTriggeredCompletionItems {
    /// Completion items builder
    ///
    ///```rust
    /// # struct MySymbol{}
    /// use auto_lsp_core::document::Document;
    /// use auto_lsp_core::ast::BuildTriggeredCompletionItems;
    ///
    /// impl BuildTriggeredCompletionItems for MySymbol {
    ///   fn build_triggered_completion_items(&self, trigger: &str, doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) {
    ///     if trigger == "." {
    ///         acc.push(lsp_types::CompletionItem {
    ///             label: "Completion Item".to_string(),
    ///             kind: Some(lsp_types::CompletionItemKind::FIELD),
    ///             ..Default::default()
    ///         });
    ///      }
    ///   }
    ///}
    fn build_triggered_completion_items(
        &self,
        trigger: &str,
        doc: &Document,
        acc: &mut Vec<CompletionItem>,
    ) {
    }
}

/// [LSP CodeAction specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeAction)
pub trait BuildCodeActions {
    fn build_code_actions(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeActionOrCommand>) {}
}

// Special capabilities

/// Tree traversal
pub trait Traverse {
    /// Finds minimal symbol at the given offset
    fn descendant_at(&self, offset: usize) -> Option<DynSymbol>;

    /// Finds minimal symbol at the given offset and collects all symbols matching the condition during the search
    fn descendant_at_and_collect(
        &self,
        offset: usize,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) -> Option<DynSymbol>;

    // Traverse all symbols starting from this one and collect all symbols matching the condition
    fn traverse_and_collect(&self, collect_fn: fn(DynSymbol) -> bool, collect: &mut Vec<DynSymbol>);
}

impl Traverse for DynSymbol {
    fn descendant_at(&self, offset: usize) -> Option<DynSymbol> {
        if self.read().is_inside_offset(offset) {
            self.read()
                .descendant_at(offset)
                .or_else(|| Some(self.clone()))
        } else {
            None
        }
    }

    fn descendant_at_and_collect(
        &self,
        offset: usize,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) -> Option<DynSymbol> {
        if self.read().is_inside_offset(offset) {
            if collect_fn(self.clone()) {
                collect.push(self.clone());
            }
            self.read()
                .descendant_at_and_collect(offset, collect_fn, collect)
        } else {
            None
        }
    }

    fn traverse_and_collect(
        &self,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) {
        if collect_fn(self.clone()) {
            collect.push(self.clone());
        }
        self.read().traverse_and_collect(collect_fn, collect);
    }
}

impl<T: AstSymbol> Traverse for Symbol<T> {
    fn descendant_at(&self, offset: usize) -> Option<DynSymbol> {
        let symbol = self.read();
        match symbol.is_inside_offset(offset) {
            true => symbol.descendant_at(offset).or_else(|| Some(self.into())),
            false => None,
        }
    }

    fn descendant_at_and_collect(
        &self,
        offset: usize,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) -> Option<DynSymbol> {
        let to_dyn: DynSymbol = self.into();
        let symbol = self.read();
        match symbol.is_inside_offset(offset) {
            true => {
                if collect_fn(to_dyn.clone()) {
                    collect.push(to_dyn);
                }
                symbol
                    .descendant_at_and_collect(offset, collect_fn, collect)
                    .or_else(|| Some(self.into()))
            }
            false => None,
        }
    }

    fn traverse_and_collect(
        &self,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) {
        let symbol = self.read();
        if collect_fn(self.into()) {
            collect.push(self.into());
        }
        symbol.traverse_and_collect(collect_fn, collect);
    }
}

impl<T: AstSymbol> Traverse for Option<Symbol<T>> {
    fn descendant_at(&self, offset: usize) -> Option<DynSymbol> {
        self.as_ref()?.descendant_at(offset)
    }

    fn descendant_at_and_collect(
        &self,
        offset: usize,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) -> Option<DynSymbol> {
        self.as_ref()?
            .descendant_at_and_collect(offset, collect_fn, collect)
    }

    fn traverse_and_collect(
        &self,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) {
        if let Some(symbol) = self.as_ref() {
            symbol.traverse_and_collect(collect_fn, collect);
        }
    }
}

impl<T: AstSymbol> Traverse for Vec<Symbol<T>> {
    fn descendant_at(&self, offset: usize) -> Option<DynSymbol> {
        self.iter().find_map(|symbol| symbol.descendant_at(offset))
    }

    fn descendant_at_and_collect(
        &self,
        offset: usize,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) -> Option<DynSymbol> {
        self.iter()
            .find_map(|symbol| symbol.descendant_at_and_collect(offset, collect_fn, collect))
    }

    fn traverse_and_collect(
        &self,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) {
        for symbol in self.iter() {
            symbol.traverse_and_collect(collect_fn, collect);
        }
    }
}

/// Delimiting a symbol's scope
pub trait Scope {
    /// Tell this symbol is a scope
    ///
    /// By default, `false`
    fn is_scope(&self) -> bool {
        false
    }
}

/// Allowing a symbol to be commented
pub trait Comment {
    /// Tell this symbol is a comment
    ///
    /// This function is used when the **comment** query find a comment above this symbol to tell if the symbol can be commented
    ///
    /// By default, `false`
    fn is_comment(&self) -> bool {
        false
    }
}

macro_rules! impl_dyn_symbol {
    ($trait:ident, $fn_name:ident(&self, $($param_name:ident: $param_type:ty),*) $( -> $return_type: ty)?) => {
        impl $trait for DynSymbol {
            fn $fn_name(&self, $($param_name: $param_type),*) $(-> $return_type)? {
                self.read().$fn_name($($param_name),*)
            }
        }
    };
}

impl_dyn_symbol!(GetHover, get_hover(&self, doc: &Document) -> Option<lsp_types::Hover>);
impl_dyn_symbol!(GetGoToDefinition, go_to_definition(&self, doc: &Document) -> Option<GotoDefinitionResponse>);
impl_dyn_symbol!(GetGoToDeclaration, go_to_declaration(&self, doc: &Document) -> Option<GotoDeclarationResponse>);
impl_dyn_symbol!(BuildDocumentSymbols, build_document_symbols(&self, doc: &Document, builder: &mut DocumentSymbolsBuilder));
impl_dyn_symbol!(BuildSemanticTokens, build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder));
impl_dyn_symbol!(BuildInlayHints, build_inlay_hints(&self, doc: &Document, acc: &mut Vec<lsp_types::InlayHint>));
impl_dyn_symbol!(BuildCodeLenses, build_code_lenses(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>));
impl_dyn_symbol!(BuildCompletionItems, build_completion_items(&self, doc: &Document, acc: &mut Vec<CompletionItem>));
impl_dyn_symbol!(BuildTriggeredCompletionItems, build_triggered_completion_items(&self, trigger: &str, doc: &Document, acc: &mut Vec<CompletionItem>));
impl_dyn_symbol!(BuildCodeActions, build_code_actions(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeActionOrCommand>));

macro_rules! impl_build {
    ($trait:ident, $fn_name:ident(&self, $($param_name:ident: $param_type:ty),*)) => {
        impl<T: AstSymbol> $trait for Option<Symbol<T>> {
            fn $fn_name(&self, $($param_name: $param_type),*) {
                if let Some(node) = self.as_ref() {
                    node.read().$fn_name($($param_name),*)
                }
            }
        }

        impl<T: AstSymbol> $trait for Vec<Symbol<T>> {
            fn $fn_name(&self, $($param_name: $param_type),*) {
                for symbol in self.iter() {
                    symbol.read().$fn_name($($param_name),*)
                }
            }
        }
    };
}

impl_build!(BuildDocumentSymbols, build_document_symbols(&self, doc: &Document, builder: &mut DocumentSymbolsBuilder));
impl_build!(BuildSemanticTokens, build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder));
impl_build!(BuildInlayHints, build_inlay_hints(&self, doc: &Document, acc: &mut Vec<lsp_types::InlayHint>));
impl_build!(BuildCodeLenses, build_code_lenses(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>));
impl_build!(BuildCompletionItems, build_completion_items(&self, doc: &Document,  acc: &mut Vec<CompletionItem>));
impl_build!(BuildTriggeredCompletionItems, build_triggered_completion_items(&self, trigger: &str, doc: &Document,  acc: &mut Vec<CompletionItem>));
impl_build!(BuildCodeActions, build_code_actions(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeActionOrCommand>));
