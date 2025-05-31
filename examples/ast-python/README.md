# Python AST Example

This is an example of Python Ast generated from [`tree_sitter_python`](https://github.com/tree-sitter/tree-sitter-python) crate.

## Project Structure

- `src/db.rs`: Defines the Salsa database used to store and compute parsed documents and ASTs.

- `src/generated.rs`: Contains the generated AST based on tree_sitter_python.

- `src/tests/corpus/*`: Snapshot tests based on the Python corpus. Used to validate AST generation via debug output.

- `src/capabilities/*`: Demonstrates simple LSP request handlers that interact with the Python AST via downcasting. 

- `src/tests/db/salsa.rs`: Tests that verify Salsa correctly invalidates inputs when changes occur. 

- `src/tests/db/type_errors.rs`: Tests to validate the behavior of **salsa::accumulator**. 

- `src/tests/db/capabilities.rs`: Tests that ensure LSP capabilities work correctly across various scenarios. 

## Notes

- `generated.rs` is only generated if **AST_GEN** env variable is set to a value higher than 0. 

- This is a minimal example -not a full Python LSP server— but it serves as a good testbed. 