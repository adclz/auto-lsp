# Changelog

## [Unreleased]

## [0.6.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-v0.5.1...auto-lsp-v0.6.0)

### Features

- *(codegen)* Allow extending the default Token list - ([b632c9f](https://github.com/adclz/auto-lsp/commit/b632c9f0c9dd58f6d73b6b15006bb31c6d2927e1))
- *(dispatch)* Add macro for dispatching methods based on type - ([6900746](https://github.com/adclz/auto-lsp/commit/6900746833288e0cf5aa08010355fa0f6241e0fe))
- *(doc)* Add README.md for Html and Python examples - ([2ed08c6](https://github.com/adclz/auto-lsp/commit/2ed08c6cc6437582b17159bfd68735d0d8561b87))
- *(doc)* Add ARCHITECTURE.md - ([8a58173](https://github.com/adclz/auto-lsp/commit/8a5817384191268a78fec9287dfde13bd56a6ba8))
- *(errors)* Add UnexpectedSymbol variant to AstError enum - ([5c027dd](https://github.com/adclz/auto-lsp/commit/5c027dd672782851783c54023d0c888cbcaa06fa))
- *(errors)* Add last errors - ([fbd9725](https://github.com/adclz/auto-lsp/commit/fbd9725809291912ab20ffc7dfc2311e8a9ce10f))
- *(errors)* Add error types for text retrieval failures - ([872152e](https://github.com/adclz/auto-lsp/commit/872152e0989e5034effaefbab1e5060450ac2073))
- *(errors)* Add AutoLspError and related error types - ([570470f](https://github.com/adclz/auto-lsp/commit/570470fc7695d67ed7c654545f447b3b1baad63f))
- *(examples)* Add native vscode LSP extension - ([cd5201d](https://github.com/adclz/auto-lsp/commit/cd5201d8ff642533ef679f89a5a566dd11665c00))
- *(symbol)* Implement Display - ([7f33825](https://github.com/adclz/auto-lsp/commit/7f338258752d94a8bc6f5b2776a21241d0c7e86e))
- *(tests)* Salsa file events - ([83b7e7c](https://github.com/adclz/auto-lsp/commit/83b7e7c7838a35277f1524ed1c2b6d1be35aa3c3))
- Add dispatch_once! macro - ([06d4e55](https://github.com/adclz/auto-lsp/commit/06d4e55d498b7bd3d85572f41ec7a357a1ed9848))
- FileManager trait - ([bb0c753](https://github.com/adclz/auto-lsp/commit/bb0c753c6983a756a3d4b888ba5ab0fbd29a9899))
- Add fastrace tracing - ([801bd4c](https://github.com/adclz/auto-lsp/commit/801bd4c513b34b61f6182c8788465f8e6f4a80a4))
- Add lower method to `AstNode` trait - ([c11a3a7](https://github.com/adclz/auto-lsp/commit/c11a3a7e385c0a81f72005fbb099539d345afbcc))
- Builder pattern for AST with ids and parents - ([e8b57cb](https://github.com/adclz/auto-lsp/commit/e8b57cb5964736e681f37cf6a056c31b5e1eb9be))
- Ast-python crate - ([571ee85](https://github.com/adclz/auto-lsp/commit/571ee85208fcbeb471fb6594714bb3d063f926f1))
- Enhance codegen - ([e4756c6](https://github.com/adclz/auto-lsp/commit/e4756c611b48a36185713f489d5bc517126219d6))
- Codegen crate - ([4160c01](https://github.com/adclz/auto-lsp/commit/4160c010c6231b40c65e3d221292d8badfe88dbb))
- Integrate id-arena and remove Symbol wrapper - ([7ddfbdd](https://github.com/adclz/auto-lsp/commit/7ddfbdd617f209cbacf03ca05dcacc617061db67))
- Add id-arena dependency - ([a7db203](https://github.com/adclz/auto-lsp/commit/a7db203c763c5768e26c98673e9f00ccaae4ba11))
- Implement TryFrom trait for EnumBuilder and StructBuilder - ([240a8ed](https://github.com/adclz/auto-lsp/commit/240a8ed33fbec0a1e6f9dfde4572ed5ca973009e))
- Add thiserror dependency - ([693b3d8](https://github.com/adclz/auto-lsp/commit/693b3d8c4d3a3233699809b7673511a025193a0d))
- Workspace Database - ([cebf224](https://github.com/adclz/auto-lsp/commit/cebf224e180c8cb0470457ecfe3c13c78f8c9ec2))
- Add ustr - ([dc3065f](https://github.com/adclz/auto-lsp/commit/dc3065f91d69ba4eb93be38d94098919d39789de))
- Add minimal salsa setup - ([311961d](https://github.com/adclz/auto-lsp/commit/311961d5a8be3e553c7f19d6e49aac556e019ce8))

### Bug Fixes

- *(CI)* Streamline AST_GEN env variable - ([5b2a42d](https://github.com/adclz/auto-lsp/commit/5b2a42dc0bfdac5375ac589b7ef192154a648c15))
- *(ci)* Exclude auto-lsp-codegen from cargo test - ([c659fc5](https://github.com/adclz/auto-lsp/commit/c659fc52a32c5117807aee84ceb1d997f374f156))
- *(ci)* Correct path for codegen crate - ([9ff438d](https://github.com/adclz/auto-lsp/commit/9ff438d94b886e670807661901fc4b428fbcf982))
- *(codegen)* Improve node ID retrieval in generate_struct and generate_enum functions - ([f9551e5](https://github.com/adclz/auto-lsp/commit/f9551e570bd0926c75f7102da0917fc321297193))
- *(codegen)* Avoid uppercasing Token_ names - ([3445c18](https://github.com/adclz/auto-lsp/commit/3445c18be12fb176f99d010b5b28d5d5970a25b6))
- *(document)* Invalid line index when position is out-of-bounds - ([e5d920f](https://github.com/adclz/auto-lsp/commit/e5d920f9264d3074d4831f97834bbb410fc1174c))
- *(document)* Invalid offset position when line is 0 - ([599b475](https://github.com/adclz/auto-lsp/commit/599b47545fccd5bcca1bda7a1b5f58b364d2b70e))
- *(enum)* Replace unreachable with AstError in enums - ([3b3213c](https://github.com/adclz/auto-lsp/commit/3b3213c09549f4dec851752b959dbe34ff41563f))
- *(parse)* Handle unused Result - ([27743f3](https://github.com/adclz/auto-lsp/commit/27743f3361f8d1fd4bb728b363aa3c1db3597ab8))
- *(temp)* Handle potential errors during workspace initialization - ([996d248](https://github.com/adclz/auto-lsp/commit/996d248db92aa9faa64066b3f110e7fbcade879d))
- *(tests)* Replace deref calls with lower method - ([2cf31a6](https://github.com/adclz/auto-lsp/commit/2cf31a6ee1767ffeff38d06ef8d758745698ca1e))
- *(tests)* Invalid ast-html dependencies - ([19533b4](https://github.com/adclz/auto-lsp/commit/19533b49edc76d67f399c050ee3dd29f26f0cf65))
- Remove unused dependencies from Cargo.toml files - ([141bcf9](https://github.com/adclz/auto-lsp/commit/141bcf9ae6d835d6a1d6e3f3d6563cb15b65afed))
- Derive Debug for ParseErrorAccumulator - ([1cae45b](https://github.com/adclz/auto-lsp/commit/1cae45b78b2ed4a5ea3183d8f7160f6e5e7cb034))
- Add FileManager import - ([30bda23](https://github.com/adclz/auto-lsp/commit/30bda23450ccc49575efa7eb119b52c4a93a6fae))
- Import of RegexToDocumentLink in documentation - ([ec3834b](https://github.com/adclz/auto-lsp/commit/ec3834b6d6b652abfbd4f9685711dd5d41b9ee12))
- Update get_parent method to accept a slice and enhance equality check in PartialEq - ([9838231](https://github.com/adclz/auto-lsp/commit/9838231410e7e4584369482b3e0c299afe6cac54))
- Cargo workspace codegen dep - ([e40cce6](https://github.com/adclz/auto-lsp/commit/e40cce6af9281df607c0050d358adf0c34f2e76c))
- Correct file extension in snap! macro from .py to .html - ([7697af3](https://github.com/adclz/auto-lsp/commit/7697af319e36c1370c1e8a9a3ee624a545d1c451))
- Conditionally include corpus module for non-wasm32 targets - ([7018764](https://github.com/adclz/auto-lsp/commit/701876449a74828e42178809988466547bd95de5))
- Add insta binary in CI - ([d81c094](https://github.com/adclz/auto-lsp/commit/d81c09410dd4e67330cce0598d21ad14d2183480))
- Duplicate file names for snapshots - ([fe2dc0b](https://github.com/adclz/auto-lsp/commit/fe2dc0b592d0c486d434904141c855144055038e))
- Invalid file names on windows - ([d437bc6](https://github.com/adclz/auto-lsp/commit/d437bc67e104a6d1b82787e07f695d7b65010854))
- Remove parents tests - ([a82c5db](https://github.com/adclz/auto-lsp/commit/a82c5db49f0ee8469f16592e16978ad135d3130f))
- File import - ([49cb163](https://github.com/adclz/auto-lsp/commit/49cb163c87f117db7ae5f44be3313cfa517408e6))
- Correct return type in get_hover method signature - ([ee10be8](https://github.com/adclz/auto-lsp/commit/ee10be818e1be7838623dec1463243f75e756bf7))
- Add error for missing perFileParser in initialization options - ([3e2f0f4](https://github.com/adclz/auto-lsp/commit/3e2f0f404431559e8688e0e6dd0bb3933356e0c2))
- Error conversion to lsp_types::Diagnostic - ([2023eac](https://github.com/adclz/auto-lsp/commit/2023eac221ea5b824344f9980dfefa89b0823f79))
- Fix windows path - ([64bd534](https://github.com/adclz/auto-lsp/commit/64bd534a3061669bab74769067034bd577b2e27b))
- Fix fs imports on windows - ([b110347](https://github.com/adclz/auto-lsp/commit/b1103477064e77bc4150889739bdde1fb8239a71))
- Look for extension in hashmap values in open document event - ([c8bd28b](https://github.com/adclz/auto-lsp/commit/c8bd28b13564526d3e69920edbf945c1dcb73784))
- Fix(document) get position after last br index - ([1935dc6](https://github.com/adclz/auto-lsp/commit/1935dc6744341e642f09ecaffd72bdc3def4f489))
- Add 'log' feature towasi CI workflow - ([09ce1d2](https://github.com/adclz/auto-lsp/commit/09ce1d2578ca18cb8f0d30f7ba68e738db986e44))

### Refactor

- *(ast-node)* Remove capabilities module from the project - ([701d4b7](https://github.com/adclz/auto-lsp/commit/701d4b79f7ae4ea68fa97839e4cea49acabaa342))
- *(ast-python)* Replace trait implementations with dispatch functions - ([c11861e](https://github.com/adclz/auto-lsp/commit/c11861e3f2d2457bf8ad2cc382ea0e43f0421fcb))
- *(bench)* Remove obsolete benchmarking script - ([25fea02](https://github.com/adclz/auto-lsp/commit/25fea02141a9dbd6fdabe2d6089b460b52b944d8))
- *(benches)* Update benchmarks - ([ebd2592](https://github.com/adclz/auto-lsp/commit/ebd259267625bdb42a5c85f3d6e68fe15b0709de))
- *(build)* Add AST_GEN environment variable in build.rs scripts - ([2597d71](https://github.com/adclz/auto-lsp/commit/2597d7104dd53d04e2e0723d0e7bc4e3927659af))
- *(capabilities)* Use closure instead of trait and add traversal kind enum - ([9c7fc38](https://github.com/adclz/auto-lsp/commit/9c7fc38ee774e995267cf7a874fe702d363b8f46))
- *(capabilities)* Remove unused references module - ([e4a52d0](https://github.com/adclz/auto-lsp/commit/e4a52d0bba6e50f6ee82edfa179acfdadaba663d))
- *(choice)* Make choice macro return compile errors alongside input - ([039ec05](https://github.com/adclz/auto-lsp/commit/039ec051a084923a64b6aa3d12ae2217b2e344d0))
- *(choice)* Update extract_variants to accept syn::DataEnum - ([2830cb4](https://github.com/adclz/auto-lsp/commit/2830cb489b49218a6e4e74c79e15ef230d9d80f2))
- *(ci)* Rename CI workflows - ([9b12652](https://github.com/adclz/auto-lsp/commit/9b1265230d96479b55146056df67642ea99ad5ba))
- *(ci)* Remove nightly toolchain installation from CI - ([f04cf9b](https://github.com/adclz/auto-lsp/commit/f04cf9b20bd9b9c72eaf63ba066d367f00af383d))
- *(code-gen)* Supertypes - ([e6311d4](https://github.com/adclz/auto-lsp/commit/e6311d47994c71329e288be2bd3330884c971b3c))
- *(codegen)* Update token handling to extend or overwrite default tokens - ([4ef4d57](https://github.com/adclz/auto-lsp/commit/4ef4d572b125d8eb6d2185dcfe18de87947979c0))
- *(core)* Replace anyhow with TreeSitterError - ([fe11ea9](https://github.com/adclz/auto-lsp/commit/fe11ea9ff43010b77d1ab49ca3176095d2184053))
- *(core)* LSP capabilities now support error propagation - ([f713cc1](https://github.com/adclz/auto-lsp/commit/f713cc1455a7c1862e9769aaa8369fb58d525902))
- *(core_build)* Stack builder - ([604cd7b](https://github.com/adclz/auto-lsp/commit/604cd7b9365405c50afbae8aefc39ae983844023))
- *(core_build)* Remove url field - ([763aca6](https://github.com/adclz/auto-lsp/commit/763aca6f0283bb6a865d1551948849d39c9a52ba))
- *(db)* Update salsa to 0.22 - ([2c17aae](https://github.com/adclz/auto-lsp/commit/2c17aae321a8e40e5ff70fc5640cb9ced5e45bcc))
- *(doc)* README.md - ([307f2da](https://github.com/adclz/auto-lsp/commit/307f2da94eb99572e85a943ca3bfd6a77037744e))
- *(document)* Add new error types - ([aa9371c](https://github.com/adclz/auto-lsp/commit/aa9371caa95ee95e79fda8a0670d1f5431ede595))
- *(document)* Move LAST_LINE to thread-local storage - ([ace0220](https://github.com/adclz/auto-lsp/commit/ace0220d9c600caf4a68a683b187c15d063d3f11))
- *(document)* Update position retrieval methods - ([bd77be2](https://github.com/adclz/auto-lsp/commit/bd77be275c931d57f9e22a5a1aa92689ec0e37b8))
- *(errors)* Rename AutoLspError to ParseError - ([2d56839](https://github.com/adclz/auto-lsp/commit/2d56839f2b64be4d63acec0bf8546d6b93ea6478))
- *(errors)* Replace anyhow usage with new errors - ([a9e773d](https://github.com/adclz/auto-lsp/commit/a9e773d658426e2fb73f8d950ba80f5c530911fb))
- *(errors)* Add CheckErrorAccumulator in python type checking - ([2e97aca](https://github.com/adclz/auto-lsp/commit/2e97acad0c6daddea21d6bb05659b1c141aabb4e))
- *(errors)* Replace Diagnostic with AstError and update error handling across modules - ([8c47155](https://github.com/adclz/auto-lsp/commit/8c4715575af3626326624e808c795dcfce93bcef))
- *(errors)* Simplify AutoLspError and AstError enums, remove URL references - ([8f30f33](https://github.com/adclz/auto-lsp/commit/8f30f3388293963f8e0922021938d9e2e7c0fe85))
- *(errors)* Update methods to return Result with DocumentError - ([12b53bf](https://github.com/adclz/auto-lsp/commit/12b53bf69cd07fdb5ff4689d3826192795889192))
- *(examples)* Replace disptach functions - ([650d2ef](https://github.com/adclz/auto-lsp/commit/650d2ef61e6b2c6160de52084d9a72255114d778))
- *(examples)* Apply new idioms to examples - ([921662f](https://github.com/adclz/auto-lsp/commit/921662f4f123e1fe48691d1383cd51e7bc634d27))
- *(lexer)* Replace DiagnosticAccumulator with AutoLspErrorAccumulator - ([2eea510](https://github.com/adclz/auto-lsp/commit/2eea510887837ef415ea3fab8761fea791c989f3))
- *(parser)* Remove range parameter from symbol creation methods - ([ba888b1](https://github.com/adclz/auto-lsp/commit/ba888b1039429be8c43efc6a1790c9c4c925dca1))
- *(parsers)* Streamline parser structure and simplify configure_parsers macro - ([92ab0ee](https://github.com/adclz/auto-lsp/commit/92ab0ee91ca44604b320f07c0e54b6da2655b14b))
- *(proc-macros)* Remove filter utilities module and remove deprecated code - ([4ce8494](https://github.com/adclz/auto-lsp/commit/4ce84944f0faffd0ba95c3b426fb699e84b4b7bc))
- *(proc-macros)* Better utilities  for type name extraction - ([17ebdac](https://github.com/adclz/auto-lsp/commit/17ebdacf1cb86a1710fc901ff81b8c260afc30b6))
- *(root)* Remove unsolved_checks and unsolved_references fields - ([9f23af7](https://github.com/adclz/auto-lsp/commit/9f23af7099e954cf28f7d25e0fe7d0c15cb2bdee))
- *(salsa)* Restructure module and rename core components - ([e7aad9c](https://github.com/adclz/auto-lsp/commit/e7aad9cb6316960c810396fdd9378a820682cccd))
- *(semantic_tokens)* Optimize semantic_tokens_range logic - ([f2ce358](https://github.com/adclz/auto-lsp/commit/f2ce358ee34344999d460df7d7f2ee3ca3ef2f61))
- *(seq)* Make seq macro return compile errors alongside input - ([8a6c8c6](https://github.com/adclz/auto-lsp/commit/8a6c8c6da86bc7a616636e918dcf8927262b578b))
- *(server)* Update Session implementation and modify workspace folder support - ([0f21f8c](https://github.com/adclz/auto-lsp/commit/0f21f8ca15c330082bfe2ce5e342a39d240a98cb))
- *(server)* Remove document_link module - ([a08426a](https://github.com/adclz/auto-lsp/commit/a08426a8efa62f0c89dec003c35d12f917a7930d))
- *(server)* Rename capabilities to default and separate BaseDataBase methods from generic ones - ([e7f6206](https://github.com/adclz/auto-lsp/commit/e7f6206c4174762a72686a563bdefc25e7afbb2b))
- *(server)* Remove deprecated capabilities - ([cf67d1b](https://github.com/adclz/auto-lsp/commit/cf67d1bbf77c3059a5e37f6286683526e0e36631))
- *(server)* Permissive initialization options - ([fe363b2](https://github.com/adclz/auto-lsp/commit/fe363b2ca297f926be696b16b7a3e16854bbd63e))
- *(test)* Python ast - ([a815d26](https://github.com/adclz/auto-lsp/commit/a815d2616f83a86595f012a7efab2d8f688b9d89))
- *(tests)* Rename test functions - ([cc18245](https://github.com/adclz/auto-lsp/commit/cc182458f4e1ab527e23dafd7dc007cbe79b217b))
- *(tests)* Rewrite default_parameters type checks - ([98d6c68](https://github.com/adclz/auto-lsp/commit/98d6c683df2942ea0ac510c1c789464a1c2afb46))
- *(vscode)* Refactor vscode examples with new dispatch API - ([b8ac105](https://github.com/adclz/auto-lsp/commit/b8ac105b32fa135e26f1edba86abbc4df17dcfc5))
- Remove unused imports - ([6b6379f](https://github.com/adclz/auto-lsp/commit/6b6379f419dcf0304bb5b318ffafbc177d96f211))
- Split server and database modules into separate crates - ([1f768f1](https://github.com/adclz/auto-lsp/commit/1f768f12695e1ca2001bd1e1964a3528f71ac26b))
- Replace BaseDatabase with salsa::Database in session modules - ([4b74fd8](https://github.com/adclz/auto-lsp/commit/4b74fd830d07302123bf1751f09531796647e832))
- Rename InvokeParserFn2 to InvokeParserFn - ([0c0e5cd](https://github.com/adclz/auto-lsp/commit/0c0e5cd572c19ef4f93708ad3cf1cc349c4841d0))
- Remove Document RwLock in db - ([e6c44d6](https://github.com/adclz/auto-lsp/commit/e6c44d6cda21c7909580d65a09de7b348cd6b1c8))
- (tests) replace dispatch functions with direct calls to capabilities - ([2d017a5](https://github.com/adclz/auto-lsp/commit/2d017a593600f8b9b9c7e7f5547a0ffc6331964b))
- Simplify LSP requests handling - ([a398fed](https://github.com/adclz/auto-lsp/commit/a398fed650c38aae9e5eacc788cfc69fa1fdb738))
- Remove default implementation of capabilities - ([f779f28](https://github.com/adclz/auto-lsp/commit/f779f2854b44077f79626852f23f7d88682f1469))
- Remove min_specialization - ([85f2fc6](https://github.com/adclz/auto-lsp/commit/85f2fc6b8dfb3aeb7bde89cc10f31f906f0a213c))
- Update AST node creationto use database and improve error handling - ([9597099](https://github.com/adclz/auto-lsp/commit/9597099c38c55499cd7d90bd9e9b8057907c611b))
- Update configure_parsers! macro - ([4b12122](https://github.com/adclz/auto-lsp/commit/4b121227542259d71e8f5d63f1ec92300c04a4c8))
- Rename ParsedAst2 to ParsedAst and update trait impls - ([a9c1fcb](https://github.com/adclz/auto-lsp/commit/a9c1fcbef706acef3d62cff54d33a1837285a5c1))
- Remove old AST errors - ([2ebacb2](https://github.com/adclz/auto-lsp/commit/2ebacb2cdef326f6b8bb3059b6dadf2269048b9a))
- Remove old test files - ([6f7a28b](https://github.com/adclz/auto-lsp/commit/6f7a28b76ebc5183c1ad4235fcbc37ce3c58daea))
- Rewrite CI and snapshot tests - ([b5d808d](https://github.com/adclz/auto-lsp/commit/b5d808d21f46177505a04b7c50eca4c17438cedd))
- Remove core_build, unused core_ast modules and proc_macro crate - ([1decdec](https://github.com/adclz/auto-lsp/commit/1decdec4d50bd4b0ed06e11a3c71ba27608d7e5a))
- Unlink proc-macro crate - ([054415e](https://github.com/adclz/auto-lsp/commit/054415e23e103b0594a54684a2c18de75d8a1e86))
- Clean up example code in parsers and lib files - ([22d0a36](https://github.com/adclz/auto-lsp/commit/22d0a36d373360ee66183ad9b84d9ef71b9f9fc7))
- Reorganize AST-related and DB modules - ([d9f4dfb](https://github.com/adclz/auto-lsp/commit/d9f4dfb4ab72a67a995404b31a956a409449c320))
- Codegen - ([d3469a0](https://github.com/adclz/auto-lsp/commit/d3469a0388ea9511e857d2bf28a932642f88a57b))
- Disable tests and core_build modules - ([3e1c457](https://github.com/adclz/auto-lsp/commit/3e1c45751b2da7cf2bd3b7b3d72ae8560419ea73))
- Move html tests to ast-html crate - ([6dd78ba](https://github.com/adclz/auto-lsp/commit/6dd78baa4851095d47fa01c1fb19e02c9159194f))
- Move python tests to new ast-python crate - ([4d11b0b](https://github.com/adclz/auto-lsp/commit/4d11b0bb3f1f42c8a41feb50c8f56d68bf7efb96))
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
- Remove stderrlog dependency - ([46b21dd](https://github.com/adclz/auto-lsp/commit/46b21dd86947d9e98d19c23b8ad85fe70d10357b))
- Text retrieval methods now return Results - ([4de460d](https://github.com/adclz/auto-lsp/commit/4de460d09b03714eba62b5cb172ccf1ef6e2aab6))
- Buildable trait range getter - ([3927d68](https://github.com/adclz/auto-lsp/commit/3927d688c4320349202d60d17a96dc51e535a24c))
- Remove new_and_check method and add From trait idioms - ([40a1c42](https://github.com/adclz/auto-lsp/commit/40a1c4284aa7abce1071a8a99802566649b57bbf))
- Disable salsa default features - ([7161c72](https://github.com/adclz/auto-lsp/commit/7161c728b9656ff12edc1eb6f9ebbacbeccd77fd))
- Move core and proc-macro to crates folder - ([9ca4d9c](https://github.com/adclz/auto-lsp/commit/9ca4d9c260d764dda4256a0bbbd85684a968c864))
- Remove root module - ([2658b45](https://github.com/adclz/auto-lsp/commit/2658b45532eb426e2db05f430001419334925da1))
- Remove workspace module - ([c62d95e](https://github.com/adclz/auto-lsp/commit/c62d95e1deaeb51f1de8fb61ddd14527718f3d74))
- Remove comment-related fields and references across modules - ([7a3ebf6](https://github.com/adclz/auto-lsp/commit/7a3ebf6d58be6699b945b085e047c32007fa4185))
- Use salsa accumulators for diagnostics - ([2346eb2](https://github.com/adclz/auto-lsp/commit/2346eb2b5afb1d676040c1c31ad17d64bc864452))
- Remove Check and IsCheck traits - ([d25ccdd](https://github.com/adclz/auto-lsp/commit/d25ccddd5c51a0962434bd3589e25dd25f7c8793))
- Remove edit_text_document and related document handling code - ([7cefad4](https://github.com/adclz/auto-lsp/commit/7cefad4f85d1f9dcef23e960d8782f0cee5dce23))
- Update changed_watched_files method - ([5f11967](https://github.com/adclz/auto-lsp/commit/5f119678277413cbcbd2ac1bd725d1db063981f0))
- Transform capabilities into standalone methods - ([80c0e4c](https://github.com/adclz/auto-lsp/commit/80c0e4cb0121a36ec669eb944b2e199bb445689e))
- Reorganize request and notification registration - ([3083033](https://github.com/adclz/auto-lsp/commit/3083033eca90ca42bf33b9c78431e2faacedcd6d))
- Simplify database access in Session - ([0c75382](https://github.com/adclz/auto-lsp/commit/0c75382f5737d6b29088d7713a70d8f10c6c0c61))
- Integrate WorkspaceDatabase into Session - ([8535275](https://github.com/adclz/auto-lsp/commit/85352752285ca93a1fcfa5e6dd86a94533a5c9bf))
- Update tests to use WorkspaceDatabase - ([6896b7b](https://github.com/adclz/auto-lsp/commit/6896b7b6686f1513b27d4523907388b1a93e6005))
- Remove resolve_checks calls and  checks module - ([acfb8f4](https://github.com/adclz/auto-lsp/commit/acfb8f40ccf3b888dae438511377bc4d6d33f7c1))
- Remove unused fields and traits from SymbolData - ([7ac032f](https://github.com/adclz/auto-lsp/commit/7ac032ff0359adf35a9cc3a19fb79d32e70ace13))
- Remove unused Reference and IsReference traits - ([90fc798](https://github.com/adclz/auto-lsp/commit/90fc798a40fe9a04c23df959e0c3bd157068bd74))
- Remove old references code - ([43f5771](https://github.com/adclz/auto-lsp/commit/43f5771d39e0f6bc9ff6207abd2e084eeee205a5))

### Documentation

- *(book)* Rewrite entire book content - ([b19ad6c](https://github.com/adclz/auto-lsp/commit/b19ad6cc88e3a4dcac757a0453d4a63fe33d9b0b))
- *(errors)* Enhance documentation for error types - ([1af524c](https://github.com/adclz/auto-lsp/commit/1af524c47d56d60651656971f44cdb12206791a1))
- Update workflows section in ARCHITECTURE.md - ([a9c0c6c](https://github.com/adclz/auto-lsp/commit/a9c0c6cb1686f2107a27b32988f9cbb7390d4e55))
- Add README files for default and server crates - ([c9a44d6](https://github.com/adclz/auto-lsp/commit/c9a44d61052a139be4f12b51bf6e98725478eba2))
- Add new crates to ARCHITECTURE.md - ([e10a8a0](https://github.com/adclz/auto-lsp/commit/e10a8a0b51b2b8df05dede8c4ce9d4ff145c4172))
- Fix link in codegen README - ([5e413fe](https://github.com/adclz/auto-lsp/commit/5e413feb850245a1ec293c48ec8e7d19742aced7))
- Update descriptions in Cargo.toml and lib.rs - ([8501e7c](https://github.com/adclz/auto-lsp/commit/8501e7c2070e5d8d1923765fc955dd864acbab53))
- Doc(book); remove deprecated sections, update SUMMARY - ([d344d51](https://github.com/adclz/auto-lsp/commit/d344d5160c475ceed6b19cd0b8f958aeb72fe38b))
- Add codegen documentation - ([a9d4933](https://github.com/adclz/auto-lsp/commit/a9d4933e306ab7c28905115566f8b1caad8d0069))
- Add doc for dispatch macros - ([f1d4db8](https://github.com/adclz/auto-lsp/commit/f1d4db8c085911b6c3c52bf8c794298322134826))
- Add documentation for salsa module - ([5be2989](https://github.com/adclz/auto-lsp/commit/5be298902edac11bf3f92d8ddc626f7d7ff7fcec))
- AstNode trait - ([5c1d33c](https://github.com/adclz/auto-lsp/commit/5c1d33c0801d54aa5c6c373a39896711016d6a79))
- Update README, lib and book - ([653672e](https://github.com/adclz/auto-lsp/commit/653672e53e478e77e74959e70ef2033196ccabdd))

### Styling

- README files - ([aa902c1](https://github.com/adclz/auto-lsp/commit/aa902c15da8ead570eb38cc6f718c27411cd5a01))
- Remove --- in README - ([2787f7a](https://github.com/adclz/auto-lsp/commit/2787f7a5cfe1936ee5a394533ed58455f5a4c2b2))

### Testing

- *(codegen)* Add tests for HTML, JavaScript, C, C#, and Haskell code generation - ([f38555c](https://github.com/adclz/auto-lsp/commit/f38555c8ea8c0108346cdf79ada8a38646abefc8))
- *(iter)* Add tests for traversing AST nodes - ([9fffc3e](https://github.com/adclz/auto-lsp/commit/9fffc3ede4c0c391c5948a43aed43669b8828d67))
- Add error handling  in snap! macro - ([5742ac6](https://github.com/adclz/auto-lsp/commit/5742ac6f55cedf8dbfa2c038469bbcb59fde1b95))
- Add hover and iter tests - ([1437cd9](https://github.com/adclz/auto-lsp/commit/1437cd94684ca8cf523657e4bdf14bb32b313695))
- Add python debug snapshots - ([3b0f50f](https://github.com/adclz/auto-lsp/commit/3b0f50f7b482d4ac2482685ca9fabc5c55b32fb4))
- Remove obsolete traverse tests - ([5478181](https://github.com/adclz/auto-lsp/commit/5478181bf96ef17773c8afbf9c4581d224a33f4d))
- Remove deadlock tests from the test suite - ([73b6d9a](https://github.com/adclz/auto-lsp/commit/73b6d9a2a859eb2faa921348cf05dc636d7508fc))

### Miscellaneous Tasks

- *(license)* Update Cargo.toml files - ([dd5971f](https://github.com/adclz/auto-lsp/commit/dd5971f8d8c5e0ffa5fa0b97c0a3b3c517c2f82c))
- *(license)* Add GPLv3 license header to all source files - ([60d6d5a](https://github.com/adclz/auto-lsp/commit/60d6d5abe8a3e10f79fe651de074fa61cad9e7f6))
- *(license)* Replace MIT License with GNU General Public License v3 - ([8b62227](https://github.com/adclz/auto-lsp/commit/8b62227b799cb4b49d9b1d585b22a705c9750ea3))
- Set release_always to false in release-plz config - ([3c7e784](https://github.com/adclz/auto-lsp/commit/3c7e784b5c820176180b6c0ac48e78d727753791))
- Use cargo-nextest  for codegen - ([faf2ac3](https://github.com/adclz/auto-lsp/commit/faf2ac3db1445d1b9c3e0b42ba746717b31baf57))
- Update generated files - ([d001d2e](https://github.com/adclz/auto-lsp/commit/d001d2ed48cbfb6d825a988786717d329466ccb5))
- Add codegen workflow and update CI status badges - ([44a43db](https://github.com/adclz/auto-lsp/commit/44a43db5ca41cc2473558d194e72b2275b79709a))
- Remove last mention of queries - ([16bd2b8](https://github.com/adclz/auto-lsp/commit/16bd2b84275b15943ea6975cd1b25467b21b1bd5))
- Remove mut in update_fil test - ([d359ede](https://github.com/adclz/auto-lsp/commit/d359edea6e5f934f2c4865cf468d0fead6ee4368))
- Update dependencies - ([c49d0de](https://github.com/adclz/auto-lsp/commit/c49d0de96c4b9e3445cb085e05b6569af1c78eb8))
- Typos in ARCHITECTURE and README - ([1a096df](https://github.com/adclz/auto-lsp/commit/1a096df5dabf12801f273b5c9c25344b539539bd))
- Update tree-sitter to 0.25 - ([ffdc41e](https://github.com/adclz/auto-lsp/commit/ffdc41e237637b6c1a3d207f59237effc9596b96))
- Remove criterion benchmarks - ([9bd97c2](https://github.com/adclz/auto-lsp/commit/9bd97c22567e35367f70d71507ffb6a1f371a3ad))
- New generated files - ([9b691ec](https://github.com/adclz/auto-lsp/commit/9b691ece236cde05d9a9e3e68b160a54cd0c3468))
- Remove unused dependencies from Cargo.toml - ([b85491a](https://github.com/adclz/auto-lsp/commit/b85491a98ac62f54c319a338950947c573a5fb30))
- Update snapshots - ([0ae7ac1](https://github.com/adclz/auto-lsp/commit/0ae7ac174b9e6a8491ff0659866aceabceea4e8b))
- Add new generated files - ([7888c8a](https://github.com/adclz/auto-lsp/commit/7888c8a3c2eb42704fbf9e928e601abbdcfb62f8))
- Update WASI and Server ci - ([42e2f2f](https://github.com/adclz/auto-lsp/commit/42e2f2f9de927240d6891bfe79c6f10883c56616))
- Ensure nightly toolchain is set as default in CI workflow - ([a7f4664](https://github.com/adclz/auto-lsp/commit/a7f4664ec4304020f79bddb030359b620d7ec154))
- Update ci - ([c0f7556](https://github.com/adclz/auto-lsp/commit/c0f75567ca0b1bb457a02c44d5ba6e91b920e8d5))
- Update settings to include new crates - ([22c378f](https://github.com/adclz/auto-lsp/commit/22c378f9835f54dec170f0b221a3b7f3f415f0a9))
- Update WASI CI - ([9ff7031](https://github.com/adclz/auto-lsp/commit/9ff7031818a08143db17b8aa0ed7e64a0acccadf))
- Remove id-arena dependency - ([b4b5923](https://github.com/adclz/auto-lsp/commit/b4b592359f45cf072234d8c1b6769ce0a091e1be))
- Update examples - ([ebc48eb](https://github.com/adclz/auto-lsp/commit/ebc48ebb19f34b9d150017013fe1ed0fd1700440))
- Update license header in errors.rs - ([90e5b8f](https://github.com/adclz/auto-lsp/commit/90e5b8f577d6dd4ba76183c7398c6b96c75edf78))
- Salsa macros feature - ([b9e90dd](https://github.com/adclz/auto-lsp/commit/b9e90ddc54f1c9ad3fb84190bfca784cbb326d50))
- Logs for file events - ([219a6b2](https://github.com/adclz/auto-lsp/commit/219a6b273633bccddfc62c708ec4517bc36dbb5b))
- Remove ustr - ([8f38ec5](https://github.com/adclz/auto-lsp/commit/8f38ec5c6f33c034a388c7a34bcafd4ddcc38ba0))


## [0.5.1](https://github.com/adclz/auto-lsp/compare/auto-lsp-v0.5.0...auto-lsp-v0.5.1)

### Bug Fixes

- *(server)* Prevent duplicate completion of request IDs in main loop - ([9ae8d71](https://github.com/adclz/auto-lsp/commit/9ae8d71492e6ff79e62f6d00abb99237a44fcf22))

### Documentation

- Fix AST traits link - ([6e83bb2](https://github.com/adclz/auto-lsp/commit/6e83bb2d495432e115f098ba4515e2f38260f2ca))


## [0.5.0](https://github.com/adclz/auto-lsp/compare/auto-lsp-v0.4.0...auto-lsp-v0.5.0)

### Features

- *(ci)* Add workflow for running LSP server tests in native environment - ([f209901](https://github.com/adclz/auto-lsp/commit/f20990174f969249ba1be13ebdacfbc36aa05c2f))
- *(examples)* Add stdio LSP server and tests - ([27a569a](https://github.com/adclz/auto-lsp/commit/27a569a59567284dd389a5ac7f047e4f84689f80))
- *(server)* Enhance open_text_document method to add file to workspace if not present - ([6d02a1f](https://github.com/adclz/auto-lsp/commit/6d02a1faf1e34e091f5c33bcd93a34a81b199a23))
- *(server)* Send error notifications for failed message handling - ([376dbcd](https://github.com/adclz/auto-lsp/commit/376dbcd32ee2e47c29866d08533f62c420da8549))
- *(server)* Add get_workspace method to access workspace mutex - ([5886cf8](https://github.com/adclz/auto-lsp/commit/5886cf86fcda21558b27758efb0f1378315f5e9f))
- *(server)* Custom LSP notifications registration - ([ae769e3](https://github.com/adclz/auto-lsp/commit/ae769e35d9e0a905614170072d11d294b8f4f0e7))
- *(server)* Custom LSP requests registration - ([5b03384](https://github.com/adclz/auto-lsp/commit/5b03384edfdc0370f8c3cb7b18330157b650bb26))
- *(session)* Implement request queue for handling incoming LSP requests - ([82626c9](https://github.com/adclz/auto-lsp/commit/82626c9a5c76200a0a12d2fbcb41b531f84e8400))
- Add optional rayon support for workspace init - ([7c79786](https://github.com/adclz/auto-lsp/commit/7c79786274400404ca125950d2f89cb12f1e13dd))

### Bug Fixes

- *(core)* Expose workspace module in core - ([914d6ac](https://github.com/adclz/auto-lsp/commit/914d6accea0293441d0944950ab7ca5e1fee5b90))
- *(doc)* Server module - ([baa96ad](https://github.com/adclz/auto-lsp/commit/baa96ad7c998c1f6766943402217fdaeb844b5aa))
- *(session)* Temporarily disable certain LSP notifications - ([5b17179](https://github.com/adclz/auto-lsp/commit/5b17179de334657e3d0158ebc2466b1a2e52b705))
- Invalid re-exports of texter and lsp_server - ([9636dee](https://github.com/adclz/auto-lsp/commit/9636deedac47132c20a4b58a546026316da403a5))
- Add GCC multilib installation step in WASI CI - ([5a53315](https://github.com/adclz/auto-lsp/commit/5a53315bfe7615f4394fb261e29644fd56ba42b3))

### Refactor

- *(ci)* Rename CI jobs - ([6725518](https://github.com/adclz/auto-lsp/commit/67255185f730b5a61436b5d5276e06555e586efa))
- *(code_actions)* Update build_code_actions signature to use CodeActionOrCommand - ([d74ba87](https://github.com/adclz/auto-lsp/commit/d74ba87280e9bf4cfcf66d64be283cc3630a7e1f))
- *(examples)* Rename extensions to examples - ([4f4048d](https://github.com/adclz/auto-lsp/commit/4f4048d000f5654c1ba05162a533f41c362100e3))
- *(server)* Rename file_to_root method to read_file - ([d6da3f8](https://github.com/adclz/auto-lsp/commit/d6da3f849f78732911d7ecb41baad88b56366dc9))
- *(tests)* Replace Document and Root with Workspace in test fixtures - ([235c9fe](https://github.com/adclz/auto-lsp/commit/235c9fe42f0dfaf814a13b520309beb07f94271d))
- Split out tree sitter and ast diagnostics - ([e19eb4d](https://github.com/adclz/auto-lsp/commit/e19eb4de7ddae36485a6d4306c888bef18588c0c))
- Allow defining lsp_server Connection before Session init - ([97da1c7](https://github.com/adclz/auto-lsp/commit/97da1c7568255cf41c01ac6588c8dd0579af3b16))

### Documentation

- Correct diagnostics terminology in workspace documentation - ([20c1c75](https://github.com/adclz/auto-lsp/commit/20c1c759c7e7cf3f3a9dd51de639acb086ea99cb))
- Update book - ([7710acc](https://github.com/adclz/auto-lsp/commit/7710acc025ebe32aa5229806952371f031f36aba))
- Update crates doc - ([27ba4c2](https://github.com/adclz/auto-lsp/commit/27ba4c28be55a58fd7759551ef9a82459af109dc))
- Update README files - ([e38b623](https://github.com/adclz/auto-lsp/commit/e38b623dd8c5265a591e57df6a876846a8ff6948))

### Testing

- WASI config and CI workflow - ([0589308](https://github.com/adclz/auto-lsp/commit/058930885c682cbdb73efb781e43eaabe7a2d4b2))
- Add utility functions for HTML and Python workspace creation - ([9fbd614](https://github.com/adclz/auto-lsp/commit/9fbd614120e427e1c6731cb8f7f63b359da3b5fd))

### Miscellaneous Tasks

- *(tests)* Dead code warnings in HTML and Python utilities - ([2c9f57e](https://github.com/adclz/auto-lsp/commit/2c9f57e502e3a2e97fa1effb4fadc0350cfb94a7))
- Update dependencies - ([4a1b3a4](https://github.com/adclz/auto-lsp/commit/4a1b3a4011dbc119b4fa5c453722af391caf2c83))
- Remove duplicated 'Unreleased' section from changelogs - ([cc416ef](https://github.com/adclz/auto-lsp/commit/cc416efc6cc0737360c993d2b0d86b8a77c416ca))
- Remove unused test file main copy.py - ([5703fdc](https://github.com/adclz/auto-lsp/commit/5703fdc3d69cbd427c1bcb72d7a1a06e1672b344))


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
