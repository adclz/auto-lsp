extern crate proc_macro;

use darling::{ast, util, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path};

use crate::{
    utilities::{extract_fields::StructFields, format_tokens::path_to_dot_tokens},
    AccessorFeatures, FeaturesCodeGen, StructHelpers, SymbolFeatures, PATHS,
};

#[derive(Debug, FromMeta)]
pub struct DuplicateCheck {
    check_fn: Path,
    #[darling(multiple)]
    merge: Vec<OtherDuplicateCheck>,
}

#[derive(Debug, Clone, FromMeta)]
struct OtherDuplicateCheck {
    vec: Path,
    check_fn: Path,
}

pub struct CheckDuplicateBuilder<'a> {
    pub input_name: &'a Ident,
    pub fields: &'a StructFields,
    pub helper: &'a ast::Data<util::Ignored, StructHelpers>,
}

impl<'a> CheckDuplicateBuilder<'a> {
    pub fn new(
        input_name: &'a Ident,
        helper: &'a ast::Data<util::Ignored, StructHelpers>,
        fields: &'a StructFields,
    ) -> Self {
        Self {
            input_name,
            fields,
            helper,
        }
    }

    pub fn default_impl(&self) -> TokenStream {
        let input_name = &self.input_name;
        let check_duplicate = &PATHS.check_duplicate.path;
        let must_check_sig = &PATHS.check_duplicate.methods.must_check.sig;
        let must_check_default = &PATHS.check_duplicate.methods.must_check.default;

        let check_sig = &PATHS.check_duplicate.methods.check.sig;
        let check_default = &PATHS.check_duplicate.methods.check.default;

        quote! {
            impl #check_duplicate for #input_name {
                #must_check_sig {
                    #must_check_default
                }

                #check_sig {
                    #check_default
                }
            }
        }
    }
}

impl<'a> FeaturesCodeGen for CheckDuplicateBuilder<'a> {
    fn code_gen(&self, params: &SymbolFeatures) -> impl quote::ToTokens {
        let input_name = &self.input_name;
        let check_duplicate = &PATHS.check_duplicate.path;
        let must_check_sig = &PATHS.check_duplicate.methods.must_check.sig;

        let check_sig = &PATHS.check_duplicate.methods.check.sig;

        let fields = self.helper.as_ref().take_struct().unwrap();

        let fields = fields
            .iter()
            .filter(|f| f.dup.is_some())
            .collect::<Vec<_>>();

        if fields.is_empty() {
            self.default_impl()
        } else {
            let mut tokens = TokenStream::new();
            fields.iter().for_each(|f| {
                let dup = f.dup.as_ref().unwrap();
                let mut other = vec![];

                dup.merge.iter().for_each(|f| {
                    let other_vec = path_to_dot_tokens(&f.vec, None);
                    let other_check_fn = path_to_dot_tokens(&f.check_fn, None);

                    other.push(quote! {
                        #other_vec.check(
                            #other_check_fn,
                            doc,
                            &mut hash_set,
                            diagnostics
                        );
                    });
                });

                let field_name = &f.ident.as_ref().unwrap();
                let check_fn = path_to_dot_tokens(&dup.check_fn, None);

                tokens.extend(quote! {
                    impl #check_duplicate for #input_name {
                        #must_check_sig {
                            true
                        }

                        #check_sig {
                            let mut hash_set = std::collections::HashSet::new();

                            self.#field_name.check(
                                #check_fn,
                                doc,
                                &mut hash_set,
                                diagnostics
                            );

                            #(#other)*
                        }
                    }
                });
            });
            tokens
        }
    }

    fn code_gen_accessor(&self, _params: &AccessorFeatures) -> impl quote::ToTokens {
        self.default_impl()
    }
}
