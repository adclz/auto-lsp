#![allow(unused)]
use crate::{
    utilities::extract_fields::StructFields, Features, ReferenceOrSymbolFeatures, StructHelpers,
    PATHS,
};
use darling::{ast, util};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Path};

pub struct StructBuilder<'a> {
    // Input data
    pub input_attr: &'a Vec<Attribute>,
    pub input_name: &'a Ident,
    pub query_name: &'a str,
    pub input_builder_name: &'a Ident,
    pub fields: &'a StructFields,
    // Features
    pub features: Features<'a>,
}

impl<'a> StructBuilder<'a> {
    pub fn new(
        params: &'a ReferenceOrSymbolFeatures<'a>,
        helpers: &'a ast::Data<util::Ignored, StructHelpers>,
        input_attr: &'a Vec<Attribute>,
        input_name: &'a Ident,
        input_buider_name: &'a Ident,
        query_name: &'a str,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            input_name,
            input_attr,
            query_name,
            input_builder_name: input_buider_name,
            fields,
            features: Features::new(&params, &helpers, &input_name, &fields),
        }
    }
}

impl<'a> ToTokens for StructBuilder<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // generate ast item

        let mut builder = FieldBuilder::default();

        self.struct_input(&mut builder);

        self.impl_ast_symbol(&mut builder);
        self.impl_locator(&mut builder);
        self.impl_parent(&mut builder);
        self.impl_dynamic_swap(&mut builder);
        self.impl_edit_range(&mut builder);
        self.impl_collect_references(&mut builder);

        builder.add(self.features.to_token_stream());
        builder.stage();

        // Generate builder

        self.struct_input_builder(&mut builder);

        builder.add(quote! {
            fn get_url(&self) -> std::sync::Arc<auto_lsp::lsp_types::Url> {
                self.url.clone()
            }

            fn get_range(&self) -> std::ops::Range<usize>{
                self.range.clone()
            }

            fn get_query_index(&self) -> usize {
                self.query_index
            }
        });
        self.fn_new(&mut builder);
        self.fn_add(&mut builder);
        builder.stage_trait(&self.input_builder_name, &PATHS.symbol_builder_trait.path);

        self.impl_try_from(&mut builder);

        self.impl_queryable(&mut builder);

        tokens.extend(builder.to_token_stream());
    }
}

impl<'a> StructBuilder<'a> {
    fn struct_input(&self, builder: &mut FieldBuilder) {
        let symbol = &PATHS.symbol;
        let symbol_data = &PATHS.symbol_data;

        builder
            .add(quote! { pub _data: #symbol_data })
            .add_iter(&self.fields, |ty, _, name, field_type, _| match ty {
                FieldType::Normal => quote! {
                    pub #name: #symbol<#field_type>
                },
                FieldType::Vec => quote! {
                    pub #name: Vec<#symbol<#field_type>>
                },
                FieldType::Option => quote! {
                    pub #name: Option<#symbol<#field_type>>
                },
            })
            .stage_struct(&self.input_name);
    }

    fn impl_ast_symbol(&self, builder: &mut FieldBuilder) {
        let get_data = &PATHS.symbol_trait.methods.get_data.sig;
        let get_mut_data = &PATHS.symbol_trait.methods.get_mut_data.sig;

        builder
            .add(quote! { #get_data { &self._data } })
            .add(quote! { #get_mut_data { &mut self._data } })
            .stage_trait(&self.input_name, &PATHS.symbol_trait.path);
    }

    fn impl_locator(&self, builder: &mut FieldBuilder) {
        let symbol_trait = &PATHS.symbol_trait.path;
        builder
            .add_fn_iter(
                &self.fields,
                &PATHS.locator.methods.find_at_offset.sig,
                Some(quote! {
                    use #symbol_trait;
                    if (!self.is_inside_offset(offset)) {
                        return None;
                    }
                }),
                |_, _, name, _, _| {
                    quote! {
                        if let Some(symbol) = self.#name.find_at_offset(offset) {
                           return Some(symbol);
                        }
                    }
                },
                Some(quote! { None }),
            )
            .stage_trait(&self.input_name, &PATHS.locator.path);
    }

    fn impl_parent(&self, builder: &mut FieldBuilder) {
        builder
            .add_fn_iter(
                &self.fields,
                &PATHS.parent.methods.inject_parent.sig,
                None,
                |_, _, name, _, _| {
                    quote! {
                        self.#name.inject_parent(parent.clone());
                    }
                },
                None,
            )
            .stage_trait(&self.input_name, &PATHS.parent.path);
    }

    fn impl_queryable(&self, builder: &mut FieldBuilder) {
        let queryable = &PATHS.queryable.path;
        let query_name = self.query_name;

        builder
            .add(quote! { const QUERY_NAMES: &'static [&'static str] = &[#query_name]; })
            .stage_trait(&self.input_name, queryable);

        builder
            .add(quote! { const QUERY_NAMES: &'static [&'static str] = &[#query_name]; })
            .stage_trait(&self.input_builder_name, queryable);

        let names = self
            .fields
            .get_field_names()
            .iter()
            .map(|name| quote! { stringify!(#name) })
            .collect::<Vec<_>>();

        #[cfg(feature = "assertions")]
        {
        let check_queryable = &PATHS.check_queryable.path;

        let names = quote! { &[#(#names),*] };

        let concat = self
            .fields
            .get_field_builder_names()
            .iter()
            .map(|name| quote! { #name::QUERY_NAMES })
            .collect::<Vec<_>>();

        let input_name = self.input_name;
        let check_conflicts = &PATHS.check_conflicts;

        builder
            .add(quote! { const CHECK: () = {
                use #queryable;
                use #check_queryable;
                let queries = auto_lsp::constcat::concat_slices!([&str]: #(#concat),*);
                #check_conflicts(stringify!(#input_name), #names, queries);
            }; })
            .stage_trait(&self.input_name, check_queryable);
        
            builder
                .add(quote! { const _: () = <#input_name as  #check_queryable>::CHECK; })
                .stage();
        }
    }

    fn impl_dynamic_swap(&self, builder: &mut FieldBuilder) {
        builder
            .add_fn_iter(
                &self.fields,
                &PATHS.dynamic_swap.methods.swap.sig,
                None,
                |_, _, name, _, _| {
                    quote! {
                        self.#name.to_swap(start, offset, builder_params)?;
                    }
                },
                Some(quote! { std::ops::ControlFlow::Continue(()) }),
            )
            .stage_trait(&self.input_name, &PATHS.dynamic_swap.path);
    }

    fn impl_edit_range(&self, builder: &mut FieldBuilder) {
        builder
            .add_fn_iter(
                &self.fields,
                &PATHS.edit_range.methods.edit_range.sig,
                None,
                |_, _, name, _, _| {
                    quote! {
                        self.#name.edit_range(start, offset);
                    }
                },
                None,
            )
            .stage_trait(&self.input_name, &PATHS.edit_range.path);
    }

    fn impl_collect_references(&self, builder: &mut FieldBuilder) {
        builder
            .add_fn_iter(
                &self.fields,
                &PATHS.collect_references.methods.collect_references.sig,
                None,
                |_, _, name, _, _| {
                    quote! {
                        self.#name.collect_references(builder_params);
                    }
                },
                None,
            )
            .stage_trait(&self.input_name, &PATHS.collect_references.path);
    }

    fn struct_input_builder(&self, builder: &mut FieldBuilder) {
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;
        let pending_symbol = &PATHS.pending_symbol;

        builder
            .add(quote! { url: std::sync::Arc<auto_lsp::lsp_types::Url> })
            .add(quote! { query_index: usize })
            .add(quote! { range: std::ops::Range<usize> })
            .add_iter(&self.fields, |ty, _, name, _, _| match ty {
                FieldType::Vec => quote! { #name: Vec<#pending_symbol> },
                _ => quote! { #name: #maybe_pending_symbol },
            })
            .stage_struct(&self.input_builder_name)
            .to_token_stream();
    }

    fn fn_new(&self, builder: &mut FieldBuilder) {
        let maybe_pending_symbol = &PATHS.maybe_pending_symbol;
        let sig = &PATHS.symbol_builder_trait.methods.new.sig;

        let fields = FieldBuilder::default()
            .add_iter(&self.fields, |ty, _, name, _, _| match ty {
                FieldType::Vec => quote! { #name: vec![] },
                _ => quote! { #name: #maybe_pending_symbol::none() },
            })
            .stage_fields()
            .to_token_stream();

        builder.add(quote! {
          #sig {
            let range = capture.node.range();
            Some(Self {
                url,
                query_index: capture.index as usize,
                range: std::ops::Range {
                    start: range.start_byte,
                    end: range.end_byte,
                },
                #fields
            })
          }
        });
    }

    fn fn_add(&self, builder: &mut FieldBuilder) {
        let input_name = &self.input_name;
        let add_symbol_trait = &PATHS.add_symbol_trait;
        builder.add_fn_iter(
            &self.fields,
            &PATHS.symbol_builder_trait.methods.add.sig,
            Some(quote! { use #add_symbol_trait; }),
            |_, _, name, field_type, builder| {
                quote! {
                    
                    if let Some(node) =  self.#name.add::<#builder>(capture, params, stringify!(#input_name), stringify!(#field_type))? {
                       return Ok(Some(node))
                    };
                }
            },
            Some(quote! { Ok(None) }),
        );
    }

    fn impl_try_from(&self, builder: &mut FieldBuilder) {
        let fields = self.fields.get_field_names();

        let input_name = self.input_name;
        let input_builder_name = &self.input_builder_name;

        let try_from_builder = &PATHS.try_from_builder;

        let symbol_data = &PATHS.symbol_data;
        let builder_params = &PATHS.builder_params;
        let try_downcast = &PATHS.try_downcast_trait;
        let finalize = &PATHS.finalize_trait;

        let _builder = FieldBuilder::default()
            .add(quote! {
                use #try_downcast;
                use #finalize;
            })
            .add_iter(&self.fields,
                |ty, _, name, field_type, _| match ty  {
                FieldType::Normal  => quote! {
                    let #name = Symbol::new_and_check(builder
                        .#name
                        .as_ref()
                        .ok_or(auto_lsp::core::builder_error!(
                            auto_lsp,
                            builder_range,
                            format!(
                                "Invalid {:?} for {:?}",
                                stringify!(#name), stringify!(#input_name)
                            )
                        ))?
                        .try_downcast(params, stringify!(#field_type), builder_range, stringify!(#input_name))?, params);
                },
                _=> quote! {
                        let #name = builder
                            .#name
                            .try_downcast(params, stringify!(#field_type), builder_range, stringify!(#input_name))?.finalize(params);
                    }
            })
            .stage()
            .to_token_stream();

        let builder_trait = &PATHS.symbol_builder_trait.path;

        builder.add(quote! {
            impl #try_from_builder<&#input_builder_name> for #input_name {
                type Error = auto_lsp::lsp_types::Diagnostic;

                fn try_from_builder(builder: &#input_builder_name, params: &mut #builder_params) -> Result<Self, Self::Error> {
                    use #builder_trait;
                    let builder_range = builder.get_lsp_range(params.document);

                    #_builder

                    Ok(#input_name {
                        _data: #symbol_data::new(builder.url.clone(), builder.range.clone()),
                        #(#fields),*
                    })
                }
            }
        });
        builder.stage();
    }
}

#[derive(Default)]
pub struct FieldBuilder {
    staged: Vec<TokenStream>,
    unstaged: Vec<TokenStream>,
}

pub enum FieldType {
    Normal,
    Vec,
    Option,
}

impl FieldBuilder {
    pub fn add(&mut self, field: TokenStream) -> &mut Self {
        self.unstaged.push(field);
        self
    }

    pub fn add_iter<F>(&mut self, fields: &StructFields, f: F) -> &mut Self
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        let mut _fields: Vec<TokenStream> = vec![];

        if !fields.field_names.is_empty() {
            _fields.extend::<Vec<TokenStream>>(self.apply(&fields, &f) as _);
        }
        if !fields.field_option_names.is_empty() {
            _fields.extend::<Vec<TokenStream>>(self.apply_opt(&fields, &f) as _);
        }
        if !fields.field_vec_names.is_empty() {
            _fields.extend::<Vec<TokenStream>>(self.apply_vec(&fields, &f) as _);
        }
        self.unstaged.extend(_fields);
        self
    }

    pub fn add_fn_iter<F>(
        &mut self,
        fields: &StructFields,
        sig_path: &TokenStream,
        before: Option<TokenStream>,
        body: F,
        after: Option<TokenStream>,
    ) -> &mut Self
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        let mut _body: Vec<TokenStream> = vec![];
        if !fields.field_names.is_empty() {
            _body.extend::<Vec<TokenStream>>(self.apply(&fields, &body) as _);
        }
        if !fields.field_option_names.is_empty() {
            _body.extend::<Vec<TokenStream>>(self.apply_opt(&fields, &body) as _);
        }
        if !fields.field_vec_names.is_empty() {
            _body.extend::<Vec<TokenStream>>(self.apply_vec(&fields, &body) as _);
        }

        let mut result = TokenStream::default();
        if let Some(before) = before {
            result.extend(before);
        }

        result.extend(_body);

        if let Some(after) = after {
            result.extend(after);
        }

        self.unstaged.push(quote! {
            #sig_path {
                #result
            }
        });
        self
    }

    fn drain(&mut self) -> Vec<TokenStream> {
        std::mem::take(&mut self.unstaged)
    }

    pub fn stage(&mut self) -> &mut Self {
        let drain = self.drain();
        self.staged.extend(drain);
        self
    }

    pub fn stage_fields(&mut self) -> &mut Self {
        let fields = self.drain();
        self.staged.push(quote! { #(#fields,)* });
        self
    }

    pub fn stage_trait(&mut self, input_name: &Ident, trait_path: &Path) -> &mut Self {
        let drain = self.drain();
        let result = quote! {
            impl #trait_path for #input_name {
                #(#drain)*
            }
        };
        self.staged.push(result);
        self
    }

    pub fn stage_struct(&mut self, input_name: &Ident) -> &mut Self {
        let drain = self.drain();
        let result = quote! {
            #[derive(Clone)]
            pub struct #input_name {
                #(#drain,)*
            }
        };
        self.staged.push(result);
        self
    }

    fn apply<F>(&mut self, fields: &StructFields, f: F) -> Vec<TokenStream>
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        fields
            .field_names
            .iter()
            .zip(fields.field_types_names.iter())
            .zip(fields.field_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldType::Normal,
                    &field.attr,
                    &field.ident,
                    &field_type,
                    &field_builder,
                )
            })
            .collect::<Vec<_>>()
    }

    fn apply_opt<F>(&mut self, fields: &StructFields, f: F) -> Vec<TokenStream>
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        fields
            .field_option_names
            .iter()
            .zip(fields.field_option_types_names.iter())
            .zip(fields.field_option_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldType::Option,
                    &field.attr,
                    &field.ident,
                    &field_type,
                    &field_builder,
                )
            })
            .collect::<Vec<_>>()
    }

    fn apply_vec<F>(&mut self, fields: &StructFields, f: F) -> Vec<TokenStream>
    where
        F: Fn(FieldType, &Vec<Attribute>, &Ident, &Ident, &Ident) -> TokenStream,
    {
        fields
            .field_vec_names
            .iter()
            .zip(fields.field_vec_types_names.iter())
            .zip(fields.field_vec_builder_names.iter())
            .map(|((field, field_type), field_builder)| {
                f(
                    FieldType::Vec,
                    &field.attr,
                    &field.ident,
                    &field_type,
                    &field_builder,
                )
            })
            .collect::<Vec<_>>()
    }
}

impl ToTokens for FieldBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.staged.clone());
    }
}

impl From<FieldBuilder> for Vec<TokenStream> {
    fn from(builder: FieldBuilder) -> Self {
        builder.staged
    }
}
