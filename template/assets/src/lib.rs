use rasset::prelude::*;

asset_def! {
    Sprite: {
        size: (u32, u32),
        texture: Vec<u8>,
    }
}

pub fn register(binary: Vec<u8>) -> Result<Registry, Error> {
    Registry::builder().reg_type::<Sprite>().load(&binary)
}

#[cfg(feature = "declare")]
pub mod declare {
    use super::*;

    assets!(
        PlayerSprite: Sprite {
            size: (64, 64),
            texture: include_bytes!("../Cargo.toml").to_vec(),
        },
        EnemySprite: Sprite {
            size: (32, 32),
            texture: include_bytes!("../../build.rs").to_vec(),
        }
    );
}
