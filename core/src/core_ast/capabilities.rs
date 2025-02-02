#![allow(unused)]

use std::ops::Deref;

use lsp_types::{
    request::GotoDeclarationResponse, CompletionItem, Diagnostic, DocumentSymbol,
    GotoDefinitionResponse,
};

use super::core::AstSymbol;
use super::data::*;
use super::symbol::*;
use crate::document_symbols_builder::DocumentSymbolsBuilder;
use crate::{document::Document, semantic_tokens_builder::SemanticTokensBuilder};

/// A trait to be implemented by any [AstSymbol] that can provide document symbols
pub trait BuildDocumentSymbols {
    /// Either return an optional single symbol or a vector of symbols, see [VecOrSymbol]
    ///
    /// [LSP DocumentSymbol](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_documentSymbol)
    fn build_document_symbols(&self, doc: &Document, builder: &mut DocumentSymbolsBuilder) {}
}

/// A trait to be implemented by any [AstSymbol] that can provide hover information
pub trait GetHover {
    /// Return hover information
    ///
    /// [LSP Hover](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#hover)
    ///
    /// By default, `None`
    fn get_hover(&self, doc: &Document) -> Option<lsp_types::Hover> {
        None
    }
}

/// A trait to be implemented by any [AstSymbol] that can provide goto definition information
pub trait GetGoToDefinition {
    /// Return a goto definition information
    ///
    /// [LSP GotoDefinitionResponse](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_definition)
    ///
    /// By default, `None`
    fn go_to_definition(&self, doc: &Document) -> Option<GotoDefinitionResponse> {
        None
    }
}

/// A trait to be implemented by any [AstSymbol] that can provide goto declaration information
pub trait GetGoToDeclaration {
    /// Return a goto declaration information
    ///
    /// [LSP GotoDeclarationResponse](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_declaration)
    ///
    /// By default, `None`
    fn go_to_declaration(&self, doc: &Document) -> Option<GotoDeclarationResponse> {
        None
    }
}

/// A trait to be implemented by any [AstSymbol] that can provide goto type definition information
pub trait BuildSemanticTokens {
    /// Semantic tokens builder
    ///
    /// [LSP BuildSemanticTokens](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_semanticTokens)
    ///
    /// Use [SemanticTokensBuilder] to build the semantic tokens
    fn build_semantic_tokens(&self, doc: &Document, builder: &mut SemanticTokensBuilder) {}
}

/// A trait to be implemented by any [AstSymbol] that can provide inlay hints
pub trait BuildInlayHints {
    /// Inlay hints builder
    ///
    /// [LSP InlayHint](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_inlayHints)
    ///
    /// Push inlay hints to the accumulator
    fn build_inlay_hints(&self, doc: &Document, acc: &mut Vec<lsp_types::InlayHint>) {}
}

/// A trait to be implemented by any [AstSymbol] that can provide code lens
pub trait BuildCodeLens {
    /// Code lens builder
    ///
    /// [LSP BuildCodeLens](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeLens)
    ///
    /// Push code lens to the accumulator
    fn build_code_lens(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>) {}
}

/// A trait to be implemented by any [AstSymbol] that can provide completion items when manually invoked
pub trait BuildCompletionItems {
    /// Completion items builder
    ///
    /// [LSP CompletionItem](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionItem)
    ///
    /// Push completion items to the accumulator
    fn build_completion_items(&self, doc: &Document, acc: &mut Vec<CompletionItem>) {}
}

/// A trait to be implemented by any [AstSymbol] that can provide completion items when invoked by a character trigger
pub trait BuildInvokedCompletionItems {
    /// Completion items builder
    ///
    /// [LSP CompletionItem](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionItem)
    ///
    /// Push completion items to the accumulator
    fn build_invoked_completion_items(
        &self,
        trigger: &str,
        doc: &Document,
        acc: &mut Vec<CompletionItem>,
    ) {
    }
}
/// Special capabilities

/// Trait implemented by all [AstSymbol]
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
            true => symbol.descendant_at(offset).or_else(|| Some(self.to_dyn())),
            false => None,
        }
    }

    fn descendant_at_and_collect(
        &self,
        offset: usize,
        collect_fn: fn(DynSymbol) -> bool,
        collect: &mut Vec<DynSymbol>,
    ) -> Option<DynSymbol> {
        let to_dyn = self.to_dyn();
        let symbol = self.read();
        match symbol.is_inside_offset(offset) {
            true => {
                if collect_fn(to_dyn.clone()) {
                    collect.push(to_dyn);
                }
                symbol
                    .descendant_at_and_collect(offset, collect_fn, collect)
                    .or_else(|| Some(self.to_dyn()))
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
        if collect_fn(self.to_dyn()) {
            collect.push(self.to_dyn());
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

/// Trait implemented by all [AstSymbol]
pub trait IsReference {
    /// Check if the symbol is an reference
    ///
    /// References are symbols used to access other symbols.
    ///
    /// By default, `false`
    fn is_reference(&self) -> bool {
        false
    }
}

/// Trait to be implemented by any [AstSymbol] to find a reference
pub trait Reference: IsReference {
    /// Find the target symbol
    ///
    /// This method is called during the last step of the build process
    ///
    /// If the symbol cannot be found, return a [Diagnostic] with the reason
    ///
    /// By default, `Ok(None)`
    fn find(&self, doc: &Document) -> Result<Option<DynSymbol>, Diagnostic> {
        Ok(None)
    }
}
/// Trait implemented by all [AstSymbol]
pub trait IsCheck {
    /// Tell this symbol has to be checked during the build process
    ///
    /// By default, `false`
    fn must_check(&self) -> bool {
        false
    }
}

/// Trait to be implemented by any [AstSymbol] to perform a check
pub trait Check: IsCheck {
    /// Perform a check on the symbol during the last step of build process
    ///
    /// If the check fails, push your [`Diagnostic`] to the accumulator and return `std::Err(())`
    /// If the check succeeds, return `Ok(())`
    ///
    /// By default, `Ok(())`
    fn check(&self, doc: &Document, diagnostics: &mut Vec<Diagnostic>) -> Result<(), ()> {
        Ok(())
    }
}

/// Trait implemented by all [AstSymbol]
pub trait IsScope {
    /// Tell this symbol is a scope
    ///
    /// By default, `false`
    fn is_scope(&self) -> bool {
        false
    }
}

/// Trait to be implemented by any [AstSymbol] that is a scope
pub trait Scope: IsScope {
    /// When a symbol is a scope, it provides a list of ranges where its content is valid
    ///
    /// This is useful when trying to find a reference, a completion item, etc. because this delimits the search area
    ///
    /// By default, `Vec::new()`
    fn get_scope_range(&self) -> Vec<[usize; 2]> {
        Vec::new()
    }
}

/// Trait implemented by all [AstSymbol]
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

/// Trait implemented by all [AstSymbol]
pub trait Finder {
    /// Find a symbol in a document
    ///
    /// This function first try to find the nearest scope that contains the symbol,
    /// then it searches for the symbol in the scope via its text
    ///
    /// If a symbol is not found, it tries to get the parent scope and search again until a symbol is found or there's no more parent scope
    fn find_in_file(&self, doc: &Document) -> Option<DynSymbol>;
}

impl<T: AstSymbol> Finder for T {
    fn find_in_file(&self, doc: &Document) -> Option<DynSymbol> {
        let source_code = &doc.texter.text;
        let pattern = match self.get_text(source_code.as_bytes()) {
            Some(a) => a,
            None => return None,
        };

        let mut curr = self.get_parent_scope();
        while let Some(scope) = curr {
            let scope = scope.read();
            let ranges = scope.get_scope_range();

            for range in ranges {
                let area = match source_code.as_str().get(range[0]..range[1]) {
                    Some(a) => a,
                    None => {
                        #[cfg(feature = "log")]
                        log::warn!("Invalid document range: {:?}", range);
                        continue;
                    }
                };

                for (index, _) in area.match_indices(pattern) {
                    if let Some(elem) = scope.descendant_at(range[0] + index) {
                        if elem.read().get_range() != self.get_range() {
                            match elem.read().get_text(source_code.as_bytes()) {
                                Some(a) => {
                                    if a == pattern {
                                        return Some(elem.clone());
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                }
            }
            curr = scope.get_parent_scope();
        }
        None
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
impl_dyn_symbol!(BuildCodeLens, build_code_lens(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>));
impl_dyn_symbol!(BuildCompletionItems, build_completion_items(&self, doc: &Document, acc: &mut Vec<CompletionItem>));
impl_dyn_symbol!(BuildInvokedCompletionItems, build_invoked_completion_items(&self, trigger: &str, doc: &Document, acc: &mut Vec<CompletionItem>));

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
impl_build!(BuildCodeLens, build_code_lens(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeLens>));
impl_build!(BuildCompletionItems, build_completion_items(&self, doc: &Document,  acc: &mut Vec<CompletionItem>));
impl_build!(BuildInvokedCompletionItems, build_invoked_completion_items(&self, trigger: &str, doc: &Document,  acc: &mut Vec<CompletionItem>));
