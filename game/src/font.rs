use binrw::BinRead;
use fyrox::{
    asset::state::ResourceState,
    core::{algebra::Vector2, parking_lot::Mutex},
    fxhash::FxHashMap,
    gui::{
        draw::SharedTexture,
        image::ImageBuilder,
        ttf::{Font, FontGlyph},
        widget::WidgetBuilder,
        UserInterface,
    },
    resource::texture::{Texture, TextureKind, TexturePixelKind},
    utils::into_gui_texture,
};
use std::{io::Cursor, sync::Arc};

pub fn set_default_font(ui: &mut UserInterface) {
    let fnt_bytes = std::fs::read("DataUnpacked/textures/fonts/glow_monofonto_medium.fnt").unwrap();

    let mut r = Cursor::new(fnt_bytes);
    let font = RawBitmapFont::read(&mut r).unwrap();
    let font_name = {
        let mut name_bytes = font.name;

        // Drop all null bytes
        if let Some(null_index) = name_bytes.iter().position(|value| 0.eq(value)) {
            name_bytes.truncate(null_index);
        }

        String::from_utf8(name_bytes).expect("Font name invalid")
    };

    let texture_path = format!(
        "DataUnpacked/textures/fonts/{}.tex",
        font_name.to_lowercase()
    );

    let (tex_width, tex_height, tex_bytes) = {
        let tex_bytes = std::fs::read(texture_path).unwrap();

        // Load the texture size
        let (width, height) = {
            let header = &tex_bytes[0..8];
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
            let data = &tex_bytes[8..];
            let data_length = (width * height * 4) as usize;
            data.get(..data_length).unwrap().to_vec()
        };

        (width, height, texture_data)
    };

    let texture = Texture::from_bytes(
        TextureKind::Rectangle {
            width: tex_width,
            height: tex_height,
        },
        TexturePixelKind::RGBA8,
        tex_bytes,
        false,
    )
    .unwrap();

    let texture = SharedTexture(Arc::new(Mutex::new(ResourceState::new_ok(texture))));

    let mut glyphs = Vec::new();
    let mut char_map = FxHashMap::default();

    let mut ascender = font.font_size;
    let mut descender = 0.;

    for i in 0..256 {
        let glyph = &font.data[i];

        let mut ascent = glyph.ascent - glyph.height;

        if ascent > ascender {
            ascender = font.font_size - ascent;
        }
        if ascent < descender {
            descender = ascent;
        }

        println!("{} {}", i as u8 as char, glyph.kerning);

        glyphs.push(FontGlyph {
            left: 0.,
            top: glyph.ascent - glyph.height,
            pixels: Vec::new(),
            advance: glyph.width + glyph.kerning,
            tex_coords: [
                Vector2::new(glyph.top_left.x, glyph.top_left.y),
                Vector2::new(glyph.top_right.x, glyph.top_right.y),
                Vector2::new(glyph.bottom_right.x, glyph.bottom_right.y),
                Vector2::new(glyph.bottom_left.x, glyph.bottom_left.y),
            ],
            bitmap_width: glyph.width as usize,
            bitmap_height: glyph.height as usize,
        });

        char_map.insert(i as u32, i);
    }

    let mut nf = Font {
        height: font.font_size,
        glyphs,
        ascender,
        descender,
        char_map,
        atlas: Vec::new(),
        atlas_size: 0,
        texture: Some(texture),
    };

    // Set as default font.
    ui.default_font.set(nf);
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
