/// Calls a method on the first node that matches any of the provided types.
/// Returns the result of the method call.
///
/// # Example
///
/// ```rust, ignore
/// use ast::generated::{FunctionDefinition, ClassDefinition};
/// use auto_lsp::dispatch_once;
///
/// /* ... */
/// let result = dispatch_once!(node.lower(), [
///     FunctionDefinition => return_something(db, param),
///     ClassDefinition => return_something(db, param)
/// ]);
/// ```
#[macro_export]
macro_rules! dispatch_once {
    ($node:expr, [$($ty:ty => $method:ident($($param:expr),*)),*]) => {
        $(
            if let Some(n) = $node.downcast_ref::<$ty>() {
                return n.$method($($param),*);
            };
        )*
    };
}

/// Calls a method on any node that matches any of the provided types.
/// Unlike dispatch_once, it will not return.
///
/// # Example
///
/// ```rust, ignore
/// use ast::generated::{FunctionDefinition, ClassDefinition};
/// use auto_lsp::dispatch;
///
/// /* ... */
/// dispatch!(node.lower(), [
///     FunctionDefinition => get_something(db, param),
///     ClassDefinition => get_something(db, param)
/// ]);
/// ```
#[macro_export]
macro_rules! dispatch {
    ($node:expr, [$($ty:ty => $method:ident($($param:expr),*)),*]) => {
        $(
            if let Some(n) = $node.downcast_ref::<$ty>() {
                n.$method($($param),*)?;
            };
        )*
    };
}
