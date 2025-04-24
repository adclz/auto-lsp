/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

#![allow(unused_variables)]

use std::sync::Arc;

use super::core::AstSymbol;
use super::symbol::*;
use crate::document_symbols_builder::DocumentSymbolsBuilder;
use crate::{document::Document, semantic_tokens_builder::SemanticTokensBuilder};
use lsp_types::{request::GotoDeclarationResponse, CompletionItem, GotoDefinitionResponse};

/// [LSP DocumentSymbol specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_documentSymbol)
pub trait BuildDocumentSymbols {
    /// ```rust
    /// # struct MySymbol{}
    ///  use auto_lsp_core::document::Document;
    ///  use auto_lsp_core::ast::BuildDocumentSymbols;
    ///  use auto_lsp_core::document_symbols_builder::DocumentSymbolsBuilder;
    ///
    /// impl BuildDocumentSymbols for MySymbol {
    ///    fn build_document_symbols(&self, doc: &Document, acc: &mut DocumentSymbolsBuilder) -> anyhow::Result<()> {
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
    ///       Ok(())
    ///    }
    /// }
    fn build_document_symbols(
        &self,
        doc: &Document,
        acc: &mut DocumentSymbolsBuilder,
    ) -> anyhow::Result<()> {
        Ok(())
    }
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
    ///     fn get_hover(&self, doc: &Document) -> anyhow::Result<Option<lsp_types::Hover>> {
    ///        Ok(Some(lsp_types::Hover {
    ///             contents: lsp_types::HoverContents::Markup(
    ///                 lsp_types::MarkupContent {
    ///                     kind: lsp_types::MarkupKind::Markdown,
    ///                     value: "Hello, World!".to_string(),
    ///                 }
    ///             ),
    ///             range: None
    ///        }))
    ///     }
    /// }
    /// ```
    fn get_hover(&self, doc: &Document) -> anyhow::Result<Option<lsp_types::Hover>> {
        Ok(None)
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
    ///    fn go_to_definition(&self, doc: &Document) -> anyhow::Result<Option<lsp_types::GotoDefinitionResponse>> {
    ///       Ok(Some(lsp_types::GotoDefinitionResponse::Scalar(lsp_types::Location::new(
    ///          Url::parse("file:///path/to/file").unwrap(),
    ///         lsp_types::Range::default()
    ///      ))))
    ///   }
    /// }
    fn go_to_definition(&self, doc: &Document) -> anyhow::Result<Option<GotoDefinitionResponse>> {
        Ok(None)
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
    ///    fn go_to_declaration(&self, doc: &Document) -> anyhow::Result<Option<lsp_types::request::GotoDeclarationResponse>> {
    ///       Ok(Some(lsp_types::request::GotoDeclarationResponse::Scalar(lsp_types::Location::new(
    ///          Url::parse("file:///path/to/file").unwrap(),
    ///          lsp_types::Range::default()
    ///      ))))
    ///   }
    /// }
    fn go_to_declaration(&self, doc: &Document) -> anyhow::Result<Option<GotoDeclarationResponse>> {
        Ok(None)
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
    ///   fn build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder) -> anyhow::Result<()> {
    ///      builder.push(
    ///         lsp_types::Range::default(),
    ///         10,
    ///         0
    ///      );
    ///      Ok(())
    ///   }
    /// }
    ///
    ///
    fn build_semantic_tokens(
        &self,
        doc: &Document,
        builder: &mut SemanticTokensBuilder,
    ) -> anyhow::Result<()> {
        Ok(())
    }
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
    ///   fn build_inlay_hints(&self, doc: &Document, acc: &mut Vec<lsp_types::InlayHint>) -> anyhow::Result<()> {
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
    ///     Ok(())
    ///   }
    /// }
    /// ```
    fn build_inlay_hints(
        &self,
        doc: &Document,
        acc: &mut Vec<lsp_types::InlayHint>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
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
    ///   fn build_code_lenses(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) -> anyhow::Result<()> {
    ///     acc.push(lsp_types::CodeLens {
    ///         range: lsp_types::Range::default(),
    ///         command: None,
    ///         data: None,
    ///     });
    ///     Ok(())
    ///   }
    /// }
    /// ```
    fn build_code_lenses(
        &self,
        doc: &Document,
        acc: &mut Vec<lsp_types::CodeLens>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
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
    ///   fn build_completion_items(&self, doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) -> anyhow::Result<()> {
    ///     acc.push(lsp_types::CompletionItem {
    ///         label: "Completion Item".to_string(),
    ///         kind: Some(lsp_types::CompletionItemKind::FIELD),
    ///         ..Default::default()
    ///     });
    ///     Ok(())
    ///   }
    /// }
    /// ```
    fn build_completion_items(
        &self,
        doc: &Document,
        acc: &mut Vec<CompletionItem>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
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
    ///   fn build_triggered_completion_items(&self, trigger: &str, doc: &Document, acc: &mut Vec<lsp_types::CompletionItem>) -> anyhow::Result<()> {
    ///     if trigger == "." {
    ///         acc.push(lsp_types::CompletionItem {
    ///             label: "Completion Item".to_string(),
    ///             kind: Some(lsp_types::CompletionItemKind::FIELD),
    ///             ..Default::default()
    ///         });
    ///      };
    ///      Ok(())
    ///    }
    /// }
    fn build_triggered_completion_items(
        &self,
        trigger: &str,
        doc: &Document,
        acc: &mut Vec<CompletionItem>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

/// [LSP CodeAction specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeAction)
pub trait BuildCodeActions {
    fn build_code_actions(
        &self,
        doc: &Document,
        acc: &mut Vec<lsp_types::CodeActionOrCommand>,
    ) -> anyhow::Result<()> {
        Ok(())
    }
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
        if self.0.is_inside_offset(offset) {
            self.0.descendant_at(offset).or_else(|| Some(self.clone()))
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
        if self.0.is_inside_offset(offset) {
            if collect_fn(self.clone()) {
                collect.push(self.clone());
            }
            self.0
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
        self.0.traverse_and_collect(collect_fn, collect);
    }
}

/*
impl<T: AstSymbol> Traverse for Arc<T> {
    fn descendant_at(&self, offset: usize) -> Option<DynSymbol> {
        match self.0.is_inside_offset(offset) {
            true => self.0.descendant_at(offset).or_else(|| Some(self.into())),
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
        match self.0.is_inside_offset(offset) {
            true => {
                if collect_fn(to_dyn.clone()) {
                    collect.push(to_dyn);
                }
                self.0
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
        let symbol = &self.0;
        if collect_fn(self.into()) {
            collect.push(self.into());
        }
        symbol.traverse_and_collect(collect_fn, collect);
    }
}
*/
impl<T: AstSymbol> Traverse for Option<Arc<T>> {
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

impl<T: AstSymbol> Traverse for Vec<Arc<T>> {
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
    ($trait:ident, $fn_name:ident(&self, $($param_name:ident: $param_type:ty),*)-> $return_type: ty) => {
        impl $trait for DynSymbol {
            fn $fn_name(&self, $($param_name: $param_type),*) -> anyhow::Result<$return_type> {
                self.0.$fn_name($($param_name),*)
            }
        }
    };
}

impl_dyn_symbol!(GetHover, get_hover(&self, doc: &Document) -> Option<lsp_types::Hover>);
impl_dyn_symbol!(GetGoToDefinition, go_to_definition(&self, doc: &Document) -> Option<GotoDefinitionResponse>);
impl_dyn_symbol!(GetGoToDeclaration, go_to_declaration(&self, doc: &Document) -> Option<GotoDeclarationResponse>);
impl_dyn_symbol!(BuildDocumentSymbols, build_document_symbols(&self, doc: &Document, builder: &mut DocumentSymbolsBuilder) -> ());
impl_dyn_symbol!(BuildSemanticTokens, build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder) -> ());
impl_dyn_symbol!(BuildInlayHints, build_inlay_hints(&self, doc: &Document, acc: &mut Vec<lsp_types::InlayHint>) -> ());
impl_dyn_symbol!(BuildCodeLenses, build_code_lenses(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) -> ());
impl_dyn_symbol!(BuildCompletionItems, build_completion_items(&self, doc: &Document, acc: &mut Vec<CompletionItem>) -> ());
impl_dyn_symbol!(BuildTriggeredCompletionItems, build_triggered_completion_items(&self, trigger: &str, doc: &Document, acc: &mut Vec<CompletionItem>)  -> ());
impl_dyn_symbol!(BuildCodeActions, build_code_actions(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeActionOrCommand>)  -> ());

macro_rules! impl_build {
    ($trait:ident, $fn_name:ident(&self, $($param_name:ident: $param_type:ty),*)) => {
        impl<T: AstSymbol> $trait for Option<Arc<T>> {
            fn $fn_name(&self, $($param_name: $param_type),*) -> anyhow::Result<()> {
                if let Some(node) = self.as_ref() {
                    node.$fn_name($($param_name),*)?;
                }
                Ok(())
            }
        }

        impl<T: AstSymbol> $trait for Vec<Arc<T>> {
            fn $fn_name(&self, $($param_name: $param_type),*) -> anyhow::Result<()> {
                for symbol in self.iter() {
                    symbol.$fn_name($($param_name),*)?;
                }
                Ok(())
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
