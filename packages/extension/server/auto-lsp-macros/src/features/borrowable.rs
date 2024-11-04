extern crate proc_macro;

use darling::{util::PathList, FromMeta};
use quote::quote;
use syn::Path;

use crate::{utilities::format_tokens::path_to_dot_tokens, Features, FeaturesCodeGen};

#[derive(Debug, FromMeta)]
pub enum BorrowableFeature {
    WhiteList(PathList),
    BlackList(PathList),
    BorrowableFn(Path),
}

pub fn generate_borrowable_feature(
    features: &Features,
    code_gen_impl: &mut Vec<proc_macro2::TokenStream>,
    code_gen_impl_ast_item: &mut Vec<proc_macro2::TokenStream>,
) {
    if let Some(borrowable) = &features.borrowable {
        let code_gen = codegen_borrowable_feature(&borrowable);
        code_gen_impl_ast_item.push(code_gen.impl_ast_item.unwrap())
    }
}

fn codegen_borrowable_feature(feature: &BorrowableFeature) -> FeaturesCodeGen {
    match feature {
        BorrowableFeature::WhiteList(white_list) => {
            let white_list = white_list.iter().map(|path| path_to_dot_tokens(path, None));

            FeaturesCodeGen {
                fields: None,
                impl_base: None,
                impl_ast_item: quote! {
                    fn is_borrowable(&self, other: &dyn AstItem) -> bool {
                        #(
                            if other.is::<#white_list>() {
                                return true;
                            };
                        )*
                        false
                    }
                }
                .into(),
            }
        }
        BorrowableFeature::BlackList(black_list) => {
            let black_list = black_list.iter().map(|path| path_to_dot_tokens(path, None));

            FeaturesCodeGen {
                fields: None,
                impl_base: None,
                impl_ast_item: quote! {
                    fn is_borrowable(&self, other: &dyn AstItem) -> bool {
                        #(
                            if other.is::<#black_list>() {
                                return false;
                            };
                        )*
                        true
                    }
                }
                .into(),
            }
        }
        BorrowableFeature::BorrowableFn(borrowable_fn) => {
            let borrowable_fn = path_to_dot_tokens(borrowable_fn, None);
            FeaturesCodeGen {
                fields: None,
                impl_base: None,
                impl_ast_item: quote! {
                    fn is_borrowable(&self, other: &dyn AstItem) -> bool {
                        #borrowable_fn(other)
                    }
                }
                .into(),
            }
        }
    }
}
