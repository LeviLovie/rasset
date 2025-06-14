pub fn main() {
    let path = std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get parent directory of executable")
        .join("assets.bin");
    if !path.exists() {
        panic!("Assets file not found at: {}", path.display());
    }
    let binary = std::fs::read(path).expect("Failed to read assets file");

    let registry = assets::register(binary).expect("Failed to register assets");
    let sprites = registry
        .get_asset::<assets::Sprites>("Sprites")
        .expect("Failed to get PlayerSprite asset");
    for sprite_name in &sprites.sprites {
        println!(
            "Sprite: {:?}",
            registry
                .get_asset::<assets::Sprite>(sprite_name)
                .expect("Failed to get sprite asset")
        );
    }
}
