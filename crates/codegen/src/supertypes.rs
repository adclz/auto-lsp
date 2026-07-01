use crate::json::TypeInfo;
use crate::NodeType;

#[derive(Clone, Default)]
pub(crate) struct SuperType {
    pub(crate) variants: Vec<TypeInfo>,
    pub(crate) types: Vec<String>,
}

pub(crate) fn generate_super_type(node: &NodeType) -> SuperType {
    // Get enum variants
    let variants = node
        .subtypes
        .as_ref()
        .map(|subtypes| subtypes.to_vec())
        .unwrap_or_default();

    // Get enum types
    let types = node
        .subtypes
        .as_ref()
        .map(|subtypes| {
            subtypes
                .iter()
                .map(|subtype| subtype.kind.clone())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    SuperType { variants, types }
}
