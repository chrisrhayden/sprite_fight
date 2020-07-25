use std::error::Error;

use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::{
    scenes::Scene,
    tileset::{SpriteCode, Tileset},
};

pub fn render_tile(
    canvas: &mut Canvas<Window>,
    tileset: &Tileset,
    code: SpriteCode,
    dest: &Rect,
) -> Result<(), Box<dyn Error>> {
    let src_r = &tileset.tile_codes[code as usize];

    canvas.copy(&tileset.texture, src_r.to_owned(), dest.to_owned())?;

    Ok(())
}
