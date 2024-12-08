extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{
        extract_fields::{FieldInfoExtract, StructFields},
        format_tokens::path_to_dot_tokens,
    },
    FeaturesCodeGen, Paths, ToCodeGen, PATHS,
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
    pub params: Option<&'a Feature<SemanticTokenFeature>>,
    pub fields: &'a StructFields,
    pub is_accessor: bool,
}

impl<'a> SemanticTokensBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        params: Option<&'a Feature<SemanticTokenFeature>>,
        fields: &'a StructFields,
        is_accessor: bool,
    ) -> Self {
        Self {
            input_name,
            params,
            fields,
            is_accessor,
        }
    }
}

impl<'a> ToCodeGen for SemanticTokensBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let semantic_tokens_path = &PATHS.lsp_semantic_token.path;
        let semantic_tokens_builder_path = &PATHS.semantic_tokens_builder;
        let sig = &PATHS.lsp_semantic_token.methods.build_semantic_tokens.sig;
        let default = &PATHS
            .lsp_semantic_token
            .methods
            .build_semantic_tokens
            .default;

        if self.is_accessor {
            codegen.input.other_impl.push(quote! {
                impl #semantic_tokens_path for #input_name {
                    #sig {
                        if let Some(accessor) = &self.accessor {
                            if let Some(accessor) = accessor.to_dyn() {
                                accessor.read().build_semantic_tokens(builder)
                            }
                        }
                    }
                }
            });
            return;
        }

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #semantic_tokens_path for #input_name {
                    #sig { #default }
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(semantic) => {
                    let token_types = &semantic.token_types;
                    let token_index = &semantic.token_type_index;
                    let range = path_to_dot_tokens(&semantic.range, Some(quote! { read() }));

                    let field_names = &self.fields.field_names.get_field_names();
                    let field_vec_names = &self.fields.field_vec_names.get_field_names();
                    let field_option_names = &self.fields.field_option_names.get_field_names();

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
                                #sig {
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
                                        self.#field_names.read().build_semantic_tokens(builder);
                                    )*
                                    #(
                                        if let Some(field) = self.#field_option_names.as_ref() {
                                            field.read().build_semantic_tokens(builder);
                                        };
                                    )*
                                    #(
                                        for field in self.#field_vec_names.iter() {
                                            field.read().build_semantic_tokens(builder);
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
