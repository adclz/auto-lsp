/*
This file is part of auto-lsp.
Copyright (C) 2025 CLAUZEL Adrien

auto-lsp is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/

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
            Kind::Base => quote! { std::sync::Arc<#pascal_name> },
            Kind::Vec => quote! { Vec<std::sync::Arc<#pascal_name>> },
            Kind::Option => quote! { Option<std::sync::Arc<#pascal_name>> },
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
                builder.on_field_id::<#pascal_name, #kind>(&mut #field_name)
            },
            Kind::Vec => quote! {
                builder.on_vec_field_id::<#pascal_name, #kind>(&mut #field_name)
            },
            Kind::Option => quote! {
                builder.on_field_id::<#pascal_name, #kind>(&mut #field_name)
            },
        }
    }

    fn generate_field_finalize(&self) -> TokenStream {
        let field_name = format_ident!("{}", sanitize_string(&self.tree_sitter_type));
        match self.kind {
            Kind::Base => quote! {
                 #field_name:  #field_name?.unwrap()
            },
            Kind::Vec => quote! {  #field_name },
            Kind::Option => quote! {  #field_name:  #field_name? },
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
            Kind::Base => quote! { std::sync::Arc<#pascal_name> },
            Kind::Vec => quote! { Vec<std::sync::Arc<#pascal_name>> },
            Kind::Option => quote! { Option<std::sync::Arc<#pascal_name>> },
        };

        quote! {
            pub children: #field_type
        }
    }

    fn generate_field_init(&self) -> TokenStream {
        let pascal_name = &self.field_name;
        match self.kind {
            Kind::Base => quote! { let mut children = Ok(None); },
            Kind::Vec => quote! { let mut children = vec![]; },
            Kind::Option => quote! { let mut children = Ok(None); },
        }
    }

    fn generate_field_collect(&self) -> TokenStream {
        match self.kind {
            Kind::Base => quote! {
                builder.on_children_id(&mut children);
            },
            Kind::Vec => quote! {
                builder.on_vec_children_id(&mut children);
            },
            Kind::Option => quote! {
                builder.on_children_id(&mut children);
            },
        }
    }

    fn generate_field_finalize(&self) -> TokenStream {
        match self.kind {
            Kind::Base => quote! {
                children: children?.unwrap()
            },
            Kind::Vec => quote! { children },
            Kind::Option => quote! { children: children? },
        }
    }
}
