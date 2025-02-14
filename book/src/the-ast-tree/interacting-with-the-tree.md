# Interacting with the tree

## Static and Dynamic Symbols

The AST tree is composed of two types of symbols:

- Static symbols: [`Symbol<T>`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/struct.Symbol.html) where T implements [`AstSymbol`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.AstSymbol.html).
- Dynamic symbols: [`DynSymbol`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/struct.DynSymbol.html)  which is a trait object that wraps a `Symbol<T>`.
- Weak symbols: [`WeakSymbol`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/struct.WeakSymbol.html) which is a weak reference to a `DynSymbol`.

Dynamic symbols implement downcasting thanks to the [`downcast_rs`](https://crates.io/crates/downcast-rs) crate.

Weak symbols can be upgraded to a dynamic symbol using the [`WeakSymbol::to_dyn`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/struct.WeakSymbol.html#method.to_dyn) method.

Static symbols offer better performance due to static dispatch and type safety, while dynamic symbols are useful for referencing symbols anywhere in the tree or performing method calls without needing to worry about the type.

## Walking the tree

While the tree does not implement iterators, it still provides methods to locate a node or walk inside:

- [`descendant_at`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.Traverse.html#tymethod.descendant_at): Find the lowest node in the tree at the given offset.
- [`descendant_at_and_collect`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.Traverse.html#tymethod.descendant_at_and_collect): Find the lowest node in the tree at the given offset and clones all nodes matching the closure's condition.
- [`traverse_and_collect`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.Traverse.html#tymethod.traverse_and_collect): Find the lowest node in the tree at the given offset and clone all nodes that match the closure's condition.

All methods that imply walking the tree will return a [`DynSymbol`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/struct.Symbol.html) that can be downcasted to the desired type.

In addition, all symbols have a [`get_parent`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetSymbolData.html#tymethod.get_parent) mmethod to retrieve the parent symbol.
Since the parent might be dropped, which could invalidate the child nodes, a `WeakSymbol` returned and must be upgraded to a `DynSymbol` when used.

```admonish warning
It is strongly discouraged to store symbols or manually edit them, as this may lead to memory leaks or an inconsistent tree.
```
