mod core_ast;
mod core_build;

/// This module contains everything related to ast symbols already created
pub mod ast {
    pub use crate::core_ast::capabilities::*;
    pub use crate::core_ast::core::*;
    pub use crate::core_ast::data::*;
    pub use crate::core_ast::symbol::*;
    pub use crate::core_ast::update::*;
}

/// This module contains everything related to building ast symbols
pub mod build {
    pub use crate::core_build::buildable::*;
    pub use crate::core_build::downcast::*;
    pub use crate::core_build::main_builder::*;
    pub use crate::core_build::symbol::*;
}

/// Semantic tokens builder
pub mod semantic_tokens;

/// Workspace and document handling
pub mod workspace;
