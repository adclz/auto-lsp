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

# Pattern Matching

The `#[choice]` attribute generates standard Rust enums that fully support pattern matching. This makes it easy to work with nested AST structures.

For example, consider an expression that can contain nested types:

```rust, ignore
#[choice]
pub enum Expression {
    PrimaryExpression(PrimaryExpression),
    Identifier(Identifier),
}

#[choice]
pub enum PrimaryExpression {
     Integer(Integer),
     Bool(Bool)
}
```

You can pattern match through multiple layers using standard Rust match expressions:


```rust, ignore
impl Expression {
    pub fn is_integer(&self) -> bool {
        matches!(self, Expression::PrimaryExpression(PrimaryExpression::Integer(_)))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Expression::PrimaryExpression(PrimaryExpression::Bool(_)))
    }
}

```