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