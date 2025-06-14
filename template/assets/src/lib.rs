use rasset::prelude::*;

asset_def! {
    Sprite: {
        size: (i64, i64),
        texture: Vec<u8>,
    },
    Sprites: {
        sprites: Vec<String>,
    }
}

pub fn register(binary: Vec<u8>) -> Result<Registry, Error> {
    Registry::builder()
        .reg_type::<Sprite>()
        .reg_type::<Sprites>()
        .load(&binary)
}

#[cfg(feature = "declare")]
pub mod declare {
    use super::*;

    asset_file!("../assets.yaml");
}
