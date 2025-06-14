use rasset::prelude::*;

asset_def! {
    Sprite: {
        size: (u64, u64),
        texture: String,
    }
}

asset_def! {
    Sprites: {
        sprites: Vec<String>,
    }
}

asset_file!("examples/assets.yaml");

fn main() -> Result<(), Error> {
    let compiled_assets = compile_assets()?;

    let registry = Registry::builder()
        .reg_type::<Sprite>()
        .reg_type::<Sprites>()
        .load(&compiled_assets)?;

    let sprites = registry.get_asset::<Sprites>("Sprites").unwrap();
    for sprite_name in &sprites.sprites {
        println!(
            "Sprite: {:#?}",
            registry.get_asset::<Sprite>(sprite_name).unwrap()
        );
    }

    Ok(())
}
