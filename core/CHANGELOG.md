# Changelog

## [Unreleased]

## [0.4.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-core-v0.3.0...auto-lsp-core-v0.4.0)

### Features

- *(display)* Add IndentedDisplay trait and implement Display - ([e6c1dd6](https://github.com/adclz/auto-lsp/commit/e6c1dd6cbd2dd535e10cbef9829634cd7cce0fd7))

### Bug Fixes

- *(incremental)* Ensure correct symbol generation when vector has only one end node - ([fb40915](https://github.com/adclz/auto-lsp/commit/fb40915256afaddfb73ba5dac3990a8679e28da5))

### Refactor

- *(build)* Add parent context in error message - ([4e62199](https://github.com/adclz/auto-lsp/commit/4e62199142fddd5385247aded0dc9964ea4dd33d))
- *(check)* Update check method to return CheckStatus  enum instead of Result - ([f3330bb](https://github.com/adclz/auto-lsp/commit/f3330bbeb4a682724ef2dc048868969b286250a8))
- *(code-lenses)* Rename build_code_lens to build_code_lenses for consistency - ([519fcc0](https://github.com/adclz/auto-lsp/commit/519fcc0743a83c42aa7e850d973355c130a39528))
- *(completion-items)* Scoped-based and triggered completion items - ([e358a24](https://github.com/adclz/auto-lsp/commit/e358a247bef9529a9b2db3f27d24039c717a9b0f))
- *(core_build)* Remove unused add method - ([633b7cd](https://github.com/adclz/auto-lsp/commit/633b7cde3b0957617a7850c69a322efd9f8dde98))
- *(document)* Search methods - ([00086e9](https://github.com/adclz/auto-lsp/commit/00086e96417585a40e379268d9a47c07c7212de1))
- *(parse)* Implement fmt::Display - ([9c4c5fb](https://github.com/adclz/auto-lsp/commit/9c4c5fbb2568b2feee7ed3a0109647ead70e34c2))
- *(parse)* Rename try_parse to test_parse and update return type to TestParseResult - ([26a305d](https://github.com/adclz/auto-lsp/commit/26a305dd7b66b9c002bbe4a8aaccfb5a38cfead2))
- *(try_parse)* Replace miette with ariadne - ([8211f55](https://github.com/adclz/auto-lsp/commit/8211f5557d7e10236ce791843919ff7c1707f046))
- Remove incremental feature and related code - ([b8b9a4f](https://github.com/adclz/auto-lsp/commit/b8b9a4ff7285d806e90fb959b59ee3dd8de49139))

### Documentation

- Update main and core crates documentation - ([3c5c9c3](https://github.com/adclz/auto-lsp/commit/3c5c9c3f2a0254b5a1353337b7f21131cef41366))

### Testing

- *(html)* Refactor html AST and add html_corpus module - ([0b5a056](https://github.com/adclz/auto-lsp/commit/0b5a0565d894e3b1bdfcdeb4c23fe32903ad827e))


## [0.3.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-core-v0.2.0...auto-lsp-core-v0.3.0)

### Features

- *(core_build/parse)* Enable invoking parsers from any symbol with miette error reporting - ([18dafd4](https://github.com/adclz/auto-lsp/commit/18dafd48ba380511d04421a7b9ba7bf8101d46c9))
- *(deadlock_detection)* Add deadlock detection feature and tests - ([bef0e20](https://github.com/adclz/auto-lsp/commit/bef0e204f79b71b84c26ff4367db439fc4c87155))
- *(document_symbols)* Introduce DocumentSymbolsBuilder for cleaner symbol creation - ([73b282c](https://github.com/adclz/auto-lsp/commit/73b282cd644564ee932347a61c51bbd51524a7e0))
- *(parse)* Add miette report - ([c29416a](https://github.com/adclz/auto-lsp/commit/c29416a33230575d10d90b416d761132d869c1fd))
- *(parse)* Add parse_symbols method for symbol extraction - ([430ed9d](https://github.com/adclz/auto-lsp/commit/430ed9dd6e326c5cdebea27f7728e3ae28fe64df))
- *(traverse)* Introduce Traverse trait - ([c60f1fd](https://github.com/adclz/auto-lsp/commit/c60f1fd0ebeac019436e0ae0b9e01e3b3caa3286))
- *(update)* Add incremental cargo feature - ([ee4a639](https://github.com/adclz/auto-lsp/commit/ee4a639526d60c8546bd5a2bf5f47f472f2692b1))
- *(update)* Add more cases for incremntal updates - ([a2a2efa](https://github.com/adclz/auto-lsp/commit/a2a2efa76fd130c0dc0e91293ea6075ffa899325))
- *(update)* Enhance Change struct with trim_start field and add trim_start_index function - ([dccaf0d](https://github.com/adclz/auto-lsp/commit/dccaf0d259c93e832e73b3c6cbf9dc0dd1357b1b))
- *(update)* Implement incremental updates with vectors and ChangeReport struct - ([1c9c37e](https://github.com/adclz/auto-lsp/commit/1c9c37ed203c8c8a5daff19dff36fc10f05878f3))
- LSP Code actions - ([53b39d2](https://github.com/adclz/auto-lsp/commit/53b39d2e1d6c2a622dfae9cf24df36bd6474eb9b))
- Completion items - ([1631484](https://github.com/adclz/auto-lsp/commit/1631484ba78d6be0edbe04df6b80eb76322b7133))
- Find_at_offset method in Workspace struct - ([c011a3c](https://github.com/adclz/auto-lsp/commit/c011a3c46b2a2e016930be74c0b25b80103ef36f))
- Add regex support for document link extraction - ([4a95271](https://github.com/adclz/auto-lsp/commit/4a95271fb4a7fa7c25cb412bc7a9694a72616d69))
- Enhance comments support - ([a2d6995](https://github.com/adclz/auto-lsp/commit/a2d6995d14ee7423c831c259780b8054d2b8cb29))
- Add update method for Document - ([b296099](https://github.com/adclz/auto-lsp/commit/b296099cc538bcf7a36aa9be45dcd6440ebc2500))

### Bug Fixes

- Parse AST when incremental flag is unset - ([1a31c1b](https://github.com/adclz/auto-lsp/commit/1a31c1b8328fce7ea4ea3beb60114b6144facf8b))
- Remove assertions feature and related checks from proc-macros and core - ([71d55fc](https://github.com/adclz/auto-lsp/commit/71d55fc4f87b331358d3d3aeccaff22b3f7283d5))
- Empty documents - ([9d9fcfb](https://github.com/adclz/auto-lsp/commit/9d9fcfbd3975ed99efda2a038a8e63c01425d6df))
- Use scopes to determine if node should be updated - ([6d35728](https://github.com/adclz/auto-lsp/commit/6d3572877784a974d274169bd287e94c48da7c4e))
- Reparsing of root node when incremental is not available - ([b4b1223](https://github.com/adclz/auto-lsp/commit/b4b1223d842335324c808122f2065b2071635c00))
- Workspace checks - ([19d09d4](https://github.com/adclz/auto-lsp/commit/19d09d400636d89758ad23384fdb2dfa40b0adcb))

### Refactor

- *(update)* Merge traits and enhance vector updates - ([e2329bc](https://github.com/adclz/auto-lsp/commit/e2329bcf90931c480a9adefb064e1b8c275ebe76))
- Make get_tree_sitter_errors public - ([793c797](https://github.com/adclz/auto-lsp/commit/793c797c95808bea93e8902d1ea817558d194451))
- Rename BuildCodeLens trait to BuildCodeLenses - ([0d220d0](https://github.com/adclz/auto-lsp/commit/0d220d0a2594e0b1c02cff2aa80953472a331afc))
- Rename IsScope trait to Scope and remove get_scope_range method - ([d1504bc](https://github.com/adclz/auto-lsp/commit/d1504bcc036fd8a6a211e079896f3352fe62c30c))
- FindPattern trait with AhoCorasick - ([a7d7160](https://github.com/adclz/auto-lsp/commit/a7d716014be648bf91d941254191894b75f0e02e))
- Relocate Parent trait to core_build module - ([5fb9bd0](https://github.com/adclz/auto-lsp/commit/5fb9bd074a15d34af078da149979575e1987b95c))
- Rename parse method to miette_parse for clarity - ([e54f477](https://github.com/adclz/auto-lsp/commit/e54f4777e99785100bab22bc0b4fa6865fd59fbd))
- Update conditions for initiating AST construction - ([f0da3e0](https://github.com/adclz/auto-lsp/commit/f0da3e076f53bf137a0b8f462229ee0d3bf077ab))
- Simplify code generation for features and #seq proc macro attributes - ([9704ebe](https://github.com/adclz/auto-lsp/commit/9704ebeda5c9dee49c94e91911956d387d66dd10))
- Remove unused Constructor trait and Queryable impl on AstSymbol - ([9f01673](https://github.com/adclz/auto-lsp/commit/9f01673b34c87f69511446d84f42cc7f5615cf65))
- Incremental updates - ([013f870](https://github.com/adclz/auto-lsp/commit/013f870bbc59620496821a8b99c662a9cdbc7a53))
- Rename build_inlay_hint - ([9781c91](https://github.com/adclz/auto-lsp/commit/9781c9128dce135fcef08e927165a1efe7612d04))
- Logging in core crate - ([1863970](https://github.com/adclz/auto-lsp/commit/1863970035e2deff189fcb612c58e06f61821749))
- Move texter_impl to core/document - ([a14fb00](https://github.com/adclz/auto-lsp/commit/a14fb00752ef7b5698697b6d1e56388668dec3f0))
- Eliminate redundant function calls in Workspace - ([da6964a](https://github.com/adclz/auto-lsp/commit/da6964a43933dcb3bf50dffd855100b0c62226be))

### Documentation

- Doc(update) - ([31950e3](https://github.com/adclz/auto-lsp/commit/31950e3e5926b58370bc10db1d4eeeeff5e6e2ac))
- Add examples for LSp traits - ([0943335](https://github.com/adclz/auto-lsp/commit/094333531559b3f66aaa19a2decd16a46f89369f))
- Add missing doc for core_build modules - ([55daad4](https://github.com/adclz/auto-lsp/commit/55daad47f91eabe64f5f359931b5f0bd33cc1e34))

### Testing

- *(update)* Add tests for range-based edits - ([cec62b9](https://github.com/adclz/auto-lsp/commit/cec62b950048dab54ffed69ad3beccb0c3b71df6))

### Miscellaneous Tasks

- Improve doc - ([cb8e513](https://github.com/adclz/auto-lsp/commit/cb8e5135b1295db0a16eee1ef79ac2b53b0bd4be))
- Fmt - ([6daa0cb](https://github.com/adclz/auto-lsp/commit/6daa0cb08cacafcb68e9f515ee9c724a6d9699d0))

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-core-v0.1.0...auto-lsp-core-v0.2.0) - 2025-01-24

### Added

- add Workspace::new constructor

### Fixed

- multi-line edits

### Other

- move semantic tokens and parsers macros to configuration module
- add multiple constructors for Workspace and move lexer to core crate
- enhance Workspace struct
- integrate comment handling into Workspace and remove Session::add_comments
- add documentation for StackBuilder
- replace StaticBuildable with InvokeStackBuilder in core_ast and core_build modules
- update workspace and document handling, remove MainBuilder struct
- core_ast/update.rs module
- rename accessor methods to reference methods for consistency
- improve error messages for invalid field inputs with expected and received values

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
