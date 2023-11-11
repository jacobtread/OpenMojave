use bevy::{
    asset::{AssetLoader, LoadedAsset, ReadAssetBytesError},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::hashbrown::HashMap,
};
use binrw::{BinRead, BinReaderExt};
use bytemuck::{cast, cast_ref, cast_slice, Pod, Zeroable};
use futures::AsyncReadExt;
use std::{ffi::CStr, io::Cursor, sync::Arc};
use thiserror::Error;

#[derive(Default)]
pub struct FntLoader;

type GlyphId = usize;

#[derive(Debug, Asset, TypePath)]
pub struct BitmapFont {
    /// Texture atlas used by this font
    atlas: TextureAtlas,

    /// Glyphs for this font
    glyphs: Vec<FontGlyph>,

    /// Mapping between characters and their glyphs
    char_map: HashMap<char, GlyphId>,

    ascender: f32,
    descender: f32,

    font_size: f32,
}

#[derive(Debug, Error)]
pub enum FntError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Read(#[from] binrw::Error),

    #[error("Font name was invalid utf8")]
    InvalidName,

    #[error("Failed to read texture bytes: {0}")]
    TextureReadFailed(#[from] ReadAssetBytesError),

    #[error("Texture was malformed")]
    MalformedTexture,
}

impl AssetLoader for FntLoader {
    type Asset = BitmapFont;
    type Error = FntError;
    type Settings = ();

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let raw_font: RawBitmapFont = {
                // Read to font bytes into an internal buffer
                let mut font_bytes = Vec::new();
                reader.read_to_end(&mut font_bytes).await?;

                // Read the raw structure from the buffer
                RawBitmapFont::read_le(&mut Cursor::new(font_bytes))?
            };

            // Extract the font name as a string
            let font_name = CStr::from_bytes_until_nul(&raw_font.name)
                .ok()
                // Try to convert it to a string
                .and_then(|value| value.to_str().ok())
                // All errors result in generic invalid name
                .ok_or(FntError::InvalidName)?;

            debug!("Loading font \"{}\"", font_name);

            let (texture, texture_size): (Handle<Image>, Vec2) = {
                let texture_path = format!("textures/fonts/{}.tex", font_name.to_lowercase());
                let mut texture_bytes = load_context.read_asset_bytes(texture_path).await?;

                if texture_bytes.len() <= 8 {
                    error!("Bytes weren't long enough to be a texture");
                    return Err(FntError::MalformedTexture);
                }

                let texture_data = texture_bytes.split_off(8);

                let TexHeader { width, height } = {
                    // Extract and cast the header data
                    let mut header: [u8; 8] = [0u8; 8];
                    header.copy_from_slice(&texture_bytes[..8]);
                    cast(header)
                };

                let expected_size = (width * height * 4) as usize;

                // Ensure texture length matches
                if texture_data.len() < expected_size {
                    error!(
                        "Texture data wasn't large enough size: {} expected: {}",
                        texture_data.len(),
                        expected_size
                    );
                    return Err(FntError::MalformedTexture);
                }

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

                // Add the texture as a loaded asset
                let image: Handle<Image> = load_context.add_loaded_labeled_asset(
                    "Texture",
                    LoadedAsset::new_with_dependencies(image, None),
                );

                (image, Vec2::new(width as f32, height as f32))
            };

            let mut texture_atlas: TextureAtlas = TextureAtlas::new_empty(texture, texture_size);

            let mut glyphs = Vec::new();
            let mut char_map = HashMap::new();

            let font_size: f32 = raw_font.font_size;

            let mut ascender = font_size;
            let mut descender = 0.;

            for i in 0..256 {
                let glyph = &raw_font.data[i];
                let ascent = glyph.ascent - glyph.height;
                if ascent > ascender {
                    ascender = font_size - ascent;
                }
                if ascent < descender {
                    descender = ascent;
                }

                let p0 = Vec2::new(glyph.top_left.x, glyph.top_left.y) * texture_size;
                let p1 = Vec2::new(glyph.bottom_right.x, glyph.bottom_right.y) * texture_size;

                let texture_rect = Rect::from_corners(p0, p1);
                let glyph_texture = texture_atlas.add_texture(texture_rect);

                let size = Vec2::new(glyph.width, glyph.height);

                glyphs.push(FontGlyph {
                    glyph_texture,
                    size,
                    top: ascent,
                    advance: glyph.width + glyph.height,
                });

                char_map.insert((i as u8) as char, i);
            }

            Ok(BitmapFont {
                atlas: texture_atlas,
                glyphs,
                char_map,
                ascender,
                descender,
                font_size,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["fnt"]
    }
}

#[derive(Debug, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct TexHeader {
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub struct FontGlyph {
    /// Font texture atlas index
    glyph_texture: usize,
    /// Glyph size
    size: Vec2,
    /// Glyph y offset
    top: f32,
    /// Glyph x size
    advance: f32,
}

/// Raw format of the .fnt files
#[derive(Debug, BinRead)]
#[brw(little)]
#[repr(C)]
struct RawBitmapFont {
    /// Size of the font
    font_size: f32,
    /// Unknown field
    _u1: u32,
    /// Unknown field
    _u2: u32,
    /// Font file name (Used to get its texture)
    #[br(count = 284)]
    name: Vec<u8>,
    /// The font glyph data
    #[br(count = 256)]
    data: Vec<RawGlyphInfo>,
}

#[derive(Debug, BinRead)]
#[brw(little)]
#[repr(C)]
struct RawPoint {
    // Point x position
    x: f32,
    // Point y position
    y: f32,
}

#[derive(Debug, BinRead)]
#[brw(little)]
#[repr(C)]
struct RawGlyphInfo {
    /// Unknown field
    _u1: f32,
    /// Top left texture atlas position
    top_left: RawPoint,
    /// Top right texture atlas position
    top_right: RawPoint,
    /// Bottom left texture atlas position
    bottom_left: RawPoint,
    /// Bottom right texture atlas position
    bottom_right: RawPoint,
    /// Glyph width
    width: f32,
    /// Gglyph height
    height: f32,
    /// Unknown field
    _u2: f32,
    /// Glyph kerning
    kerning: f32,
    /// Glyph ascent
    ascent: f32,
}
