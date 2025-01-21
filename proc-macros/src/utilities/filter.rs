use syn::{Type, TypePath};

/// Extracts the raw type name by recursively unwrapping wrapper types
///
/// This function traverses through common wrapper types (Arc, RwLock, Vec, Option)
/// to get the underlying type name.
pub fn get_raw_type_name(ty: &Type) -> String {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let type_name = path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            // Check if the type is Arc, RwLock, Vec, Option and recursively get the inner type
            if type_name == "Arc"
                || type_name == "RwLock"
                || type_name == "Vec"
                || type_name == "Option"
            {
                if let Some(inner_type) = get_inner_type(ty, 0) {
                    return get_raw_type_name(&inner_type);
                }
            }

            type_name
        }
        _ => panic!("Expected a type path"),
    }
}

pub fn get_inner_type(ty: &Type, index: usize) -> Option<Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(syn::GenericArgument::Type(inner_type)) = args.args.iter().nth(index) {
                    return Some(inner_type.clone());
                }
            }
        }
    }
    None
}

/// Checks if the type is a Vec
pub fn is_vec(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.first() {
            let type_name = segment.ident.to_string();
            return type_name == "Vec";
        }
    }
    false
}

/// Checks if the type is an Option
pub fn is_option(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.first() {
            let type_name = segment.ident.to_string();
            return type_name == "Option";
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_get_raw_type_name() {
        let ty = parse_quote! { Arc<RwLock<Vec<Option<String>>>> };
        assert_eq!(get_raw_type_name(&ty), "String");
    }

    #[test]
    fn test_is_vec() {
        let ty = parse_quote! { Vec<String> };
        assert!(is_vec(&ty));
    }

    #[test]
    fn test_is_option() {
        let ty = parse_quote! { Option<String> };
        assert!(is_option(&ty));
    }
}
