# Rust Asset System

Rasset is a library that provides proc macros to serialize Rust structs into a binary file at buildtime and deserialize at runtime.

## Quick start

Please take a look at [exaples](./examples/) and [template](./template).

```rust
asset_def! {
    Sprite: {
        size: (u32, u32),
        texture: String,
    }
}

assets!(
    PlayerSprite: Sprite {
        size: (64, 64),
        texture: "path/to/player_texture.png".to_string(),
    },
    EnemySprite: Sprite {
        size: (32, 32),
        texture: "path/to/enemy_texture.png".to_string(),
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
```
