use rasset::prelude::*;

asset_def! {
    Sprite: {
        size: (u64, u64),
        texture: String,
    }
}

asset_file!("examples/assets.yaml");

fn main() -> Result<(), Error> {
    let compiled_assets = compile_assets()?;

    let registry = Registry::builder()
        .reg_type::<Sprite>()
        .load(&compiled_assets)?;

    println!("Player: {:?}", registry.get_asset::<Sprite>("Player"));
    println!("Enemy: {:?}", registry.get_asset::<Sprite>("Enemy"));

    Ok(())
}
