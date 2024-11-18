use crate::{
    utilities::extract_fields::{FieldInfoExtract, StructFields},
    CodeGen, Paths, ToCodeGen,
};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
pub struct AstItemBuilder<'a> {
    pub paths: &'a Paths,
    pub fields: &'a StructFields,
    pub query_name: &'a str,
    pub input_name: &'a Ident,
    pub input_builder_name: Ident,
}

impl<'a> AstItemBuilder<'a> {
    pub fn new(
        paths: &'a Paths,
        query_name: &'a str,
        input_name: &'a Ident,
        input_builder_name: Ident,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            paths,
            query_name,
            fields,
            input_name,
            input_builder_name,
        }
    }
}

impl<'a> ToCodeGen for AstItemBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut CodeGen) {
        // Symbol
        codegen.input.fields.extend(self.generate_struct_fields());

        let query_name = self.query_name;

        codegen.input.impl_base.push(quote! {
            pub const QUERY_NAMES: &[&str] = &[#query_name];
        });

        codegen
            .input
            .impl_ast_item
            .push(self.generate_ast_item_methods());

        // Builder
        let input_builder_name = &self.input_builder_name;
        let ast_item_builder = &self.paths.ast_item_builder_trait_path;

        let builder_fields = self.generate_builder_fields();
        let new = self.generate_builder_new();
        let query_binder = self.generate_query_binder();
        let add = self.generate_add();
        let try_from = self.generate_try_from();

        codegen.new_structs.push(quote! {
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

                fn get_url(&self) -> Arc<lsp_types::Url> {
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

impl<'a> AstItemBuilder<'a> {
    fn generate_struct_fields(&self) -> Vec<TokenStream> {
        let mut fields = vec![
            quote! { pub url: Arc<lsp_types::Url> },
            quote! { pub parent: Option<Weak<RwLock<dyn AstItem>>> },
            quote! { pub range: tree_sitter::Range },
            quote! { pub start_position: tree_sitter::Point },
            quote! { pub end_position: tree_sitter::Point },
        ];
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
                           pub #name: Arc<RwLock<#_type>>
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
                           pub #name: Option<Arc<RwLock<#_type>>>
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
                           pub #name: Vec<Arc<RwLock<#_type>>>
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
                           pub #name: HashMap<String, Arc<RwLock<#_type>>>
                        }
                    }),
            )
        };
        fields
    }

    fn generate_ast_item_methods(&self) -> TokenStream {
        let field_names = &self.fields.field_names.get_field_names();
        let field_option_names = &self.fields.field_option_names.get_field_names();
        let field_vec_names = &self.fields.field_vec_names.get_field_names();
        let field_hashmap_names = &self.fields.field_hashmap_names.get_field_names();

        quote! {
            fn get_url(&self) -> Arc<lsp_types::Url> {
                self.url.clone()
            }

            fn get_range(&self) -> tree_sitter::Range {
                self.range
            }

            fn get_parent(&self) -> Option<Weak<RwLock<dyn AstItem>>> {
                self.parent.as_ref().map(|p| p.clone())
            }

            fn set_parent(&mut self, parent: Weak<RwLock<dyn AstItem>>) {
                self.parent = Some(parent);
            }

            fn inject_parent(&mut self, parent: Weak<RwLock<dyn AstItem>>) {
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

            fn swap_at_offset(&mut self, offset: &usize, item: &std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>) {
                todo!()
            }
        }
    }
}

impl<'a> AstItemBuilder<'a> {
    fn generate_builder_fields(&self) -> Vec<TokenStream> {
        let ast_item_builder = &self.paths.ast_item_builder_trait_path;

        [
            self.fields.field_names.apply_to_fields(|field| {
                quote! { #field: Option<std::rc::Rc<std::cell::RefCell<dyn #ast_item_builder>>> }
            }),
            self.fields.field_option_names.apply_to_fields(|field| {
                quote! { #field: Option<std::rc::Rc<std::cell::RefCell<dyn #ast_item_builder>>> }
            }),
            self.fields.field_vec_names.apply_to_fields(|field| {
                quote! { #field: Vec<std::rc::Rc<std::cell::RefCell<dyn #ast_item_builder>>> }
            }),
            self.fields.field_hashmap_names.apply_to_fields(|field| {
                quote! { #field: HashMap<String, std::rc::Rc<std::cell::RefCell<dyn #ast_item_builder>>> }
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
                quote_spanned! { field.span() => #field: HashMap::new() }
            }),
        ]
        .concat();

        quote! {
            fn new(url: Arc<lsp_types::Url>, _query: &tree_sitter::Query, query_index: usize, range: tree_sitter::Range, start_position: tree_sitter::Point, end_position: tree_sitter::Point) -> Self {
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

        quote! {
            fn query_binder(&self, url: Arc<lsp_types::Url>, capture: &tree_sitter::QueryCapture, query: &tree_sitter::Query) -> Option<std::rc::Rc<std::cell::RefCell<dyn auto_lsp::traits::ast_item_builder::AstItemBuilder>>> {
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
            }
        }
    }

    fn generate_add(&self) -> TokenStream {
        let input_builder_name = &self.input_builder_name;
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
            fn add(&mut self, query: &tree_sitter::Query, node: std::rc::Rc<std::cell::RefCell<dyn AstItemBuilder>>, source_code: &[u8]) ->
            Result<auto_lsp::traits::ast_item_builder::DeferredAstItemBuilder, lsp_types::Diagnostic> {
            use auto_lsp::traits::ast_item_builder::DeferredAstItemBuilder;
            let query_name = query.capture_names()[node.borrow().get_query_index() as usize];
            #(
                if #field_types_names::QUERY_NAMES.contains(&query_name) {
                    match self.#field_names {
                        Some(_) => return Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Field {:?} is already present in {:?}", stringify!(#field_names), stringify!(#input_builder_name)))),
                        None => self.#field_names = Some(node.clone())
                    }
                    return Ok(DeferredAstItemBuilder::None)
                };
            )*
            #(
                if #field_option_types_names::QUERY_NAMES.contains(&query_name) {
                    if self.#field_option_names.is_some() {
                        return Err(auto_lsp::builder_error!(self.get_lsp_range(), format!("Field {:?} is already present in {:?}", stringify!(#field_option_names), stringify!(#input_builder_name))));
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
        let input_builder_name = &self.input_builder_name;
        let field_names = &self.fields.field_names.get_field_names();
        let field_option_names = &self.fields.field_option_names.get_field_names();
        let field_vec_names = &self.fields.field_vec_names.get_field_names();
        let field_hashmap_names = &self.fields.field_hashmap_names.get_field_names();

        let field_builder_names = &self.fields.field_builder_names;
        let field_vec_builder_names = &self.fields.field_vec_builder_names;
        let field_option_builder_names = &self.fields.field_option_builder_names;
        let field_hashmap_builder_names = &self.fields.field_hashmap_builder_names;

        quote! {
            impl auto_lsp::traits::convert::TryFromCtx<#input_builder_name> for #name {
                type Error = lsp_types::Diagnostic;

                fn try_from_ctx(builder: #input_builder_name, ctx: &dyn auto_lsp::traits::workspace::WorkspaceContext) -> Result<Self, Self::Error> {
                    use std::sync::{Arc, RwLock};
                    let builder_range = builder.get_lsp_range();

                    #(let #field_names =
                        builder
                        .#field_names
                        .ok_or(auto_lsp::builder_error!(builder_range, format!("Missing field {:?} in {:?}", stringify!(#field_names), stringify!(#input_builder_name))))?
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
                        url: builder.url,
                        range: builder.range,
                        start_position: builder.start_position,
                        end_position: builder.end_position,
                        parent: None,
                        #(#fields),*
                    })
                }
            }

            impl auto_lsp::traits::convert::TryFromCtx<#input_builder_name> for std::sync::Arc<std::sync::RwLock<#name>> {
                type Error = lsp_types::Diagnostic;

                fn try_from_ctx(builder: #input_builder_name, ctx: &dyn auto_lsp::traits::workspace::WorkspaceContext) -> Result<Self, Self::Error> {
                    let item = #name::try_from_ctx(builder, ctx)?;
                    let result = std::sync::Arc::new(std::sync::RwLock::new(item));
                    result.write().unwrap().inject_parent(std::sync::Arc::downgrade(&result) as _);
                    Ok(result)
                }
            }
        }
    }
}
