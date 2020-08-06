use std::error::Error;

use num_enum::TryFromPrimitive;

use sdl2::{rect::Rect, render::Texture};

// TODO: this fucking bad but i dont know the problem well enough yet
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum SpriteCode {
    Building10,
    Building11,
    Building12,
    Building1,
    Building2,
    Building3,
    Building4,
    Building5,
    Building6,
    Building7,
    Building8,
    Building9,
    Charf10,
    Charf11,
    Charf12,
    Charf13,
    Charf1,
    Charf2,
    Charf3,
    Charf4,
    Charf5,
    Charf6,
    Charf7,
    Charf8,
    Charf9,
    Charm10,
    Charm11,
    Charm12,
    Charm13,
    Charm1,
    Charm2,
    Charm3,
    Charm4,
    Charm5,
    Charm6,
    Charm7,
    Charm8,
    Charm9,
    Charo1,
    Charo2,
    Charo3,
    Charo4,
    Charo5,
    Charo6,
    Charo7,
    Creature1,
    Creature2,
    Creature3,
    Creature4,
    Creature5,
    Creature6,
    Creature7,
    Creature8,
    Exploration10,
    Exploration11,
    Exploration12,
    Exploration13,
    Exploration14,
    Exploration15,
    Exploration16,
    Exploration17,
    Exploration18,
    Exploration19,
    Exploration1,
    Exploration20,
    Exploration21,
    Exploration2,
    Exploration3,
    Exploration4,
    Exploration5,
    Exploration6,
    Exploration7,
    Exploration8,
    Exploration9,
    Fauna10,
    Fauna11,
    Fauna12,
    Fauna13,
    Fauna1,
    Fauna2,
    Fauna3,
    Fauna4,
    Fauna5,
    Fauna6,
    Fauna7,
    Fauna8,
    Fauna9,
    Food10,
    Food11,
    Food12,
    Food13,
    Food14,
    Food15,
    Food16,
    Food17,
    Food18,
    Food19,
    Food1,
    Food20,
    Food21,
    Food22,
    Food2,
    Food3,
    Food4,
    Food5,
    Food6,
    Food7,
    Food8,
    Food9,
    Furniture10,
    Furniture11,
    Furniture1,
    Furniture2,
    Furniture3,
    Furniture4,
    Furniture5,
    Furniture6,
    Furniture7,
    Furniture8,
    Furniture9,
    Magic10,
    Magic11,
    Magic12,
    Magic13,
    Magic14,
    Magic15,
    Magic1,
    Magic2,
    Magic3,
    Magic4,
    Magic5,
    Magic6,
    Magic7,
    Magic8,
    Magic9,
    Outfit10,
    Outfit11,
    Outfit12,
    Outfit13,
    Outfit14,
    Outfit15,
    Outfit16,
    Outfit17,
    Outfit18,
    Outfit19,
    Outfit1,
    Outfit20,
    Outfit21,
    Outfit22,
    Outfit23,
    Outfit24,
    Outfit25,
    Outfit26,
    Outfit27,
    Outfit2,
    Outfit3,
    Outfit4,
    Outfit5,
    Outfit6,
    Outfit7,
    Outfit8,
    Outfit9,
    Overworld10,
    Overworld11,
    Overworld12,
    Overworld13,
    Overworld1,
    Overworld2,
    Overworld3,
    Overworld4,
    Overworld5,
    Overworld6,
    Overworld7,
    Overworld8,
    Overworld9,
    Symbol10,
    Symbol11,
    Symbol12,
    Symbol13,
    Symbol14,
    Symbol15,
    Symbol16,
    Symbol17,
    Symbol18,
    Symbol19,
    Symbol1,
    Symbol20,
    Symbol21,
    Symbol22,
    Symbol23,
    Symbol2,
    Symbol3,
    Symbol4,
    Symbol5,
    Symbol6,
    Symbol7,
    Symbol8,
    Symbol9,
    Trap1,
    Trap2,
    Trap3a,
    Trap3b,
    Trap4a,
    Trap4b,
    Trap5,
    Trap6,
    Trap7a,
    Trap7b,
    Trap8a,
    Trap8b,
    Troll1,
    Troll2,
    Troll3,
    Troll4,
    Unliving1,
    Unliving2,
    Unliving3,
    Unliving4,
    Unliving5,
    Unliving6,
    Unliving7,
    Unliving8,
    Wall10,
    Wall11,
    Wall12,
    Wall1,
    Wall2,
    Wall3,
    Wall4,
    Wall5,
    Wall6,
    Wall7,
    Wall8,
    Wall9,
    Cursor,
    NoSprite,
}

#[derive(Clone)]
pub struct TileInfo {
    pub total_count: u32,
    pub row_count: u32,
    pub width: u32,
    pub height: u32,
    pub orig_w: u32,
    pub orig_h: u32,
}

// TODO: init sprite_set
// fn init_texture<'t>(
//     texture_creator: &'t TextureCreator<WindowContext>,
//     tile_info: TileInfo,
//     texture_path: &str,
// ) -> Result<Tileset<'t>, Box<dyn Error>> {
//     let mut surface = Surface::from_file(texture_path)?;
//
//     surface.set_color_key(true, Color::RGB(0, 0, 0))?;
//
//     let texture = texture_creator.create_texture_from_surface(surface)?;
//
//     let tileset = Tileset::new(texture, tile_info)?;
//
//     Ok(tileset)
// }

fn make_tile_codes(tile_map: &TileInfo) -> Vec<Rect> {
    let mut tile_codes = Vec::new();

    let total_count = tile_map.total_count as usize;
    let row_count = tile_map.row_count as usize;

    let tile_w: i32 = tile_map.orig_w as i32;
    let tile_h: i32 = tile_map.orig_h as i32;

    let mut x: i32 = 0;
    // we start with y negative the offset to when its incremented by the
    // i % map_cols condition on the first loop y will be 0
    let mut y: i32 = 0 - tile_h;

    for i in 0..total_count {
        // we need figure out when to move to the next row and if we test
        // after we will be trying to mod an odd number
        if i % row_count == 0 {
            // reset the x axis
            x = 0;
            // move one row down
            y += tile_h;
        }

        let new_r = Rect::new(x, y, tile_map.orig_w, tile_map.orig_h);

        // add it to the tile array
        tile_codes.push(new_r);

        // move to the next x tile
        x += tile_w;
    }

    tile_codes
}

pub struct Tileset<'t> {
    pub texture: Texture<'t>,
    pub tile_codes: Vec<Rect>,
    pub tile_info: TileInfo,
}

impl<'t> Tileset<'t> {
    pub fn new(
        texture: Texture<'t>,
        tile_map: TileInfo,
    ) -> Result<Self, Box<dyn Error>> {
        let tile_codes = make_tile_codes(&tile_map);

        Ok(Self {
            texture,
            tile_codes,
            tile_info: tile_map,
        })
    }
}
