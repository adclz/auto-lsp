# Changelog

## [Unreleased]

## [0.2.2](https://github.com/adclz/auto-lsp/compare/auto-lsp-codegen-v0.2.1...auto-lsp-codegen-v0.2.2)

### Features

- *(codegen)* Add is_missing field to node codegen - ([f532143](https://github.com/adclz/auto-lsp/commit/f5321439a44107b31033fd77fa417e0ad0c6824b))

### Bug Fixes

- *(cd)* Add missing version field in codegen crate - ([b889a5e](https://github.com/adclz/auto-lsp/commit/b889a5e61696c95421e0be49cc10579b674da87c))
- *(codegen)* Add numeric string sanitization to RUST_KEYWORDS - ([b1a2c7a](https://github.com/adclz/auto-lsp/commit/b1a2c7a96d2f2f8882c64842bd4969f502153ea6))
- Ambiguous associated item conflicting with enum variant `Error` ([#32](https://github.com/adclz/auto-lsp/pull/32)) - ([fce63ef](https://github.com/adclz/auto-lsp/commit/fce63eff3e0d3d7bf11f31cfe1298196c680acb0))

### Miscellaneous Tasks

- Remove GPL headers - ([1e077cc](https://github.com/adclz/auto-lsp/commit/1e077cc1c6f35eb1806fcfbe17072a2998fc90dd))
- Bump deps and use rust 2024 edition ([#28](https://github.com/adclz/auto-lsp/pull/28)) - ([981f658](https://github.com/adclz/auto-lsp/commit/981f6582466f1042b68f94872ae1649b8a0bdcb7))


## [0.2.1](https://github.com/adclz/auto-lsp/compare/auto-lsp-codegen-v0.2.0...auto-lsp-codegen-v0.2.1)

### Features

- *(codegen)* Add numeric tokens and add sanitization tests - ([14b7d01](https://github.com/adclz/auto-lsp/commit/14b7d01a05d7cb9a0c938bd1f881fb793c81e521))


## [0.2.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-codegen-v0.1.0...auto-lsp-codegen-v0.2.0)

### Features

- *(codegen)* Allow extending the default Token list - ([b632c9f](https://github.com/adclz/auto-lsp/commit/b632c9f0c9dd58f6d73b6b15006bb31c6d2927e1))

### Bug Fixes

- *(codegen)* Improve node ID retrieval in generate_struct and generate_enum functions - ([f9551e5](https://github.com/adclz/auto-lsp/commit/f9551e570bd0926c75f7102da0917fc321297193))
- *(codegen)* Avoid uppercasing Token_ names - ([3445c18](https://github.com/adclz/auto-lsp/commit/3445c18be12fb176f99d010b5b28d5d5970a25b6))

### Refactor

- *(codegen)* Update token handling to extend or overwrite default tokens - ([4ef4d57](https://github.com/adclz/auto-lsp/commit/4ef4d572b125d8eb6d2185dcfe18de87947979c0))

### Documentation

- Fix link in codegen README - ([5e413fe](https://github.com/adclz/auto-lsp/commit/5e413feb850245a1ec293c48ec8e7d19742aced7))
- Update descriptions in Cargo.toml and lib.rs - ([8501e7c](https://github.com/adclz/auto-lsp/commit/8501e7c2070e5d8d1923765fc955dd864acbab53))
- Add codegen documentation - ([a9d4933](https://github.com/adclz/auto-lsp/commit/a9d4933e306ab7c28905115566f8b1caad8d0069))

### Styling

- README files - ([aa902c1](https://github.com/adclz/auto-lsp/commit/aa902c15da8ead570eb38cc6f718c27411cd5a01))

### Testing

- *(codegen)* Add tests for HTML, JavaScript, C, C#, and Haskell code generation - ([f38555c](https://github.com/adclz/auto-lsp/commit/f38555c8ea8c0108346cdf79ada8a38646abefc8))

