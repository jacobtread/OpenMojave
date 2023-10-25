//! Font loader for the .fnt format

use std::{io::Seek, path::Path, sync::Arc};

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::{dbg, debug, info},
    text::{Font, FontLoader},
};

use binrw::{BinRead, BinReaderExt, BinResult, NullString};

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
            load_bitmap_font(bytes, load_context).await.unwrap();
            // fnt.chars.

            // let texture = load_context
            //     .asset_io()
            //     .load_path(Path::new("textures/fonts/glow_monofonto_large_0_lod_a.dds"))
            //     .await?;

            // let texture = Arc::from(texture);

            // let fnt_font = FntFont { font: fnt, texture };

            // // debug!("Loaded font {:?}", fnt);

            // // Ok(())
            // load_context.set_default_asset(LoadedAsset::new(fnt_font));
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
struct GlyphInfo {
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
struct BitmapFont {
    font_size: f32,
    a: i32,
    b: i32,
    #[br(count = 284)]
    name: Vec<u8>,

    #[br(count = 256)]
    data: Vec<GlyphInfo>,
}

async fn load_bitmap_font<'a>(
    bytes: &[u8],
    load_context: &'a mut bevy::asset::LoadContext<'_>,
) -> BinResult<()> {
    let mut r = std::io::Cursor::new(bytes);

    let font = BitmapFont::read(&mut r)?;

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
        .asset_io()
        .load_path(texture_path)
        .await
        .expect("Font texture missing");

    let mut r = std::io::Cursor::new(texture_bytes);

    Ok(())
}

fn load_bitmap_font_texture(bytes: &[u8]) {}

/// Transform the custom game font format into something that can be used by
/// the engine
fn transform_font() {}
