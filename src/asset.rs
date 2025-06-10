use crate::{error::Error, r#type::Type};
use std::any::Any;

pub trait Asset: Send + Sync + Any + 'static {
    /// Returns the unique type identifier for the asset.
    fn get_type(&self) -> Type;

    /// Returns the type name of the asset as a string.
    fn type_name(&self) -> &'static str;

    /// Returns a reference to the asset as a trait object for dynamic type checking.
    fn as_any(&self) -> &dyn Any;

    /// Serializes the asset to a byte array.
    fn to_bytes(&self) -> Result<Vec<u8>, Error>;

    /// Deserializes the asset from a byte array.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}
