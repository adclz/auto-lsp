extern crate proc_macro;

use darling::{util::PathList, FromMeta};
use quote::quote;
use syn::Path;

use crate::{utilities::format_tokens::path_to_dot_tokens, CodeGen, AstStructFeatures};

#[derive(Debug, FromMeta)]
pub enum BorrowableFeature {
    WhiteList(PathList),
    BlackList(PathList),
    BorrowableFn(Path),
}

pub fn generate_borrowable_feature(features: &AstStructFeatures, code_gen: &mut CodeGen) {
    if let Some(borrowable) = &features.borrowable {
        codegen_borrowable_feature(&borrowable, code_gen);
    }
}

fn codegen_borrowable_feature(feature: &BorrowableFeature, code_gen: &mut CodeGen) {
    match feature {
        BorrowableFeature::WhiteList(white_list) => {
            let white_list = white_list.iter().map(|path| path_to_dot_tokens(path, None));

            code_gen.impl_ast_item.push(quote! {
                fn is_borrowable(&self, other: &dyn AstItem) -> bool {
                    #(
                        if other.is::<#white_list>() {
                            return true;
                        };
                    )*
                    false
                }
            });
        }
        BorrowableFeature::BlackList(black_list) => {
            let black_list = black_list.iter().map(|path| path_to_dot_tokens(path, None));

            code_gen.impl_ast_item.push(quote! {
                fn is_borrowable(&self, other: &dyn AstItem) -> bool {
                    #(
                        if other.is::<#black_list>() {
                            return false;
                        };
                    )*
                    true
                }
            });
        }
        BorrowableFeature::BorrowableFn(borrowable_fn) => {
            let borrowable_fn = path_to_dot_tokens(borrowable_fn, None);

            code_gen.impl_ast_item.push(quote! {
                fn is_borrowable(&self, other: &dyn AstItem) -> bool {
                    #borrowable_fn(other)
                }
            });
        }
    }
}
