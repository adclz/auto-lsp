#[macro_export]
macro_rules! dispatch {
    ($node:expr, [$($ty:ty => $method:ident($($param:ident),*)),*]) => {
        $(
            if let Some(n) = $node.downcast_ref::<$ty>() {
                return n.$method($($param),*);
            };
        )+
    };
}
