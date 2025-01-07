use super::{common::name::*, pous::function::*};
use auto_lsp_macros::choice;

#[choice]
pub enum SourceFile {
    Function(Function),
}
