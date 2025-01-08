# Auto lsp

A rust crate to facilitate the creation of [Language Servers](https://microsoft.github.io/language-server-protocol/) for [tree sitter](https://tree-sitter.github.io/tree-sitter/) grammars.

Inspired from
 - [rust-sitter](https://github.com/hydro-project/rust-sitter)
 - [github stack graphs](https://github.com/github/stack-graphs)


Tree-sitter is an incredible parser generator, however despite the huge amount of grammars available online, it's often frustrating to see we can't take full advantage of it's features.

Many people have tried to create LSP in the past using tree-sitter, including Github [themselves](https://github.com/github/semantic/commit/0edef40b998d339b50abf597579c894e072f8fc0), this crate is just another attempt to solve this problem using a slighly different approach.

Instead of creating a new DSL or change tree-sitter's behavior, this crate allows you to define a strong-typed AST structure using rust types, and then instantiate the AST with tree sitter queries.

Auto lsp does not use the full CST nodes, but relies on queries instead.

## Difference with rust-sitter

rust-sitter creates both the tree-sitter grammar and the strong typed AST using rust macros.
auto_lsp can work with any existing grammar and uses queries to fetch data, allowing to create a fully functionnal AST with just a few symbols at the beginning without having to write everything all at once.

## Difference with stackgraphs

stackgraphs defines a query langage based on actual tree sitter queries to generate an AST with scopes.
auto_lsp won't do this work for you but instead will let you implement your logic to check if a node should be used, using both helper functions and downcast.

# Syntax

Auto-Lsp use rust primitives to define the AST schema.
The major tree sitters actions are correlated to a primitive type. 

## seq

To define a sequence of elements, use a rust struct with the ast_struct macro and at least the query_name and kind attributes.

```rust
#[ast_struct(query_name = "function"), kind(symbol())]
struct Function {
    name: Name
}
``` 

Auto lsp do not use field names to populate the ast but the query names of field types, therefore you can use any field name you want.

## repeat

A repeating sequence is created through a Vec.

You can't set a limitation such as repeat1 or + as tree-sitter should take care of it the underlying tree-sitter grammar.

In other words if repeat1 condition is not met because the vec is empty, the symbol won't be built because tree-sitter will detect the missing node.

```rust
#[ast_struct(query_name = "function"), kind(symbol())]
struct Function {
    variables: Vec<Variables>
}
``` 

## optional

An optionnal field is defined with Option.

```rust
#[ast_struct(query_name = "function"), kind(symbol())]
struct Function {
    ret_type: Option<Type>
}
``` 

## choice

```rust
#[ast_enum]
enum Type {
    Int(IntType),
    Word(WordType)
}

```

# Macros

## [ast_struct]

ast_struct has **query_name** and **kind** as mandatory attributes.
While query_name is obvious, kind defines how the struct should behave.

### symbol

When symbol is used, the struct will behave as a normal node in the tree. 
Then, you can select the features you whish to implement for this node.

#### lsp_document_symbols



### accessor

When accessor is used, the struct will behave as a "reference type" that forces the users to implement the **Accessor** trait.
Accessor structs store a weak pointer to another node in the ast.

The Accessor trait is responsible to tell auto_lsp how to find and confirm a correct reference has been found in the AST.


### features

## [ast_enum]

## Runtime error messages

auto_lsp provides runtime error messages for the following scenarios:

 - In case of a query defined in tree-sitter but not in your AST, auto-lsp would highlight the location of the node in the corresponding text file with the following message:
 **"Failed to create builder for query: _query_, is the symbol declared in the AST ?"**.


 - If some fields, outside of vec and options, have not been populated despite being mandatory, you'll get the following error:
 **Failed to downcast _fieldbuilder_ into _field_** 
 While this error is not user friendly, if a field is mandatory in your grammar tree sitter should be not able to parse the parent node if this field is not present.


## Inlay Hints

The lsp_inlay_hint feature of the ast_struct macro has a debug mode.
This debug mode displays an inlay hint with  query name next to the node found.
This is useful when writing your first queries and you want to make sure every node is at the right location.

# Deploy

This crate has been tested on:
 - The [@vscode/wasi](https://github.com/microsoft/vscode-wasm) sdk from Microsoft and the [wasm32-wasip1-threads](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1-threads.html) target.
 - x86_64-unknown-linux-gnu 

Building WASM requires rust nightly due to current limitations of **parking_lot**.

# Debug

Many parts of auto_lsp can be tested without the need of a full LSP process and an IDE.
Check out the implemented tests.