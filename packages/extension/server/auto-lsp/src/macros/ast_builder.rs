use crate::types::binder_closures::{BinderFn, ItemBinderFn};

pub struct AstBuilder {
    pub query_to_builder: BinderFn,
    pub builder_to_item: ItemBinderFn,
}

#[macro_export]
macro_rules! create_builder {
    ($builder: ident) => {{
        AstBuilder {
            query_to_builder: $builder::query_binder,
            builder_to_item: $builder::builder_binder,
        }
    }};
}
