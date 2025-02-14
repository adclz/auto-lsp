# Seq macro

[`#seq`](https://docs.rs/auto-lsp/latest/auto_lsp/attr.seq.html) is used to define a sequence of nodes and it only works with structs.

Fields can be named however you want, `auto_lsp` relies on query names to build the AST.

A field can either be:
- Another struct (a nested sequence of nodes).
- An enum (a choice between multiple sequences of nodes).
- A `Vec` of structs or enums built with the same macros.
- An `Option` of a struct or enum.

A `Vec` can contain 0 or any number of elements.
Since tree sitter already defines **repeat** and **repeat1**, a **repeat1** would return an error from the tree-sitter lexer if the `Vec` is empty.

```rust, ignore
use auto_lsp::core::ast::*;
use auto_lsp::{seq, choice};

#[seq(query = "document")]
struct Document {
    // A simple field
    name: Identifier,
    // A vec of structs
    functions: Vec<Function>,
    // A vec of enums
    elements: Vec<Element>,
    // An optional field
    return_type: Option<Type>,
}

#[seq(query = "function")]
struct Function {}

#[choice]
enum Element {
     Statement(Statement),
     Expression(Expression),
}

#[seq(query = "statement")]
struct Statement {}

#[seq(query = "expression")]
struct Expression {}

#[seq(query = "type")]
struct Identifier {}

#[seq(query = "type")]
struct Type {}

```

## Seq Attributes

- `query`: The name of the query used to capture the node.

All other attributes are optional. By default `#seq` will generate an empty version of each trait.

(Since rust does not have stable [specialization](https://github.com/rust-lang/rust/issues/31844))

When an attribute is provided, the corresponding trait must be implemented manually.

To activate an attribute, just add it to any `#seq` macro parameters:

```rust, ignore
use auto_lsp::seq;
use auto_lsp::core::ast::{BuildDocumentSymbols, BuildCodeActions};
use auto_lsp::core::document_symbols_builder::DocumentSymbolsBuilder;

// Here, we tell auto_lsp that document_symbols and code_actions
// will be implemented manually.

// If an attribute is declared but no implementation is provided,
// your code won't compile.
#[seq(query = "function",
        document_symbols,
        code_actions,
    )]
struct Function {}

impl BuildDocumentSymbols for Function {
    fn build_document_symbols(&self, doc: &Document, builder: &mut DocumentSymbolsBuilder {
        /* ... */
    }
}

impl BuildCodeActions for Function {
    fn build_code_actions(&self, doc: &Document, acc: &mut Vec<lsp_types::CodeAction>) {
        /* ... */
    }
}
```

## LSP traits

- `code_actions`: [`BuildCodeActions`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetHover.html) trait.
- `code_lenses`: [`BuildCodeLenses`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildCodeLenses.html) trait.
- `completions`:[`BuildCompletionItems`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildCompletionItems.html) trait.
- `declaration`:[`GetGoToDeclaration`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetGoToDeclaration.html) trait.
- `definition`: [`GetGoToDefinition`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetGoToDefinition.html) trait.
- `document_symbols`: [`BuildDocumentSymbols`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildDocumentSymbols.html) trait.
- `hover`: [`GetHover`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.GetHover.html) trait.
- `inlay_hints`: [`BuildInlayHints`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildInlayHints.html) trait.
- `invoked_completions`:[`BuildInvokedCompletionItems`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildInvokedCompletionItems.html) trait.
- `semantic_tokens`: [`BuildSemanticTokens`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.BuildSemanticTokens.html) trait.

## Special traits

- `comment`: mark this node as a node that can potentially contain a comment.
  If the comments query is provided in the parser configuration, comments found above the node will be attached to it.

- `check`: [`Check`](https://docs.rs/auto-lsp/latest/auto_lsp/core/ast/trait.Check.html) trait.

Check is a special trait that is used to check the validity of a symbol, when check is implemented, `auto_lsp` will execute the check method to validate the symbol.

If the check method returns an error, the symbol will be considered invalid, will remain in memory until the next time the AST is parsed, and will be checked again.


```admonish
Check does not return a `Diagnostic`; instead, it uses `Err(())` to indicate an error. This design allows for creating multiple diagnostics if needed, such as specifying the location of the error and the affected nodes```
```

```rust, ignore
use auto_lsp::seq;
use auto_lsp::core::ast::Check;

#[seq(query = "document", check)]
struct Document {}

impl Check for Document {
    fn check(
        &self,
        doc: &Document,
        diagnostics: &mut Vec<lsp_types::Diagnostic>,
    ) -> Result<(), ()> {
        let source = doc.texter.text.as_bytes();
        let document_text = self.read().get_text(source);

        if document_text.starts_with("Hello, World") {
            return Ok(()
        } else {
            diagnostics.push(lsp_types::Diagnostic {
                range: self.get_lsp_range(document),
                severity: Some(lsp_types::DiagnosticSeverity::Error),
                message: "Document must start with 'Hello, World'".to_string(),
                ..Default::default()
            });
            return Err(());
        }
    }
}

```
