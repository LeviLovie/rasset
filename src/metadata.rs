/// Struct representing metadata for an asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub name: String,
    pub type_name: String,
    pub hash: String,
}
