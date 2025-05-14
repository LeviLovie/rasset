use rasset::{asset_def, asset_file, AssetConfig, AssetData, AssetError, AssetType};

asset_def! {
    name: Sprite,
    meta: {
        size: (u32, u32),
        origin: (i32, i32),
        frame_count: u32,
    }
}

asset_def! {
    name: Font,
    meta: {
        name: String,
    }
}

asset_file! {
    name: sprites,
    assets: {
        PlayerSprite: Sprite {
            size: (32, 48),
            origin: (16, 24),
            frame_count: 8,
        },
        EnemySprite: Sprite {
            size: (32, 32),
            origin: (16, 16),
            frame_count: 4,
        },
        PlayerFont: Font {
            name: "PlayerFont".into(),
        },
    }
}

fn debug_sprite(sprite: &Sprite) {
    println!("Sprite size: {:?}", sprite.meta.size);
    println!("Sprite origin: {:?}", sprite.meta.origin);
    println!("Sprite frame count: {:?}", sprite.meta.frame_count);
}

fn main() {
    debug_sprite(&sprites::PlayerSprite);
    debug_sprite(&sprites::EnemySprite);
    println!("PlayerFont: {:?}", sprites::PlayerFont.meta.name);
}
