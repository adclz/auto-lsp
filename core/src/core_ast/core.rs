use super::capabilities::*;
use super::data::*;
use super::symbol::*;
use crate::build::Parent;
use crate::create_ast_symbol_trait;
use crate::document::Document;
use downcast_rs::{impl_downcast, DowncastSync};
use lsp_types::Position;
use lsp_types::Range;
use lsp_types::Url;
use std::sync::Arc;

#[cfg(feature = "incremental")]
create_ast_symbol_trait!(super::update::UpdateDynamic);

#[cfg(not(feature = "incremental"))]
create_ast_symbol_trait!();

#[macro_export]
macro_rules! create_ast_symbol_trait {
    ($($extra: path)?) => {
        #[doc = "Core functionality of an AST symbol."]
        #[doc = "\n"]
        #[doc = "Any struct or enum generated by the `seq` or `choice` macro implements this trait."]
        #[doc = "\n"]
        #[doc = "This trait supports downcasting via [downcast_rs]."]
        #[doc = "\n"]
        #[doc = "It inherits the vast majority of the capabilities defined in the `ast` module."]
        pub trait AstSymbol:
                DowncastSync
                + Send
                + Sync
                // lsp
                + GetGoToDeclaration
                + GetGoToDefinition
                + GetHover
                + BuildDocumentSymbols
                + BuildCodeLenses
                + BuildCompletionItems
                + BuildInvokedCompletionItems
                + BuildInlayHints
                + BuildSemanticTokens
                + BuildCodeActions
                // special
                + Traverse
                + Check
                + Reference
                + Scope
                + Comment
                // update.rs
                + GetSymbolData
                + Parent
                + $($extra)?
                {
                /// Retrieves the data of the symbol.
                fn get_data(&self) -> &SymbolData;

                /// Retrieves the mutable data of the symbol.
                fn get_mut_data(&mut self) -> &mut SymbolData;

                /// Retrieves the text of the symbol based on its range within the provided source code.
                fn get_text<'a>(&self, source_code: &'a [u8]) -> Option<&'a str> {
                    let range = self.get_data().get_range();
                    // Check if the range is within bounds and valid
                    if range.start <= range.end && range.end <= source_code.len() {
                        std::str::from_utf8(&source_code[range.start..range.end]).ok()
                    } else {
                        None
                    }
                }

                /// Get the symbol's scope.
                ///
                /// The scope defines the search area for references and completion items.
                fn get_parent_scope(&self) -> Option<DynSymbol> {
                    let mut parent = self.get_data().get_parent();
                    while let Some(weak) = parent {
                        let symbol = match weak.to_dyn() {
                            Some(weak) => weak,
                            None => return None,
                        };
                        let read = symbol.read();
                        if symbol.read().is_scope() {
                            return Some(symbol.clone());
                        }
                        parent = read.get_parent();
                    }
                    None
                }

                /// Checks if the symbol is within the given offset.
                fn is_inside_offset(&self, offset: usize) -> bool {
                    let range = self.get_data().get_range();
                    range.start <= offset && offset <= range.end
                }

                /// Returns the LSP start position of the symbol.
                fn get_start_position(&self, workspace: &Document) -> Position {
                    let range = self.get_data().get_range();
                    let node = workspace
                        .tree
                        .root_node()
                        .descendant_for_byte_range(range.start, range.start)
                        .unwrap();

                    Position {
                        line: node.start_position().row as u32,
                        character: node.start_position().column as u32,
                    }
                }

                /// Returns the LSP end position of the symbol.
                fn get_end_position(&self, workspace: &Document) -> Position {
                    let range = self.get_data().get_range();
                    let node = workspace
                        .tree
                        .root_node()
                        .descendant_for_byte_range(range.end, range.end)
                        .unwrap();

                    Position {
                        line: node.start_position().row as u32,
                        character: node.start_position().column as u32,
                    }
                }

                /// Returns the LSP range (start and end position) of the symbol.
                fn get_lsp_range(&self, workspace: &Document) -> Range {
                    let range = self.get_data().get_range();
                    let node = workspace
                        .tree
                        .root_node()
                        .descendant_for_byte_range(range.start, range.end)
                        .unwrap();

                    lsp_types::Range {
                        start: Position {
                            line: node.start_position().row as u32,
                            character: node.start_position().column as u32,
                        },
                        end: Position {
                            line: node.end_position().row as u32,
                            character: node.end_position().column as u32,
                        },
                    }
                }
                }
    };
}

impl_downcast!(AstSymbol);

impl<T: AstSymbol + ?Sized> GetSymbolData for T {
    fn get_url(&self) -> Arc<Url> {
        self.get_data().get_url()
    }

    fn get_range(&self) -> std::ops::Range<usize> {
        self.get_data().get_range()
    }

    fn get_parent(&self) -> Option<WeakSymbol> {
        self.get_data().get_parent()
    }

    fn set_parent(&mut self, parent: WeakSymbol) {
        self.get_mut_data().set_parent(parent)
    }

    fn get_comment<'a>(&self, source_code: &'a [u8]) -> Option<&'a str> {
        self.get_data().get_comment(source_code)
    }

    fn set_comment(&mut self, range: Option<std::ops::Range<usize>>) {
        self.get_mut_data().set_comment(range)
    }

    fn get_target(&self) -> Option<&WeakSymbol> {
        self.get_data().get_target()
    }

    fn set_target_reference(&mut self, target: WeakSymbol) {
        self.get_mut_data().set_target_reference(target)
    }

    fn reset_target_reference_reference(&mut self) {
        self.get_mut_data().reset_target_reference_reference();
    }

    fn get_referrers(&self) -> &Option<Referrers> {
        self.get_data().get_referrers()
    }

    fn get_mut_referrers(&mut self) -> &mut Referrers {
        self.get_mut_data().get_mut_referrers()
    }

    fn has_check_pending(&self) -> bool {
        self.get_data().has_check_pending()
    }

    fn update_check_pending(&mut self, status: bool) {
        self.get_mut_data().update_check_pending(status)
    }
}
