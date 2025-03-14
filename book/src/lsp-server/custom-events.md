# Custom events

Custom lsp requests or notifications can be registered using the `register_request` or `register_notification` methods of `Session`.

Both methods accept a callback as the second parameter, which is executed to fulfill the event.

The callback methods take two parameters:
 - `Session`: The global state of the server.
 - `Parameters`: The request or notification parameters.

```admonish
Registering a request or notification with the same method name as an existing one in the server will override the previous registration. 

This can be useful if you plan to extend or modify certain LSP server features, but it requires careful attention.
```

To add a request, implement the `Request` trait from `lsp_types`:

```rust, ignore
pub struct GetWorkspaceFilesUris {}

impl Request for GetWorkspaceFilesUris {
    type Params = (); // Parameters for the request
    type Result = Vec<String>; // Expected response type
    const METHOD: &'static str = "custom/GetWorkspaceFilesUris"; // Method name used in the request
}
```

Similarly, to add a notification, implement the `Notification` trait instead of `Request`.


## Example: Registering a Custom Request

```rust, ignore
use auto_lsp::lsp_server::Connection;
use auto_lsp::python::PYTHON_PARSERS;
use auto_lsp::server::{InitOptions, LspOptions, Session};
use auto_lsp::lsp_types::request::Request;

pub struct GetWorkspaceFilesUris {}

impl Request for GetWorkspaceFilesUris {
    type Params = ();
    type Result = Vec<String>;
    const METHOD: &'static str = "custom/GetWorkspaceFilesUris";
}

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

    session.register_request::<GetWorkspaceFilesUris, _>(|session, _| {
        let guard = session.get_workspace();
        Ok(guard.roots.iter().map(|uri| uri.0.to_string()).collect())
    });

    // Run the server and wait for both threads to terminate (typically triggered by an LSP Exit event).
    session.main_loop()?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down server");
    Ok(())
}
```