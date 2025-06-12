use crate::{asset::Asset, error::Error, prelude::bincode};

pub struct Compiler {
    pub assets: Vec<Box<dyn Asset>>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { assets: Vec::new() }
    }

    pub fn add_asset(&mut self, asset: Box<dyn Asset>) {
        self.assets.push(asset);
    }

    pub fn compile(&self) -> Result<Vec<u8>, Error> {
        let mut assets: Vec<(String, Vec<u8>)> = Vec::new();
        for asset in &self.assets {
            let type_name = asset.type_name().to_string();
            let bytes = asset.to_bytes()?;
            assets.push((type_name, bytes));
        }

        bincode::encode_to_vec(&assets, bincode::config::standard())
            .map_err(|e| Error::Serialization(format!("Failed to serialize assets: {}", e)))
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
