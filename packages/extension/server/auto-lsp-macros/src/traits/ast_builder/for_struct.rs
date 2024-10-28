use quote::{format_ident, quote};

use crate::utilities::extract_fields::StructFields;
use auto_lsp::traits::ast_item_builder::AstItemBuilder;

pub fn generate_struct_builder_item(name: &str, input: &StructFields) -> proc_macro2::TokenStream {
    let struct_name = format_ident!("{}Builder", name);
    let name = format_ident!("{}", name);

    let field_names = &input.field_names;
    let field_vec_names = &input.field_vec_names;
    let field_option_names = &input.field_option_names;

    let field_types_names = &input.field_types_names;
    let field_vec_types_names = &input.field_vec_types_names;
    let field_option_types_names = &input.field_option_types_names;

    let field_builder_names = &input.field_builder_names;
    let field_vec_builder_names = &input.field_vec_builder_names;
    let field_option_builder_names = &input.field_option_builder_names;

    let commas = &input.commas;
    let option_commas = &input.option_commas;

    quote! {
        #[derive(Clone, Debug)]
        pub struct #struct_name {
            query_index: usize,
            range: tree_sitter::Range,
            start_position: tree_sitter::Point,
            end_position: tree_sitter::Point,
            #(#field_names: Option<std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>> ),*
            #(#commas)*
            #(#field_option_names: Option<std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>>),*
            #(#option_commas)*
            #(#field_vec_names: Vec<std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>> ),*
        }

        impl auto_lsp::traits::ast_item_builder::AstItemBuilder for #struct_name {
            fn add(&mut self, query: &tree_sitter::Query, node: std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>) -> bool {
                let query_name = query.capture_names()[node.borrow().get_query_index() as usize];
                #(
                    if let true = #field_types_names::QUERY_NAMES.contains(&query_name) {
                        match self.#field_names {
                            Some(_) => return false,
                            None => self.#field_names = Some(node.clone())
                        }
                        return true;
                    };
                )*
                #(
                    if let true = #field_option_types_names::QUERY_NAMES.contains(&query_name) {
                        if self.#field_option_names.is_some() {
                            return false;
                        }
                        self.#field_option_names = Some(node.clone());
                        return true;
                    };
                )*
                #(
                    if let true = #field_vec_types_names::QUERY_NAMES.contains(&query_name) {
                        self.#field_vec_names.push(node.clone());
                        return true;
                    };
                )*
                false
            }

            fn get_range(&self) -> tree_sitter::Range {
                self.range
            }

            fn get_query_index(&self) -> usize {
                self.query_index
            }
        }

        impl #struct_name {
            pub fn new(_query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Self {
                Self {
                    query_index,
                    range,
                    start_position,
                    end_position,
                    #(#field_names: None),*
                    #(#commas)*
                    #(#field_option_names: None),*
                    #(#option_commas)*
                    #(#field_vec_names: vec!()),*
                }
            }
        }

        impl TryFrom<#struct_name> for #name {
            type Error = ();

            fn try_from(builder: #struct_name) -> Result<Self, Self::Error> {
                use std::sync::{Arc, RwLock};

                #(let #field_names =
                    builder
                    .#field_names
                    .expect("Field not found")
                    .borrow()
                    .downcast_ref::<#field_builder_names>()
                    .expect(&format!("Failed downcast conversion of {:?}", stringify!(#field_builder_names)))
                    .clone()
                    .try_into().expect("Failed builder conversion");
                )*
                #(let #field_option_names = match builder.#field_option_names {
                        Some(builder) => {
                            let item = builder
                                .borrow()
                                .downcast_ref::<#field_option_builder_names>()
                                .unwrap()
                                .clone()
                                .try_into().expect("Failed builder conversion");
                            Some(item)
                        },
                        None => None
                    };
                )*
                #(let #field_vec_names = builder
                    .#field_vec_names
                    .into_iter()
                    .map(|b| {
                        let item = b
                            .borrow()
                            .downcast_ref::<#field_vec_builder_names>()
                            .expect("Failed downcast conversion")
                            .clone()
                            .try_into().expect("Failed builder conversion");
                        item
                    })
                    .collect();
                )*
                Ok(#name {
                    range: builder.range,
                    start_position: builder.start_position,
                    end_position: builder.end_position,
                    parent: None,
                    #(#field_names),*
                    #(#commas)*
                    #(#field_option_names),*
                    #(#option_commas)*
                    #(#field_vec_names),*
                })
            }
        }

        impl TryFrom<#struct_name> for std::sync::Arc<std::sync::RwLock<#name>> {
            type Error = ();

            fn try_from(builder: #struct_name) -> Result<Self, Self::Error> {
                let item = #name::try_from(builder)?;
                let result = std::sync::Arc::new(std::sync::RwLock::new(item));
                result.write().unwrap().inject_parent(result.clone());
                Ok(result)
            }
        }
    }
}
