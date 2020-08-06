use std::error::Error;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    isize, str,
};

use crate::{
    components::{Ai, AiType, ComponentStore, EntitySize, Render},
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
        components: &mut ComponentStore,
        entitys: &mut Entitys,
        char_map: Vec<char>,
    ) {
        for (index, map_char) in char_map.iter().enumerate() {
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

            match map_char {
                'Z' | 'z' => {
                    let zombie_id = entitys.new_id();
                    render_cell.visible = true;

                    components.ai.insert(
                        zombie_id,
                        Ai {
                            ai_type: AiType::Basic,
                        },
                    );

                    components.render.insert(
                        zombie_id,
                        Render {
                            index,
                            size: EntitySize::Medium,
                            reper_char: 'Z',
                            visible: true,
                        },
                    );
                }

                _ => {
                    if *map_char != ' ' {
                        render_cell.terrain_char = *map_char;
                        render_cell.terrain_size = EntitySize::Medium;
                        render_cell.ent_char = *map_char;
                        render_cell.ent_size = EntitySize::Medium;

                        render_cell.visible = true;
                    }
                }
            }

            self.terrain_map.push(terrain_id);

            self.render_map.push(render_cell);
        }
    }
}
