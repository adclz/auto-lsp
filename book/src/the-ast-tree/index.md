# The AST tree

The AST Tree is a linked list of strongly typed nodes.
Each node is a `Symbol<T>` where `T` is a type implementing `AstSymbol`.

When using one of the `#seq`, or `#choice` macros, `auto_lsp` will generate two types of symbols:
 - The symbol itself with thread safe fields
 - The builder associated with the symbol

For example a struct named **Module** with an optional **Function** field:

```rust, ignore
use auto_lsp::core::seq;

#[seq(query = "module")]
struct Module {
    function: Option<Function>,
}

#[seq(query = "function")]
struct Function {}
```

This would generate:

```rust, ignore
#[derive(Clone)]
pub struct Module {
    pub function: Option<Symbol<Function>>,
}

#[derive(Clone)]
pub struct ModuleBuilder {
    function: MaybePendingSymbol,
}
```
