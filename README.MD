<div align="center">
  <h1>Auto LSP</h1>
  <p>
    <strong>AST generator based on tree sitter queries</strong>
  </p>
  <p>

A rust crate to facilitate the creation of [Language Servers](https://microsoft.github.io/language-server-protocol/) for [tree sitter](https://tree-sitter.github.io/tree-sitter/) grammars using [tree-sitter queries](https://tree-sitter.github.io/tree-sitter/using-parsers/queries/1-syntax.html).
</div>

## Features

- Desgined to be language agnostic
- Easy to configure 
- Thread safe
- Compile and Runtime checks
- Partial incremental updates [WIP]
- Support of multiple parsers (per file extension)
- Support multiline edits
- All symbols implement downcasting
- Works on @vscode/wasi-lsp

## Documentation

- [Install](#install)
- [Configure](#configure)
    - [Declare semantic tokens]()
    - [Configure Parsers]() 
- [Create the AST]()
    - [Declare symbols]()
        - [seq]()
        - [repeat]()
        - [option]()
        - [choice]()
    - [Add capabilities]()
        - [LSP]()
            - [Document Symbols]()
        - [Special capabilities]()
            - [Check]()
            - [Scope]()
            - [Reference]()
- [Debug]()
    - [Compile time checking]()
    - [Runtime messages]()
- [Build]()
    - [WebAssembly]()              
- [Similar projects](#similar-projects)    
