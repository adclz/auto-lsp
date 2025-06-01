# Logging and Tracing

auto-lsp uses [`fastrace`](https://docs.rs/fastrace/latest/fastrace/) and [`log`](https://docs.rs/log/latest/log/) for tracing and logging.

## Logging

To enable logging, you can use any logger that implements the `log` crate.

For example, you can use `stderrlog` to log to stderr.

```rust, ignore
stderrlog::new()
    .modules([module_path!(), "auto_lsp"])
    .verbosity(4)
    .init()
    .unwrap();
```

## Tracing

To enable tracing, follow the instructions in the [fastrace](https://docs.rs/fastrace/latest/fastrace/) documentation.