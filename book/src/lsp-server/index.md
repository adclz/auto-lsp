# LSP Server

`auto-lsp` utilizes [`lsp_server`](https://crates.io/crates/lsp_server) from rust analyzer and [`crossbeam`](https://docs.rs/crossbeam/latest/crossbeam/) to launch the server.

```admonish
LSP Server is only available in the `lsp_server` feature.
```

## Global State

The server's global state is managed by a [`Session`](https://docs.rs/auto-lsp/latest/auto_lsp/server/struct.Session.html).