# Workspace

A `Workspace` is the top-level structure of the AST. 

It maintains a HashMap where:
 - The keys are [`Url`](https://docs.rs/lsp-types/0.95.1/lsp_types/struct.Url.html) instances
 - The values are tuples containing two other core components:
    - Root (handles AST and diagnostics).
    - Document (handles CST and text storage).

```mermaid
graph TD
    A["Workspace"]
    A1["HashMap(Url, (Root, Document))"]
    B["Root"]
    B1["Ast"]
    B2["Diagnostics"]
    B3["Ast parser"]
    C["Document"]
    C1["Cst (tree-sitter)"]
    C2["texter"]

    A ==".roots"==> A1
    A1 =="value.0"==> B
    A1 =="value.1"==> C

    B -.".ast".-> B1
    B -.".ast_diagnostics
    .tree_diagnostics".-> B2
    B -.".parsers".-> B3

    C -.".tree".-> C1
    C -.".texter".-> C2


style A stroke:red
style A1 stroke:red

style B stroke:blue
style B1 stroke:blue
style B2 stroke:blue
style B3 stroke:blue

style C stroke:green
style C1 stroke:green
style C2 stroke:green
```



In the next sections, we will see how `Root` and `Document` work