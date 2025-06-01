# Configuring a server

## Pre-requisites

The server is generic over a `salsa::Database`, so you need to implement a database before starting the server.

You can use the default `BaseDb` database provided by `auto_lsp` or create your own.

The `default` module in server contains a file storage and file event handlers and workspace loading logic that are compatible with the `BaseDatabase` trait.

If you create your own database, you will have to create your own file storage and file event handlers.

## Configuring

To configure a server, you need to use the `create` method from the [`Session`](https://docs.rs/auto-lsp/latest/auto_lsp/server/struct.Session.html) struct wich takes 4 arguments.

- `parsers`: A list of parsers (previously defined with the [`configure_parsers!`](/workspace-and-document/configuring-parsers.html) macro)
- `capabilities`: Server capabilities, see [ServerCapabilities](https://docs.rs/lsp-types/latest/lsp_types/struct.ServerCapabilities.html).
- `server_info`: Optional information about the server, such as its name and version, see [ServerInfo](https://docs.rs/lsp-types/latest/lsp_types/struct.ServerInfo.html).
- `connection`: The connection to the client, see [Connection](https://docs.rs/lsp-server/latest/lsp_server/struct.Connection.html).
- `db`: The database to use, it must implement `salsa::Database`.

`create` will return a tuple containing the `Session` and the [InitializeParams](https://docs.rs/lsp-types/latest/lsp_types/struct.InitializeParams.html) sent by the client.

The server communicates with an LSP client using one of lsp_server's tranport methods: `stdio`, `tcp` or `memory`.

```rust, ignore
use std::error::Error;
use auto_lsp::server::{InitOptions, Session, ServerCapabilities};
use ast_python::db::PYTHON_PARSERS;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Enable logging and tracing, this is optional
    stderrlog::new()
        .modules([module_path!(), "auto_lsp"])
        .verbosity(4)
        .init()
        .unwrap();

    fastrace::set_reporter(ConsoleReporter, Config::default());

    // Server options
    let options = InitOptions {
        parsers: &PYTHON_PARSERS,
        capabilities: ServerCapabilities {
            ..Default::default()
        },
        server_info: None,
    };
    // Create the connection
    let (connection, io_threads) = Connection::stdio();
    // Create a database, either BaseDb or your own
    let db = BaseDb::default();

    // Create the session
    let (mut session, params) = Session::create(
        options,
        connection,
        db,
    )?;

    // This is where you register your requests and notifications
    // See the handlers section for more information
    let mut request_registry = RequestRegistry::<BaseDb>::default();
    let mut notification_registry = NotificationRegistry::<BaseDb>::default();

    // This will add all files available in the workspace.
    // The init_workspace is only available for databases that implement BaseDatabase or BaseDb
    let init_results = session.init_workspace(params)?;
    if !init_results.is_empty() {
        init_results.into_iter().for_each(|result| {
            if let Err(err) = result {
                eprintln!("{}", err);
            }
        });
    };

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    session.main_loop(
        &mut request_registry,
        &mut notification_registry,
    )?;
    session.io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}
```

## Default Capabilities

`auto_lsp` provides helper methods to configure some default capabilities.

### Semantic Tokens

The `semantic_tokens_provider` method will configure the `semantic_tokens_provider` field of the `ServerCapabilities` struct.

Parameters:
 - `range`: Whether the client supports to send requests for semantic tokens for a specific range.
 - `token_types`: The list of token types that the server supports.
 - `token_modifiers`: The list of token modifiers that the server supports.

```rust, ignore
use auto_lsp::server::semantic_tokens_provider;

let capabilities = ServerCapabilities {
    semantic_tokens_provider: semantic_tokens_provider(false, Some(SUPPORTED_TYPES), Some(SUPPORTED_MODIFIERS)),
    ..Default::default()
};
```

```admonish
Except for semantic tokens, these default capabilities are only available if you use the `BaseDb` database.
```


### Text Document Sync

Since the `Document` supports incremental updates, the `text_document_sync` field of the `ServerCapabilities` struct is configured to `INCREMENTAL` by default.

You can use the `TEXT_DOCUMENT_SYNC` constant to configure it.

```rust, ignore
use auto_lsp::server::TEXT_DOCUMENT_SYNC;

let capabilities = ServerCapabilities {
    text_document_sync: TEXT_DOCUMENT_SYNC.clone(),
    ..Default::default()
};
```

This is meant to be used for the default handler `open_text_document` to work.

### Workspace Provider

The `WORKSPACE_PROVIDER` constant will configure the `workspace` field of the `ServerCapabilities` struct.

```rust, ignore
use auto_lsp::server::WORKSPACE_PROVIDER;

let capabilities = ServerCapabilities {
    workspace: WORKSPACE_PROVIDER.clone(),
    ..Default::default()
};
```

This is meant to be used for the default handler `changed_watched_files` to work.

#### Workspace initialization

When using `BaseDb` as the database, the `init_workspace` method will load all files in the workspace and associate them with a parser.

It will also send diagnostics for all files.

If you want to customize this behavior, you can implement your own `init_workspace` method and call it instead of the default one.

```rust, ignore
use auto_lsp::server::Session;

let (mut session, params) = Session::create(
    options,
    connection,
    db,
)?;

// This will add all files available in the workspace.
let init_results = my_init_workspace(&mut session, params)?;
if !init_results.is_empty() {
    init_results.into_iter().for_each(|result| {
        if let Err(err) = result {
            eprintln!("{}", err);
        }
    });
};
```
