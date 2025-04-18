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
use syn::{spanned::Spanned, GenericArgument, PathArguments, Type, TypePath};

/// Returns the type name inside a `Vec<T>`, if T is not generic.
pub fn get_vec_type_name(ty: &Type) -> Result<String, syn::Error> {
    check_generic_container(ty, "Vec")
}

/// Returns the type name inside an `Option<T>`, if T is not generic.
pub fn get_option_type_name(ty: &Type) -> Result<String, syn::Error> {
    check_generic_container(ty, "Option")
}

/// Returns the type name of a simple type `T`, ensuring it's not generic.
pub fn get_type_name(ty: &Type) -> Result<String, syn::Error> {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let last = path.segments.last().unwrap();
            match &last.arguments {
                PathArguments::None => Ok(last.ident.to_string()),
                _ => Err(syn::Error::new(ty.span(), "Expected a non-generic type")),
            }
        }
        _ => Err(syn::Error::new(ty.span(), "Expected a type path")),
    }
}

/// Checks if a type is a specific container (e.g., `Vec` or `Option`) and extracts its inner type name.
fn check_generic_container(ty: &Type, expected_ident: &str) -> Result<String, syn::Error> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.first() {
            if segment.ident != expected_ident {
                return Err(syn::Error::new(
                    ty.span(),
                    format!("Expected a {expected_ident}<T> type"),
                ));
            }

            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if args.args.len() != 1 {
                    return Err(syn::Error::new(
                        ty.span(),
                        "Expected exactly one generic type argument",
                    ));
                }

                if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                    // Now ensure that inner_ty is not generic
                    match inner_ty {
                        Type::Path(TypePath { path, .. }) => {
                            if let Some(inner_seg) = path.segments.last() {
                                match &inner_seg.arguments {
                                    PathArguments::None => Ok(inner_seg.ident.to_string()),
                                    _ => Err(syn::Error::new(
                                        inner_ty.span(),
                                        "Generic types inside Vec<T> or Option<T> are not allowed",
                                    )),
                                }
                            } else {
                                Err(syn::Error::new(
                                    inner_ty.span(),
                                    "Unexpected structure in type path",
                                ))
                            }
                        }
                        _ => Err(syn::Error::new(
                            inner_ty.span(),
                            "Unsupported type inside Vec or Option",
                        )),
                    }
                } else {
                    Err(syn::Error::new(
                        ty.span(),
                        "Expected a type as the first generic argument",
                    ))
                }
            } else {
                Err(syn::Error::new(
                    ty.span(),
                    format!("Expected angle-bracketed arguments for {expected_ident}"),
                ))
            }
        } else {
            Err(syn::Error::new(
                ty.span(),
                format!("Expected a {expected_ident}<T> type"),
            ))
        }
    } else {
        Err(syn::Error::new(
            ty.span(),
            format!("Expected a type path for {expected_ident}"),
        ))
    }
}

/// Check if a type is a Vec
pub fn is_vec(ty: &Type) -> bool {
    matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.first().map(|s| s.ident == "Vec").unwrap_or(false))
}

/// Check if a type is an Option
pub fn is_option(ty: &Type) -> bool {
    matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.first().map(|s| s.ident == "Option").unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, spanned::Spanned};

    #[test]
    fn get_vec_type_name_ok() {
        let ty = parse_quote! { Vec<String> };
        let result = get_vec_type_name(&ty);
        assert_eq!(result.unwrap(), "String");
    }

    #[test]
    fn get_vec_type_name_nested_generic_error() {
        let ty = parse_quote! { Vec<Vec<String>> };
        let result = get_vec_type_name(&ty);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.span().source_text(), ty.span().source_text());
        assert!(err
            .to_string()
            .contains("Generic types inside Vec<T> or Option<T> are not allowed"));
    }

    #[test]
    fn get_option_type_name_ok() {
        let ty = parse_quote! { Option<u32> };
        let result = get_option_type_name(&ty);
        assert_eq!(result.unwrap(), "u32");
    }

    #[test]
    fn get_option_type_name_nested_generic_error() {
        let ty = parse_quote! { Option<Option<u32>> };
        let result = get_option_type_name(&ty);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.span().source_text(), ty.span().source_text());
        assert!(err
            .to_string()
            .contains("Generic types inside Vec<T> or Option<T> are not allowed"));
    }

    #[test]
    fn get_type_name_ok() {
        let ty = parse_quote! { String };
        let result = get_type_name(&ty);
        assert_eq!(result.unwrap(), "String");
    }

    #[test]
    fn get_type_name_generic_error() {
        let ty = parse_quote! { Result<String, u32> };
        let result = get_type_name(&ty);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Expected a non-generic type"));
    }

    #[test]
    fn is_vec_and_option() {
        let vec_ty = parse_quote! { Vec<String> };
        let opt_ty = parse_quote! { Option<u32> };
        let other_ty = parse_quote! { Result<String, u32> };

        assert!(is_vec(&vec_ty));
        assert!(is_option(&opt_ty));
        assert!(!is_vec(&other_ty));
        assert!(!is_option(&other_ty));
    }
}
