# Choice Macro

[`#choice`](https://docs.rs/auto-lsp/latest/auto_lsp/attr.choice.html) is used to define a choice between multiple sequences of nodes, it only works with enums.

Unlike `#seq`, `#choice` does not have any attribute.
Instead, choice will try to find the correct variant at runtime by testing the query name of each variant.

Then the underlying variant will have the according trait methods called if implemented.

Variants behave similarly to `#seq` fields, they can be named however you want, only the value is important.

```admonish warning
`#choice`  only supports direct symbols, `Vec` and `Option` are not supported.
```

```rust, ignore
use auto_lsp::{seq, choice};

#[choice]
enum Element {
    AStatement(Statement),
    SimpleExpression(Expression),
}

#[seq(query = "statement")]
struct Statement {}

#[seq(query = "expression")]
struct Expression {}
```
