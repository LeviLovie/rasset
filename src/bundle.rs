use crate::{asset::Asset, metadata::Metadata};

/// Represents a collection of assets along with its metadata.
pub struct Bundle {
    pub metadata: Metadata,
    pub assets: Vec<Box<dyn Asset>>,
}
