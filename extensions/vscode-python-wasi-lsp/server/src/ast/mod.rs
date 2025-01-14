use auto_lsp::auto_lsp_core::pending_symbol::{AddSymbol, AstBuilder, Finalize, TryDownCast};
use auto_lsp::auto_lsp_core::symbol::{AstSymbol, StaticSwap, Symbol};
use auto_lsp::auto_lsp_macros::seq;

#[seq(query_name = "module", kind(symbol()))]
pub struct Module {
    functions: Vec<Function>,
}

#[seq(query_name = "function", kind(symbol()))]
pub struct Function {
    name: FunctionName,
}

#[seq(query_name = "function.name", kind(symbol()))]
pub struct FunctionName {}
