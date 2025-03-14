# Changelog

## [Unreleased]

## [0.4.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-v0.3.0...auto-lsp-v0.4.0)

### Features

- *(display)* Add IndentedDisplay trait and implement Display - ([e6c1dd6](https://github.com/adclz/auto-lsp/commit/e6c1dd6cbd2dd535e10cbef9829634cd7cce0fd7))
- *(semantic-tokens)* Enhance token type and modifier definitions - ([1f586e7](https://github.com/adclz/auto-lsp/commit/1f586e7e3f065ea91ea3b8bedbc3bc8598f0081e))
- *(server)* Code actions - ([18866c7](https://github.com/adclz/auto-lsp/commit/18866c7d294ffe8198d360b313ca9f12a5c573bc))

### Bug Fixes

- *(bench)* Remove flamegraph profiler for windows - ([3f89ad4](https://github.com/adclz/auto-lsp/commit/3f89ad465b4f5275ef5e2f36fdce774decf8565c))
- *(incremental)* Ensure correct symbol generation when vector has only one end node - ([fb40915](https://github.com/adclz/auto-lsp/commit/fb40915256afaddfb73ba5dac3990a8679e28da5))
- *(python)* Function return type - ([b483a63](https://github.com/adclz/auto-lsp/commit/b483a63c1af57739b8e242673420c91b1a7bac9c))
- Remove invalid test - ([c190be2](https://github.com/adclz/auto-lsp/commit/c190be2314617dfec6995332184ebbba8115dc18))

### Refactor

- *(bench)* Update benchmarks - ([8a82888](https://github.com/adclz/auto-lsp/commit/8a828884200cb4b4eb6c7216ca41a4c7a5fe162a))
- *(check)* Update check method to return CheckStatus  enum instead of Result - ([f3330bb](https://github.com/adclz/auto-lsp/commit/f3330bbeb4a682724ef2dc048868969b286250a8))
- *(code-lenses)* Rename build_code_lens to build_code_lenses for consistency - ([519fcc0](https://github.com/adclz/auto-lsp/commit/519fcc0743a83c42aa7e850d973355c130a39528))
- *(completion-items)* Scoped-based and triggered completion items - ([e358a24](https://github.com/adclz/auto-lsp/commit/e358a247bef9529a9b2db3f27d24039c717a9b0f))
- *(document)* Search methods - ([00086e9](https://github.com/adclz/auto-lsp/commit/00086e96417585a40e379268d9a47c07c7212de1))
- *(parse)* Rename try_parse to test_parse and update return type to TestParseResult - ([26a305d](https://github.com/adclz/auto-lsp/commit/26a305dd7b66b9c002bbe4a8aaccfb5a38cfead2))
- *(server)* Move InitOptions to a dedicated options module - ([510ddba](https://github.com/adclz/auto-lsp/commit/510ddba3d0a8b91dbe802ac284aa7fd25ca3c82b))
- *(session)* Rename init_roots - ([6fd5f2a](https://github.com/adclz/auto-lsp/commit/6fd5f2acc6e5d63c9e641370e4cc6124af4c1e3f))
- *(tests)* Enhance python AST and add tree sitter corpus - ([e205710](https://github.com/adclz/auto-lsp/commit/e2057103b45ceb1bde47e30f7f8bc2a4fce08b21))
- *(try_parse)* Replace miette with ariadne - ([8211f55](https://github.com/adclz/auto-lsp/commit/8211f5557d7e10236ce791843919ff7c1707f046))
- Remove incremental feature and related code - ([b8b9a4f](https://github.com/adclz/auto-lsp/commit/b8b9a4ff7285d806e90fb959b59ee3dd8de49139))

### Documentation

- Update book - ([ff317a6](https://github.com/adclz/auto-lsp/commit/ff317a61939edb8d007f7df7ecd62af12843a227))
- Update main and core crates documentation - ([3c5c9c3](https://github.com/adclz/auto-lsp/commit/3c5c9c3f2a0254b5a1353337b7f21131cef41366))
- Fix dead links - ([729fc49](https://github.com/adclz/auto-lsp/commit/729fc49d699dfe36193139f1c4d1db203db67d52))
- Update links - ([ebbbc0c](https://github.com/adclz/auto-lsp/commit/ebbbc0cbc4786c3c3d033ce99e692a4356059081))

### Testing

- *(html)* Refactor html AST and add html_corpus module - ([0b5a056](https://github.com/adclz/auto-lsp/commit/0b5a0565d894e3b1bdfcdeb4c23fe32903ad827e))
- *(python)* Add pattern matching tests and clean up unused code - ([5de7cfb](https://github.com/adclz/auto-lsp/commit/5de7cfbf4e6ff4ef1473ee0bac24d17ac013d190))
- Python expressions - ([97e09c8](https://github.com/adclz/auto-lsp/commit/97e09c8709163451e985ebe76179b2692bccbe07))

### Miscellaneous Tasks

- Update pprof - ([0304042](https://github.com/adclz/auto-lsp/commit/03040425413703276230145664fdabad117158b9))


## [0.3.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-v0.2.0...auto-lsp-v0.3.0)

### Features

- *(core_build/parse)* Enable invoking parsers from any symbol with miette error reporting - ([18dafd4](https://github.com/adclz/auto-lsp/commit/18dafd48ba380511d04421a7b9ba7bf8101d46c9))
- *(deadlock_detection)* Add deadlock detection feature and tests - ([bef0e20](https://github.com/adclz/auto-lsp/commit/bef0e204f79b71b84c26ff4367db439fc4c87155))
- *(document_symbols)* Introduce DocumentSymbolsBuilder for cleaner symbol creation - ([73b282c](https://github.com/adclz/auto-lsp/commit/73b282cd644564ee932347a61c51bbd51524a7e0))
- *(parse)* Add miette report - ([c29416a](https://github.com/adclz/auto-lsp/commit/c29416a33230575d10d90b416d761132d869c1fd))
- *(traverse)* Introduce Traverse trait - ([c60f1fd](https://github.com/adclz/auto-lsp/commit/c60f1fd0ebeac019436e0ae0b9e01e3b3caa3286))
- *(update)* Add incremental cargo feature - ([ee4a639](https://github.com/adclz/auto-lsp/commit/ee4a639526d60c8546bd5a2bf5f47f472f2692b1))
- *(update)* Add more cases for incremntal updates - ([a2a2efa](https://github.com/adclz/auto-lsp/commit/a2a2efa76fd130c0dc0e91293ea6075ffa899325))
- *(update)* Implement incremental updates with vectors and ChangeReport struct - ([1c9c37e](https://github.com/adclz/auto-lsp/commit/1c9c37ed203c8c8a5daff19dff36fc10f05878f3))
- Improve python AST statements - ([cfbcb9c](https://github.com/adclz/auto-lsp/commit/cfbcb9c2c831e44d5f050891f0ebf815df5f6dc0))
- LSP Code actions - ([53b39d2](https://github.com/adclz/auto-lsp/commit/53b39d2e1d6c2a622dfae9cf24df36bd6474eb9b))
- (almost) complete python AST - ([e6c6ab7](https://github.com/adclz/auto-lsp/commit/e6c6ab72a64e94720b2e8425011b094393fc45ba))
- Add bench.sh - ([4fc8c97](https://github.com/adclz/auto-lsp/commit/4fc8c97022a09e1c9ff6c5801cc1a5c8998fb3e5))
- Make parser list name configurable in configure_parsers macro - ([5f7772b](https://github.com/adclz/auto-lsp/commit/5f7772bda1f8595a2bc3c9cec35d806f31811eb5))
- Completion items - ([1631484](https://github.com/adclz/auto-lsp/commit/1631484ba78d6be0edbe04df6b80eb76322b7133))
- Find_at_offset method in Workspace struct - ([c011a3c](https://github.com/adclz/auto-lsp/commit/c011a3c46b2a2e016930be74c0b25b80103ef36f))
- Add regex support for document link extraction - ([4a95271](https://github.com/adclz/auto-lsp/commit/4a95271fb4a7fa7c25cb412bc7a9694a72616d69))
- Enhance comments support - ([a2d6995](https://github.com/adclz/auto-lsp/commit/a2d6995d14ee7423c831c259780b8054d2b8cb29))
- Add nested function to python body - ([b950932](https://github.com/adclz/auto-lsp/commit/b950932c08e34e618ebdc54c9c990a1a6e5206b2))
- Add update method for Document - ([b296099](https://github.com/adclz/auto-lsp/commit/b296099cc538bcf7a36aa9be45dcd6440ebc2500))

### Bug Fixes

- *(server)* Add windows support for Urls - ([71ddf93](https://github.com/adclz/auto-lsp/commit/71ddf93f7a6d6522868078093a0104438cb39a40))
- CD again - ([df93d8b](https://github.com/adclz/auto-lsp/commit/df93d8b2ed52c6e5b8fc0b60d3767d5c2735bedf))
- Mdbook folder - ([0f75fac](https://github.com/adclz/auto-lsp/commit/0f75fac9bebfa1a52c75a73e50ed89746a2bfb11))
- Fix windows Urls again - ([ae93f19](https://github.com/adclz/auto-lsp/commit/ae93f19b2e3e1331ebb74e3b080b937d875f8274))
- Remove assertions feature and related checks from proc-macros and core - ([71d55fc](https://github.com/adclz/auto-lsp/commit/71d55fc4f87b331358d3d3aeccaff22b3f7283d5))
- Empty documents - ([9d9fcfb](https://github.com/adclz/auto-lsp/commit/9d9fcfbd3975ed99efda2a038a8e63c01425d6df))
- Workspace checks - ([19d09d4](https://github.com/adclz/auto-lsp/commit/19d09d400636d89758ad23384fdb2dfa40b0adcb))

### Refactor

- Remove unused DocumentLinksOption - ([d894383](https://github.com/adclz/auto-lsp/commit/d894383002c5370879b4741c4414316dded59442))
- Rename BuildCodeLens trait to BuildCodeLenses - ([0d220d0](https://github.com/adclz/auto-lsp/commit/0d220d0a2594e0b1c02cff2aa80953472a331afc))
- Rename IsScope trait to Scope and remove get_scope_range method - ([d1504bc](https://github.com/adclz/auto-lsp/commit/d1504bcc036fd8a6a211e079896f3352fe62c30c))
- FindPattern trait with AhoCorasick - ([a7d7160](https://github.com/adclz/auto-lsp/commit/a7d716014be648bf91d941254191894b75f0e02e))
- Send notification - ([3633220](https://github.com/adclz/auto-lsp/commit/363322098640efd925e6d4fff7ed51cd2dfb4e6f))
- Rename parse method to miette_parse for clarity - ([e54f477](https://github.com/adclz/auto-lsp/commit/e54f4777e99785100bab22bc0b4fa6865fd59fbd))
- Simplify code generation for features and #seq proc macro attributes - ([9704ebe](https://github.com/adclz/auto-lsp/commit/9704ebeda5c9dee49c94e91911956d387d66dd10))
- Remove unused Constructor trait and Queryable impl on AstSymbol - ([9f01673](https://github.com/adclz/auto-lsp/commit/9f01673b34c87f69511446d84f42cc7f5615cf65))
- Update python workspace - ([af265fb](https://github.com/adclz/auto-lsp/commit/af265fb5ad5ea6b5b273d6c20ae83f4ec383ff0b))
- Remove html workspace from test file - ([1b5642f](https://github.com/adclz/auto-lsp/commit/1b5642fe52a688c4fc81b582d611671c32148106))
- Html and python workspaces - ([a6c7cd5](https://github.com/adclz/auto-lsp/commit/a6c7cd58fed7e9164815887345299656468cb677))
- Rename build_inlay_hint - ([9781c91](https://github.com/adclz/auto-lsp/commit/9781c9128dce135fcef08e927165a1efe7612d04))
- Logging in core crate - ([1863970](https://github.com/adclz/auto-lsp/commit/1863970035e2deff189fcb612c58e06f61821749))
- Move texter_impl to core/document - ([a14fb00](https://github.com/adclz/auto-lsp/commit/a14fb00752ef7b5698697b6d1e56388668dec3f0))
- Eliminate redundant function calls in Workspace - ([da6964a](https://github.com/adclz/auto-lsp/commit/da6964a43933dcb3bf50dffd855100b0c62226be))

### Documentation

- Official book - ([f1589c0](https://github.com/adclz/auto-lsp/commit/f1589c055a26ff524c2c7be5160170ea5797909b))
- Mdbook and CD - ([52addff](https://github.com/adclz/auto-lsp/commit/52addff751adae0c4e00b0aed473075bb0b5bc76))

### Testing

- Range and incremental tests - ([95fc0b7](https://github.com/adclz/auto-lsp/commit/95fc0b7c24065bcbe29e4040c353f2e2679f2a6f))
- Reorganize tests by corresponding features - ([b61bd66](https://github.com/adclz/auto-lsp/commit/b61bd66fcbbd64d96ed0a40ff8a339d8a8dc52be))
- Whitespaces - ([d78b50e](https://github.com/adclz/auto-lsp/commit/d78b50e6fc2e39490b94ab73a60155707a91fcc1))
- Add non-redundant type error checks - ([34902e4](https://github.com/adclz/auto-lsp/commit/34902e4777903dce353b21c530312db462dca1dc))

### Miscellaneous Tasks

- Improve doc - ([cb8e513](https://github.com/adclz/auto-lsp/commit/cb8e5135b1295db0a16eee1ef79ac2b53b0bd4be))
- Assets - ([16f8261](https://github.com/adclz/auto-lsp/commit/16f8261baea6b8a7da2907a240616b990b0d6038))
- README - ([8af818f](https://github.com/adclz/auto-lsp/commit/8af818fc7e2cf46decbbd6a799688daa0f48c38c))
- Add matrix strategy for cross-platform testing - ([09d474e](https://github.com/adclz/auto-lsp/commit/09d474ef75171f49cf3f96e915ea824496f6d45c))
- Releaze-plz scoped changelog file - ([7db4a95](https://github.com/adclz/auto-lsp/commit/7db4a95eae32a3e57eed470b85bd555761fa1a5a))
- Rm duplicated test - ([42a8787](https://github.com/adclz/auto-lsp/commit/42a878719693f6773121087af010e2e29dc46a9e))

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
