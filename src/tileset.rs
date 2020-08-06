use std::collections::HashMap;

use sdl2::{rect::Rect, render::Texture};

const EXTENDED_ASCII: &str = " ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !\"#$%&'\
()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\
[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº\
¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀\
αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■";

#[derive(Clone)]
pub struct TileInfo {
    pub total_count: u32,
    pub col_count: u32,
    pub width: u32,
    pub height: u32,
    pub orig_w: u32,
    pub orig_h: u32,
}

fn make_tile_codes(tile_map: &TileInfo) -> HashMap<char, Rect> {
    let mut tile_rects = HashMap::new();

    let row_count = tile_map.col_count as usize;

    let tile_w: i32 = tile_map.orig_w as i32;
    let tile_h: i32 = tile_map.orig_h as i32;

    let mut x: i32 = 0;
    // we start with y negative the offset to when its incremented by the
    // i % map_cols condition on the first loop y will be 0
    let mut y: i32 = 0 - tile_h;

    let new_r = Rect::new(0, 0, tile_map.orig_w, tile_map.orig_h);
    tile_rects.insert(' ', new_r);

    for (i, to_add) in EXTENDED_ASCII.chars().enumerate() {
        // we need figure out when to move to the next row and if we test
        // after we will be trying to mod an odd number
        if i % row_count == 0 {
            // reset the x axis
            x = 0;
            // move one row down
            y += tile_h;
        }

        if to_add != ' ' {
            let new_r = Rect::new(x, y, tile_map.orig_w, tile_map.orig_h);

            println!("{:?} {:?}", to_add, new_r);
            // add it to the tile array
            tile_rects.insert(to_add, new_r);
        }

        // move to the next x tile
        x += tile_w;
    }

    tile_rects
}

pub struct Tileset<'t> {
    pub texture: Texture<'t>,
    pub tile_rects: HashMap<char, Rect>,
    pub tile_info: TileInfo,
}

impl<'t> Tileset<'t> {
    pub fn new(texture: Texture<'t>, tile_info: TileInfo) -> Self {
        let tile_rects = make_tile_codes(&tile_info);

        Self {
            texture,
            tile_rects,
            tile_info,
        }
    }

    pub fn get_char(&'t self, to_get: char) -> &'t Rect {
        self.tile_rects.get(&to_get).unwrap()
    }
}
