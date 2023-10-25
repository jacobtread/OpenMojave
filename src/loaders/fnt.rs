//! Font loader for the .fnt format

use std::{path::Path, sync::Arc};

use ab_glyph::{v2::GlyphImage, Point};
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::{dbg, debug},
    text::{Font, FontLoader},
};
use bmfont_parser::BMFont;

#[derive(Default)]
pub struct FntFontLoader;

impl AssetLoader for FntFontLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let fnt = BMFont::from_loaded(
                &bmfont_parser::Format::BMFont,
                String::from_utf8_lossy(bytes),
                &["textures/fonts/glow_monofonto_large_0_lod_a.dds"],
            )
            .unwrap();

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

// pub struct FntFont {
//     font: BMFont,
//     texture: Arc<[u8]>,
// }

// impl ab_glyph::Font for FntFont {
//     fn units_per_em(&self) -> Option<f32> {
//         todo!()
//     }

//     fn ascent_unscaled(&self) -> f32 {
//         todo!()
//     }

//     fn descent_unscaled(&self) -> f32 {
//         todo!()
//     }

//     fn line_gap_unscaled(&self) -> f32 {
//         todo!()
//     }

//     fn glyph_id(&self, c: char) -> ab_glyph::GlyphId {
//         todo!()
//     }

//     fn h_advance_unscaled(&self, id: ab_glyph::GlyphId) -> f32 {
//         todo!()
//     }

//     fn h_side_bearing_unscaled(&self, id: ab_glyph::GlyphId) -> f32 {
//         todo!()
//     }

//     fn v_advance_unscaled(&self, id: ab_glyph::GlyphId) -> f32 {
//         todo!()
//     }

//     fn v_side_bearing_unscaled(&self, id: ab_glyph::GlyphId) -> f32 {
//         todo!()
//     }

//     fn kern_unscaled(&self, first: ab_glyph::GlyphId, second: ab_glyph::GlyphId) -> f32 {
//         todo!()
//     }

//     fn outline(&self, id: ab_glyph::GlyphId) -> Option<ab_glyph::Outline> {
//         todo!()
//     }

//     fn glyph_count(&self) -> usize {
//         todo!()
//     }

//     fn codepoint_ids(&self) -> ab_glyph::CodepointIdIter<'_> {
//         todo!()
//     }

//     fn glyph_raster_image2(
//         &self,
//         id: ab_glyph::GlyphId,
//         pixel_size: u16,
//     ) -> Option<ab_glyph::v2::GlyphImage> {
//         GlyphImage::Some(GlyphImage {
//             origin: Point { x: 0., y: 0. },
//             width: 32,
//             height: 32,
//             pixels_per_em: 32,
//             data: &self.texture,
//             format: ab_glyph::GlyphImageFormat::BitmapPremulBgra32,
//         })
//     }
// }
