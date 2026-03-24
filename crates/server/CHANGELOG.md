# Changelog

## [Unreleased]

## [0.2.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-server-v0.1.2...auto-lsp-server-v0.2.0)

### Features

- *(error)* Add optional error callback support in session when panic occurs in a request/notification - ([9fe1061](https://github.com/adclz/auto-lsp/commit/9fe1061f422a49d9b0420512c76c1ebe82d3ef5d))
- *(server)* Implement thread pool and QoS management from rust-analyzer - ([125137b](https://github.com/adclz/auto-lsp/commit/125137b0080d5c918e61289a287c67c7c030096a))
- Add position encoding support to Document and Db - ([dd1f4e6](https://github.com/adclz/auto-lsp/commit/dd1f4e6a90451cdd8ee5b0b466650828e51bca2d))

### Refactor

- *(default)* Update file input events - ([422a7e5](https://github.com/adclz/auto-lsp/commit/422a7e5715f8e377f595330dfb31a4f245744e39))
- *(document)* Update Document constructor to use source strings directly - ([8d639be](https://github.com/adclz/auto-lsp/commit/8d639be29c20302400b8e84921d980c4cd129e6a))
- *(server)* Remove perFileParser handling from initialization - ([333da96](https://github.com/adclz/auto-lsp/commit/333da965d5e4a3ef4100c52123d6add5503e518d))
- *(server)* Replace rayon TaskPool with new thread Pool - ([b168c60](https://github.com/adclz/auto-lsp/commit/b168c60baf09d322d502e86aba7db7928d31f04b))
- *(server)* Make TaskPool and Task public - ([6308ab8](https://github.com/adclz/auto-lsp/commit/6308ab848167f6cadab397e8e38594a5df832568))
- Replace extensions with parsers in session initialization - ([bb0dd3b](https://github.com/adclz/auto-lsp/commit/bb0dd3bc12ba9062ce8f5c50c3af21fb326fa50e))

### Miscellaneous Tasks

- Remove vscode-wasi references and related configurations - ([2c056d5](https://github.com/adclz/auto-lsp/commit/2c056d5bbc8e229214ee360e027f36995c10bd96))
- Bump deps and use rust 2024 edition ([#28](https://github.com/adclz/auto-lsp/pull/28)) - ([981f658](https://github.com/adclz/auto-lsp/commit/981f6582466f1042b68f94872ae1649b8a0bdcb7))


## [0.1.2](https://github.com/adclz/auto-lsp/compare/auto-lsp-server-v0.1.1...auto-lsp-server-v0.1.2)

### Bug Fixes

- *(server)* Fallback to default value if available_parallelism is unavailable - ([8c23b5c](https://github.com/adclz/auto-lsp/commit/8c23b5c464a86fb99e5cea5828aef43d8945b037))


## [0.1.1](https://github.com/adclz/auto-lsp/compare/auto-lsp-server-v0.1.0...auto-lsp-server-v0.1.1)

### Miscellaneous Tasks

- Updated the following local packages: auto-lsp-core - ([0000000](https://github.com/adclz/auto-lsp/commit/0000000))


## [0.1.0]

### Bug Fixes

- Remove unused dependencies from Cargo.toml files - ([141bcf9](https://github.com/adclz/auto-lsp/commit/141bcf9ae6d835d6a1d6e3f3d6563cb15b65afed))

### Refactor

- Split server and database modules into separate crates - ([1f768f1](https://github.com/adclz/auto-lsp/commit/1f768f12695e1ca2001bd1e1964a3528f71ac26b))

### Documentation

- Add README files for default and server crates - ([c9a44d6](https://github.com/adclz/auto-lsp/commit/c9a44d61052a139be4f12b51bf6e98725478eba2))

