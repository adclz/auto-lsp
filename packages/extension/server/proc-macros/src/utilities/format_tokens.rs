use quote::quote;
use syn::Path;

pub fn path_to_dot_tokens(
    path: &Path,
    modifier: Option<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    // Convert the path to a dot-separated field access expression
    let mut segments = path.segments.iter();
    let first_segment = segments.next().unwrap().ident.clone(); // Get the first segment

    if segments.len() == 0 {
        return quote! { #first_segment };
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
