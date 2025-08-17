# Changelog

## [Unreleased]

## [0.2.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-default-v0.1.2...auto-lsp-default-v0.2.0)

### Features

- *(db)* Add File struct with builders and update methods - ([df51762](https://github.com/adclz/auto-lsp/commit/df51762b814209842bfaf16d5b7af951a7c0e9e0))
- Add position encoding support to Document and Db - ([dd1f4e6](https://github.com/adclz/auto-lsp/commit/dd1f4e6a90451cdd8ee5b0b466650828e51bca2d))

### Refactor

- *(default)* Add debug on file input and replace obsolete outer attributes - ([4d6fc14](https://github.com/adclz/auto-lsp/commit/4d6fc146b5d0659332ba94363bdb7043fec2ef61))
- *(default)* Update file input events - ([422a7e5](https://github.com/adclz/auto-lsp/commit/422a7e5715f8e377f595330dfb31a4f245744e39))
- *(default)* Simplify file events and workspace init - ([97984f5](https://github.com/adclz/auto-lsp/commit/97984f5f6afa5b1330868a2b03596cefc1b5c253))
- *(document)* Update Document constructor to use source strings directly - ([8d639be](https://github.com/adclz/auto-lsp/commit/8d639be29c20302400b8e84921d980c4cd129e6a))
- *(errors)* Replace lsp_types::Range with Span in ParseError and LexerError - ([121c8a5](https://github.com/adclz/auto-lsp/commit/121c8a5c761c290e1c6a39f3834ccd164e258132))

### Testing

- Add unit tests for file content comparison - ([2c7307c](https://github.com/adclz/auto-lsp/commit/2c7307c5afb1ec21c8b667a0827fe20f15bb670d))

### Bench

- Add codspeed + divan crate ([#24](https://github.com/adclz/auto-lsp/pull/24)) - ([9b98812](https://github.com/adclz/auto-lsp/commit/9b988120f4e086047c039c7a6c526c8348cd6054))


## [0.1.2](https://github.com/adclz/auto-lsp/compare/auto-lsp-default-v0.1.1...auto-lsp-default-v0.1.2)

### Features

- *(errors)* Add additional fields to LexerError - ([0a19465](https://github.com/adclz/auto-lsp/commit/0a194651f158a520594e941e5953e1462c1b7bee))

### Bug Fixes

- *(file_events)* Ignore non file:// scheme URIs - ([3dcc8db](https://github.com/adclz/auto-lsp/commit/3dcc8db9b21bb9bf69053d9ed4e50bf79ae0fd84))


## [0.1.1](https://github.com/adclz/auto-lsp/compare/auto-lsp-default-v0.1.0...auto-lsp-default-v0.1.1)

### Bug Fixes

- *(default)* Remove debug output in syntax error message - ([7957370](https://github.com/adclz/auto-lsp/commit/7957370bcef7e37a39789489a2692576a0443a94))

### Refactor

- *(default)* Use binary search for descendant_at method - ([e03131f](https://github.com/adclz/auto-lsp/commit/e03131f6d78a05189ed42c3d8dcf2b6fc51abf51))
- *(default)* Move sort_unstable to ParsedAst constructor - ([6454f17](https://github.com/adclz/auto-lsp/commit/6454f1767958a7586cbab172d4a778b49c9f4528))
- Improve Document API with as_str and as_bytes methods - ([100fb16](https://github.com/adclz/auto-lsp/commit/100fb161f24ab255f0465535abc120d5869f376b))


## [0.1.0]

### Refactor

- *(db)* Update salsa to 0.22 - ([2c17aae](https://github.com/adclz/auto-lsp/commit/2c17aae321a8e40e5ff70fc5640cb9ced5e45bcc))
- Split server and database modules into separate crates - ([1f768f1](https://github.com/adclz/auto-lsp/commit/1f768f12695e1ca2001bd1e1964a3528f71ac26b))

### Documentation

- Add README files for default and server crates - ([c9a44d6](https://github.com/adclz/auto-lsp/commit/c9a44d61052a139be4f12b51bf6e98725478eba2))

