# Changelog

## [Unreleased]

## [0.1.4](https://github.com/adclz/auto-lsp/compare/auto-lsp-macros-v0.1.3...auto-lsp-macros-v0.1.4)

### Features

- *(core)* Update reference resolution to include workspace and diagnostics - ([42e3c64](https://github.com/adclz/auto-lsp/commit/42e3c6421401dea5237a37040854b27604858480))

### Refactor

- *(code_actions)* Update build_code_actions signature to use CodeActionOrCommand - ([d74ba87](https://github.com/adclz/auto-lsp/commit/d74ba87280e9bf4cfcf66d64be283cc3630a7e1f))

### Miscellaneous Tasks

- Remove duplicated 'Unreleased' section from changelogs - ([cc416ef](https://github.com/adclz/auto-lsp/commit/cc416efc6cc0737360c993d2b0d86b8a77c416ca))


## [0.1.3](https://github.com/adclz/auto-lsp/compare/auto-lsp-macros-v0.1.2...auto-lsp-macros-v0.1.3)

### Features

- *(display)* Add IndentedDisplay trait and implement Display - ([e6c1dd6](https://github.com/adclz/auto-lsp/commit/e6c1dd6cbd2dd535e10cbef9829634cd7cce0fd7))

### Refactor

- *(check)* Update check method to return CheckStatus  enum instead of Result - ([f3330bb](https://github.com/adclz/auto-lsp/commit/f3330bbeb4a682724ef2dc048868969b286250a8))
- *(code-lenses)* Rename build_code_lens to build_code_lenses for consistency - ([519fcc0](https://github.com/adclz/auto-lsp/commit/519fcc0743a83c42aa7e850d973355c130a39528))
- *(completion-items)* Scoped-based and triggered completion items - ([e358a24](https://github.com/adclz/auto-lsp/commit/e358a247bef9529a9b2db3f27d24039c717a9b0f))
- *(proc-macros)* Inject Paths instead of inlining const LazyCell - ([7a11ad6](https://github.com/adclz/auto-lsp/commit/7a11ad6fe87810f87f8b998547d96c7ea0df7d50))
- *(proc-macros)* Paths - ([3917eda](https://github.com/adclz/auto-lsp/commit/3917eda13e4cb36c0f6aab431d6c4fe47b3ca798))
- Remove incremental feature and related code - ([b8b9a4f](https://github.com/adclz/auto-lsp/commit/b8b9a4ff7285d806e90fb959b59ee3dd8de49139))

### Documentation

- *(proc-macros)* Update seq documentation - ([f5b308a](https://github.com/adclz/auto-lsp/commit/f5b308a25689a8cfd3bbc1b77284ab7321a52eb7))


## [0.1.2](https://github.com/adclz/auto-lsp/compare/auto-lsp-macros-v0.1.1...auto-lsp-macros-v0.1.2)

### Features

- *(document_symbols)* Introduce DocumentSymbolsBuilder for cleaner symbol creation - ([73b282c](https://github.com/adclz/auto-lsp/commit/73b282cd644564ee932347a61c51bbd51524a7e0))
- *(traverse)* Introduce Traverse trait - ([c60f1fd](https://github.com/adclz/auto-lsp/commit/c60f1fd0ebeac019436e0ae0b9e01e3b3caa3286))
- *(update)* Add incremental cargo feature - ([ee4a639](https://github.com/adclz/auto-lsp/commit/ee4a639526d60c8546bd5a2bf5f47f472f2692b1))
- *(update)* Implement incremental updates with vectors and ChangeReport struct - ([1c9c37e](https://github.com/adclz/auto-lsp/commit/1c9c37ed203c8c8a5daff19dff36fc10f05878f3))
- LSP Code actions - ([53b39d2](https://github.com/adclz/auto-lsp/commit/53b39d2e1d6c2a622dfae9cf24df36bd6474eb9b))
- Completion items - ([1631484](https://github.com/adclz/auto-lsp/commit/1631484ba78d6be0edbe04df6b80eb76322b7133))

### Bug Fixes

- Document symbols children - ([f9d58ca](https://github.com/adclz/auto-lsp/commit/f9d58ca3a82fa01a741a4f7b7155d3c20183f843))
- Remove assertions feature and related checks from proc-macros and core - ([71d55fc](https://github.com/adclz/auto-lsp/commit/71d55fc4f87b331358d3d3aeccaff22b3f7283d5))
- Use scopes to determine if node should be updated - ([6d35728](https://github.com/adclz/auto-lsp/commit/6d3572877784a974d274169bd287e94c48da7c4e))

### Refactor

- *(update)* Merge traits and enhance vector updates - ([e2329bc](https://github.com/adclz/auto-lsp/commit/e2329bcf90931c480a9adefb064e1b8c275ebe76))
- Rename BuildCodeLens trait to BuildCodeLenses - ([0d220d0](https://github.com/adclz/auto-lsp/commit/0d220d0a2594e0b1c02cff2aa80953472a331afc))
- Rename IsScope trait to Scope and remove get_scope_range method - ([d1504bc](https://github.com/adclz/auto-lsp/commit/d1504bcc036fd8a6a211e079896f3352fe62c30c))
- Relocate Parent trait to core_build module - ([5fb9bd0](https://github.com/adclz/auto-lsp/commit/5fb9bd074a15d34af078da149979575e1987b95c))
- Simplify code generation for features and #seq proc macro attributes - ([9704ebe](https://github.com/adclz/auto-lsp/commit/9704ebeda5c9dee49c94e91911956d387d66dd10))
- Remove unused Constructor trait and Queryable impl on AstSymbol - ([9f01673](https://github.com/adclz/auto-lsp/commit/9f01673b34c87f69511446d84f42cc7f5615cf65))
- Incremental updates - ([013f870](https://github.com/adclz/auto-lsp/commit/013f870bbc59620496821a8b99c662a9cdbc7a53))
- Rename build_inlay_hint - ([9781c91](https://github.com/adclz/auto-lsp/commit/9781c9128dce135fcef08e927165a1efe7612d04))

### Miscellaneous Tasks

- Improve doc - ([cb8e513](https://github.com/adclz/auto-lsp/commit/cb8e5135b1295db0a16eee1ef79ac2b53b0bd4be))

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1](https://github.com/adclz/auto-lsp/compare/auto-lsp-macros-v0.1.0...auto-lsp-macros-v0.1.1) - 2025-01-24

### Added

- add type checking

### Fixed

- proc macro hygiene

### Other

- move semantic tokens and parsers macros to configuration module
- update workspace and document handling, remove MainBuilder struct
- core_ast/update.rs module
- rename accessor methods to reference methods for consistency
- documentation for proc-macros
- rename fields_builder and variants_builder modules
- reorganize proc-macro modules and update field handling
- proc macro hygiene with StaticUpdate import
- improve error messages for invalid field inputs with expected and received values

## [0.1.0](https://github.com/adclz/auto-lsp/releases/tag/auto-lsp-macros-v0.1.0) - 2025-01-20

### Added

- add assertions feature for compile-time query checks
- replace lsp-textdocument crate with texter crate for document storage,  add support for UTF8, UTF16 and UTF-32 encodings
- add logging functionality and update dependencies

### Fixed

- remove structx for paths generation in proc-macros
- add missing trait paths in struct_builder
- enhance reference handling

### Other

- refactor main crate and add lsp_server feature
- suppress cargo warnings
- rename capabilities traits
- update CodeLens and InlayHints implementations to include Document parameter
- simplify global paths
- update module paths to use aliases for core and macros
- update build_semantic_tokens to include Document parameter
- core crate
- document_symbol and comment python tests
- introduce VecOrSymbol enum and update document symbol handling
- remove unused accessor methods and implement collect_references functionality
- reexport auto_lsp crates and clean up dependencies
- reorganize project structure by setting auto-lsp as the repository root and moving parsers and VSCode extension into test folder
