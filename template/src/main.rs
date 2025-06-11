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
    let player_sprite = registry
        .get_asset::<assets::Sprite>("PlayerSprite")
        .expect("Failed to get PlayerSprite asset");
    println!("Player Sprite: {:?}", player_sprite);
    let enemy_sprite = registry
        .get_asset::<assets::Sprite>("EnemySprite")
        .expect("Failed to get EnemySprite asset");
    println!("Enemy Sprite: {:?}", enemy_sprite);
}
