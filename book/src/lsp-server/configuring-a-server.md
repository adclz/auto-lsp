# Configuring a server

```admonish
LSP Server is only available in the `lsp_server` feature.
```

## Starting a server

`auto-lsp` uses [`lsp_server`](https://crates.io/crates/lsp_server) from rust analyzer and [`crossbeam`](https://docs.rs/crossbeam/latest/crossbeam/) to launch the server.

To configure the `lsp_server`, you need to use the `create` method from the [`Session`](https://docs.rs/auto-lsp/latest/auto_lsp/server/struct.Session.html) struct wich takes 2 arguments.

- `Parsers`: A list of parsers (previously defined with the [`configure_parsers!`](/workspace-and-document/configuring-parsers.html) macro)
- `LspOptions`: Options to configure the LSP server, see [LSP Options](#lsp-options).

```rust
To start a session, you need to provide the InitOptions struct.

```rust
use std::error::Error;
use auto_lsp::server::{InitOptions, LspOptions, Session};
use auto_lsp::python::PYTHON_PARSERS;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut session = Session::create(InitOptions {
        parsers: &PYTHON_PARSERS,
        lsp_options: LspOptions {
            document_symbols: true,
            diagnostics: true,
            ..Default::default()
        },
    })?;

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop()?;
    session.io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}
```

## LSP Options

The [`LspOptions`](https://docs.rs/auto-lsp/latest/auto_lsp/server/struct.LspOptions.html) struct contains various settings to enable or disable different LSP features like diagnostics, document symbols, and more.

Depending on how your AST is structured, all requests are fullfiled automatically.

Just 2 options require specific implementations:

### Document Links

Configuring Document Links requires a [`RegexToDocumentLink`](https://docs.rs/auto-lsp/latest/auto_lsp/server/struct.RegexToDocumentLink.html) struct.

```rust
use auto_lsp::server::{RegexToDocumentLink, Session};
use auto_lsp::core::document::Document;
use auto_lsp::core::workspace::Workspace;
use auto_lsp::lsp_types::{DocumentLink, Url};
use auto_lsp::regex::Regex;

let regex = Regex::new(r"(\w+):(\d+)").unwrap();

fn to_document_link(m: regex::Match, line: usize, document: &Document, workspace: &Workspace, acc: &mut Vec<DocumentLink>) -> lsp_types::DocumentLink {
   lsp_types::DocumentLink {
        data: None,
        tooltip: Some(m.as_str().to_string()),
        target:None,
        range: lsp_types::Range {
                    start: lsp_types::Position {
                        line: line as u32,
                        character: m.start() as u32,
                    },
                    end: lsp_types::Position {
                        line: line as u32,
                        character: m.end() as u32,
                    },
               },
         }
   }

RegexToDocumentLink {
    regex,
    to_document_link,
};

```

### Semantic Tokens

Semantic Tokens that are defined previously with the [`define_semantic_token_types!`](workspace-and-document/configuring-semantic-tokens.md)
and `define_semantic_token_modifiers!` macros
must be provided to the LSP Server.

```rust
use auto_lsp::lsp_types::SemanticTokenType;
use auto_lsp::define_semantic_token_types;
use phf::phf_map;

define_semantic_token_types! {
    standard {
         "namespace" => NAMESPACE,
         "type" => TYPE,
         "function" => FUNCTION,
    }
}

define_semantic_token_modifiers![standard {
    "declaration" => DECLARATION,
    "readonly" => READONLY,
}];

let lsp_options = LspOptions {
    semantic_tokens: Some (SemanticTokensList {
        token_types: &TOKEN_TYPES,
        token_modifiers: &TOKEN_MODIFIERS,
    }),
    ..Default::default()
},

```
