use rasset::{asset, asset_def, AssetConfig, AssetData, AssetError, AssetType};

asset_def! {
    name: Sprite,
    meta: {
        size: (u32, u32),
        origin: (i32, i32),
        frame_count: u32,
    }
}

asset! {
    name: PlayerSprite,
    base: Sprite,
    meta: {
        size: (32, 48),
        origin: (16, 24),
        frame_count: 8,
    }
}

asset! {
    name: EnemySprite,
    base: Sprite,
    meta: {
        size: (32, 32),
        origin: (16, 16),
        frame_count: 4,
    }
}

fn main() {
    println!("Player sprite size: {:?}", PlayerSprite.meta.size);
    println!("Player sprite origin: {:?}", PlayerSprite.meta.origin);
    println!("Enemy sprite size: {:?}", EnemySprite.meta.size);
    println!("Enemy sprite origin: {:?}", EnemySprite.meta.origin);
}
