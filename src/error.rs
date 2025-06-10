#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Failed to compile asset: {0}")]
    Compilation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Asset type not found: {0}")]
    TypeNotFound(String),
}
