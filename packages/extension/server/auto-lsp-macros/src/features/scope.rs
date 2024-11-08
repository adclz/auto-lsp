extern crate proc_macro;

use darling::{util::PathList, FromMeta};
use quote::quote;
use syn::Path;

use crate::{utilities::format_tokens::path_to_dot_tokens, AstStructFeatures, CodeGen};

#[derive(Debug, FromMeta)]
pub enum ScopeFeature {
    Range(ScopeRange),
    ScopeFn(Path),
}

#[derive(Debug, FromMeta)]
pub struct ScopeRange {
    start: Path,
    end: Path,
}

pub fn generate_scope_feature(features: &AstStructFeatures, code_gen: &mut CodeGen) {
    if let Some(scope) = &features.scope {
        codegen_scope_feature(&scope, code_gen);
    }
}

fn codegen_scope_feature(feature: &ScopeFeature, code_gen: &mut CodeGen) {
    match feature {
        ScopeFeature::Range(range) => {
            let start = path_to_dot_tokens(&range.start, None);
            let end = path_to_dot_tokens(&range.end, None);

            code_gen.impl_ast_item.push(quote! {
                fn is_scope(&self) -> bool {
                    true
                }

                fn get_scope_range(&self) -> [usize; 2] {

                    let start = #start.read().unwrap().get_range().start_byte;
                    let end = #end.read().unwrap().get_range().end_byte;

                    [start, end]
                }
            });
        }
        ScopeFeature::ScopeFn(scope_fn) => {
            let scope_fn = path_to_dot_tokens(scope_fn, None);

            code_gen.impl_ast_item.push(quote! {
                fn is_scope(&self) -> bool {
                    true
                }

                fn get_scope_range(&self) -> [usize; 2] {
                    #scope_fn()
                }
            });
        }
    }
}
