use std::any::TypeId;

/// Represents a unique type identifier for an asset.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type(pub TypeId);
