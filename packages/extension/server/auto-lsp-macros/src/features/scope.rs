extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    Feature, FeaturesCodeGen, Paths, ToCodeGen,
};

#[derive(Debug, FromMeta)]
pub struct ScopeFeature {
    range: ScopeRange,
}

#[derive(Debug, FromMeta)]
pub struct ScopeRange {
    start: Path,
    end: Path,
}

pub struct ScopeBuilder<'a> {
    pub input_name: &'a Ident,
    pub paths: &'a Paths,
    pub params: Option<&'a Feature<ScopeFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> ScopeBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        paths: &'a Paths,
        params: Option<&'a Feature<ScopeFeature>>,
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

impl<'a> ToCodeGen for ScopeBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let scope_path = &self.paths.scope_trait;

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #scope_path for #input_name {
                    fn is_scope(&self) -> bool {
                        false
                    }

                    fn get_scope_range(&self) -> Vec<[usize; 2]> {
                        vec!()
                    }
                }
            }),
            Some(params) => match params {
                Feature::User => (),
                Feature::CodeGen(scope) => {
                    let range = &scope.range;
                    let start = path_to_dot_tokens(&range.start, None);
                    let end = path_to_dot_tokens(&range.end, None);

                    codegen.input.other_impl.push(quote! {
                        impl #scope_path for #input_name {
                            fn is_scope(&self) -> bool {
                                true
                            }

                            fn get_scope_range(&self) -> Vec<[usize; 2]> {

                                let start = #start.read().get_range().start_byte;
                                let end = #end.read().get_range().end_byte;

                                vec!([start, end])
                            }
                        }
                    });
                }
            },
        }
    }
}
