extern crate proc_macro;

use darling::{ast, ast::NestedMeta, util, FromMeta};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser}, punctuated::Punctuated, token::Comma, Error, Ident, Meta, Path
};

use crate::{
    utilities::{
        extract_fields::StructFields,
        format_tokens::path_to_dot_tokens,
    }, FeaturesCodeGen, Paths, StructHelpers, ToCodeGen,
    PATHS
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
}

impl<'a> ToCodeGen for CheckDuplicateBuilder<'a> {
    fn to_code_gen(&self, codegen: &mut FeaturesCodeGen) {
        let input_name = &self.input_name;
        let check_duplicate = &PATHS.check_duplicate;

        let fields = self
            .helper
            .as_ref()
            .take_struct()
            .unwrap();

        let fields = fields
            .iter()
            .filter(|f| f.dup.is_some())
            .collect::<Vec<_>>();

        if fields.is_empty()  {
            codegen.input.other_impl.push(quote! {
                impl #check_duplicate for #input_name {
                    fn must_check(&self) -> bool {
                        false
                    }

                    fn check(&self, doc: &lsp_textdocument::FullTextDocument, diagnostics: &mut Vec<lsp_types::Diagnostic>)  {
                        
                    }
                }
            });
        } else {
            fields.iter().for_each(|f| {
                let dup = f.dup.as_ref().unwrap();
                let mut other = vec!(); 

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

                codegen.input.other_impl.push(quote! {
                    impl #check_duplicate for #input_name {
                        fn must_check(&self) -> bool {
                            true
                        }
    
                        fn check(&self, doc: &lsp_textdocument::FullTextDocument, diagnostics: &mut Vec<lsp_types::Diagnostic>)  {
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
        }
    }
}
