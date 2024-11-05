extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::Path;

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    CodeGen, Features,
};

#[derive(Debug, FromMeta)]
pub struct SemanticTokenFeature {
    token_types: Path,
    token_type_index: String,
    range: Path,
    modifiers_fn: Option<Path>,
}

pub fn generate_semantic_token_feature(
    features: &Features,
    code_gen: &mut CodeGen,
    input: &StructFields,
) {
    if let Some(semantic) = &features.lsp_semantic_token {
        codegen_semantic_token(&semantic, code_gen, input);
    }
}

fn codegen_semantic_token(
    features: &SemanticTokenFeature,
    code_gen: &mut CodeGen,
    input: &StructFields,
) {
    let token_types = &features.token_types;
    let token_index = &features.token_type_index;
    let range = path_to_dot_tokens(&features.range, Some(quote! { read().unwrap() }));

    let field_names = &input.field_names;
    let field_vec_names = &input.field_vec_names;
    let field_option_names = &input.field_option_names;
    let field_hashmap_names = &input.field_hashmap_names;

    let modifiers = match &features.modifiers_fn {
        None => quote! { 0 },
        Some(path) => {
            let path = path_to_dot_tokens(path, None);
            quote! {
                #path().iter().fold(0, |bitset, modifier| bitset | (1 << (*modifier)))
            }
        }
    };

    code_gen.impl_ast_item.push(
        quote! {
            fn build_semantic_tokens(&self, builder: &mut auto_lsp::builders::semantic_tokens::SemanticTokensBuilder) {
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
    );
}
