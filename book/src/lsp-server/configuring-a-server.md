# Configuring a server

## Starting a server

To configure a server, you need to use the `create` method from the [`Session`](https://docs.rs/auto-lsp/latest/auto_lsp/server/struct.Session.html) struct wich takes 2 arguments.

- `Parsers`: A list of parsers (previously defined with the [`configure_parsers!`](/workspace-and-document/configuring-parsers.html) macro)
- `LspOptions`: Options to configure the LSP server, see [LSP Options](#lsp-options).

To start a new [`Session`](https://docs.rs/auto-lsp/latest/auto_lsp/server/struct.Session.html), you need to provide the InitOptions struct.

TThe server communicates with an LSP client using one of lsp_server's tranport methods: `stdio`, `tcp` or `memory`.

```rust, ignore
use std::error::Error;
use auto_lsp::server::{InitOptions, LspOptions, Session};
use auto_lsp::python::PYTHON_PARSERS;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();

    let mut session = Session::create(
        InitOptions {
            parsers: &PYTHON_PARSERS,
            lsp_options: LspOptions {
                ..Default::default()
            },
        },
        connection,
    )?;

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
