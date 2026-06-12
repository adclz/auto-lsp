# Changelog

## [Unreleased]

## [0.2.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-default-v0.1.2...auto-lsp-default-v0.2.0)

### Features

- *(db)* Add File struct with builders and update methods - ([df51762](https://github.com/adclz/auto-lsp/commit/df51762b814209842bfaf16d5b7af951a7c0e9e0))
- *(default)* Add optional durability parameter to file creation methods - ([1be79e3](https://github.com/adclz/auto-lsp/commit/1be79e37ab78d06ac099dd7f89ff760eccc03d47))
- *(default)* Add callback support for file addition and removal - ([b6c6b2c](https://github.com/adclz/auto-lsp/commit/b6c6b2ce3de0d1fc16c5ccac7ae7af0a82d1ab0a))
- *(error)* Add optional error callback support in session when panic occurs in a request/notification - ([9fe1061](https://github.com/adclz/auto-lsp/commit/9fe1061f422a49d9b0420512c76c1ebe82d3ef5d))
- Add position encoding support to Document and Db - ([dd1f4e6](https://github.com/adclz/auto-lsp/commit/dd1f4e6a90451cdd8ee5b0b466650828e51bca2d))

### Bug Fixes

- *(default)* Use partition_point for descendant_at - ([7419fa8](https://github.com/adclz/auto-lsp/commit/7419fa837f16a312c8d7fb9d36d57626e1364e32))
- Correct plural -> singular - ([227c726](https://github.com/adclz/auto-lsp/commit/227c7260a55ec698f478a408bb32ae32204e007a))
- Update parser retrieval to use extension instead of language ID - ([c751684](https://github.com/adclz/auto-lsp/commit/c7516845e1c6468cc9c8598844c29f6c143cffa5))

### Refactor

- *(core)* Use new nomalize and denomalize fns from texter 0.3.0 - ([8af7712](https://github.com/adclz/auto-lsp/commit/8af77125e9713aea63bb326b2a97af2262b0c4a9))
- *(default)* Pass parser reference directly to file fns - ([692102c](https://github.com/adclz/auto-lsp/commit/692102ce032fe816bfa90efa3d98627af0b57a50))
- *(default)* Pass closure to decide if file should be handled to workspace_init fn - ([f186743](https://github.com/adclz/auto-lsp/commit/f186743ef0e1c0c3943a433f4b3daded23e5e8a0))
- *(default)* Refactor descendant_at to descendant_for_position - ([58615ff](https://github.com/adclz/auto-lsp/commit/58615ff066494bf94d5a80ad15e30d43791673d6))
- *(default)* Add debug on file input and replace obsolete outer attributes - ([4d6fc14](https://github.com/adclz/auto-lsp/commit/4d6fc146b5d0659332ba94363bdb7043fec2ef61))
- *(default)* Update file input events - ([422a7e5](https://github.com/adclz/auto-lsp/commit/422a7e5715f8e377f595330dfb31a4f245744e39))
- *(default)* Simplify file events and workspace init - ([97984f5](https://github.com/adclz/auto-lsp/commit/97984f5f6afa5b1330868a2b03596cefc1b5c253))
- *(document)* Update Document constructor to use source strings directly - ([8d639be](https://github.com/adclz/auto-lsp/commit/8d639be29c20302400b8e84921d980c4cd129e6a))
- *(errors)* Replace lsp_types::Range with Span in ParseError and LexerError - ([121c8a5](https://github.com/adclz/auto-lsp/commit/121c8a5c761c290e1c6a39f3834ccd164e258132))
- *(file)* Remove redundant extension registration check - ([7c11599](https://github.com/adclz/auto-lsp/commit/7c115995dd481e9be13710af835c2be05193d21d))
- Replace extensions with parsers in session initialization - ([bb0dd3b](https://github.com/adclz/auto-lsp/commit/bb0dd3bc12ba9062ce8f5c50c3af21fb326fa50e))

### Testing

- Add unit tests for file content comparison - ([2c7307c](https://github.com/adclz/auto-lsp/commit/2c7307c5afb1ec21c8b667a0827fe20f15bb670d))

### Miscellaneous Tasks

- Remove unnecessary deps - ([c8148d4](https://github.com/adclz/auto-lsp/commit/c8148d45beaa4d8f5f7796e0f020f8ed8b0f240b))
- Update dependencies - ([33b1858](https://github.com/adclz/auto-lsp/commit/33b185814bb510a9d00f3939ecdd0c485eaba1d1))
- Bump deps and use rust 2024 edition ([#28](https://github.com/adclz/auto-lsp/pull/28)) - ([981f658](https://github.com/adclz/auto-lsp/commit/981f6582466f1042b68f94872ae1649b8a0bdcb7))

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

