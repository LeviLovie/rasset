use crate::{asset::Asset, metadata::Metadata};

pub struct Bundle {
    pub metadata: Metadata,
    pub assets: Vec<Box<dyn Asset>>,
}
