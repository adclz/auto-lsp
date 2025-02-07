<div align="center" style="margin-bottom: 50px">
  <h1>Auto LSP</h1>
  <p>
    <strong>A Rust crate for creating <a href="https://en.wikipedia.org/wiki/Abstract_syntax_tree">Abstract Syntax Trees</a> (AST)
and <a href="https://microsoft.github.io/language-server-protocol/">Language Server Protocol</a> (LSP) servers powered by <a href="https://tree-sitter.github.io/tree-sitter/">Tree-sitter</a> queries</strong>
  </p>


[![CI Status](https://github.com/adclz/auto-lsp/actions/workflows/ci.yml/badge.svg)](https://github.com/adclz/auto-lsp/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/auto-lsp)](https://crates.io/crates/auto-lsp)
[![docs.rs](https://img.shields.io/docsrs/auto-lsp/latest)](https://docs.rs/auto-lsp)
![Rust Version](https://img.shields.io/badge/rustc-1.83.0%2B-orange)

</div>

> `auto_lsp` is at an early stage, expect frequent breaking changes. 

# Quick Example

`auto_lsp` is designed to be as language-agnostic as possible, allowing any Tree-sitter grammar to be used.

Defining a simple AST involves two steps: writing the queries and then defining the corresponding AST structures in Rust.

Let's say you have a toy language with a root node named **document** containing a list of **function** nodes, each containing a unique **name**.

A simple query file to capture the root document and function names:

```lisp
(document) @document
(function
    (name) @name) @function
```

The corresponding AST definition in Rust:

```rust
#[seq(query = "document")]
struct Document {
   functions: Vec<Function>
}

#[seq(query = "function")]
struct Function {
   name: Name
}

#[seq(query = "name")]
struct Name {}  
```

Now that you have your AST defined, you can:
 - Implement the LSP traits and create a LSP server (with the `lsp_server` feature).
 - Add your own logic for testing purposes, code_generation, etc.

You can find more examples in the `src/tests` folder.

# Features

- `deadlock_detection`: Enable [`parking_lot`](https://crates.io/crates/parking_lot)'s deadlock detection (not compatible with `wasm`).
- `log`: Enable logging. (uses [`stderrlog`](https://crates.io/crates/stderrlog))
- `lsp_server`: Enable the LSP server (uses [`lsp_server`](https://crates.io/crates/lsp-server)).
- `rayon`: Enable [`rayon`](https://crates.io/crates/rayon) support (not compatible with `wasm`).
- `wasm`: Enable wasm support.
- `html`: Enable the python workspace mock for testing purposes.
- `python`: Enable the html workspace mock for testing purposes.
- `incremental`: Enable incremental updates.

## A note about incremental updates

While tree-sitter provides incremental updates for its own tree, updating an ast derived from this same tree is not an easy task.

Currently, tree-sitter offers several ways to know how a tree is affected by changes:

- [`has_changes`](https://docs.rs/tree-sitter/0.25.1/tree_sitter/struct.Node.html#method.has_changes) This method indicates if a node in an edited tree (before reparsing it) has changes.  However, it marks all nodes from the changed node to the root [as changed](https://github.com/tree-sitter/tree-sitter/issues/926#issuecomment-783552289).

- [`changed_ranges`](https://docs.rs/tree-sitter/0.25.1/tree_sitter/struct.Tree.html#method.changed_ranges) This method provides a list of ranges corresponding to nodes whose syntactic structures have changed.

- [`Node IDs`](https://github.com/tree-sitter/tree-sitter/discussions/800#discussioncomment-184141) Node IDs are supposed to remain stable for unchanged nodes, even after edits. However, I could not find an idiomatic way to exploit this reliably.

**changed_ranges** seems to be the most reasonable approach, but:
 
It focuses on syntactic changes only, but for instance a modified child node may not propagate updates to its parent node unless the parent node itself has a structural change. See [#3044](https://github.com/tree-sitter/tree-sitter/discussions/3044)

It only returns changes after the tree has been re-parsed with the new version of the document. This means that the returned list of changes may not align precisely with the edits received from the IDE, especially for complex edits like multi-line modifications.

`auto_lsp` uses a two-step process with vector-based updates.

1 - Range Adjustment and Deletions:

The entire AST is traversed, and the ranges of all nodes are adjusted to reflect the changes.
If a node’s range falls within the span of the new end byte and the old end byte, that node is removed.

2 - Finding the Lowest Descendant:

- The lowest descendant node affected by the change is identified.
- Using Rust 1.83’s [ControlFlow](https://doc.rust-lang.org/std/ops/enum.ControlFlow.html), the traversal continues upward through the AST until a vector is found.
- Once the vector is located:
- - The vector determines the index of the changed node.
- - It inserts or replaces nodes by executing queries at this specific location.

So far this approach seems to be effective but further testing is necessary.

# Documentation

The complete documentation will be available in the upcoming book.

# Inspirations / Similar projects

- [Volar](https://volarjs.dev/)
- [Rust-sitter](https://github.com/hydro-project/rust-sitter)
- [StackGraphs](https://github.com/github/stack-graphs)
- [airblast-dev](https://github.com/airblast-dev)'s [texter](https://github.com/airblast-dev/texter), which saved hours of headache