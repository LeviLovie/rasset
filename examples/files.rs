use rasset::prelude::*;

asset_def! {
    Sprite: {
        size: (u32, u32),
        texture: Vec<u8>,
    }
}

assets!(
    PlayerSprite: Sprite {
        size: (64, 64),
        texture: include_bytes!("../LICENSE").to_vec(),
    },
    EnemySprite: Sprite {
        size: (32, 32),
        texture: include_bytes!("../README.md").to_vec(),
    }
);

fn main() -> Result<(), Error> {
    let compiled_assets = compile_assets()?;

    let registry = Registry::builder()
        .reg_type::<Sprite>()
        .load(&compiled_assets)?;

    println!("Loaded registry with {} assets", registry.amount());
    println!("Player: {:?}", registry.get_asset::<Sprite>("PlayerSprite"));
    println!("Enemy: {:?}", registry.get_asset::<Sprite>("EnemySprite"));

    Ok(())
}
