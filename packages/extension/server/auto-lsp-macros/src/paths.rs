use syn::{parse_quote, Path};

pub struct Paths {
    pub ast_item_builder_trait_path: Path,
    pub ast_item_trait_path: Path,
    pub deferred_ast_item_builder: Path,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            ast_item_builder_trait_path: parse_quote!(
                auto_lsp::traits::ast_item_builder::AstItemBuilder
            ),
            ast_item_trait_path: parse_quote!(auto_lsp::traits::ast_item::AstItem),
            deferred_ast_item_builder: parse_quote!(
                auto_lsp::traits::ast_item_builder::DeferredAstItemBuilder
            ),
        }
    }
}
