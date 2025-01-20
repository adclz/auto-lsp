# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
