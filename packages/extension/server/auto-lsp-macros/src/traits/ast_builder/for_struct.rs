use quote::{format_ident, quote};
use syn::{parse::Parser, DeriveInput};

use crate::{
    utilities::{
        extract_fields::StructFields,
        filter::{get_raw_type_name, is_hashmap, is_option, is_vec},
    },
    CodeGen,
};

pub fn generate_fields(input: &mut DeriveInput, code_gen: &mut CodeGen, weak: bool) {
    let pointer = format_ident!(
        "{}",
        match weak {
            true => "Weak",
            false => "Arc",
        }
    );

    // Fields cannot be generated from the quote! macro, so we need to manually add them
    match &mut input.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    // Transform each field's type to Arc<RwLock<OriginalType>>
                    for field in fields.named.iter_mut() {
                        let attributes = field.attrs.clone();
                        let raw_type_name = format_ident!("{}", get_raw_type_name(&field.ty));
                        let name = field.ident.clone();

                        *field = if let true = is_vec(&field.ty) {
                            syn::Field::parse_named
                                .parse2(quote! {
                                   #(#attributes)*
                                   #name: Vec<#pointer<RwLock<#raw_type_name>>>
                                })
                                .unwrap()
                        } else if let true = is_option(&field.ty) {
                            syn::Field::parse_named
                                .parse2(quote! {
                                   #(#attributes)*
                                   #name: Option<#pointer<RwLock<#raw_type_name>>>
                                })
                                .unwrap()
                        } else if let true = is_hashmap(&field.ty) {
                            syn::Field::parse_named
                                .parse2(quote! {
                                   #(#attributes)*
                                   #name: HashMap<String, #pointer<RwLock<#raw_type_name>>>
                                })
                                .unwrap()
                        } else {
                            syn::Field::parse_named
                                .parse2(quote! {
                                   #(#attributes)*
                                   #name: #pointer<RwLock<#raw_type_name>>
                                })
                                .unwrap()
                        };
                    }

                    for field in &code_gen.fields {
                        fields
                            .named
                            .push(syn::Field::parse_named.parse2(field.clone()).unwrap());
                    }

                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote! { parent: Option<#pointer<RwLock<dyn AstItem>>> })
                            .unwrap(),
                    );
                }
                _ => (),
            }
        }
        _ => panic!("This proc macro only works with struct"),
    };
}

pub fn generate_struct_builder_item(name: &str, input: &StructFields) -> proc_macro2::TokenStream {
    let struct_name = format_ident!("{}Builder", name);
    let name = format_ident!("{}", name);

    let field_names = &input.field_names;
    let field_vec_names = &input.field_vec_names;
    let field_option_names = &input.field_option_names;
    let field_hashmap_names = &input.field_hashmap_names;

    let field_types_names = &input.field_types_names;
    let field_vec_types_names = &input.field_vec_types_names;
    let field_option_types_names = &input.field_option_types_names;
    let field_hashmap_types_names = &input.field_hashmap_types_names;

    let field_builder_names = &input.field_builder_names;
    let field_vec_builder_names = &input.field_vec_builder_names;
    let field_option_builder_names = &input.field_option_builder_names;
    let field_hashmap_builder_names = &input.field_hashmap_builder_names;

    let commas = &input.first_commas;
    let option_commas = &input.after_option_commas;
    let vec_commas = &input.after_vec_commas;

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
            #(#vec_commas)*
            #(#field_hashmap_names: HashMap<String, std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>> ),*
        }

        impl auto_lsp::traits::ast_item_builder::AstItemBuilder for #struct_name {
            fn new(_query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Self {
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
                    #(#vec_commas)*
                    #(#field_hashmap_names: HashMap::new()),*
                }
            }

            fn query_binder(&self, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>> {
                let query_name = query.capture_names()[capture.index as usize];
                #(
                    if let true = #field_types_names::QUERY_NAMES.contains(&query_name)  {
                            return Some(std::rc::Rc::new(std::cell::RefCell::new(#field_builder_names::new(
                                &query,
                                capture.index as usize,
                                capture.node.range(),
                                capture.node.start_position(),
                                capture.node.end_position(),
                            ))))
                    };
                )*
                #(
                    if let true = #field_option_types_names::QUERY_NAMES.contains(&query_name)  {
                            return Some(std::rc::Rc::new(std::cell::RefCell::new(#field_option_builder_names::new(
                                &query,
                                capture.index as usize,
                                capture.node.range(),
                                capture.node.start_position(),
                                capture.node.end_position(),
                            ))))
                    };
                )*
                #(
                    if let true = #field_vec_types_names::QUERY_NAMES.contains(&query_name)  {
                            return Some(std::rc::Rc::new(std::cell::RefCell::new(#field_vec_builder_names::new(
                                &query,
                                capture.index as usize,
                                capture.node.range(),
                                capture.node.start_position(),
                                capture.node.end_position(),
                            ))))
                    };
                )*
                #(
                    if let true = #field_hashmap_types_names::QUERY_NAMES.contains(&query_name)  {
                            return Some(std::rc::Rc::new(std::cell::RefCell::new(#field_hashmap_builder_names::new(
                                &query,
                                capture.index as usize,
                                capture.node.range(),
                                capture.node.start_position(),
                                capture.node.end_position(),
                            ))))
                    };
                )*
                None
            }

            fn add(&mut self, query: &tree_sitter::Query, node: std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>, source_code: &[u8]) ->
                Result<auto_lsp::traits::ast_item_builder::DeferredAstItemBuilder, lsp_types::Diagnostic> {
                use auto_lsp::traits::ast_item_builder::DeferredAstItemBuilder;
                let query_name = query.capture_names()[node.borrow().get_query_index() as usize];
                #(
                    if #field_types_names::QUERY_NAMES.contains(&query_name) {
                        match self.#field_names {
                            Some(_) => return Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Field {:?} is already present in {:?}", stringify!(#field_names), stringify!(#struct_name)))),
                            None => self.#field_names = Some(node.clone())
                        }
                        return Ok(DeferredAstItemBuilder::None)
                    };
                )*
                #(
                    if #field_option_types_names::QUERY_NAMES.contains(&query_name) {
                        if self.#field_option_names.is_some() {
                            return Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Field {:?} is already present in {:?}", stringify!(#field_option_names), stringify!(#struct_name))));
                        }
                        self.#field_option_names = Some(node.clone());
                        return Ok(DeferredAstItemBuilder::None);
                    };
                )*
                #(
                    if #field_vec_types_names::QUERY_NAMES.contains(&query_name) {
                        self.#field_vec_names.push(node.clone());
                        return Ok(DeferredAstItemBuilder::None);
                    };
                )*
                #(
                    if #field_hashmap_types_names::QUERY_NAMES.contains(&query_name) {
                        return Ok(DeferredAstItemBuilder::HashMap(Box::new(|
                                parent: Rc<RefCell<dyn AstItemBuilder>>,
                                node: Rc<RefCell<dyn AstItemBuilder>>,
                                source_code: &[u8]
                            | {
                                let field = node.borrow();
                                let field = field.downcast_ref::<#field_hashmap_builder_names>().expect("Not a builder!");
                                let key = field.get_key(source_code);

                                let mut parent = parent.borrow_mut();
                                let parent = parent.downcast_mut::<#struct_name>().expect("Not the builder!");

                                if parent.#field_hashmap_names.contains_key(key) {
                                    return Err(auto_lsp::builder_error!(
                                        field.get_lsp_range(),
                                        format!(
                                            "Field {:?} is already declared in {:?}",
                                            key,
                                            stringify!(#struct_name)
                                        )
                                    ));
                                };
                                eprintln!("Inserting key {:?} of type {:?} in {}", key, stringify!(#field_hashmap_builder_names), stringify!(#struct_name));
                                parent.#field_hashmap_names.insert(key.into(), node.clone());
                                Ok(())
                        })));
                    };
                )*
                Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Invalid field {:?} in {:?}", query_name, stringify!(#struct_name))))
            }

            fn get_range(&self) -> tree_sitter::Range {
                self.range
            }

            fn get_query_index(&self) -> usize {
                self.query_index
            }
        }
    }
}

pub fn generate_try_from_ctx(name: &str, input: &StructFields) -> proc_macro2::TokenStream {
    let struct_name = format_ident!("{}Builder", name);
    let name = format_ident!("{}", name);

    let field_names = &input.field_names;
    let field_vec_names = &input.field_vec_names;
    let field_option_names = &input.field_option_names;
    let field_hashmap_names = &input.field_hashmap_names;

    let field_builder_names = &input.field_builder_names;
    let field_vec_builder_names = &input.field_vec_builder_names;
    let field_option_builder_names = &input.field_option_builder_names;
    let field_hashmap_builder_names = &input.field_hashmap_builder_names;

    let commas = &input.first_commas;
    let option_commas = &input.after_option_commas;
    let vec_commas = &input.after_vec_commas;

    quote! {        impl auto_lsp::traits::convert::TryFromCtx<#struct_name> for #name {
            type Error = lsp_types::Diagnostic;

            fn try_from_ctx(builder: #struct_name, ctx: &dyn auto_lsp::traits::workspace::WorkspaceContext) -> Result<Self, Self::Error> {
                use std::sync::{Arc, RwLock};
                let builder_range = builder.get_lsp_range();

                #(let #field_names =
                    builder
                    .#field_names
                    .ok_or(auto_lsp::builder_error!(builder_range, format!("Missing field {:?} in {:?}", stringify!(#field_names), stringify!(#struct_name))))?
                    .borrow()
                    .downcast_ref::<#field_builder_names>()
                    .ok_or(auto_lsp::builder_error!(builder_range, format!("Failed downcast conversion of {:?}", stringify!(#field_builder_names))))?
                    .clone()
                    .try_into_ctx(ctx)?;
                )*
                #(let #field_option_names = match builder.#field_option_names {
                        Some(builder) => {
                            let item = builder
                                .borrow()
                                .downcast_ref::<#field_option_builder_names>()
                                .ok_or(auto_lsp::builder_error!(builder_range, format!("Failed downcast conversion of {:?}", stringify!(#field_option_builder_names))))?
                                .clone()
                                .try_into_ctx(ctx)?;
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
                            .ok_or(auto_lsp::builder_error!(builder_range, format!("Failed downcast conversion of {:?}", stringify!(#field_vec_builder_names))))?
                            .clone()
                            .try_into_ctx(ctx)?;
                        Ok(item)
                    })
                    .collect::<Result<Vec<_>, lsp_types::Diagnostic>>()?;
                )*
                #(
                    let #field_hashmap_names = builder
                        .#field_hashmap_names
                        .into_iter()
                        .map(|(key, b)| {
                            let item = b
                                .borrow()
                                .downcast_ref::<#field_hashmap_builder_names>()
                                .ok_or(auto_lsp::builder_error!(builder_range, format!("Failed downcast conversion of {:?} at key {}", stringify!(#field_hashmap_builder_names), key)))?
                                .clone()
                                .try_into_ctx(ctx)?;
                            Ok((key, item))
                        })
                        .collect::<Result<HashMap<String, _>, lsp_types::Diagnostic>>()?;
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
                    #(#vec_commas)*
                    #(#field_hashmap_names),*
                })
            }
        }

        impl auto_lsp::traits::convert::TryFromCtx<#struct_name> for std::sync::Arc<std::sync::RwLock<#name>> {
            type Error = lsp_types::Diagnostic;

            fn try_from_ctx(builder: #struct_name, ctx: &dyn auto_lsp::traits::workspace::WorkspaceContext) -> Result<Self, Self::Error> {
                let item = #name::try_from_ctx(builder, ctx)?;
                let result = std::sync::Arc::new(std::sync::RwLock::new(item));
                result.write().unwrap().inject_parent(result.clone());
                Ok(result)
            }
        }
    }
}
