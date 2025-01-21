use quote::quote;
use syn::Path;

/// Converts a path to a dot-separated field access expression
pub fn path_to_dot_tokens(
    path: &Path,
    modifier: Option<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    // Convert the path to a dot-separated field access expression
    let mut segments = path.segments.iter();
    let first_segment = segments.next().unwrap().ident.clone(); // Get the first segment

    if segments.len() == 0 {
        if let Some(modifier) = modifier {
            return quote! { #first_segment.#modifier};
        } else {
            return quote! { #first_segment };
        }
    }

    let tokens = segments.fold(quote! { #first_segment }, |acc, segment| {
        let ident = &segment.ident;
        quote! { #acc.#ident }
    });

    if let Some(modifier) = modifier {
        quote! { #tokens.#modifier}
    } else {
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_path_to_dot_tokens() {
        let path = parse_quote! { std::collections::HashMap };
        let tokens = path_to_dot_tokens(&path, None);
        assert_eq!(tokens.to_string(), "std . collections . HashMap");

        let path = parse_quote! { std::collections::HashMap };
        let modifier = quote! { len };
        let tokens = path_to_dot_tokens(&path, Some(modifier));
        assert_eq!(tokens.to_string(), "std . collections . HashMap . len");
    }

    #[test]
    fn test_path_to_dot_tokens_single() {
        let path = parse_quote! { HashMap };
        let tokens = path_to_dot_tokens(&path, None);
        assert_eq!(tokens.to_string(), "HashMap");

        let path = parse_quote! { HashMap };
        let modifier = quote! { len };
        let tokens = path_to_dot_tokens(&path, Some(modifier));
        assert_eq!(tokens.to_string(), "HashMap . len");
    }

    #[test]
    fn test_path_to_dot_tokens_self() {
        let path = parse_quote! { Self };
        let tokens = path_to_dot_tokens(&path, None);
        assert_eq!(tokens.to_string(), "Self");

        let path = parse_quote! { Self::field };
        let modifier = quote! { len };
        let tokens = path_to_dot_tokens(&path, Some(modifier));
        assert_eq!(tokens.to_string(), "Self . field . len");
    }
}
