# Miette

```admonish
Miette is only available in the `miette` feature.
```

It is possible to test different parts of the AST independently.
Enable the `miette` feature, and then use the [`miette_parse`] method from the symbol you wish to test.

```rust
use auto_lsp::core::ast::*;
use auto_lsp::{seq, choice};

#[seq(query = "document")]
struct Document {
    functions: Vec<Function>,
}

#[seq(query = "function")]
struct Function {
   name: Identifier,
}

#[seq(query = "identifier")]
struct Identifier {}

// Test Function independently from Document
#[test]
fn function() -> miette::Result<()> {
   Function::miette_parse(
    r#"function foo()"#)
}
```

## Example with a type error in Python

```rust, ignore
#[test]
fn function() -> miette::Result<()> {
   Function::miette_parse(
       r#"
       def foo(param1, param2: int = "5"):
           pass
       "#,
       &PYTHON_PARSERS.get("python").unwrap(),
   )
}
```

This code will return the following error when running tests:

<img src="https://raw.githubusercontent.com/adclz/auto-lsp/refs/heads/main/assets/miette.png">
