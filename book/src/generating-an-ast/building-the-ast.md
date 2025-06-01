# Building an AST

The generated AST includes:

- Structs representing AST nodes.
- Implementations of the AstNode trait.
- A TryFrom implementation to build nodes from Tree-sitter.

Building an AST requires a `salsa::Database`, which is used to accumulate errors during parsing.

## Using TryFrom

Each AST node type provides a `TryFrom` implementation that accepts a `TryFromParams` tuple. This is used to convert a Tree-sitter node into an AST node.

```rust, ignore
/// Parameters passed to `TryFrom` implementations for AST nodes.
pub type TryFromParams<'from> = (
    &'from Node<'from>,         // Tree-sitter node
    &'from dyn salsa::Database, // Salsa database
    &'from mut Builder,         // AST builder
    usize,                      // Node ID (auto-incremented by the builder)
    Option<usize>,              // Optional parent node ID
);
```

### Example: Building a root node

```rust, ignore
// Create the AST builder
let mut builder = auto_lsp::core::ast::Builder::default();

// Build the root node from the Tree-sitter parse tree
let root = ast::generated::SourceFile::try_from((
    &tree.root_node(),
    db,            // Your salsa database
    &mut builder,
    0,             // Root node ID
    None,          // Root has no parent
))?;

// Retrieve all non-root nodes from the builder
let mut nodes = builder.take_nodes();

// Add the root node manually
nodes.push(std::sync::Arc::new(root));

// Optional: Sort the nodes by ID
nodes.sort_unstable();
```

## Retrieving Errors

Errors that occur during AST construction are accumulated using the `ParseErrorAccumulator` struct. This allows partial AST construction even when some nodes fail to parse.

It’s recommended to use `TryFrom` inside a `salsa::tracked` function so you can retrieve errors using `salsa::accumulated`.

The default crate provides a `get_ast` query that builds the AST and collects errors. It is compatible with `BaseDatabase`.

### ParseError Structure

The `ParseError` enum represents errors that can be encountered during parsing.

```rust, ignore
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("{error:?}")]
    LexerError {
        range: lsp_types::Range,
        #[source]
        error: LexerError,
    },
    #[error("{error:?}")]
    AstError {
        range: lsp_types::Range,
        #[source]
        error: AstError,
    },
}
```

- LexerError — Issues from Tree-sitter's lexer
- AstError — Issues from a TryFrom implementation

You can retrieve lexer errors via get_tree_sitter_errors() from the `default` crate.

`LexerError` can either be a missing symbol error or a syntax error.

```rust, ignore
#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum LexerError {
    #[error("{error:?}")]
    Missing {
        range: lsp_types::Range,
        error: String,
    },
    #[error("{error:?}")]
    Syntax {
        range: lsp_types::Range,
        error: String,
    },
}
```

## ParsedAst struct

The result of `get_ast` is a `ParsedAst` struct, which holds the list of AST nodes and implements `Deref` for direct iteration.

```rust, ignore
pub struct ParsedAst {
    pub nodes: Arc<Vec<Arc<dyn AstNode>>>,
}
```

You can work with AST nodes in two ways:
- Downcast a node to a concrete type and access its fields.
- Iterate over all nodes and filter or match on their type.

### Methods

 - `get_root`: Returns the root node.
 - `descendant_at`: Returns the first node that contains the given offset.

### Example: Filtering nodes by type

```rust, ignore
let functions = get_ast(db file)
    .iter()
    .filter_map(|node| node.is::<FunctionDefinition>())
    .collect();
```

For convenience when calling methods on multiple node types, use the `dispatch` or `dispatch_once` macros.
See [Dispatch Pattern](../patterns/dispatch.md).
