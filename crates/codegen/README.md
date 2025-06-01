# Auto LSP Codegen

To generate an AST, simply provide a Tree-sitter [node-types.json](https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html#static-node-types) and [LanguageFn](https://docs.rs/tree-sitter/latest/tree_sitter/struct.Language.html) of any language to the `generate` function of the `auto_lsp_codegen` crate.

```sh
cargo add auto_lsp_codegen
```
> [!NOTE]
> Although `auto_lsp_codegen` is a standalone crate, the generated code depends on the main `auto_lsp` crate.

## Usage

The `auto_lsp_codegen` crate exposes a single `generate` function, which takes:
 - A [`node-types.json`](https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html),
 - A [`LanguageFn`](https://docs.rs/tree-sitter-language/0.1.5/tree_sitter_language/struct.LanguageFn.html)
 - A `HashMap<&str, &str>` to rename tokens (see [Custom Tokens](#custom-tokens))
 - And returns a **TokenStream**.

How you choose to use the `TokenStream` is up to you.

The most common setup is to call it from a **build.rs** script and write the generated code to a Rust file.

Note, however, that the output can be quite large—for example, Python’s AST results in ~11,000 lines of code.

```rust, ignore
use auto_lsp_codegen::generate;
use std::{fs, path::PathBuf};

fn main() {
    if std::env::var("AST_GEN").unwrap_or("0".to_string()) == "0" {
        return;
    }

    let output_path = PathBuf::from("./src/generated.rs");

    fs::write(
        output_path,
        generate(
            tree_sitter_python::NODE_TYPES,
            &tree_sitter_python::LANGUAGE.into(),
            None,
        )
        .to_string(),
    )
    .unwrap();
}
```

You can also invoke it from your own CLI or tool if needed.

## How Codegen Works

The generated code structure depends on the Tree-sitter grammar.

### Structs for Rules

Each rule in `node-types.json` becomes a dedicated Rust struct. For example, given the rule:

```js
function_definition: $ => seq(
      optional('async'),
      'def',
      field('name', $.identifier),
      field('type_parameters', optional($.type_parameter)),
      field('parameters', $.parameters),
      optional(
        seq(
          '->',
          field('return_type', $.type),
        ),
      ),
      ':',
      field('body', $._suite),
    ),
```

The generated struct would look like this:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub name: std::sync::Arc<Identifier>,
    pub body: std::sync::Arc<Block>,
    pub type_parameters: Option<std::sync::Arc<TypeParameter>>,
    pub parameters: std::sync::Arc<Parameters>,
    pub return_type: Option<std::sync::Arc<Type>>,
    /* ... */
}
```

### Field Matching

To match fields, codegen uses the `field_id()` method from the Tree-sitter cursor.

From the above example, the generated builder might look like this:

```rust, ignore
builder.builder(db, &node, Some(id), |b| {
  b.on_field_id::<Identifier, 19u16>(&mut name)?
    .on_field_id::<Block, 6u16>(&mut body)?
    .on_field_id::<TypeParameter, 31u16>(&mut type_parameters)?
    .on_field_id::<Parameters, 23u16>(&mut parameters)?
    .on_field_id::<Type, 24u16>(&mut return_type)
});
```

Each **u16** represents the unique field ID assigned by the Tree-sitter language parser.

### Handling Children

If a node has no named fields, a children enum is generated to represent all possible variants.

- If the children are **unnamed**, a generic "Operator_" enum is generated
- If the children are **named**, the enum will be a concatenation of all possible child node types with underscores, using sanitized Rust-friendly names.

For example, given the rule:

```js
  _statement: $ => choice(
      $._simple_statement,
      $._compound_statement,
    ),
```

The generated enum would look like this:

```rust
pub enum SimpleStatement_CompoundStatement {
    SimpleStatement(SimpleStatement),
    CompoundStatement(CompoundStatement),
}
```

> [!NOTE]
>If the generated enum name becomes too long, consider using a Tree-sitter <a href="https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html#supertype-nodes">supertype</a> to group nodes together.

The `kind_id()` method is used to determine child kinds during traversal.

The `AstNode::contains` method relies on this to check whether a node kind belongs to a specific struct or enum variant.

### Vec and Option Fields

`repeat` and `repeat1` in the grammar will generate a `Vec` field.
`optional(...)` will generate an `Option<T>` field.

### Token Naming

Unnamed tokens are mapped to Rust enums using a built-in token map. For instance:

```json
  { "type": "+", "named": false },
  { "type": "+=", "named": false },
  { "type": ",", "named": false },
  { "type": "-", "named": false },
  { "type": "-=", "named": false },
```

Generates:

```rust
pub enum Token_Plus {}
pub enum Token_PlusEqual {}
pub enum Token_Comma {}
pub enum Token_Minus {}
pub enum Token_MinusEqual {}
```

Tokens with regular identifiers are converted to PascalCase.

### Custom Tokens

If your grammar defines additional unnamed tokens not covered by the default map, you can provide a custom token mapping to generate appropriate Rust enum names.

```rust, ignore
use auto_lsp_codegen::generate;

let _result = generate(
        &tree_sitter_python::NODE_TYPES,
        &tree_sitter_python::LANGUAGE.into(),
        Some(HashMap::from([
            ("+", "Plus"),
            ("+=", "PlusEqual"),
            (",", "Comma"),
            ("-", "Minus"),
            ("-=", "MinusEqual"),
        ])),
    );
```

Tokens that are not in the map will be added, and tokens that already exist in the map will be overwritten.

### Super Types

Tree-sitter supports [supertypes](https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html#supertype-nodes), which allow grouping related nodes under a common type.

For example, in the Python grammar:

```json
  {
    "type": "_compound_statement",
    "named": true,
    "subtypes": [
      {
        "type": "class_definition",
        "named": true
      },
      {
        "type": "decorated_definition",
        "named": true
      },
      /* ... */
      {
        "type": "with_statement",
        "named": true
      }
    ]
  },
```

This becomes a Rust enum:

```rust
pub enum CompoundStatement {
    ClassDefinition(ClassDefinition),
    DecoratedDefinition(DecoratedDefinition),
    /* ... */
    WithStatement(WithStatement),
}
```

> [!NOTE]
> Some super types might contain other super types, in which case, the generated enum will flatten the hierarchy.
