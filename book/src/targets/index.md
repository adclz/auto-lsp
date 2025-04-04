# Targets

`auto-lsp` has been tested on both windows and linux targets.

If you plan to use WebAssembly, you can use the vscode [`Wasi Lsp`](https://code.visualstudio.com/blogs/2024/06/07/wasm-part2) wich runs on [`was32-wasip1-threads`](https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1-threads.html) target.
You'll also need to enable the `wasm` feature.

Note that some functionalities, such as deadlock detection are not available on WebAssembly.

```admonish
 - [Simple LSP Server](https://github.com/adclz/auto-lsp/tree/main/examples/native)
 - [Vscode extension with WASI](https://github.com/adclz/auto-lsp/tree/main/examples/vscode-wasi)
```
