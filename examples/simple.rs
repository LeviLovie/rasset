use rasset::{asset, asset_def, asset_file, AssetConfig, AssetData, AssetError, AssetType};

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

asset! {
    name: SimpleFont,
    base: Font,
    data: {
        name: "SimpleFont".into(),
    }
}

asset_file! {
    name: Sprites,
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
    }
}

fn main() {
    for sprite in Sprites.iter() {
        println!("Sprite name: {}", sprite.name);
        println!("Size: {:?}", sprite.meta.size);
    }

    println!("Font: {:?}", SimpleFont.name);
}
