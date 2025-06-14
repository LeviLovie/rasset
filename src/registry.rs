use crate::{asset::Asset, error::Error};
use std::collections::HashMap;

/// RegistryBuilder is used to build a registry of assets with their respective types.
pub struct RegistryBuilder {
    registry: Registry,
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryBuilder {
    /// Creates a new instance of RegistryBuilder.
    pub fn new() -> Self {
        RegistryBuilder {
            registry: Registry::new(),
        }
    }

    /// Registers an asset type in the registry.
    pub fn reg_type<T>(mut self) -> Self
    where
        T: Asset + 'static,
    {
        self.registry.reg_type::<T>();
        self
    }

    /// Loads assets from a binary slice into the registry.
    pub fn load(mut self, binary: &[u8]) -> Result<Registry, Error> {
        let (assets_with_types, _bytes_read): (Vec<(String, Vec<u8>)>, usize) =
            bincode::decode_from_slice(binary, bincode::config::standard()).map_err(|e| {
                Error::Deserialization(format!("Failed to deserialize assets: {}", e))
            })?;

        for (type_name, bytes) in assets_with_types {
            if let Some(deserializer) = self.registry.deserializers.get(&type_name) {
                let asset = deserializer(&bytes)?;
                self.registry.assets.push(asset);
            } else {
                return Err(Error::Deserialization(format!(
                    "No deserializer registered for asset type '{}'",
                    type_name
                )));
            }
        }

        Ok(self.registry)
    }
}

pub type Deserializer = Box<dyn Fn(&[u8]) -> Result<Box<dyn Asset>, Error> + Send + Sync>;

pub struct Registry {
    deserializers: HashMap<String, Deserializer>,
    assets: Vec<Box<dyn Asset>>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            deserializers: HashMap::new(),
            assets: Vec::new(),
        }
    }

    /// Registers a type in the registry with its deserializer.
    fn reg_type<T>(&mut self)
    where
        T: Asset + 'static,
    {
        let type_name = std::any::type_name::<T>().to_string();
        self.deserializers.insert(
            type_name,
            Box::new(|bytes| T::from_bytes(bytes).map(|asset| Box::new(asset) as Box<dyn Asset>)),
        );
    }

    /// Creates a new RegistryBuilder to build a registry.
    pub fn builder() -> RegistryBuilder {
        RegistryBuilder::new()
    }

    /// Returns a reference to the assets in the registry.
    pub fn amount(&self) -> usize {
        self.assets.len()
    }

    /// Returns a reference to the assets in the registry.
    pub fn get_asset<T: Asset + 'static>(&self, name: &str) -> Option<&T> {
        let type_id = std::any::TypeId::of::<T>();
        self.assets.iter().find_map(|asset| {
            if asset.get_type().0 == type_id && asset.name() == name {
                asset.as_any().downcast_ref::<T>()
            } else {
                None
            }
        })
    }

    /// Returns a vector of all assets of a specific type in the registry.
    pub fn get_assets<T: Asset + 'static>(&self) -> Vec<&T> {
        self.assets
            .iter()
            .filter_map(|asset| {
                if asset.get_type().0 == std::any::TypeId::of::<T>() {
                    asset.as_any().downcast_ref::<T>()
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns a reference to all assets in the registry.
    pub fn get_all_assets(&self) -> &Vec<Box<dyn Asset>> {
        &self.assets
    }
}
