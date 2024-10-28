use std::collections::HashSet;
use crate::FeaturesCodeGen;
use crate::{
    utilities::extract_fields::StructFields,
};
use darling::FromMeta;
use proc_macro2::Ident;
use quote::{format_ident, quote};

pub fn generate_struct_ast_item(query_name: &str, input: &StructFields) -> FeaturesCodeGen {
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

    FeaturesCodeGen {
        fields: Some(vec![
            quote! { range: tree_sitter::Range },
            quote! { start_position: tree_sitter::Point },
            quote! { end_position: tree_sitter::Point },
        ]),
        impl_base: Some(quote! {
            pub const QUERY_NAMES: &[&str] = &[#query_name];
        }),
        impl_ast_item: Some(
            quote! {
                fn get_range(&self) -> tree_sitter::Range {
                    self.range
                }

                fn get_parent(&self) -> Option<std::sync::Arc<std::sync::RwLock<dyn AstItem>>> {
                    self.parent.as_ref().map(|p| p.clone())
                }

                fn set_parent(&mut self, parent: std::sync::Arc<std::sync::RwLock<dyn AstItem>>) {
                    self.parent = Some(parent);
                }

                fn inject_parent(&mut self, parent: std::sync::Arc<std::sync::RwLock<dyn AstItem>>) {
                    #(
                        self.#field_names.write().unwrap().set_parent(parent.clone());
                    )*
                    #(
                        if let Some(ref mut field) = self.#field_option_names {
                            field.write().unwrap().set_parent(parent.clone());
                        };
                    )*
                    #(
                        for field in self.#field_vec_names.iter_mut() {
                            field.write().unwrap().set_parent(parent.clone());
                        };
                    )*
                }

                fn find_at_offset(&self, offset: &usize) -> Option<std::sync::Arc<std::sync::RwLock<dyn AstItem>>> {
                    // It's pointless to keep searching if the parent item is not inside the offset
                    if (!self.is_inside_offset(offset)) {
                        return None;
                    }
                    
                    #(if let true = self.#field_names.read().unwrap().is_inside_offset(offset) {
                        match self.#field_names.read().unwrap().find_at_offset(offset) {
                            Some(a) => return Some(a),
                            None => return Some(self.#field_names.clone())
                        }
                    })*
                    #(
                        match self.#field_option_names {
                            Some(ref field) => {
                                if let true = field.read().unwrap().is_inside_offset(offset) {
                                    match field.read().unwrap().find_at_offset(offset) {
                                        Some(a) => return Some(a),
                                        None => return Some(field.clone())
                                    }
                                }
                            },
                            None => {}
                        }
                    )*
                    #(
                      if let Some(item) = self.#field_vec_names
                        .iter()
                        .find(|field| field.read().unwrap().is_inside_offset(offset)) {
                            match item.read().unwrap().find_at_offset(offset) {
                                Some(a) => return Some(a),
                                None => return Some(item.clone())
                            }
                        }
                    )*
                    None
                }

                fn swap_at_offset(&mut self, offset: &usize, item: &std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>) {
                    // It's pointless to keep searching if the parent item is not inside the offset
                    if !self.is_inside_offset(offset) {
                        return;
                    }
                    
                    #(
                        let #field_names = self.#field_names.read().unwrap();
                        if #field_names.is_inside_offset(offset) {
                            match #field_names.find_at_offset(offset) {
                                Some(a) => a.write().unwrap().swap_at_offset(offset, item),
                                None => {
                                    if let Some(field) = item.borrow().downcast_ref::<#field_builder_names>() {
                                        drop(#field_names);
                                        // todo: add drop handler when arc goes out of scope
                                        self.#field_names = Arc::new(RwLock::new(field.clone().try_into().unwrap()));
                                    }
                                }
                            }
                        }
                    )*
                }
            }
            .into(),
        ),
    }
}
