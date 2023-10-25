//! Font loader for the .fnt format

use std::{
    io::{Read, Seek},
    path::Path,
    sync::Arc,
};

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::{
        dbg, debug, info, Commands, GlobalTransform, Handle, Image, Rect, Transform, Vec2, Vec3,
    },
    reflect::{TypePath, TypeUuid},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite},
    text::{Font, FontLoader},
    utils::HashMap,
};

use binrw::{BinRead, BinReaderExt, BinResult, NullString};
use owned_ttf_parser::RawFaceTables;

use crate::loaders::tex::load_tex_asset_2d;

#[derive(Default)]
pub struct FntFontLoader;

impl AssetLoader for FntFontLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // Load bitmap font -> Convert font to otf format -> Use bevy font loader
            let image = load_bitmap_font(bytes, load_context).await.unwrap();
            // fnt.chars.

            // let texture = load_context
            //     .asset_io()
            //     .load_path(Path::new("textures/fonts/glow_monofonto_large_0_lod_a.dds"))
            //     .await?;

            // let texture = Arc::from(texture);

            // let fnt_font = FntFont { font: fnt, texture };

            // // debug!("Loaded font {:?}", fnt);

            // // Ok(())
            // load_context.set_default_asset(LoadedAsset::new(image));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["fnt"]
    }
}

#[derive(Debug, BinRead)]
#[brw(little)]
#[repr(C)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, BinRead)]
#[brw(little)]
#[repr(C)]
struct RawGlyphInfo {
    u1: f32,
    top_left: Point,
    top_right: Point,
    bottom_left: Point,
    bottom_right: Point,
    width: f32,
    height: f32,
    u2: f32,
    kerning: f32,
    ascent: f32,
}

#[derive(Debug, BinRead)]
#[br(little)]
struct RawBitmapFont {
    font_size: f32,
    a: i32,
    b: i32,
    #[br(count = 284)]
    name: Vec<u8>,

    #[br(count = 256)]
    data: Vec<RawGlyphInfo>,
}

async fn load_bitmap_font<'a>(
    bytes: &[u8],
    load_context: &'a mut bevy::asset::LoadContext<'_>,
) -> BinResult<()> {
    let mut r = std::io::Cursor::new(bytes);

    let font = RawBitmapFont::read(&mut r)?;

    let font_name = {
        let mut name_bytes = font.name;

        // Drop all null bytes
        if let Some(null_index) = name_bytes.iter().position(|value| 0.eq(value)) {
            name_bytes.truncate(null_index);
        }

        String::from_utf8(name_bytes).expect("Font name invalid")
    };

    info!("Font name: {}", font_name);

    let texture_path = format!("textures/fonts/{}.tex", font_name.to_lowercase());
    let texture_path = Path::new(&texture_path);

    let texture_bytes = load_context
        .read_asset_bytes(texture_path)
        .await
        .expect("Font texture missing");

    let ((width, height), font_image) = load_tex_asset_2d(&texture_bytes).unwrap();

    let font_image: Handle<Image> =
        load_context.set_labeled_asset("Texture", LoadedAsset::new(font_image));

    let mut atlas = TextureAtlas::new_empty(font_image, Vec2::new(width as f32, height as f32));

    let mut glyph_atlas_map = HashMap::new();
    let mut glyphs = HashMap::new();

    for i in 0..256 {
        let glyph = &font.data[i];

        let p0 = Vec2::new(glyph.top_left.x, glyph.top_left.y);
        let p1 = Vec2::new(glyph.bottom_right.x, glyph.bottom_right.y);

        info!("{:?}", p1 - p0);

        let index = atlas.add_texture(Rect::from_corners(p0, p1));

        let id = GlyphId(i as u16);

        glyph_atlas_map.insert(id, index);
        glyphs.insert(
            id,
            GlyphData {
                width: glyph.width,
                height: glyph.height,
                kerning: glyph.kerning,
                ascent: glyph.ascent,
            },
        );
    }

    let atlas: Handle<TextureAtlas> =
        load_context.set_labeled_asset("Atlas", LoadedAsset::new(atlas));

    let font_data = BitmapFontData {
        font_size: font.font_size,
        name: font_name,
        atlas,
        glyph_atlas_map,
        glyphs,
    };

    load_context.set_default_asset(LoadedAsset::new(BitmapFont {
        data: Arc::new(font_data),
    }));

    Ok(())
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphId(u16);

#[derive(Debug)]
pub struct GlyphData {
    pub width: f32,
    pub height: f32,
    pub kerning: f32,
    pub ascent: f32,
}

pub struct BitmapFontData {
    /// The size of the font
    pub font_size: f32,
    /// The font name
    pub name: String,
    /// Texture atlas
    pub atlas: Handle<TextureAtlas>,
    /// Mapping between glyphs and textures
    pub glyph_atlas_map: HashMap<GlyphId, usize>,
    /// Glyph related data
    pub glyphs: HashMap<GlyphId, GlyphData>,
}

#[derive(Clone, TypeUuid, TypePath)]
#[uuid = "bc5a01f8-bdd2-4989-9c86-ff06fb1eb1f1"]
pub struct BitmapFont {
    data: Arc<BitmapFontData>,
}

impl BitmapFont {
    pub fn spawn_text(&self, value: &str, x: f32, y: f32, commands: &mut Commands) {
        let mut offset = 0.;

        for char in value.chars() {
            let id = GlyphId(char as u16);
            let index = match self.data.glyph_atlas_map.get(&id) {
                Some(index) => index,
                None => continue,
            };

            let glyph = &self.data.glyphs[&id];

            info!("Spawned sprite {:?}", glyph);
            commands.spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(150.0, 0.0, 0.0),
                    scale: Vec3::splat(40.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(*index),
                texture_atlas: self.data.atlas.clone(),

                ..Default::default()
            });

            offset += glyph.width + 4.;
        }
    }
}
