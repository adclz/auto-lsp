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

/// Define the standard and custom semantic token types.
///
/// This macro generates three components to streamline working with semantic token types:
/// 1. **Constants**: Creates a constant for each standard and custom token type.
/// 2. **Supported Token Types**: Generates a slice (`SUPPORTED_TYPES`) containing all supported token types.
/// 3. **Token Type Map**: Constructs a function (`standard_fallback_type`) that maps custom token types to a fallback if applicable.
///
/// # Example
/// ```rust
/// use auto_lsp::define_semantic_token_types;
///
/// define_semantic_token_types![
///     standard {
///         NAMESPACE,
///         TYPE,
///         FUNCTION,
///     }
///
///     custom {
///         (BOOLEAN, "boolean"),
///         (GENERIC, "generic") => TYPE_PARAMETER,
///     }
/// ];
/// ```
/// This generates:
/// - Constants for **standard** (`NAMESPACE`, `TYPE`, `FUNCTION`) and **custom** (`BOOLEAN`, `GENERIC`) types.
/// - A `SUPPORTED_TYPES` slice that includes both standard and custom types.
/// - A `standard_fallback_type` function to map `GENERIC` to `TYPE_PARAMETER`.
#[macro_export]
macro_rules! define_semantic_token_types {
    (
        standard {
            $($standard:ident),*$(,)?
        }
        custom {
            $(($custom:ident, $string:literal) $(=> $fallback:ident)?),*$(,)?
        }
    ) => {
        $(pub const $standard: auto_lsp::lsp_types::SemanticTokenType = auto_lsp::lsp_types::SemanticTokenType::$standard;)*
        $(pub const $custom: auto_lsp::lsp_types::SemanticTokenType = auto_lsp::lsp_types::SemanticTokenType::new($string);)*

        pub const SUPPORTED_TYPES: &[auto_lsp::lsp_types::SemanticTokenType] = &[
            $(auto_lsp::lsp_types::SemanticTokenType::$standard,)*
            $($custom),*
        ];

        #[allow(unused)]
        pub fn standard_fallback_type(token: auto_lsp::lsp_types::SemanticTokenType) -> Option<auto_lsp::lsp_types::SemanticTokenType> {
            $(
                if token == $custom {
                    None $(.or(Some(auto_lsp::lsp_types::SemanticTokenType::$fallback)))?
                } else
            )*
            { Some(token) }
        }
    };
}

/// Define the standard and custom semantic token modifiers.
///
/// This macro generates two components to manage semantic token modifiers:
/// 1. **Constants**: Defines a constant for each standard and custom token modifier.
/// 2. **Supported Modifiers**: Creates a slice (`SUPPORTED_MODIFIERS`) listing all supported token modifiers.
///
/// # Example
/// ```rust
/// use auto_lsp::define_semantic_token_modifiers;
///
/// define_semantic_token_modifiers![
///     standard {
///         DOCUMENTATION,
///         DECLARATION,
///     }
///
///     custom {
///         (READONLY, "readonly"),
///         (STATIC, "static"),
///     }
/// ];
/// ```
/// This generates:
/// - Constants for **standard** (`DOCUMENTATION`, `DECLARATION`) and **custom** (`READONLY`, `STATIC`) modifiers.
/// - A `SUPPORTED_MODIFIERS` slice that includes both standard and custom modifiers.
#[macro_export]
macro_rules! define_semantic_token_modifiers {
    (
        standard {
            $($standard:ident),*$(,)?
        }
        custom {
            $(($custom:ident, $string:literal)),*$(,)?
        }
    ) => {
        $(pub const $standard: auto_lsp::lsp_types::SemanticTokenModifier = auto_lsp::lsp_types::SemanticTokenModifier::$standard;)*
        $(pub const $custom: auto_lsp::lsp_types::SemanticTokenModifier = auto_lsp::lsp_types::SemanticTokenModifier::new($string);)*

        pub const SUPPORTED_MODIFIERS: &[auto_lsp::lsp_types::SemanticTokenModifier] = &[
            $(auto_lsp::lsp_types::SemanticTokenModifier::$standard,)*
            $($custom),*
        ];
    };
}
