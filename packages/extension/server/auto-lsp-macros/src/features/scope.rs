extern crate proc_macro;

use darling::FromMeta;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    Feature, FeaturesCodeGen, Paths, ToCodeGen, PATHS,
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
    pub params: Option<&'a Feature<ScopeFeature>>,
    pub fields: &'a StructFields,
}

impl<'a> ScopeBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        params: Option<&'a Feature<ScopeFeature>>,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            input_name,
            params,
            fields,
        }
    }
}

impl<'a> ToCodeGen for ScopeBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let scope_path = &PATHS.scope.path;
        let is_scope_sig = &PATHS.scope.methods.is_scope.sig;
        let is_scope_default = &PATHS.scope.methods.is_scope.default;

        let get_scope_range_sig = &PATHS.scope.methods.get_scope_range.sig;
        let get_scope_range_default = &PATHS.scope.methods.get_scope_range.default;

        match self.params {
            None => codegen.input.other_impl.push(quote! {
                impl #scope_path for #input_name {
                    #is_scope_sig {
                        #is_scope_default
                    }

                    #get_scope_range_sig {
                        #get_scope_range_default
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
                            #is_scope_sig {
                                true
                            }

                            #get_scope_range_sig {
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
