extern crate proc_macro;

use auto_lsp::builders::semantic_tokens;
use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{
        extract_fields::{FieldInfoExtract, StructFields},
        format_tokens::path_to_dot_tokens,
    },
    CodeGen, Paths, ToCodeGen,
};

use crate::Feature;

#[derive(Debug, FromMeta)]
pub struct SemanticTokenFeature {
    token_types: Path,
    token_type_index: String,
    range: Path,
    modifiers_fn: Option<Path>,
}

pub struct SemanticTokensBuilder<'a> {
    pub input_name: &'a Ident,
    pub paths: &'a Paths,
    pub params: Option<&'a Feature<SemanticTokenFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> SemanticTokensBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        paths: &'a Paths,
        params: Option<&'a Feature<SemanticTokenFeature>>,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            paths,
            input_name,
            params,
            fields,
        }
    }
}

impl<'a> ToCodeGen for SemanticTokensBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut CodeGen) {
        let input_name = &self.input_name;
        let semantic_tokens_path = &self.paths.semantic_tokens_trait;
        let semantic_tokens_builder_path = &self.paths.semantic_tokens_builder;

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #semantic_tokens_path for #input_name {
                    fn build_semantic_tokens(&self, _builder: &mut #semantic_tokens_builder_path) {}
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(semantic) => {
                    let token_types = &semantic.token_types;
                    let token_index = &semantic.token_type_index;
                    let range =
                        path_to_dot_tokens(&semantic.range, Some(quote! { read().unwrap() }));

                    let field_names = &self.fields.field_names.get_field_names();
                    let field_vec_names = &self.fields.field_vec_names.get_field_names();
                    let field_option_names = &self.fields.field_option_names.get_field_names();
                    let field_hashmap_names = &self.fields.field_hashmap_names.get_field_names();

                    let modifiers = match &semantic.modifiers_fn {
                        None => quote! { 0 },
                        Some(path) => {
                            let path = path_to_dot_tokens(path, None);
                            quote! {
                                #path().iter().fold(0, |bitset, modifier| bitset | (1 << (*modifier)))
                            }
                        }
                    };

                    codegen.input.other_impl.push(
                        quote! {
                            impl #semantic_tokens_path for #input_name {
                                fn build_semantic_tokens(&self, builder: &mut #semantic_tokens_builder_path) {
                                    let range = #range.get_range();
                                    match #token_types.get_index(#token_index) {
                                        Some(index) => builder.push(
                                            lsp_types::Range::new(
                                                lsp_types::Position::new(
                                                    range.start_point.row as u32,
                                                    range.start_point.column as u32,
                                                ),
                                                lsp_types::Position::new(range.end_point.row as u32, range.end_point.column as u32),
                                            ),
                                            index as u32,
                                            #modifiers,
                                        ),
                                        None => {
                                            eprintln!("Warning: Token type not found {:?}", #token_index);
                                            return
                                        },
                                    }
                                    #(
                                        self.#field_names.read().unwrap().build_semantic_tokens(builder);
                                    )*
                                    #(
                                        if let Some(field) = self.#field_option_names.as_ref() {
                                            field.read().unwrap().build_semantic_tokens(builder);
                                        };
                                    )*
                                    #(
                                        for field in self.#field_vec_names.iter() {
                                            field.read().unwrap().build_semantic_tokens(builder);
                                        };
                                    )*
                                    #(
                                        for field in self.#field_hashmap_names.values() {
                                            field.read().unwrap().build_semantic_tokens(builder);
                                        };
                                    )*
                                }
                            }
                        }
                    );
                }
            },
        }
    }
}
