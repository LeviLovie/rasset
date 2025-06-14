use rasset::prelude::*;

asset_def! {
    Sprite: {
        position: (f32, f32),
        size: (u32, u32),
        texture: String,
    }
}

asset_def! {
    Sprites: {
        sprites: Vec<Sprite>,
    }
}

assets!(
    BackgroundSprites: Sprites {
        sprites: vec![
            Sprite {
                name: "MainBackground".to_string(),
                position: (0.0, 0.0),
                size: (800, 600),
                texture: "path/to/background_texture.png".to_string(),
            },
            Sprite {
                name: "SecondaryBackground".to_string(),
                position: (100.0, 150.0),
                size: (200, 150),
                texture: "path/to/another_background_texture.png".to_string(),
            },
        ],
    }
);

fn main() -> Result<(), Error> {
    let compiled_assets = compile_assets()?;

    let registry = Registry::builder()
        .reg_type::<Sprites>()
        .load(&compiled_assets)?;

    println!("Loaded registry with {} assets", registry.amount());

    let sprites = registry
        .get_asset::<Sprites>("BackgroundSprites")
        .map(|s| &s.sprites)
        .expect("Failed to get BackgroundSprites asset");
    println!("Background sprites: {:?}", sprites.len());
    sprites.iter().for_each(|sprite| {
        println!("Sprite: {:?}", sprite);
    });

    Ok(())
}
