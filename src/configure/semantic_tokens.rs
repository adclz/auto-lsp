/// Define the standard semantic token types.
///
/// This macro generates three components to streamline working with semantic token types:
/// 1. **Constants**: Creates a constant for each standard token type, which serves as a type alias for [`lsp_types::SemanticTokenType`].
/// 2. **Supported Token Types**: Generates a slice (`SUPPORTED_TYPES`) containing all supported token types. This slice is used to inform an LSP client about the token types supported by the server.
/// 3. **Token Type Map**: Constructs a map (`TOKEN_TYPES`) that links token type names (as strings) to their corresponding constants, enabling efficient lookups.
///
/// # Example
/// ```rust
/// # use auto_lsp::lsp_types::SemanticTokenType;
/// # use auto_lsp::define_semantic_token_types;
/// # use phf::phf_map;
/// define_semantic_token_types! {
///     standard {
///         "namespace" => NAMESPACE,
///         "type" => TYPE,
///         "function" => FUNCTION,
///     }
/// }
/// ```
/// This would generate:
/// - Constants `NAMESPACE`, `TYPE`, and `FUNCTION`.
/// - A `SUPPORTED_TYPES` slice containing these token types.
/// - A `TOKEN_TYPES` map associating "namespace", "type", and "function" with their respective constants.
#[macro_export]
macro_rules! define_semantic_token_types {
    (
        standard {
            $($ts_name: expr => $standard:ident),*$(,)?
        }
    ) => {
        $(pub const $standard: auto_lsp::lsp_types::SemanticTokenType = auto_lsp::lsp_types::SemanticTokenType::$standard;)*

        pub const SUPPORTED_TYPES: &[auto_lsp::lsp_types::SemanticTokenType] = &[
            $(auto_lsp::lsp_types::SemanticTokenType::$standard,)*
        ];

        pub static TOKEN_TYPES: phf::OrderedMap<&'static str, auto_lsp::lsp_types::SemanticTokenType> = phf::phf_ordered_map! {
            $( $ts_name => auto_lsp::lsp_types::SemanticTokenType::$standard,)*
        };
    };
}

/// Define the standard semantic token modifiers.
///
/// This macro generates three components to manage semantic token modifiers:
/// 1. **Constants**: Defines a constant for each standard token modifier wich serves a type alias for [`lsp_types::SemanticTokenModifier`].
/// 2. **Supported Modifiers**: Creates a slice (`SUPPORTED_MODIFIERS`) listing all supported token modifiers.
/// 3. **Modifier Map**: Constructs a map (`TOKEN_MODIFIERS`) linking modifier names (as strings) to their respective constants for easy lookup.
///
/// # Example
/// ```rust
/// # use auto_lsp::lsp_types::SemanticTokenModifier;
/// # use auto_lsp::define_semantic_token_modifiers;
/// # use phf::phf_map;
/// define_semantic_token_modifiers! {
///     standard {
///         "declaration" => DECLARATION,
///         "readonly" => READONLY,
///         "static" => STATIC,
///     }
/// }
/// ```
/// This would generate:
/// - Constants `DECLARATION`, `READONLY`, and `STATIC`.
/// - A `SUPPORTED_MODIFIERS` slice containing these token modifiers.
/// - A `TOKEN_MODIFIERS` map associating "declaration", "readonly", and "static" with their corresponding constants.
#[macro_export]
macro_rules! define_semantic_token_modifiers {
    (
        standard {
            $($ts_name: expr => $standard:ident),*$(,)?
        }
    ) => {
        $(pub const $standard: auto_lsp::lsp_types::SemanticTokenModifier = auto_lsp::lsp_types::SemanticTokenModifier::$standard;)*

        pub const SUPPORTED_MODIFIERS: &[auto_lsp::lsp_types::SemanticTokenModifier] = &[
            $(auto_lsp::lsp_types::SemanticTokenModifier::$standard,)*
        ];

        pub static TOKEN_MODIFIERS: phf::OrderedMap<&'static str, auto_lsp::lsp_types::SemanticTokenModifier> = phf::phf_ordered_map! {
            $( $ts_name => auto_lsp::lsp_types::SemanticTokenModifier::$standard,)*
        };
    };
}
