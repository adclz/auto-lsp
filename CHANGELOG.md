# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-v0.1.0...auto-lsp-v0.2.0) - 2025-01-24

### Added

- update document link fn signature
- add Workspace::new constructor
- add type checking
- add rstest for improved test structure and parameter validation in Python tests
- enhance Python workspace AST

### Fixed

- proc macro hygiene

### Other

- move semantic tokens and parsers macros to configuration module
- enable git release in configuration
- add traits tests for proc macros
- add proc_macros tests
- add multiple constructors for Workspace and move lexer to core crate
- enhance Workspace struct
- integrate comment handling into Workspace and remove Session::add_comments
- rename workspace module to fs
- replace StaticBuildable with InvokeStackBuilder in core_ast and core_build modules
- update workspace and document handling, remove MainBuilder struct
- improve handling of file extensions and fs operations
- re-export proc macros at root of main crate
- reorganize proc-macro modules and update field handling
- proc macro hygiene with StaticUpdate import
- update extension README
- add README for WASM Language Server example and update build script
- update README for crates.io

## [0.1.0](https://github.com/adclz/auto-lsp/releases/tag/auto-lsp-v0.1.0) - 2025-01-20

### Added

- enhance document link handling with user-defined regex options
- add node-types.json and update lexer
- add assertions feature for compile-time query checks
- implement watched files change handling in session
- add optional rayon support for parallel processing
- add Python workspace module accessible across crates
- add wasm and deadlock detection features
- add benchmarks and log feature
- add CI workflow for Rust and remove obsolete vsce workflow
- update tree-sitter dependencies and enhance query handling in CstParser
- replace lsp-textdocument crate with texter crate for document storage,  add support for UTF8, UTF16 and UTF-32 encodings
- add logging functionality and update dependencies
- implement Finalize and StaticSwap traits for Option<Symbol>
- enhance error handling and incremental updates
- enhance document editing and error handling in workspace, improve symbol trait constraints
- add comment support
- add check field to Session and Workspace, refactor traits for clarity
- add conflict checking for query names and enhance queryable traits
- update range handling (tree_sitter::Range -> std::ops::Range) and integrate BuilderParams across multiple modules
- EditRange trait
- add dynamic swap functionality to enum and struct builders
- use StaticBuilder Trait + blanket implementations for incremental updates and refactor builders.rs
- incremental update of ast
- implement naive references drop mechanism
- add lsp go to declaration feature
- simplify default implementations by removing unnecessary method signatures and code gen
- add references lsp request
- enable referrers
- use AstSymbolData struct instead of code gen
- add Referrers struct for managing weak symbol references
- macro for lsp builder traits
- remove Key trait and replace duplicate module to check
- update build script to support parking_lot thread parker https://github.com/Amanieu/parking_lot/blob/ca920b31312839013b4455aba1d53a4aede21b2f/core/src/thread_parker/mod.rs#L69
- enhance ast_struct macro with accessor attributes and refactor features builders
- add full trait signatures in paths.rs for lsp features using Structx and inwelling
- add VariantBuilder and improve enum code gen
- add lsp go to definition feature
- enhance accessor functionality with set_accessor method and update find methods
- remove KeySet derive macro,  add duplicate check feature and support for helper attributes
- introduce AddSymbol and Queryable trait, integrate them into AstItemBuilder and related macros
- implement Locator and Parent traits in StructBuilder and EnumBuilder
- introduce FieldBuilder for structured field processing and token generation
- implement LSP traits on accessors
- add TryDownCast traits and implement downcasting for PendingSymbol and MaybePendingSymbol
- introduce PendingSymbol and MaybePendingSymbol types for builders
- enhance StructBuilder to accept input attributes
- QueryHint derive macro
- (re)introduce TryFromBuilder and TryIntoBuilder traits with implementations
- add Finder trait with and blanket implementation
- accessor feature
- add try_into_item method to AST item builders
- add static query binder to AST item builders
- get_parent_scope
- Implement scope feature
- Add support for Accessors
- separate builder for features codegen
- split up LSP methods from AstItemtrait
- introduce traits for LSP features and give possibility for manual implementation
- introduce builder patterns for proc macros and LSP features
- add Arc<Url> field and get_url method to AstItem traits and implementations
- split up TryFromCtx implementation for struct macros
- update WorkspaceContext trait and impl Session
- move ast_item builders from auto_lsp to main crate
- add convert module and implement TryFromCtx and TryIntoCtx traits
- workspace trait
- DeferredAstItemBuilder struct and closures for improved handling of deferred items
- Scope Range feature
- extend AstItem trait to be Send and Sync
- reference struct
- enhance AstItemBuilder with query_binder method and improve error handling
- completion items without char trig
- KeySet feature for HashMaps
- feat and fix: workspace diagnostics
- CompletionItem capability
- Ast builders can be defined alongside cst parsers in main.rs
- CodeLens capability
- InlayHint capability
- HashMap support for ast fields
- borrowable macro feature
- vsce test workflow
- DidChangedWatchedFiles notification
- document deletion
- add publish diagnostic notification for added document
- generic senders
- add crossbeam-channel
- feat: use dispatchers from:
- Lsp diagnostics instead of panics for builder errors
- DidChangeWatchedFiles event handlers
- feat  DidChangeWatchedFiles notification
- client can now send a list of parsers to use per file extension

### Fixed

- remove structx for paths generation in proc-macros
- change CI workflow
- attempt to repair CI (fails to build Structx dependency)
- use https://github.com/dtolnay/rust-toolchain for CI workflow
- update CI workflow
- enhance reference handling
- rustc wasm output file name
- update AddSymbol trait to use BuilderParams instead of Range
- adjust range start condition in edit function
- document symbol feature returns None if name is falsy
- closures in FieldBuilder must return a vec of token stream
- remove HashMap support
- update InlayHints trait to include FullTextDocument parameter in build_inlay_hint method
- Builders do not need to be cloned anymore when calling try_from or try_into
- remove TryFromCtx and TryIntoCtx
- update WorkspaceContext trait
- rm borrowable feature
- document link
- HashMap support
- changed notification must override workspace
- edited document handler
- diagnostic request report
- fix: downgrade of lsp_types and lsp_document crates due to regression:

### Other

- fix README
- add release-plz
- update package metadata and enhance README content
- remove nested_struct macro
- update README badges
- add Dependabot
- update README
- add documentation for lsp_server module
- remove unused InitResult struct
- update docs
- update Cargo.toml and lib.rs for feature adjustments and dependency organization
- refactor main crate and add lsp_server feature
- improve tree sitter error retrieval
- vscode-python-wasi-lsp package
- suppress cargo warnings
- rename capabilities traits
- hidden visiblity for build module
- update CodeLens and InlayHints implementations to include Document parameter
- update html and python tests
- update module paths to use aliases for core and macros
- update build_semantic_tokens to include Document parameter
- core crate
- add HTML parsing tests and restructure Python test module
- document_symbol and comment python tests
- rename NewChange and NewTree, enhance incremntal updates
- introduce VecOrSymbol enum and update document symbol handling
- remove iec parsers and replace vscode extension with python
- replace workspaces HashMap with a global WORKSPACES mutex for improved concurrency
- add support for semantic tokens in LSP options
- session module
- simplify session creation and add more initialization options
- streamline symbol reading and editing logic in AST handling
- enhance AST swapping logic and improve logging for incremental updates
- add MIT License file
- check if parent can have a comment if named sibling can't have comments
- remove unused accessor methods and implement collect_references functionality
- fix formatting in README
- update README
- remove Cargo.lock
- reexport auto_lsp crates and clean up dependencies
- update tree-sitter to version 0.24.6
- move main.rs and symbols module to vscode-wasi-lsp and update workspace configuration
- reorganize project structure by setting auto-lsp as the repository root and moving parsers and VSCode extension into test folder
- remove Debug implementation for AstBuilder trait
- remove unused dynamic symbol methods and clean up builder interfaces
- rename server package to auto-lsp
- split up main package into lib and main
- rename auto_lsp to auto_lsp_core, rename workspace folders and update imports across the codebase
- remove query_binder function and simplify create_child_node logic
- simplify AstBuilder and StructBuilder signatures by using QueryCapture
- improve incremental updates and range edit handling in builders and macros
- remove custom Debug implementations for PendingSymbol and MaybePendingSymbol
- rename proc macros ast_struct to seq and ast_enum to choice for clarity
- enum and variant builders
- struct and field builders, add AstSymbol and AstBuilder traits methods in global paths
- enhance symbol handling with new methods and finalize trait
- Locator trait implementations
- simplify dispatch method and remove SignatureAndBody struct
- add more trait signatures in PATHS
- remove unnecessary paths parameter from builders and use PATHS directly
- rename AstItem, AstItemBuilder traits to AstSymbol and AstBuilder
- use darling take_struct instead of pattern matching
- auto_lsp crate and update paths
- remove DeferredClosure and refactor ast builder
- move Builders from main crate into auto_lsp
- streamline field handling in StructBuilder and introduce OffsetLocator and ParentInject traits
- use associated types in TryDownCast instead of multiple traits
- add parking_lot, introduce Symbol<T>, DynSymbol, WeakSymbol new types and refactor implementations
- update FieldInfoExtract trait and StructFields
- simplify field types and builders retrieval in StructBuilder
- update Accessor trait
- add support for no #[key] helper in KeySet macro
- remove DeferredAstItemBuilder struct
- improve error handling
- dynamic inlay hint query generation
- change get_scope_range return type from array to vector
- update find method to include &FullTextDocument parameter
- Update paths in proc macros
- remove ast macro
- Builder trait and blanket implementation for AstItemBuilder
- auto lsp macros
- proc macro for ast enum symbol
- update trait path references in LSP macros
- streamline struct field generation and remove unused references
- proc macro for ast struct symbol
- change parent references from Arc to Weak in AstItem traits and implementations
- remove url parameter from WorkspaceContext find method
- streamline AstItemBuilder traits
- auto_lsp struct proc macros
- auto_lsp crate
- remove lifetime parameter from Session. CstParsers and AstBuilders are now &'static references
- rm unused code
- auto_lsp macros CodeGen
- borrowable
- .vscodeignore file for vsce package
- document symbols
- lsp_server::Connection is now a field of session
- root crate
- switch thiserror -> anyhow
- initial commit
