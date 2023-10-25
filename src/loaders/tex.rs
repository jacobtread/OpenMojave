use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::{info, Image},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

/// Texture asset loader
#[derive(Default)]
pub struct TexLoader;

impl AssetLoader for TexLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let (_, image) = load_tex_asset_2d(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(image));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tex"]
    }
}

/// Attempts to load a texture asset from the provided bytes
pub fn load_tex_asset_2d(bytes: &[u8]) -> Result<((u32, u32), Image), bevy::asset::Error> {
    // Buffer is too small to be a texture
    if bytes.len() < 8 {
        return Err(bevy::asset::Error::msg("Invalid texture file"));
    }

    // Load the texture size
    let (width, height) = {
        let header = &bytes[0..8];
        // Load the texture width
        let mut width_bytes = [0u8; 4];
        width_bytes.copy_from_slice(&header[0..4]);

        // Load the texture height
        let mut height_bytes = [0u8; 4];
        height_bytes.copy_from_slice(&header[4..8]);

        (
            u32::from_le_bytes(width_bytes),
            u32::from_le_bytes(height_bytes),
        )
    };

    // Load the texture data
    let texture_data = {
        let data = &bytes[8..];
        let data_length = (width * height * 4) as usize;
        data.get(..data_length)
            .ok_or(bevy::asset::Error::msg("Texture data was malformed"))?
            .to_vec()
    };

    // Create the texture image
    let image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texture_data,
        TextureFormat::Rgba8Unorm,
    );

    Ok(((width, height), image))
}
