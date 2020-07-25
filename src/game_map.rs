use std::error::Error;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    isize, str,
};

use std::convert::TryFrom;

use crate::{
    components::{ComponentStore, Terrain},
    entitys::Entitys,
    tileset::SpriteCode,
};

#[allow(dead_code)]
pub fn load_map_file(
    map_path: &str,
) -> Result<(Vec<SpriteCode>, MapInfo), Box<dyn Error>> {
    let map_info = MapInfo {
        column_count: 20,
        row_count: 20,
        total_count: 20 * 20,
    };

    let mut sprite_map = vec![];

    let fd = File::open(map_path)?;

    let reader = BufReader::new(fd);

    for line in reader.lines() {
        let line = line?;

        for num in line.split(",") {
            let real_num = isize::from_str_radix(num, 10)?;

            let sprite_code = if real_num > 0 {
                SpriteCode::try_from(real_num as usize)?
            } else {
                SpriteCode::NoSprite
            };

            sprite_map.push(sprite_code);
        }
    }

    Ok((sprite_map, map_info))
}

pub struct RenderCell {
    pub visible: bool,
    pub sprite_code: SpriteCode,
    pub visited: bool,
    pub lit: bool,
}

#[derive(Clone)]
pub struct MapInfo {
    pub column_count: usize,
    pub row_count: usize,
    pub total_count: usize,
}

pub struct GameMap {
    pub terrain_map: Vec<usize>,
    pub render_map: Vec<RenderCell>,
    pub map_info: MapInfo,
}

impl GameMap {
    pub fn new(map_info: MapInfo) -> Self {
        Self {
            terrain_map: Vec::new(),
            render_map: Vec::new(),
            map_info,
        }
    }

    pub fn load_from_file(
        &mut self,
        components: &mut ComponentStore,
        entitys: &mut Entitys,
        map_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let (sprite_map, map_info) = load_map_file(map_path)?;

        self.map_info = map_info;

        self.init_map(components, entitys, sprite_map);

        Ok(())
    }

    pub fn init_map(
        &mut self,
        components: &mut ComponentStore,
        entitys: &mut Entitys,
        sprite_map: Vec<SpriteCode>,
    ) {
        for (index, sprite_code) in sprite_map.iter().enumerate() {
            let terrain_id = entitys.new_id();

            let mut terrain = Terrain {
                index,
                visible: false,
                sprite_code: *sprite_code,
            };

            let mut render_cell = RenderCell {
                visited: false,
                lit: false,
                visible: false,
                sprite_code: *sprite_code,
            };

            if *sprite_code != SpriteCode::NoSprite {
                terrain.visible = true;
                render_cell.visible = true;
            }

            components.terrain.insert(terrain_id, terrain);

            self.terrain_map.push(terrain_id);

            self.render_map.push(render_cell);
        }
    }
}
