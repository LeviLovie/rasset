use rasset::prelude::*;

asset_def! {
    Sprite: {
        size: (u32, u32),
        texture: String,
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
            texture: "path/to/player.png".to_string(),
        },
        EnemySprite: Sprite {
            size: (32, 32),
            texture: "path/to/enemy.png".to_string(),
        }
    );
}
