# Configuring Semantic Tokens

To configure semantic tokens, you need to use the `define_semantic_token_types` and `define_semantic_token_modifiers` macros.

# Token Types

```rust, ignore
use auto_lsp::define_semantic_token_types;

define_semantic_token_types![
    standard {
         "namespace" => NAMESPACE,
         "type" => TYPE,
         "function" => FUNCTION,
    }
    
    custom {
        "custom" => CUSTOM,
    }
];
```

This macro generates three components to streamline working with semantic token types:
1. **Constants**: Creates a constant for each standard and custom token type.
2. **Supported Token Types**: Generates a slice (`SUPPORTED_TYPES`) containing all supported token types that can be reused to inform the LSP client about available tokens.

# Token Modifiers

```rust, ignore
use auto_lsp::define_semantic_token_modifiers;
define_semantic_token_modifiers![
    standard {
        DOCUMENTATION,
        DECLARATION,
    }

    custom {
        (READONLY, "readonly"),
        (STATIC, "static"),
    }
];
```

This generates:
- Constants for **standard** (`DOCUMENTATION`, `DECLARATION`) and **custom** (`READONLY`, `STATIC`) modifiers.
- A `SUPPORTED_MODIFIERS` slice that includes both standard and custom modifiers.

# Example in AST

```rust, ignore
use auto_lsp::core::semantic_tokens_builder::SemanticTokensBuilder;

define_semantic_token_types![
    standard {
        FUNCTION,
    }

    custom {}
];

define_semantic_token_modifiers![
    standard {
        DECLARATION,
    }

    custom {}
];

impl MyType {
    fn build_semantic_tokens(&self, builder: &mut SemanticTokensBuilder) {
        builder.push(
            self.name.get_lsp_range(),
            SUPPORTED_TYPES.iter().position(|x| *x == FUNCTION).unwrap() as u32,
            SUPPORTED_MODIFIERS.iter().position(|x| *x == DECLARATION).unwrap() as u32,
        );
    }
}
```