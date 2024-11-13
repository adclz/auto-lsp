use quote::{format_ident, quote};

use crate::utilities::extract_fields::EnumFields;
pub fn generate_enum_builder_item(name: &str, input: &EnumFields) -> proc_macro2::TokenStream {
    let struct_name = format_ident!("{}Builder", name);
    let name = format_ident!("{}", name);

    let variant_names = &input.variant_names;

    let variant_types_names = &input.variant_types_names;

    let variant_builder_names = &input.variant_builder_names;

    quote! {
        pub struct #struct_name {
            pub unique_field: std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>,
        }

        impl auto_lsp::traits::ast_item_builder::AstItemBuilder for #struct_name {
            fn new(query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Self {
                let query_name = query.capture_names()[query_index as usize];
                #(
                    if let true = #variant_types_names::QUERY_NAMES.contains(&query_name) {
                        return Self {
                            unique_field: Rc::new(RefCell::new(#variant_builder_names::new(
                                query,
                                query_index,
                                range,
                                start_position,
                                end_position
                            )))
                        };
                    };
                )*
                panic!("Unexpected")
            }

            fn query_binder(&self, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>> {
                self.unique_field.borrow().query_binder(capture, query)
            }

            fn add(&mut self, query: &tree_sitter::Query, node: Rc<RefCell<dyn AstItemBuilder>>, source_code: &[u8]) ->
            Result<auto_lsp::traits::ast_item_builder::DeferredAstItemBuilder, lsp_types::Diagnostic> {
                self.unique_field.borrow_mut().add(query, node, source_code)
            }

            fn get_range(&self) -> tree_sitter::Range {
                self.unique_field.borrow().get_range()
            }

            fn get_query_index(&self) -> usize {
                self.unique_field.borrow().get_query_index()
            }
        }

        impl auto_lsp::traits::convert::TryFromCtx<&#struct_name> for #name {
            type Error = lsp_types::Diagnostic;

            fn try_from_ctx(builder: &#struct_name, ctx: &dyn auto_lsp::traits::workspace::WorkspaceContext) -> Result<Self, Self::Error> {
                use std::sync::{Arc, RwLock};
                #(
                    if let Some(variant) = builder.unique_field.borrow().downcast_ref::<#variant_builder_names>() {
                        return Ok(Self::#variant_names(variant.clone().try_into_ctx(ctx)?));
                    };
                )*
                panic!("")
            }
        }

        impl auto_lsp::traits::convert::TryFromCtx<&#struct_name> for std::sync::Arc<std::sync::RwLock<#name>> {
            type Error = lsp_types::Diagnostic;

            fn try_from_ctx(builder: &#struct_name, ctx: &dyn auto_lsp::traits::workspace::WorkspaceContext) -> Result<Self, Self::Error> {
                let item = #name::try_from_ctx(builder, ctx)?;
                let result = std::sync::Arc::new(std::sync::RwLock::new(item));
                result.write().unwrap().inject_parent(result.clone());
                Ok(result)
            }
        }
    }
}
