use rasset::prelude::*;

#[derive(Debug, Clone, Encode, Decode)]
struct SpriteAsset {
    size: (u32, u32),
    texture: String,
}

impl Asset for SpriteAsset {
    fn get_type(&self) -> Type {
        Type(std::any::TypeId::of::<SpriteAsset>())
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        bincode::encode_to_vec(self, bincode::config::standard())
            .map_err(|e| Error::Serialization(format!("Failed to serialize SpriteAsset: {}", e)))
    }

    fn from_bytes(bytes: &[u8]) -> Result<SpriteAsset, Error> {
        bincode::decode_from_slice(bytes, bincode::config::standard())
            .map_err(|e| {
                Error::Deserialization(format!("Failed to deserialize SpriteAsset: {}", e))
            })
            .map(|(asset, _)| asset)
    }
}

fn main() -> Result<(), Error> {
    let binary = {
        let mut compiler = Compiler::new();

        let sprite = SpriteAsset {
            size: (64, 64),
            texture: "path/to/texture.png".to_string(),
        };
        compiler.add_asset(Box::new(sprite));

        let result = compiler.compile()?;
        println!("Compiled assets: {:?}", result);

        result
    };

    {
        let registry = Registry::builder()
            .reg_type::<SpriteAsset>()
            .load(&binary)?;
        println!("Loaded registry with {} assets", registry.amount());
        println!("Asset: {:?}", registry.get_asset::<SpriteAsset>());
    };

    Ok(())
}
