# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/adclz/auto-lsp/releases/tag/auto-lsp-core-v0.1.0) - 2025-01-20

### Added

- add node-types.json and update lexer
- add assertions feature for compile-time query checks
- add optional rayon support for parallel processing
- update tree-sitter dependencies and enhance query handling in CstParser
- replace lsp-textdocument crate with texter crate for document storage,  add support for UTF8, UTF16 and UTF-32 encodings
- add logging functionality and update dependencies

### Fixed

- enhance reference handling

### Other

- refactor main crate and add lsp_server feature
- rename capabilities traits
- update CodeLens and InlayHints implementations to include Document parameter
- update build_semantic_tokens to include Document parameter
- core crate
- rename NewChange and NewTree, enhance incremntal updates
- introduce VecOrSymbol enum and update document symbol handling
- streamline symbol reading and editing logic in AST handling
- enhance AST swapping logic and improve logging for incremental updates
- improve logging output for node capture visualization
- remove unused accessor methods and implement collect_references functionality
- reexport auto_lsp crates and clean up dependencies
- reorganize project structure by setting auto-lsp as the repository root and moving parsers and VSCode extension into test folder
