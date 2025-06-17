# Changelog

## [Unreleased]

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

