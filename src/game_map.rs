// components::{Ai, AiType, ComponentStore, EntitySize, Render},

use crate::{
    components::{ComponentStore, EntitySize},
    entitys::Entitys,
};

pub struct RenderCell {
    pub lit: bool,
    pub visible: bool,
    pub visited: bool,
    pub ent_size: EntitySize,
    pub ent_char: char,
    pub terrain_size: EntitySize,
    pub terrain_char: char,
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

    pub fn init_map(
        &mut self,
        _components: &mut ComponentStore,
        entitys: &mut Entitys,
        char_map: Vec<char>,
    ) {
        for map_char in char_map.iter() {
            let terrain_id = entitys.new_id();

            let mut render_cell = RenderCell {
                visited: false,
                lit: false,
                visible: false,
                ent_size: EntitySize::Nothing,
                terrain_size: EntitySize::Nothing,
                ent_char: ' ',
                terrain_char: ' ',
            };

            if *map_char != ' ' {
                render_cell.terrain_char = *map_char;
                render_cell.terrain_size = EntitySize::Medium;
                render_cell.ent_char = *map_char;
                render_cell.ent_size = EntitySize::Medium;

                render_cell.visible = true;
            }

            self.terrain_map.push(terrain_id);

            self.render_map.push(render_cell);
        }
    }
}
