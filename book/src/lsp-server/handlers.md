# Handlers

All LSP requests and notifications must be registered before calling `main_loop`.

Handlers are registered using the `RequestRegistry` and `NotificationRegistry` structs. Both store handlers in internal HashMaps, using the method name as the key. When a request or notification is received, the corresponding handler is looked up and invoked based on the method name.

Handler callbacks receive two parameters:

- **session**: The global state of the server.
- **parameters**: The request or notification parameters.

Both registries implement `Default`, but require a s`alsa::Database` type parameter.

## Adding Handlers

The `RequestRegistry` and `NotificationRegistry` structs provide two methods to register a handler:
- .on: Executes the handler in a separate thread. This is cancelable.
- .on_mut: Executes the handler synchronously with mutable access to the session.

```rust, ignore
use capabilities::handle_document_symbols;
use capabilities::handle_folding_ranges;
use capabilities::handle_watched_files;
use auto_lsp::lsp_types::notification::DidOpenTextDocument;
use auto_lsp::lsp_types::request::{DocumentSymbolRequest, FoldingRangeRequest};
use auto_lsp::server::{NotificationRegistry, RequestRegistry};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    /* ... */

    let mut request_registry = RequestRegistry::<BaseDb>::default();
    let mut notification_registry = NotificationRegistry::<BaseDb>::default();

    request_registry
        // read only, will be executed in a separate thread
        .on::<DocumentSymbolRequest, _>(handle_document_symbols)
        .on::<FoldingRangeRequest, _>(handle_folding_ranges);

    notification_registry
        // mutable because we need to update the database
        .on_mut::<DidChangeWatchedFiles, _>(handle_watched_files);

    /* ... */
}
```

## Custom Request

You can define your own request types by implementing the [`Request`](https://docs.rs/lsp-types/latest/lsp_types/request/trait.Request.html) trait from `lsp_types`.

```rust, ignore
pub struct GetWorkspaceFilesUris {}

impl Request for GetWorkspaceFilesUris {
    type Params = (); // Parameters for the request
    type Result = Vec<String>; // Expected response type
    const METHOD: &'static str = "custom/GetWorkspaceFilesUris"; // Method name used in the request
}
```

Similarly, to define a custom notification, implement the [`Notification`](https://docs.rs/lsp-types/latest/lsp_types/notification/trait.Notification.html) trait instead of `Request`.

## Default Handlers

The `default` crates provides handlers for several LSP requests and notifications.

```admonish
These default handlers are available only if you use the `BaseDb` database.
```

#### changed_watched_files

This notification is handled by the `changed_watched_files` function in the `default` crate. It updates files in the workspace when external changes are detected.

To enable this handler, use the `WORKSPACE_PROVIDER` constant when configuring the server capabilities.

#### open_text_document

This notification is handled by the `open_text_document` function in the default module. It ensures that the file is added to the workspace if not already present.

To enable this handler, use the `TEXT_DOCUMENT_SYNC` constant during server capabilities configuration.
