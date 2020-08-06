use sdl2::{rect::Rect, render::Texture};

pub trait Tileset<'tex, 'font> {
    fn get_char(&mut self, to_get: char) -> (&mut Texture<'tex>, Rect);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
