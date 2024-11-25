use crate::{
    utilities::extract_fields::{FieldInfoExtract, StructFields},
    BuildAstItem, BuildAstItemBuilder, Features, FeaturesCodeGen, Paths, SymbolFeatures, ToCodeGen,
};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::Attribute;

pub struct StructBuilder<'a> {
    // Input data
    pub input_attr: &'a Vec<Attribute>,
    pub input_name: &'a Ident,
    pub query_name: &'a str,
    pub input_buider_name: &'a Ident,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
    // Paths
    pub paths: &'a Paths,
    // Features
    pub features: Features<'a>,
}

impl<'a> StructBuilder<'a> {
    pub fn new(
        params: Option<&'a SymbolFeatures>,
        input_attr: &'a Vec<Attribute>,
        input_name: &'a Ident,
        input_buider_name: &'a Ident,
        query_name: &'a str,
        fields: &'a StructFields,
        paths: &'a Paths,
        is_accessor: bool,
    ) -> Self {
        Self {
            input_name,
            input_attr,
            query_name,
            input_buider_name,
            fields,
            is_accessor,
            paths,
            features: Features::new(params, is_accessor, input_name, paths, fields),
        }
    }
}

impl<'a> ToTokens for StructBuilder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let input_name = &self.input_name;
        let input_attr = &self.input_attr;
        let query_name = self.query_name;

        // Generate features
        let mut code_gen = FeaturesCodeGen::default();
        self.features.to_code_gen(&mut code_gen);

        let input_fields = code_gen.input.fields;
        let features_impl = code_gen.input.impl_base;
        let features_impl_ast = code_gen.input.impl_ast_item;
        let features_others_impl = code_gen.input.other_impl;

        // generate ast item
        let ast_item_trait = &self.paths.ast_item_trait;

        let fields = self.generate_fields();
        let methods = self.generate_ast_item_methods();

        tokens.extend(quote! {
            #(#input_attr)*
            #[derive(Clone)]
            pub struct #input_name {
                #(#fields,)*
                #(#input_fields),*
            }

            impl #input_name {
                pub const QUERY_NAMES: &[&str] = &[#query_name];
                #(#features_impl)*
            }

            impl #ast_item_trait for #input_name {
                #methods
                #(#features_impl_ast)*
            }

            #(#features_others_impl)*
        });

        // Generate builder

        let input_builder_name = &self.input_buider_name;
        let ast_item_builder = &self.paths.ast_item_builder_trait;

        let builder_fields = self.generate_builder_fields();
        let new = self.generate_builder_new();
        let query_binder = self.generate_query_binder();
        let add = self.generate_add();
        let try_from = self.generate_try_from();

        let ast_item_trait_object_arc = &self.paths.ast_item_trait_object_arc;
        let try_from_builder = &self.paths.try_from_builder;

        let into = quote! {
            fn try_into_item(&self, check: &mut Vec<#ast_item_trait_object_arc>) -> Result<#ast_item_trait_object_arc, lsp_types::Diagnostic> {
                use #try_from_builder;

                let item = #input_name::try_from_builder(self, check)?;
                Ok(std::sync::Arc::new(std::sync::RwLock::new(item)))
            }
        };

        tokens.extend(quote! {
            #[derive(Clone, Debug)]
            pub struct #input_builder_name {
                url: std::sync::Arc<lsp_types::Url>,
                query_index: usize,
                range: tree_sitter::Range,
                start_position: tree_sitter::Point,
                end_position: tree_sitter::Point,
                #(#builder_fields),*
            }

            impl #ast_item_builder for #input_builder_name {
                #new
                #query_binder
                #add
                #into

                fn get_url(&self) -> std::sync::Arc<lsp_types::Url> {
                    self.url.clone()
                }

                fn get_range(&self) -> tree_sitter::Range {
                    self.range
                }

                fn get_query_index(&self) -> usize {
                    self.query_index
                }
            }

            #try_from
        });
    }
}

impl<'a> BuildAstItem for StructBuilder<'a> {
    fn generate_fields(&self) -> Vec<TokenStream> {
        let ast_item_trait_object_weak = &self.paths.ast_item_trait_object_weak;
        let mut fields = vec![
            quote! { pub url: std::sync::Arc<lsp_types::Url> },
            quote! { pub parent: Option<#ast_item_trait_object_weak> },
            quote! { pub range: tree_sitter::Range },
            quote! { pub start_position: tree_sitter::Point },
            quote! { pub end_position: tree_sitter::Point },
        ];

        if self.is_accessor {
            fields.push(quote! { pub accessor: Option<#ast_item_trait_object_weak> });
        }

        if !self.fields.field_names.is_empty() {
            fields.extend(
                self.fields
                    .field_names
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let attributes = &field.attr;
                        let name = &field.ident;
                        let _type = &self.fields.field_types_names[i];

                        quote! {
                           #(#attributes)*
                           pub #name: std::sync::Arc<std::sync::RwLock<#_type>>
                        }
                    }),
            )
        };
        if !self.fields.field_option_names.is_empty() {
            fields.extend(
                self.fields
                    .field_option_names
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let attributes = &field.attr;
                        let name = &field.ident;
                        let _type = &self.fields.field_option_types_names[i];

                        quote! {
                           #(#attributes)*
                           pub #name: Option<std::sync::Arc<std::sync::RwLock<#_type>>>
                        }
                    }),
            )
        };
        if !self.fields.field_vec_names.is_empty() {
            fields.extend(
                self.fields
                    .field_vec_names
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let attributes = &field.attr;
                        let name = &field.ident;
                        let _type = &self.fields.field_vec_types_names[i];

                        quote! {
                           #(#attributes)*
                           pub #name: Vec<std::sync::Arc<std::sync::RwLock<#_type>>>
                        }
                    }),
            )
        };
        if !self.fields.field_hashmap_names.is_empty() {
            fields.extend(
                self.fields
                    .field_hashmap_names
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let attributes = &field.attr;
                        let name = &field.ident;
                        let _type = &self.fields.field_hashmap_types_names[i];

                        quote! {
                           #(#attributes)*
                           pub #name: HashMap<String, std::sync::Arc<std::sync::RwLock<#_type>>>
                        }
                    }),
            )
        };
        fields
    }

    fn generate_ast_item_methods(&self) -> TokenStream {
        let ast_item_builder_trait_object = &self.paths.ast_item_builder_trait_object;
        let ast_item_trait_object_arc_path = &self.paths.ast_item_trait_object_arc;
        let ast_item_trait_object_weak_path = &self.paths.ast_item_trait_object_weak;

        let field_names = &self.fields.field_names.get_field_names();
        let field_option_names = &self.fields.field_option_names.get_field_names();
        let field_vec_names = &self.fields.field_vec_names.get_field_names();
        let field_hashmap_names = &self.fields.field_hashmap_names.get_field_names();

        quote! {
            fn get_url(&self) -> std::sync::Arc<lsp_types::Url> {
                self.url.clone()
            }

            fn get_range(&self) -> tree_sitter::Range {
                self.range
            }

            fn get_parent(&self) -> Option<#ast_item_trait_object_weak_path> {
                self.parent.as_ref().map(|p| p.clone())
            }

            fn set_parent(&mut self, parent: #ast_item_trait_object_weak_path) {
                self.parent = Some(parent);
            }

            fn inject_parent(&mut self, parent: #ast_item_trait_object_weak_path) {
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
                #(
                    for field in self.#field_hashmap_names.values() {
                        field.write().unwrap().set_parent(parent.clone());
                    };
                )*
            }

            fn find_at_offset(&self, offset: &usize) -> Option<#ast_item_trait_object_arc_path> {
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
                #(
                    for field in self.#field_hashmap_names.values() {
                        if let true = field.read().unwrap().is_inside_offset(offset) {
                            match field.read().unwrap().find_at_offset(offset) {
                                Some(a) => return Some(a),
                                None => return Some(field.clone())
                            }
                        }
                    }
                )*
                None
            }

            fn swap_at_offset(&mut self, offset: &usize, item: &#ast_item_builder_trait_object) {
                todo!()
            }
        }
    }
}

impl<'a> BuildAstItemBuilder for StructBuilder<'a> {
    fn generate_builder_fields(&self) -> Vec<TokenStream> {
        let ast_item_builder_trait_object = &self.paths.ast_item_builder_trait_object;

        [
            self.fields.field_names.apply_to_fields(|field| {
                quote! { #field: Option<#ast_item_builder_trait_object> }
            }),
            self.fields.field_option_names.apply_to_fields(|field| {
                quote! { #field: Option<#ast_item_builder_trait_object> }
            }),
            self.fields.field_vec_names.apply_to_fields(|field| {
                quote! { #field: Vec<#ast_item_builder_trait_object> }
            }),
            self.fields.field_hashmap_names.apply_to_fields(|field| {
                quote! { #field: HashMap<String, #ast_item_builder_trait_object> }
            }),
        ]
        .concat()
    }

    fn generate_builder_new(&self) -> TokenStream {
        let fields = [
            self.fields.field_names.apply_to_fields(|field| {
                quote_spanned! { field.span() => #field: None }
            }),
            self.fields.field_option_names.apply_to_fields(|field| {
                quote_spanned! { field.span() => #field: None }
            }),
            self.fields.field_vec_names.apply_to_fields(|field| {
                quote_spanned! { field.span() => #field: vec![] }
            }),
            self.fields.field_hashmap_names.apply_to_fields(|field| {
                quote_spanned! { field.span() => #field: std::collections::HashMap::new() }
            }),
        ]
        .concat();

        quote! {
            fn new(url: std::sync::Arc<lsp_types::Url>, _query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Self {
                Self {
                    url,
                    query_index,
                    range,
                    start_position,
                    end_position,
                    #(#fields),*
                }
            }
        }
    }

    fn generate_query_binder(&self) -> TokenStream {
        let ast_item_builder_trait_object = &self.paths.ast_item_builder_trait_object;

        let mut fields_types = vec![];
        fields_types.extend(self.fields.field_types_names.iter());
        fields_types.extend(self.fields.field_option_types_names.iter());
        fields_types.extend(self.fields.field_vec_types_names.iter());
        fields_types.extend(self.fields.field_hashmap_types_names.iter());

        let mut fields_builder = vec![];
        fields_builder.extend(self.fields.field_builder_names.iter());
        fields_builder.extend(self.fields.field_option_builder_names.iter());
        fields_builder.extend(self.fields.field_vec_builder_names.iter());
        fields_builder.extend(self.fields.field_hashmap_builder_names.iter());

        let query_binder = quote! {
            let query_name = query.capture_names()[capture.index as usize];
            #(
                if #fields_types::QUERY_NAMES.contains(&query_name)  {
                        return Some(std::rc::Rc::new(std::cell::RefCell::new(#fields_builder::new(
                            url,
                            &query,
                            capture.index as usize,
                            capture.node.range(),
                            capture.node.start_position(),
                            capture.node.end_position(),
                        ))))
                };
            )*
            None
        };

        quote! {
            fn static_query_binder(url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<#ast_item_builder_trait_object> {
                #query_binder
            }

            fn query_binder(&self, url: std::sync::Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<#ast_item_builder_trait_object> {
                #query_binder
            }
        }
    }

    fn generate_add(&self) -> TokenStream {
        let ast_item_builder_trait_object = &self.paths.ast_item_builder_trait_object;
        let deferred_ast_item_builder = &self.paths.deferred_ast_item_builder;

        let input_builder_name = &self.input_buider_name;
        let field_names = &self.fields.field_names.get_field_names();
        let field_option_names = &self.fields.field_option_names.get_field_names();
        let field_vec_names = &self.fields.field_vec_names.get_field_names();
        let field_hashmap_names = &self.fields.field_hashmap_names.get_field_names();

        let field_types_names = &self.fields.field_types_names;
        let field_vec_types_names = &self.fields.field_vec_types_names;
        let field_option_types_names = &self.fields.field_option_types_names;
        let field_hashmap_types_names = &self.fields.field_hashmap_types_names;

        let field_hashmap_builder_names = &self.fields.field_hashmap_builder_names;

        quote! {
            fn add(&mut self, query: &tree_sitter::Query, node: #ast_item_builder_trait_object, source_code: &[u8]) ->
                Result<#deferred_ast_item_builder, lsp_types::Diagnostic> {

                let query_name = query.capture_names()[node.borrow().get_query_index() as usize];
            #(
                if #field_types_names::QUERY_NAMES.contains(&query_name) {
                    match self.#field_names {
                        Some(_) => return Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Field {:?} is already present in {:?}", stringify!(#field_names), stringify!(#input_builder_name)))),
                        None => self.#field_names = Some(node.clone())
                    }
                    return Ok(#deferred_ast_item_builder::None)
                };
            )*
            #(
                if #field_option_types_names::QUERY_NAMES.contains(&query_name) {
                    if self.#field_option_names.is_some() {
                        return Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Field {:?} is already present in {:?}", stringify!(#field_option_names), stringify!(#input_builder_name))));
                    }
                    self.#field_option_names = Some(node.clone());
                    return Ok(#deferred_ast_item_builder::None);
                };
            )*
            #(
                if #field_vec_types_names::QUERY_NAMES.contains(&query_name) {
                    self.#field_vec_names.push(node.clone());
                    return Ok(#deferred_ast_item_builder::None);
                };
            )*
            #(
                if #field_hashmap_types_names::QUERY_NAMES.contains(&query_name) {
                    return Ok(#deferred_ast_item_builder::HashMap(Box::new(|
                            parent: #ast_item_builder_trait_object,
                            node: #ast_item_builder_trait_object,
                            source_code: &[u8]
                        | {
                            let field = node.borrow();
                            let field = field.downcast_ref::<#field_hashmap_builder_names>().expect("Not a builder!");
                            let key = field.get_key(source_code);

                            let mut parent = parent.borrow_mut();
                            let parent = parent.downcast_mut::<#input_builder_name>().expect("Not the builder!");

                            if parent.#field_hashmap_names.contains_key(key) {
                                return Err(auto_lsp::builder_error!(
                                    field.get_lsp_range(),
                                    format!(
                                        "Field {:?} is already declared in {:?}",
                                        key,
                                        stringify!(#input_builder_name)
                                    )
                                ));
                            };
                            eprintln!("Inserting key {:?} of type {:?} in {}", key, stringify!(#field_hashmap_builder_names), stringify!(#input_builder_name));
                            parent.#field_hashmap_names.insert(key.into(), node.clone());
                            Ok(())
                    })));
                };
            )*
            Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Invalid field {:?} in {:?}", query_name, stringify!(#input_builder_name))))
            }
        }
    }

    fn generate_try_from(&self) -> TokenStream {
        let fields = self.fields.get_field_names();

        let name = self.input_name;
        let input_builder_name = &self.input_buider_name;
        let field_names = &self.fields.field_names.get_field_names();
        let field_option_names = &self.fields.field_option_names.get_field_names();
        let field_vec_names = &self.fields.field_vec_names.get_field_names();
        let field_hashmap_names = &self.fields.field_hashmap_names.get_field_names();

        let field_builder_names = &self.fields.field_builder_names;
        let field_vec_builder_names = &self.fields.field_vec_builder_names;
        let field_option_builder_names = &self.fields.field_option_builder_names;
        let field_hashmap_builder_names = &self.fields.field_hashmap_builder_names;

        let ast_item_object_arc = &self.paths.ast_item_trait_object_arc;
        let try_from_builder = &self.paths.try_from_builder;
        let try_into_builder = &self.paths.try_into_builder;

        let init_accessor = if self.is_accessor {
            quote! { accessor: None, }
        } else {
            quote! {}
        };

        quote! {
            impl #try_from_builder<&#input_builder_name> for #name {
                type Error = lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, check: &mut Vec<#ast_item_object_arc>) -> Result<Self, Self::Error> {
                    use #try_from_builder;
                    use #try_into_builder;

                    let builder_range = builder.get_lsp_range();

                    #(let #field_names =
                        builder
                        .#field_names
                        .as_ref()
                        .ok_or(auto_lsp::builder_error!(builder_range, format!("Missing field {:?} in {:?}", stringify!(#field_names), stringify!(#input_builder_name))))?
                        .borrow()
                        .downcast_ref::<#field_builder_names>()
                        .ok_or(auto_lsp::builder_error!(builder_range, format!("Failed downcast conversion of {:?}", stringify!(#field_builder_names))))?
                        .try_into_builder(check)?;
                    )*
                    #(let #field_option_names = match builder.#field_option_names {
                            Some(builder) => {
                                let item = builder
                                    .borrow()
                                    .downcast_ref::<#field_option_builder_names>()
                                    .ok_or(auto_lsp::builder_error!(builder_range, format!("Failed downcast conversion of {:?}", stringify!(#field_option_builder_names))))?
                                    .try_into_builder(check)?;
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
                                .as_ref()
                                .borrow()
                                .downcast_ref::<#field_vec_builder_names>()
                                .ok_or(auto_lsp::builder_error!(builder_range, format!("Failed downcast conversion of {:?}", stringify!(#field_vec_builder_names))))?
                                .try_into_builder(check)?;
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
                                    .as_ref()
                                    .borrow()
                                    .downcast_ref::<#field_hashmap_builder_names>()
                                    .ok_or(auto_lsp::builder_error!(builder_range, format!("Failed downcast conversion of {:?} at key {}", stringify!(#field_hashmap_builder_names), key)))?
                                    .try_into_builder(check)?;
                                Ok((key, item))
                            })
                            .collect::<Result<HashMap<String, _>, lsp_types::Diagnostic>>()?;
                    )*
                    Ok(#name {
                        #init_accessor
                        url: builder.url.clone(),
                        range: builder.range.clone(),
                        start_position: builder.start_position.clone(),
                        end_position: builder.end_position.clone(),
                        parent: None,
                        #(#fields),*
                    })
                }
            }

            impl #try_from_builder<&#input_builder_name> for std::sync::Arc<std::sync::RwLock<#name>> {
                type Error = lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, check: &mut Vec<#ast_item_object_arc>) -> Result<Self, Self::Error> {
                    use #try_from_builder;
                    use #try_into_builder;

                    let item = #name::try_from_builder(builder, check)?;
                    let result = std::sync::Arc::new(std::sync::RwLock::new(item));
                    result.write().unwrap().inject_parent(std::sync::Arc::downgrade(&result) as _);
                    Ok(result)
                }
            }
        }
    }
}
