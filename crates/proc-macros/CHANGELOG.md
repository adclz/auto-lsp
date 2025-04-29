# Changelog

## [Unreleased]

## [0.1.6](https://github.com/adclz/auto-lsp/compare/auto-lsp-macros-v0.1.5...auto-lsp-macros-v0.1.6)

### Features

- Integrate id-arena and remove Symbol wrapper - ([7ddfbdd](https://github.com/adclz/auto-lsp/commit/7ddfbdd617f209cbacf03ca05dcacc617061db67))
- Implement TryFrom trait for EnumBuilder and StructBuilder - ([240a8ed](https://github.com/adclz/auto-lsp/commit/240a8ed33fbec0a1e6f9dfde4572ed5ca973009e))

### Bug Fixes

- *(enum)* Replace unreachable with AstError in enums - ([3b3213c](https://github.com/adclz/auto-lsp/commit/3b3213c09549f4dec851752b959dbe34ff41563f))
- Correct return type in get_hover method signature - ([ee10be8](https://github.com/adclz/auto-lsp/commit/ee10be818e1be7838623dec1463243f75e756bf7))

### Refactor

- *(choice)* Make choice macro return compile errors alongside input - ([039ec05](https://github.com/adclz/auto-lsp/commit/039ec051a084923a64b6aa3d12ae2217b2e344d0))
- *(choice)* Update extract_variants to accept syn::DataEnum - ([2830cb4](https://github.com/adclz/auto-lsp/commit/2830cb489b49218a6e4e74c79e15ef230d9d80f2))
- *(core)* LSP capabilities now support error propagation - ([f713cc1](https://github.com/adclz/auto-lsp/commit/f713cc1455a7c1862e9769aaa8369fb58d525902))
- *(core_build)* Remove url field - ([763aca6](https://github.com/adclz/auto-lsp/commit/763aca6f0283bb6a865d1551948849d39c9a52ba))
- *(errors)* Replace Diagnostic with AstError and update error handling across modules - ([8c47155](https://github.com/adclz/auto-lsp/commit/8c4715575af3626326624e808c795dcfce93bcef))
- *(proc-macros)* Remove filter utilities module and remove deprecated code - ([4ce8494](https://github.com/adclz/auto-lsp/commit/4ce84944f0faffd0ba95c3b426fb699e84b4b7bc))
- *(proc-macros)* Better utilities  for type name extraction - ([17ebdac](https://github.com/adclz/auto-lsp/commit/17ebdacf1cb86a1710fc901ff81b8c260afc30b6))
- *(seq)* Make seq macro return compile errors alongside input - ([8a6c8c6](https://github.com/adclz/auto-lsp/commit/8a6c8c6da86bc7a616636e918dcf8927262b578b))
- *(tests)* Rename test functions - ([cc18245](https://github.com/adclz/auto-lsp/commit/cc182458f4e1ab527e23dafd7dc007cbe79b217b))
- Use id_ctr instead of tree sitter subtree pointers - ([6bcfb41](https://github.com/adclz/auto-lsp/commit/6bcfb41328e7059b601f78d6d7f186662a1e8400))
- Simplify borrowing for pending symbols - ([d55c2c0](https://github.com/adclz/auto-lsp/commit/d55c2c0b0db2e2ecfc89920e6bf9ab776acea1ae))
- Remove symbol module (WeakSymbol and DynSymbol) - ([89e3748](https://github.com/adclz/auto-lsp/commit/89e3748f48055116eaf0e240deeb4285a2de9685))
- Remove Traverse trait - ([2deea94](https://github.com/adclz/auto-lsp/commit/2deea946373b6b98d27fdbb32cde0d400f43af35))
- Remove unused parameters from AddSymbol trait methods - ([2fc7f4b](https://github.com/adclz/auto-lsp/commit/2fc7f4b10460e75ee80f62e7630f349c0a0eb715))
- Remove Finalize trait - ([e9421f5](https://github.com/adclz/auto-lsp/commit/e9421f55f2c54621c21fccbc1c9aa15c4a9b10b6))
- Update AST symbol handling with new ID management and mutable data access - ([af49550](https://github.com/adclz/auto-lsp/commit/af495504a24f0f04df899c1665294f6eba8b3d57))
- Replace Symbol wrapper with Arc - ([1ebce96](https://github.com/adclz/auto-lsp/commit/1ebce96656cadba64271231e8ac51266c3b7a05c))
- Remove RwLock, mutable traits and methods from AST - ([2d0015b](https://github.com/adclz/auto-lsp/commit/2d0015b4106c151891abdae37a129498a740e570))
- Remove TryFromBuilder and TryIntoBuilder traits - ([7ce632c](https://github.com/adclz/auto-lsp/commit/7ce632ca1120de57a5f4d2daaab4b8973eb258d5))
- Streamline GetSymbolData implementation and add inline attributes - ([342bb7c](https://github.com/adclz/auto-lsp/commit/342bb7c1e7396f8f79481415b6820e7a20c7df8c))
- Remove Url references - ([9da8416](https://github.com/adclz/auto-lsp/commit/9da84165da43c37f8905a784c7279b337dcb1a2c))
- Text retrieval methods now return Results - ([4de460d](https://github.com/adclz/auto-lsp/commit/4de460d09b03714eba62b5cb172ccf1ef6e2aab6))
- Remove new_and_check method and add From trait idioms - ([40a1c42](https://github.com/adclz/auto-lsp/commit/40a1c4284aa7abce1071a8a99802566649b57bbf))
- Move core and proc-macro to crates folder - ([9ca4d9c](https://github.com/adclz/auto-lsp/commit/9ca4d9c260d764dda4256a0bbbd85684a968c864))

### Miscellaneous Tasks

- *(license)* Update Cargo.toml files - ([dd5971f](https://github.com/adclz/auto-lsp/commit/dd5971f8d8c5e0ffa5fa0b97c0a3b3c517c2f82c))
- *(license)* Add GPLv3 license header to all source files - ([60d6d5a](https://github.com/adclz/auto-lsp/commit/60d6d5abe8a3e10f79fe651de074fa61cad9e7f6))


## [0.1.5](https://github.com/adclz/auto-lsp/compare/auto-lsp-macros-v0.1.4...auto-lsp-macros-v0.1.5)

### Bug Fixes

- *(docs)* Update TestServer struct reference in README and remove outdated comment in meta.rs - ([5348539](https://github.com/adclz/auto-lsp/commit/5348539e5e988ca23a27bfe77953325f3b151043))


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
