use crate::{utils::sanitize_string, FIELD_ID_FOR_NAME};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) enum FieldOrChildren {
    Child(Child),
    Field(Field),
}

impl FieldOrChildren {
    pub(crate) fn generate_field(&self) -> TokenStream {
        match self {
            FieldOrChildren::Field(field) => field.generate_field(),
            FieldOrChildren::Child(child) => child.generate_field(),
        }
    }

    pub(crate) fn generate_field_init(&self) -> TokenStream {
        match self {
            FieldOrChildren::Field(field) => field.generate_field_init(),
            FieldOrChildren::Child(child) => child.generate_field_init(),
        }
    }

    pub(crate) fn generate_field_collect(&self) -> TokenStream {
        match self {
            FieldOrChildren::Field(field) => field.generate_field_collect(),
            FieldOrChildren::Child(child) => child.generate_field_collect(),
        }
    }

    pub(crate) fn generate_field_finalize(&self) -> TokenStream {
        match self {
            FieldOrChildren::Field(field) => field.generate_field_finalize(),
            FieldOrChildren::Child(child) => child.generate_field_finalize(),
        }
    }
}

pub(crate) enum Kind {
    Base,
    Vec,
    Option,
}
pub(crate) struct Field {
    pub(crate) tree_sitter_type: String,
    pub(crate) kind: Kind,
    pub(crate) field_name: TokenStream,
}

impl Field {
    fn generate_field(&self) -> TokenStream {
        let field_name = format_ident!("{}", &sanitize_string(&self.tree_sitter_type));
        let pascal_name = &self.field_name;
        let field_type = match self.kind {
            Kind::Base => quote! { auto_lsp::core::ast::AstNodeId<#pascal_name> },
            Kind::Vec => quote! { Vec<auto_lsp::core::ast::AstNodeId<#pascal_name>> },
            Kind::Option => quote! { Option<auto_lsp::core::ast::AstNodeId<#pascal_name>> },
        };

        quote! {
            pub #field_name: #field_type
        }
    }

    fn generate_field_init(&self) -> TokenStream {
        let field_name = format_ident!("{}", sanitize_string(&self.tree_sitter_type));
        match self.kind {
            Kind::Base => quote! { let mut #field_name = Ok(None); },
            Kind::Vec => quote! { let mut #field_name = vec![]; },
            Kind::Option => quote! { let mut #field_name = Ok(None); },
        }
    }

    fn generate_field_collect(&self) -> TokenStream {
        let field_name = format_ident!("{}", sanitize_string(&self.tree_sitter_type));

        let lock = FIELD_ID_FOR_NAME.lock().unwrap();
        let kind = lock.get(&self.tree_sitter_type).unwrap();
        let pascal_name = &self.field_name;

        match self.kind {
            Kind::Base => quote! {
                on_field_id::<#pascal_name, #kind>(&mut #field_name)
            },
            Kind::Vec => quote! {
                on_vec_field_id::<#pascal_name, #kind>(&mut #field_name)
            },
            Kind::Option => quote! {
                on_field_id::<#pascal_name, #kind>(&mut #field_name)
            },
        }
    }

    fn generate_field_finalize(&self) -> TokenStream {
        let field_name = format_ident!("{}", sanitize_string(&self.tree_sitter_type));
        match self.kind {
            Kind::Base => quote! {
                 #field_name:  #field_name?.ok_or_else(|| {
                    auto_lsp::core::errors::AstError::UnexpectedSymbol {
                        range: node.range(),
                        symbol: node.kind(),
                        parent_name: stringify!(#field_name),
                    }
                })?
            },
            Kind::Vec => quote! {  #field_name },
            Kind::Option => quote! {  #field_name: #field_name? },
        }
    }
}

pub(crate) struct Child {
    pub(crate) kind: Kind,
    pub(crate) field_name: TokenStream,
}

impl Child {
    fn generate_field(&self) -> TokenStream {
        let pascal_name = &self.field_name;
        let field_type = match self.kind {
            Kind::Base => quote! { auto_lsp::core::ast::AstNodeId<#pascal_name> },
            Kind::Vec => quote! { Vec<auto_lsp::core::ast::AstNodeId<#pascal_name>> },
            Kind::Option => quote! { Option<auto_lsp::core::ast::AstNodeId<#pascal_name>> },
        };

        quote! {
            pub children: #field_type
        }
    }

    fn generate_field_init(&self) -> TokenStream {
        match self.kind {
            Kind::Base => quote! { let mut children = Ok(None); },
            Kind::Vec => quote! { let mut children = vec![]; },
            Kind::Option => quote! { let mut children = Ok(None); },
        }
    }

    fn generate_field_collect(&self) -> TokenStream {
        match self.kind {
            Kind::Base => quote! {
                on_children_id(&mut children)
            },
            Kind::Vec => quote! {
                on_vec_children_id(&mut children)
            },
            Kind::Option => quote! {
                on_children_id(&mut children)
            },
        }
    }

    fn generate_field_finalize(&self) -> TokenStream {
        let name = &self.field_name;
        match self.kind {
            Kind::Base => quote! {
                children: children?.ok_or_else(|| {
                    auto_lsp::core::errors::AstError::UnexpectedSymbol {
                        range: node.range(),
                        symbol: node.kind(),
                        parent_name: stringify!(#name),
                    }
                })?
            },
            Kind::Vec => quote! { children },
            Kind::Option => quote! { children: children? },
        }
    }
}
