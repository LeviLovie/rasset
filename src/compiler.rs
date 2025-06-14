use crate::{asset::Asset, error::Error, prelude::bincode};

/// Compiler is responsible for compiling a collection of assets into a binary format.
pub struct Compiler {
    pub assets: Vec<Box<dyn Asset>>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    /// Creates a new instance of the Compiler.
    pub fn new() -> Self {
        Compiler { assets: Vec::new() }
    }

    /// Adds an asset to the compiler's collection.
    pub fn add_asset(&mut self, asset: Box<dyn Asset>) {
        self.assets.push(asset);
    }

    /// Compiles all added assets into a binary format.
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
