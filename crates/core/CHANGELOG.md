# Changelog

## [Unreleased]

## [0.8.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-core-v0.7.0...auto-lsp-core-v0.8.0)

### Features

- *(core)* Add Span newtype - ([1b80c3e](https://github.com/adclz/auto-lsp/commit/1b80c3eb8110687f8e367801d74d0f8cd7dece05))
- Add position encoding support to Document and Db - ([dd1f4e6](https://github.com/adclz/auto-lsp/commit/dd1f4e6a90451cdd8ee5b0b466650828e51bca2d))

### Bug Fixes

- *(test)* Fix stack overflow in span equality test - ([c151ffa](https://github.com/adclz/auto-lsp/commit/c151ffaf94e57d92168c175cc1c484409059caf5))

### Refactor

- *(document)* Update Document constructor to use source strings directly - ([8d639be](https://github.com/adclz/auto-lsp/commit/8d639be29c20302400b8e84921d980c4cd129e6a))
- *(document)* Make encoding optional - ([037bb68](https://github.com/adclz/auto-lsp/commit/037bb6828f2eb517c5357ac040dbe619e2dbb444))
- *(errors)* Replace lsp_types::Range with Span in ParseError and LexerError - ([121c8a5](https://github.com/adclz/auto-lsp/commit/121c8a5c761c290e1c6a39f3834ccd164e258132))
- *(span)* Simplify Span struct by removing lifetime and Cow usage - ([1386547](https://github.com/adclz/auto-lsp/commit/13865470e6f6aec5ee8ad39bdd5e09b65e5bcc54))

### Bench

- Add codspeed + divan crate ([#24](https://github.com/adclz/auto-lsp/pull/24)) - ([9b98812](https://github.com/adclz/auto-lsp/commit/9b988120f4e086047c039c7a6c526c8348cd6054))


## [0.7.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-core-v0.6.1...auto-lsp-core-v0.7.0)

### Features

- *(errors)* Add additional fields to LexerError - ([0a19465](https://github.com/adclz/auto-lsp/commit/0a194651f158a520594e941e5953e1462c1b7bee))


## [0.6.1](https://github.com/adclz/auto-lsp/compare/auto-lsp-core-v0.6.0...auto-lsp-core-v0.6.1)

### Bug Fixes

- *(ast)* Add assertion for sorted node list in get_parent method - ([2ef5623](https://github.com/adclz/auto-lsp/commit/2ef56232061f39ffa13e76a60a4dd1234f355b28))
- *(errors)* Improve error message formatting by removing debug output - ([e5b8218](https://github.com/adclz/auto-lsp/commit/e5b821856accb4961f76a052cb68947c56b9568f))

### Refactor

- *(core)* Rename ast_node module to ast - ([a76f141](https://github.com/adclz/auto-lsp/commit/a76f1414cef0c7b9bb24bcbeeb0a28d73c9b37fd))
- Improve Document API with as_str and as_bytes methods - ([100fb16](https://github.com/adclz/auto-lsp/commit/100fb161f24ab255f0465535abc120d5869f376b))


## [0.6.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-core-v0.5.0...auto-lsp-core-v0.6.0)

### Features

- *(errors)* Add UnexpectedSymbol variant to AstError enum - ([5c027dd](https://github.com/adclz/auto-lsp/commit/5c027dd672782851783c54023d0c888cbcaa06fa))
- *(errors)* Add last errors - ([fbd9725](https://github.com/adclz/auto-lsp/commit/fbd9725809291912ab20ffc7dfc2311e8a9ce10f))
- *(errors)* Add error types for text retrieval failures - ([872152e](https://github.com/adclz/auto-lsp/commit/872152e0989e5034effaefbab1e5060450ac2073))
- *(errors)* Add AutoLspError and related error types - ([570470f](https://github.com/adclz/auto-lsp/commit/570470fc7695d67ed7c654545f447b3b1baad63f))
- *(symbol)* Implement Display - ([7f33825](https://github.com/adclz/auto-lsp/commit/7f338258752d94a8bc6f5b2776a21241d0c7e86e))
- Add dispatch_once! macro - ([06d4e55](https://github.com/adclz/auto-lsp/commit/06d4e55d498b7bd3d85572f41ec7a357a1ed9848))
- FileManager trait - ([bb0c753](https://github.com/adclz/auto-lsp/commit/bb0c753c6983a756a3d4b888ba5ab0fbd29a9899))
- Add fastrace tracing - ([801bd4c](https://github.com/adclz/auto-lsp/commit/801bd4c513b34b61f6182c8788465f8e6f4a80a4))
- Add lower method to `AstNode` trait - ([c11a3a7](https://github.com/adclz/auto-lsp/commit/c11a3a7e385c0a81f72005fbb099539d345afbcc))
- Builder pattern for AST with ids and parents - ([e8b57cb](https://github.com/adclz/auto-lsp/commit/e8b57cb5964736e681f37cf6a056c31b5e1eb9be))
- Integrate id-arena and remove Symbol wrapper - ([7ddfbdd](https://github.com/adclz/auto-lsp/commit/7ddfbdd617f209cbacf03ca05dcacc617061db67))
- Add id-arena dependency - ([a7db203](https://github.com/adclz/auto-lsp/commit/a7db203c763c5768e26c98673e9f00ccaae4ba11))
- Add thiserror dependency - ([693b3d8](https://github.com/adclz/auto-lsp/commit/693b3d8c4d3a3233699809b7673511a025193a0d))

### Bug Fixes

- *(document)* Invalid line index when position is out-of-bounds - ([e5d920f](https://github.com/adclz/auto-lsp/commit/e5d920f9264d3074d4831f97834bbb410fc1174c))
- *(document)* Invalid offset position when line is 0 - ([599b475](https://github.com/adclz/auto-lsp/commit/599b47545fccd5bcca1bda7a1b5f58b364d2b70e))
- Remove unused dependencies from Cargo.toml files - ([141bcf9](https://github.com/adclz/auto-lsp/commit/141bcf9ae6d835d6a1d6e3f3d6563cb15b65afed))
- Derive Debug for ParseErrorAccumulator - ([1cae45b](https://github.com/adclz/auto-lsp/commit/1cae45b78b2ed4a5ea3183d8f7160f6e5e7cb034))
- Update get_parent method to accept a slice and enhance equality check in PartialEq - ([9838231](https://github.com/adclz/auto-lsp/commit/9838231410e7e4584369482b3e0c299afe6cac54))
- Add error for missing perFileParser in initialization options - ([3e2f0f4](https://github.com/adclz/auto-lsp/commit/3e2f0f404431559e8688e0e6dd0bb3933356e0c2))
- Error conversion to lsp_types::Diagnostic - ([2023eac](https://github.com/adclz/auto-lsp/commit/2023eac221ea5b824344f9980dfefa89b0823f79))
- Fix windows path - ([64bd534](https://github.com/adclz/auto-lsp/commit/64bd534a3061669bab74769067034bd577b2e27b))
- Fix(document) get position after last br index - ([1935dc6](https://github.com/adclz/auto-lsp/commit/1935dc6744341e642f09ecaffd72bdc3def4f489))

### Refactor

- *(ast-node)* Remove capabilities module from the project - ([701d4b7](https://github.com/adclz/auto-lsp/commit/701d4b79f7ae4ea68fa97839e4cea49acabaa342))
- *(ast-python)* Replace trait implementations with dispatch functions - ([c11861e](https://github.com/adclz/auto-lsp/commit/c11861e3f2d2457bf8ad2cc382ea0e43f0421fcb))
- *(core)* Replace anyhow with TreeSitterError - ([fe11ea9](https://github.com/adclz/auto-lsp/commit/fe11ea9ff43010b77d1ab49ca3176095d2184053))
- *(core)* LSP capabilities now support error propagation - ([f713cc1](https://github.com/adclz/auto-lsp/commit/f713cc1455a7c1862e9769aaa8369fb58d525902))
- *(core_build)* Stack builder - ([604cd7b](https://github.com/adclz/auto-lsp/commit/604cd7b9365405c50afbae8aefc39ae983844023))
- *(core_build)* Remove url field - ([763aca6](https://github.com/adclz/auto-lsp/commit/763aca6f0283bb6a865d1551948849d39c9a52ba))
- *(document)* Add new error types - ([aa9371c](https://github.com/adclz/auto-lsp/commit/aa9371caa95ee95e79fda8a0670d1f5431ede595))
- *(document)* Move LAST_LINE to thread-local storage - ([ace0220](https://github.com/adclz/auto-lsp/commit/ace0220d9c600caf4a68a683b187c15d063d3f11))
- *(errors)* Rename AutoLspError to ParseError - ([2d56839](https://github.com/adclz/auto-lsp/commit/2d56839f2b64be4d63acec0bf8546d6b93ea6478))
- *(errors)* Replace anyhow usage with new errors - ([a9e773d](https://github.com/adclz/auto-lsp/commit/a9e773d658426e2fb73f8d950ba80f5c530911fb))
- *(errors)* Replace Diagnostic with AstError and update error handling across modules - ([8c47155](https://github.com/adclz/auto-lsp/commit/8c4715575af3626326624e808c795dcfce93bcef))
- *(errors)* Simplify AutoLspError and AstError enums, remove URL references - ([8f30f33](https://github.com/adclz/auto-lsp/commit/8f30f3388293963f8e0922021938d9e2e7c0fe85))
- *(errors)* Update methods to return Result with DocumentError - ([12b53bf](https://github.com/adclz/auto-lsp/commit/12b53bf69cd07fdb5ff4689d3826192795889192))
- *(lexer)* Replace DiagnosticAccumulator with AutoLspErrorAccumulator - ([2eea510](https://github.com/adclz/auto-lsp/commit/2eea510887837ef415ea3fab8761fea791c989f3))
- *(parser)* Remove range parameter from symbol creation methods - ([ba888b1](https://github.com/adclz/auto-lsp/commit/ba888b1039429be8c43efc6a1790c9c4c925dca1))
- *(parsers)* Streamline parser structure and simplify configure_parsers macro - ([92ab0ee](https://github.com/adclz/auto-lsp/commit/92ab0ee91ca44604b320f07c0e54b6da2655b14b))
- Split server and database modules into separate crates - ([1f768f1](https://github.com/adclz/auto-lsp/commit/1f768f12695e1ca2001bd1e1964a3528f71ac26b))
- Rename InvokeParserFn2 to InvokeParserFn - ([0c0e5cd](https://github.com/adclz/auto-lsp/commit/0c0e5cd572c19ef4f93708ad3cf1cc349c4841d0))
- Remove Document RwLock in db - ([e6c44d6](https://github.com/adclz/auto-lsp/commit/e6c44d6cda21c7909580d65a09de7b348cd6b1c8))
- Remove default implementation of capabilities - ([f779f28](https://github.com/adclz/auto-lsp/commit/f779f2854b44077f79626852f23f7d88682f1469))
- Remove min_specialization - ([85f2fc6](https://github.com/adclz/auto-lsp/commit/85f2fc6b8dfb3aeb7bde89cc10f31f906f0a213c))
- Update AST node creationto use database and improve error handling - ([9597099](https://github.com/adclz/auto-lsp/commit/9597099c38c55499cd7d90bd9e9b8057907c611b))
- Rename ParsedAst2 to ParsedAst and update trait impls - ([a9c1fcb](https://github.com/adclz/auto-lsp/commit/a9c1fcbef706acef3d62cff54d33a1837285a5c1))
- Remove old AST errors - ([2ebacb2](https://github.com/adclz/auto-lsp/commit/2ebacb2cdef326f6b8bb3059b6dadf2269048b9a))
- Remove core_build, unused core_ast modules and proc_macro crate - ([1decdec](https://github.com/adclz/auto-lsp/commit/1decdec4d50bd4b0ed06e11a3c71ba27608d7e5a))
- Reorganize AST-related and DB modules - ([d9f4dfb](https://github.com/adclz/auto-lsp/commit/d9f4dfb4ab72a67a995404b31a956a409449c320))
- Disable tests and core_build modules - ([3e1c457](https://github.com/adclz/auto-lsp/commit/3e1c45751b2da7cf2bd3b7b3d72ae8560419ea73))
- Remove log feature - ([ba5c57b](https://github.com/adclz/auto-lsp/commit/ba5c57bf333d0745077804a148adf28ea3753420))
- Remove rayon and deadlock_detection features - ([bdb21c0](https://github.com/adclz/auto-lsp/commit/bdb21c0e98d5aefe9d614b7ea9713f4ef784bdd9))
- Use id_ctr instead of tree sitter subtree pointers - ([6bcfb41](https://github.com/adclz/auto-lsp/commit/6bcfb41328e7059b601f78d6d7f186662a1e8400))
- Simplify borrowing for pending symbols - ([d55c2c0](https://github.com/adclz/auto-lsp/commit/d55c2c0b0db2e2ecfc89920e6bf9ab776acea1ae))
- Replace tuple parameters with TryFromParams type alias - ([ef57211](https://github.com/adclz/auto-lsp/commit/ef572119115a16ffd3963b7ebc352d5d24b8dfdd))
- Remove symbol module (WeakSymbol and DynSymbol) - ([89e3748](https://github.com/adclz/auto-lsp/commit/89e3748f48055116eaf0e240deeb4285a2de9685))
- Replace parse result tuple with vec only - ([21586b9](https://github.com/adclz/auto-lsp/commit/21586b986ab95ada53885b4caa89948c195f9ca5))
- Remove Traverse trait - ([2deea94](https://github.com/adclz/auto-lsp/commit/2deea946373b6b98d27fdbb32cde0d400f43af35))
- Remove unused parameters from AddSymbol trait methods - ([2fc7f4b](https://github.com/adclz/auto-lsp/commit/2fc7f4b10460e75ee80f62e7630f349c0a0eb715))
- Remove Finalize trait - ([e9421f5](https://github.com/adclz/auto-lsp/commit/e9421f55f2c54621c21fccbc1c9aa15c4a9b10b6))
- Update AST symbol handling with new ID management and mutable data access - ([af49550](https://github.com/adclz/auto-lsp/commit/af495504a24f0f04df899c1665294f6eba8b3d57))
- Simplify InvalidSymbol error message in AstError enum - ([0a05c64](https://github.com/adclz/auto-lsp/commit/0a05c64b82e7bb645c2bc70cdc0d23ed65cb0e9d))
- Rewrite tests and capabilities with new Arc wrapper - ([ce66d7d](https://github.com/adclz/auto-lsp/commit/ce66d7dfcabfd42fa4eea0fbd5c5950826564e47))
- Replace Symbol wrapper with Arc - ([1ebce96](https://github.com/adclz/auto-lsp/commit/1ebce96656cadba64271231e8ac51266c3b7a05c))
- Remove RwLock, mutable traits and methods from AST - ([2d0015b](https://github.com/adclz/auto-lsp/commit/2d0015b4106c151891abdae37a129498a740e570))
- Remove TryFromBuilder and TryIntoBuilder traits - ([7ce632c](https://github.com/adclz/auto-lsp/commit/7ce632ca1120de57a5f4d2daaab4b8973eb258d5))
- Replace TryFromBuilder with TryFrom in downcasting and parsing traits - ([1036c4b](https://github.com/adclz/auto-lsp/commit/1036c4b2ff253d947a086bfcd2d6f45e1a638940))
- Streamline GetSymbolData implementation and add inline attributes - ([342bb7c](https://github.com/adclz/auto-lsp/commit/342bb7c1e7396f8f79481415b6820e7a20c7df8c))
- Remove Url references - ([9da8416](https://github.com/adclz/auto-lsp/commit/9da84165da43c37f8905a784c7279b337dcb1a2c))
- Text retrieval methods now return Results - ([4de460d](https://github.com/adclz/auto-lsp/commit/4de460d09b03714eba62b5cb172ccf1ef6e2aab6))
- Buildable trait range getter - ([3927d68](https://github.com/adclz/auto-lsp/commit/3927d688c4320349202d60d17a96dc51e535a24c))
- Remove new_and_check method and add From trait idioms - ([40a1c42](https://github.com/adclz/auto-lsp/commit/40a1c4284aa7abce1071a8a99802566649b57bbf))
- Disable salsa default features - ([7161c72](https://github.com/adclz/auto-lsp/commit/7161c728b9656ff12edc1eb6f9ebbacbeccd77fd))
- Move core and proc-macro to crates folder - ([9ca4d9c](https://github.com/adclz/auto-lsp/commit/9ca4d9c260d764dda4256a0bbbd85684a968c864))

### Documentation

- *(errors)* Enhance documentation for error types - ([1af524c](https://github.com/adclz/auto-lsp/commit/1af524c47d56d60651656971f44cdb12206791a1))
- Update descriptions in Cargo.toml and lib.rs - ([8501e7c](https://github.com/adclz/auto-lsp/commit/8501e7c2070e5d8d1923765fc955dd864acbab53))
- Add doc for dispatch macros - ([f1d4db8](https://github.com/adclz/auto-lsp/commit/f1d4db8c085911b6c3c52bf8c794298322134826))
- Add documentation for salsa module - ([5be2989](https://github.com/adclz/auto-lsp/commit/5be298902edac11bf3f92d8ddc626f7d7ff7fcec))
- AstNode trait - ([5c1d33c](https://github.com/adclz/auto-lsp/commit/5c1d33c0801d54aa5c6c373a39896711016d6a79))

### Testing

- *(iter)* Add tests for traversing AST nodes - ([9fffc3e](https://github.com/adclz/auto-lsp/commit/9fffc3ede4c0c391c5948a43aed43669b8828d67))

### Miscellaneous Tasks

- *(license)* Update Cargo.toml files - ([dd5971f](https://github.com/adclz/auto-lsp/commit/dd5971f8d8c5e0ffa5fa0b97c0a3b3c517c2f82c))
- *(license)* Add GPLv3 license header to all source files - ([60d6d5a](https://github.com/adclz/auto-lsp/commit/60d6d5abe8a3e10f79fe651de074fa61cad9e7f6))
- Remove id-arena dependency - ([b4b5923](https://github.com/adclz/auto-lsp/commit/b4b592359f45cf072234d8c1b6769ce0a091e1be))
- Update license header in errors.rs - ([90e5b8f](https://github.com/adclz/auto-lsp/commit/90e5b8f577d6dd4ba76183c7398c6b96c75edf78))
- Salsa macros feature - ([b9e90dd](https://github.com/adclz/auto-lsp/commit/b9e90ddc54f1c9ad3fb84190bfca784cbb326d50))
- Logs for file events - ([219a6b2](https://github.com/adclz/auto-lsp/commit/219a6b273633bccddfc62c708ec4517bc36dbb5b))


## [0.5.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-core-v0.4.0...auto-lsp-core-v0.5.0)

### Features

- *(core)* Update reference resolution to include workspace and diagnostics - ([42e3c64](https://github.com/adclz/auto-lsp/commit/42e3c6421401dea5237a37040854b27604858480))
- *(core)* Move checks and references modulesto core_ast module - ([f185fb7](https://github.com/adclz/auto-lsp/commit/f185fb7cc819e1df94d567e84122ebc9d7fe2d92))
- Add optional rayon support for workspace init - ([7c79786](https://github.com/adclz/auto-lsp/commit/7c79786274400404ca125950d2f89cb12f1e13dd))

### Bug Fixes

- Update capabilities - ([b7d903a](https://github.com/adclz/auto-lsp/commit/b7d903ab06ae06733a2c160870149946c08f5cdd))

### Refactor

- Split out tree sitter and ast diagnostics - ([e19eb4d](https://github.com/adclz/auto-lsp/commit/e19eb4de7ddae36485a6d4306c888bef18588c0c))
- Remove unused parse_symbols method and simplify build function - ([8be148f](https://github.com/adclz/auto-lsp/commit/8be148f72b28022939f236ead13597c547110d2a))

### Documentation

- Update crates doc - ([27ba4c2](https://github.com/adclz/auto-lsp/commit/27ba4c28be55a58fd7759551ef9a82459af109dc))

### Miscellaneous Tasks

- Update dependencies - ([4a1b3a4](https://github.com/adclz/auto-lsp/commit/4a1b3a4011dbc119b4fa5c453722af391caf2c83))
- Remove duplicated 'Unreleased' section from changelogs - ([cc416ef](https://github.com/adclz/auto-lsp/commit/cc416efc6cc0737360c993d2b0d86b8a77c416ca))


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
