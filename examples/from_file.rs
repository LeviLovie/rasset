use rasset::prelude::*;

asset_def! {
    Sprite: {
        width: i64,
        height: i64,
        texture: String,
    },
    Sprites: {
        sprites: Vec<String>,
    }
}

asset_file!("examples/assets.ron");

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
