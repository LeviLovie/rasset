[![Crates.io](https://img.shields.io/crates/v/rasset.svg)](https://crates.io/crates/rasset)
[![Docs.rs](https://docs.rs/rasset/badge.svg)](https://docs.rs/rasset)
[![License](https://img.shields.io/crates/l/rasset.svg)](LICENSE)
[![CI](https://github.com/levilovie/rasset/actions/workflows/ci.yml/badge.svg)](https://github.com/levilovie/rasset/actions/workflows/ci.yml/)

# Rust Asset System

Rasset is a library that provides proc macros to serialize Rust structs into a binary file at build time and deserialize at runtime.

## Quick start

Please take a look at [examples](./examples/) and the [template](./template).

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

## Documentation

### Asset definiton

Proc macro `asset_def` creates a struct for the asset type.

### Asset declaration

Proc macro `assets` takes instances of a struct defined in `asset_def` and creates a `compile_assets` func.

Proc macro `asset_file` takes a [YAML](https://en.wikipedia.org/wiki/YAML) file and generates assets from there, similar to `assets`. Example:

```yaml
- name: Player
  type: Sprite
  metadata:
    size: [64, 64]
    texture: "/path/to/player/texture/"

- name: Enemy
  type: Sprite
  metadata:
    size: [32, 32]
    texture: "/path/to/enemy/texture/"
```

YAML supports there tags:

- `!Rust`: Instead of storing the string, parser with interpret the data as a Rust expression. `texture: !Rust include_bytes!("texture.png").to_vec()`
- `!IncludeBytes`: Generates `include_bytes!(STRING)`.
- `!IncludeStr`: Generates `include_str!(STRING)`.
- `!IncludeVec`: Generates `include_bytes!(STRING).to_vec()`.
