# Core Query

When defining the main query for creating the AST, it is important to keep in mind that `auto_lsp` captures nodes in the order they appear in the Tree-sitter tree.

The following query works as expected:

```lisp
(document) @document
(function
    (identifier) @name) @function
```

## Duplicate nodes

If you use common nodes like **identifier**, Tree-sitter will capture them multiple times.

Given the following AST:

```rust, ignore
use auto_lsp::seq;

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
```

The core query could be written as:

```lisp
(document) @document
(function
    (identifier) @name) @function

(identifier) @identifier
```

```admonish warning
In this case, **identifier** will be captured twice, once as a **name** and once as an **identifier** — which will result in an unknown symbol error.
```

You can resolve this in two ways:

1 - **Constrain the Capture**

Use one of Tree-sitter's [`operators`](https://tree-sitter.github.io/tree-sitter/using-parsers/queries/2-operators.html) or [`predicates`](https://tree-sitter.github.io/tree-sitter/using-parsers/queries/3-predicates-and-directives.html) to constrain the capture of duplicate nodes.

2 - **Merge parts of the Query**

Remove the **name** capture, since **name** is already an **identifier**:

```lisp
(document) @document
(function) @function

(identifier) @identifier
```

## Anonymous nodes

Sometimes, Tree-sitter has anonymous nodes that are not visible in the tree or can't be captured via queries.

In this case, you can identify the part where the anonymous rules occur, add a wildcard node, and create a `#seq` node to handle it.

If a field is already defined, this makes it even easier.

```lisp
(function
    "body" (_) @body) @function

(identifier) @identifier
```

```rust, ignore
use auto_lsp::seq;

#[seq(query = "function")]
struct Function {
    body: Body,
}

#[seq(query = "body")]
struct Body {
    /* ... */
}
```

# Aliased nodes

When creating a new tree-sitter grammar, be cautious with aliased nodes.

Tree-sitter allows a single node type to represent multiple different syntax structures through aliasing. 

However, this creates a problem: you can only write one query definition per node type, and tree-sitter doesn't provide a way to determine if a node is using its primary type or an alias.

This limitation means that if you use aliased nodes, your AST might not accurately represent the different syntactic structures in your code.


[#2567](https://github.com/tree-sitter/tree-sitter/discussions/2567#discussioncomment-8144839)