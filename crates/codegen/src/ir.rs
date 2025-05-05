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
            Kind::Base => quote! { std::sync::Arc<#pascal_name> },
            Kind::Vec => quote! { Vec<std::sync::Arc<#pascal_name>> },
            Kind::Option => quote! { Option<std::sync::Arc<#pascal_name>> },
        };

        quote! {
            pub #field_name: #field_type
        }
    }

    fn generate_field_collect(&self) -> TokenStream {
        let field_name = format_ident!("{}", sanitize_string(&self.tree_sitter_type));

        let lock = FIELD_ID_FOR_NAME.lock().unwrap();
        let kind = lock.get(&self.tree_sitter_type);
        let pascal_name = &self.field_name;

        match self.kind {
            Kind::Base => quote! {
                let #field_name = node
                    .children_by_field_id(std::num::NonZero::new(19u16).unwrap(), &mut cursor)
                    .next()
                    .ok_or_else(|| auto_lsp::core::errors::AstError::UnexpectedSymbol {
                        range: node.range(),
                        symbol: node.kind(),
                        parent_name: stringify!(stringify!(#field_name)),
                })?;
                let #field_name = std::sync::Arc::new(#pascal_name::try_from((&#field_name, &mut *index))?);
                index.push(#field_name.clone() as _);
            },
            Kind::Vec => quote! {
                let #field_name = node
                    .children_by_field_id(std::num::NonZero::new(#kind).unwrap(), &mut cursor)
                    .map(|node| {
                        let result = std::sync::Arc::new(#pascal_name::try_from((&node, &mut *index))?);
                        index.push(result.clone() as _);
                        Ok(result)
                    })
                    .collect::<Result<Vec<_>, Self::Error>>()?;
            },
            Kind::Option => quote! {
                let #field_name = node
                    .children_by_field_id(std::num::NonZero::new(1u16).unwrap(), &mut cursor)
                    .next()
                    .map(|node| {
                        let #field_name = std::sync::Arc::new(#pascal_name::try_from((&node, &mut *index))?);
                        index.push(#field_name.clone() as _);
                        Ok(#field_name)
                    }).transpose()?;
            },
        }
    }

    fn generate_field_finalize(&self) -> TokenStream {
        let field_name = format_ident!("{}", sanitize_string(&self.tree_sitter_type));
        quote! { #field_name }
    }
}

pub(crate) struct Child {
    pub(crate) kind: Kind,
    pub(crate) field_name: TokenStream,
}

impl Child {
    fn generate_field(&self) -> TokenStream {
        let field_name = format_ident!("children");
        let pascal_name = &self.field_name;
        let field_type = match self.kind {
            Kind::Base => quote! { std::sync::Arc<#pascal_name> },
            Kind::Vec => quote! { Vec<std::sync::Arc<#pascal_name>> },
            Kind::Option => quote! { Option<std::sync::Arc<#pascal_name>> },
        };

        quote! {
            pub #field_name: #field_type
        }
    }

    fn generate_field_collect(&self) -> TokenStream {
        let pascal_name = &self.field_name;

        quote! {
            let children = node
                .children(&mut cursor)
                .filter(|n| #pascal_name::contains(n))
                .map(|node| {
                    let result = std::sync::Arc::new(#pascal_name::try_from((&node, &mut *index))?);
                    index.push(result.clone() as _);
                    Ok(result)
                })
                .collect::<Result<Vec<_>, Self::Error>>()?;
        }
    }

    fn generate_field_finalize(&self) -> TokenStream {
        match self.kind {
            Kind::Base => quote! {
                children: children.first().unwrap().clone()
            },
            Kind::Vec => quote! { children },
            Kind::Option => quote! {
                children: children.first().map(Clone::clone)
            },
        }
    }
}
