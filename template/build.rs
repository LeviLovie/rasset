fn main() {
    let compiled_assets = assets::declare::compile_assets().expect("Failed to compile assets");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = std::path::Path::new(&out_dir).ancestors().nth(3).unwrap();
    if !target_dir.exists() {
        std::fs::create_dir_all(&out_dir).expect("Failed to create OUT_DIR");
    }
    let out_path = std::path::Path::new(&target_dir).join("assets.bin");
    std::fs::write(out_path, compiled_assets).expect("Failed to write assets to file");

    println!("cargo:rerun-if-changed=assets/");
}
