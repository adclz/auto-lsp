# The AstNode Trait

The `AstNode` trait is the core abstraction for all AST nodes in auto-lsp.

## Definition

The `AstNode` trait is implemented by all generated AST types. It extends:

- `Debug + Clone + Send + Sync` — for thread safety and logging
- `PartialEq + Eq + PartialOrd + Ord` — nodes can be sorted or compared
- `Downcast` — enables safe casting to concrete node types

Each AST node has a unique identifier, generated during the Tree-sitter traversal. This ID is used to implement comparison traits.

Eq is based on the unique ID and the range of the node, although comparing Arc pointers should be preferred because comparing 2 nodes of different trees might yield false negatives.

## Downcasting to Concrete Types

The `AstNode` trait supports safe downcasting to concrete types through the `Downcast` trait from the `downcast_rs` crate.

### With is::<T>()

```rust, ignore
if node.is::<FunctionDefinition>() {
    println!("It's a function!");
}
```

### With downcast_ref::<T>()

```rust, ignore
// Attempt to downcast to a specific type
if let Some(function) = node.downcast_ref::<FunctionDefinition>() {
    // Work with the concrete FunctionDefinition type
    println!("Function name: {}", function.name);
}
```

## Pattern Matching with Downcasting

```rust, ignore
match pass_statement.downcast_ref::<CompoundStatement_SimpleStatement>() {
    Some(CompoundStatement_SimpleStatement::SimpleStatement(
        SimpleStatement::PassStatement(PassStatement { .. }),
    )) => {
        // Successfully matched a `pass` statement
    },
    _ => panic!("Expected PassStatement"),
}
```