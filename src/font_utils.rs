use std::{collections::HashMap, fs::File};

use rusttype::{gpu_cache::Cache, point, Font, PositionedGlyph, Scale};

use unicode_normalization::UnicodeNormalization;

use sdl2::{
    pixels::Color,
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{Texture, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

use crate::tileset::Tileset;

const ASCII: &'static str = " !\"#$%&'()\
*+,-./0123\
456789:;<=\
>?@ABCDEFG\
HIJKLMNOPQ\
RSTUVWXYZ[\
\\]^_`abcde\
fghijklmno\
pqrstuvwxy\
z{|}~";

// fn make_ascii() {
//     for (i, not_char) in (32u8..127u8).enumerate() {
//         if i > 0 && i % 10 == 0 {
//             println!("\\");
//         }
//
//         print!("{}", not_char as char);
//     }
//     println!();
//
//     println!("{}", ASCII);
// }

pub struct RTFontCache<'tex, 'font> {
    pub font_data: Font<'font>,
    pub cache: Cache<'font>,
    pub texture: Texture<'tex>,

    font_w: u32,
    font_h: u32,

    glyphs: HashMap<char, PositionedGlyph<'font>>,
}

impl<'tex, 'font> RTFontCache<'tex, 'font> {
    pub fn new(texture_creator: &'tex TextureCreator<WindowContext>) -> Self {
        use std::io::Read;
        let scale = Scale::uniform(8.0);

        let mut glyphs = HashMap::new();

        let mut fl = File::open("assets/ttf/unscii-16-full.ttf").unwrap();

        let mut buf = Vec::new();

        fl.read_to_end(&mut buf).unwrap();

        let font_data = match Font::try_from_vec(buf) {
            Some(f_data) => f_data,
            None => {
                panic!("no font i guess");
            }
        };

        let v_metrics = font_data.v_metrics(scale);

        let mut cache = Cache::builder()
            .dimensions((v_metrics.ascent as u32 * 110) + 1, 12)
            .build();

        let mut p = point(1.0, 1.0);

        for c in ASCII.nfc() {
            let g = font_data.glyph(c).scaled(scale).positioned(p);

            cache.queue_glyph(0, g.clone());

            glyphs.insert(c, g);

            p = point(p.x + 9.0, 1.0);
        }

        let (w, h) = cache.dimensions();

        let mut surfance = Surface::new(w, h, PixelFormatEnum::RGB24).unwrap();

        surfance.set_color_mod(Color::RGB(0, 0, 0));

        let mut texture = texture_creator
            .create_texture_from_surface(surfance)
            .unwrap();

        texture.set_color_mod(200, 200, 200);

        cache
            .cache_queued(|region, data| {
                let rect =
                    Rect::new(region.min.x as i32, region.max.y as i32, 8, 8);

                texture.update(rect, data, 10).unwrap();
            })
            .unwrap();

        Self {
            font_w: 8,
            font_h: 8,
            font_data,
            cache,
            texture,
            glyphs,
        }
    }
}

impl<'tex, 'font> Tileset<'tex, '_> for RTFontCache<'tex, 'font> {
    fn get_char(&mut self, to_get: char) -> (&mut Texture<'tex>, Rect) {
        let g = self.glyphs.get(&to_get).unwrap();

        let (uv_rect, _screen_rect) =
            self.cache.rect_for(0, g).unwrap().unwrap();

        let sdl_rect = Rect::new(10, 1, 8, 8);

        (&mut self.texture, sdl_rect)
    }

    fn width(&self) -> u32 {
        self.font_w
    }

    fn height(&self) -> u32 {
        self.font_h
    }
}
