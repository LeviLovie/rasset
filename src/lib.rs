use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

pub use proc_macros::{asset, asset_def, asset_file};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetType(pub TypeId);

#[derive(Debug, Clone, Default)]
pub struct AssetConfig {
    _hot_reload: bool,

    _processing: HashMap<String, ConfigValue>,
}

#[derive(Debug, Clone)]
pub enum ConfigValue {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Vec(Vec<ConfigValue>),
    Map(HashMap<String, ConfigValue>),
}

#[derive(Debug, Error)]
pub enum AssetError {
    #[error("Failed to load asset data: {0}")]
    LoadError(String),

    #[error("Asset not found: {0}")]
    NotFound(String),

    #[error("Invalid asset format: {0}")]
    InvalidFormat(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub trait AssetData: Send + Sync + 'static {
    fn asset_type() -> AssetType
    where
        Self: Sized;

    fn from_bytes(bytes: &[u8], config: &AssetConfig) -> Result<Self, AssetError>
    where
        Self: Sized;
}

pub struct AssetManager {
    assets: HashMap<Uuid, Box<dyn std::any::Any + Send + Sync>>,

    path_to_id: HashMap<String, Uuid>,

    default_config: AssetConfig,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            path_to_id: HashMap::new(),
            default_config: AssetConfig::default(),
        }
    }

    pub fn load<T: AssetData>(&mut self, path: impl AsRef<Path>) -> Result<Uuid, AssetError> {
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();

        if let Some(id) = self.path_to_id.get(&path_str) {
            return Ok(*id);
        }

        let bytes = std::fs::read(path)?;
        let asset = T::from_bytes(&bytes, &self.default_config)?;

        let id = Uuid::new_v4();
        self.assets.insert(id, Box::new(asset));
        self.path_to_id.insert(path_str, id);

        Ok(id)
    }

    pub fn get<T: AssetData + 'static>(&self, id: Uuid) -> Option<&T> {
        self.assets
            .get(&id)
            .and_then(|asset| asset.downcast_ref::<T>())
    }

    pub fn get_by_path<T: AssetData + 'static>(&self, path: impl AsRef<Path>) -> Option<&T> {
        let path = path.as_ref().to_string_lossy().to_string();
        self.path_to_id.get(&path).and_then(|id| self.get(*id))
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}
