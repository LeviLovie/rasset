use std::any::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type(pub TypeId);
