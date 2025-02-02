#[cfg(any(feature = "html", test))]
pub mod html_workspace;
#[cfg(any(feature = "python", test))]
pub mod python_workspace;

#[cfg(test)]
pub mod ast;
#[cfg(test)]
pub mod code_lens;
#[cfg(test)]
pub mod comments;
#[cfg(test)]
pub mod completion_items;
#[cfg(test)]
#[cfg(feature = "deadlock_detection")]
pub mod deadlock;
#[cfg(test)]
pub mod document_links;
#[cfg(test)]
pub mod document_symbols;
#[cfg(test)]
pub mod hover;
#[cfg(test)]
pub mod incremental;
#[cfg(test)]
pub mod inlay_hints;
#[cfg(test)]
pub mod proc_macros;
#[cfg(test)]
pub mod semantic_tokens;
#[cfg(test)]
pub mod traverse;
#[cfg(test)]
pub mod type_errors;
